use crate::error::{Error, Result};
use bincode;
use log::{error, warn};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::process::exit;

static DB: Lazy<sled::Db> = Lazy::new(|| match sled::open("auth_proxy") {
    Ok(db) => db,
    _ => {
        error!("failed to open db");
        exit(1);
    }
});

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Api {
    client_limit: u16,
    protected_paths: Vec<String>,
    unprotected_paths: Vec<String>,
}

pub fn get_api(name: &str) -> Result<Option<Api>> {
    match DB.get(name.as_bytes()) {
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

pub async fn new_api(name: &str, api: &Api) -> Result<()> {
    let data = bincode::serialize(api).map_err(|e| Error::Bincode(e))?;
    match DB.get(name.as_bytes()) {
        Ok(None) => {}
        Ok(Some(_)) => return Err(Error::ApiAlreadyExists(String::from(name))),
        Err(e) => {
            return Err(Error::Db(e));
        }
    }
    match DB.insert(name.as_bytes(), data) {
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
    DB.flush_async().await.map_err(|e| Error::Db(e))?;
    Ok(())
}
