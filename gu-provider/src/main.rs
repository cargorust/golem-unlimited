extern crate futures;
extern crate tokio;

extern crate actix;
extern crate actix_web;
extern crate clap;
extern crate gu_actix;
extern crate gu_base;
extern crate gu_p2p;
extern crate gu_persist;
extern crate gu_lan;
extern crate gu_ethkey;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

//#[macro_use]
//extern crate error_chain;
extern crate directories;

extern crate env_logger;
#[macro_use]
extern crate log;

extern crate bytes;
extern crate flate2;
extern crate rand;
extern crate tar;
extern crate uuid;
extern crate mdns;

mod hdman;
mod server;
mod write_to;

const VERSION: &str = env!("CARGO_PKG_VERSION");

use clap::App;
use gu_base::*;

fn main() {
    GuApp(|| App::new("Golem Unlimited Provider").version(VERSION)).run(
        LogModule
            .chain(AutocompleteModule::new())
            .chain(gu_persist::config::ConfigModule::new())
            .chain(gu_lan::rest_client::LanModule)
            .chain(server::ServerModule::new()),
    );
}
