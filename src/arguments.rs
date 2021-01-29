use bcrypt::{hash, DEFAULT_COST};
use clap::{crate_authors, crate_description, crate_name, App, Arg, ArgMatches};
use log::{error, warn};
use std::env;
use std::net::{IpAddr, SocketAddr};
use std::process::exit;
use std::str::FromStr;

use shadow_rs::shadow;

shadow!(build);

#[derive(Debug)]
pub struct Arguments {
    pub addr: SocketAddr,
    pub user: String,
    pub password_hash: String,
}

pub fn parse() -> Arguments {
    let matches = App::new(crate_name!())
        .version(build::clap_version().as_str())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("Port to listen on, can also be set via env variable 'AUTHPRX_PORT'")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("user")
                .short("u")
                .long("user")
                .value_name("USER")
                .help("Superuser name, can also be set via env variable 'AUTHPRX_USER'")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("password")
                .long("password")
                .value_name("PASSWORD")
                .help("Superuser password, can also be set via env variable 'AUTHPRX_PASSWORD'")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("address")
                .long("address")
                .short("a")
                .value_name("ADDRESS")
                .help("address to bind to, can also be set via env variable 'AUTHPRX_ADDRESS'")
                .takes_value(true),
        )
        .get_matches();

    Arguments {
        addr: SocketAddr::from((get_address(&matches), get_port(&matches))),
        user: get_user(&matches),
        password_hash: get_password(&matches),
    }
}

fn get_address(matches: &ArgMatches) -> IpAddr {
    let mut addr = env::var("AUTHPRX_ADDRESS")
        .ok()
        .map(|s| IpAddr::from_str(&s).ok())
        .flatten();

    if let Some(a) = matches
        .value_of("address")
        .map(|s| IpAddr::from_str(s).ok())
        .flatten()
    {
        addr = Some(a);
    }

    if let Some(a) = addr {
        a
    } else {
        warn!("no address given, using 0.0.0.0");
        IpAddr::from_str("0.0.0.0").unwrap()
    }
}

fn get_port(matches: &ArgMatches) -> u16 {
    let mut port = env::var("AUTHPRX_PORT")
        .ok()
        .map(|p| u16::from_str(&p).ok())
        .flatten();

    if let Some(p) = matches
        .value_of("port")
        .map(|p| u16::from_str(p).ok())
        .flatten()
    {
        port = Some(p);
    }

    if let Some(p) = port {
        p
    } else {
        warn!("no port given, using 80");
        80
    }
}

fn get_user(matches: &ArgMatches) -> String {
    let mut user = env::var("AUTHPRX_USER").ok();
    if let Some(u) = matches.value_of("user").map(|p| p.to_string()) {
        user = Some(u);
    }

    if let Some(user) = user {
        user
    } else {
        error!("no user given, exiting...");
        exit(1);
    }
}

fn get_password(matches: &ArgMatches) -> String {
    let mut password = env::var("AUTHPRX_PASSWORD").ok();
    if let Some(p) = matches.value_of("password").map(|p| p.to_string()) {
        password = Some(p);
    }

    if let Some(password) = password {
        hash(password.clone(), DEFAULT_COST).unwrap()
    } else {
        error!("no password given, exiting...");
        exit(1);
    }
}
