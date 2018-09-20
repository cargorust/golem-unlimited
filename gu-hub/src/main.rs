extern crate futures;
extern crate tokio;

extern crate actix;
extern crate actix_web;
extern crate clap;
extern crate gu_actix;
extern crate gu_lan;
extern crate gu_p2p;
extern crate gu_persist;
extern crate tokio_uds;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

extern crate directories;

#[macro_use]
extern crate failure;

extern crate gu_base;
extern crate mdns;
extern crate rand;
extern crate semver;

#[macro_use]
extern crate prettytable;

extern crate flate2;

extern crate gu_hardware;

use clap::App;
use gu_base::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

mod peer;
mod plugin;
mod server;

fn main() {
    GuApp(|| App::new("Golem Unlimited").version(VERSION)).run(
        LogModule
            .chain(AutocompleteModule::new())
            .chain(gu_persist::config::ConfigModule::new())
            .chain(gu_lan::rest_client::LanModule)
            .chain(plugin::PluginManager::default())
            .chain(peer::PeerModule::new())
            .chain(gu_hardware::module())
            .chain(server::ServerModule::new()),
    );
}
