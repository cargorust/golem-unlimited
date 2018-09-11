extern crate futures;
extern crate tokio;

extern crate actix;
extern crate actix_web;
extern crate clap;
extern crate gu_actix;
extern crate gu_p2p;
extern crate gu_persist;
extern crate gu_lan;
extern crate tokio_uds;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

#[macro_use]
extern crate error_chain;
extern crate directories;

extern crate mdns;
extern crate rand;
extern crate gu_base;

extern crate env_logger;

use clap::{App, Arg, SubCommand};
use gu_base::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

mod lan;
mod server;
mod peer;

fn main() {

    GuApp(||App::new("Golem Unlimited")
        .version(VERSION))
        .run( LogModule
            .chain(gu_persist::config::ConfigModule)
            .chain(lan::LanModule)
            .chain(server::ServerModule::new())
            .chain(peer::PeerModule)
            .chain(CompleteModule::new()));

}
