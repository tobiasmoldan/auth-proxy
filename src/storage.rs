use crate::error::{Error, Result};
use bincode;
use log::{error, warn};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::process::exit;

static DB: OnceCell<Storage> = OnceCell::new();

pub fn init() -> Result<()> {
    DB.set(Storage::default()).map_err(|_| Error::CellSetError)
}

pub fn get() -> Option<&'static Storage> {
    DB.get()
}

pub struct Storage {
    db: sled::Db,
}

impl Storage {
    pub fn get_api(&self, name: &str) -> Result<Option<Api>> {
        match self.db.get(name.as_bytes()) {
            Ok(data) => {
                if let Some(data) = data {
                    match bincode::deserialize(&data) {
                        Ok(api) => {
                            let api: Api = api;
                            Ok(Some(api))
                        }
                        Err(e) => Err(Error::Bincode(e)),
                    }
                } else {
                    Ok(None)
                }
            }
            Err(e) => Err(Error::Db(e)),
        }
    }

    pub async fn new_api(&self, name: &str, api: &Api) -> Result<()> {
        let data = bincode::serialize(api).map_err(|e| Error::Bincode(e))?;
        match self.db.get(name.as_bytes()) {
            Ok(None) => {}
            Ok(Some(_)) => return Err(Error::ApiAlreadyExists(String::from(name))),
            Err(e) => {
                return Err(Error::Db(e));
            }
        }
        match self.db.insert(name.as_bytes(), data) {
            Ok(None) => {}
            Ok(Some(existing)) => match bincode::deserialize(&existing) {
                Err(_) => warn!("existing api not deserializable on insert of new"),
                Ok(api) => {
                    let api: Api = api;
                    warn!("existing api {}: {:?} will be overwritten", name, api);
                }
            },
            Err(e) => {
                return Err(Error::Db(e));
            }
        }
        self.db.flush_async().await.map_err(|e| Error::Db(e))?;
        Ok(())
    }
}

impl Default for Storage {
    fn default() -> Self {
        Storage {
            db: match sled::open("auth_proxy") {
                Ok(db) => db,
                _ => {
                    error!("failed to open db");
                    exit(1);
                }
            },
        }
    }
}

impl From<sled::Db> for Storage {
    fn from(db: sled::Db) -> Self {
        Storage { db }
    }
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct Api {
    client_limit: u16,
    protected_paths: Vec<String>,
    unprotected_paths: Vec<String>,
}

mod test {
    use super::Error;
    use super::{Api, Storage};

    fn temp_storage() -> Storage {
        let db = sled::Config::new().temporary(true).open().unwrap();
        Storage::from(db)
    }

    #[test]
    fn get_api_no_api() {
        let storage = temp_storage();
        assert!(storage.get_api("not existing").unwrap().is_none());
    }

    #[tokio::test]
    async fn new_api() {
        let storage = temp_storage();

        assert!(storage.get_api("api name").unwrap().is_none());

        let api = Api::default();

        storage.new_api("api name", &api).await.unwrap();

        assert_eq!(api, storage.get_api("api name").unwrap().unwrap());
    }

    #[tokio::test]
    async fn new_api_no_override() {
        let storage = temp_storage();

        let api = Api::default();

        storage.new_api("api name", &api).await.unwrap();

        if let Error::ApiAlreadyExists(name) = storage.new_api("api name", &api).await.unwrap_err()
        {
            assert_eq!("api name", name);
        } else {
            assert!(false);
        }
    }
}
