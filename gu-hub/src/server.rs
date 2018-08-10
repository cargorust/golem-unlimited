use actix::fut;
use actix::prelude::*;
use futures::future;
use futures::prelude::*;
use tokio;

use super::config;
use actix_web::server::HttpServer;
use actix_web::server::StopServer;
use actix_web::*;
use clap::{self, ArgMatches, SubCommand};
use std::borrow::Cow;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use tokio_uds::UnixListener;

#[derive(Serialize, Deserialize, Default)]
struct ServerConfig {
    p2p_port: Option<u16>,
}

impl ServerConfig {
    fn p2p_addr(&self) -> impl ToSocketAddrs {
        ("0.0.0.0", self.p2p_port.unwrap_or(61622))
    }
}

impl config::HasSectionId for ServerConfig {
    const SECTION_ID: &'static str = "server-cfg";
}

pub fn clap_declare<'a, 'b>() -> clap::App<'a, 'b> {
    SubCommand::with_name("server")
}

pub fn clap_match(m: &ArgMatches) {
    if let Some(m) = m.subcommand_matches("server") {
        println!("server");
        run_server();
    }
}

fn run_server() {
    use actix;

    let sys = actix::System::new("gu-hub");

    let config = ServerConfigurer(None).start();
    /*
    let listener = UnixListener::bind("/tmp/gu.socket").expect("bind failed");
    server::new(|| {
        App::new()
            // enable logger
            .middleware(middleware::Logger::default())
            .resource("/index.html", |r| r.f(|_| "Hello world!"))
    }).start_incoming(listener.incoming(), false);
*/
    println!("[[sys");
    let _ = sys.run();
    println!("sys]]");
}

fn p2p_server(r: &HttpRequest) -> &'static str {
    "ok"
}

struct ServerConfigurer(Option<Recipient<StopServer>>);

impl Actor for ServerConfigurer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut <Self as Actor>::Context) {
        let config = config::ConfigManager::from_registry();

        ctx.spawn(
            config
                .send(config::GetConfig::new())
                .map_err(|e| config::Error::from(e))
                .and_then(|r| r)
                .map_err(|_e| ())
                .and_then(|c: Arc<ServerConfig>| {
                    let server = server::new(move || App::new().handler("/p2p", p2p_server));
                    let s = server.bind(c.p2p_addr()).unwrap().start();

                    //act.0 = Some(s.recipient());
                    //fut::ok(())
                    Ok(())
                })
                .into_actor(self)
                .and_then(|_, _, ctx| fut::ok(ctx.stop())),
        );
        println!("configured");
    }
}

impl Drop for ServerConfigurer {
    fn drop(&mut self) {
        println!("drop")
    }
}