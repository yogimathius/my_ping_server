use my_ping_server::server::Server;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "server")]
struct Opt {
    /// Host to bind the server to
    #[structopt(short, long, default_value = "127.0.0.1")]
    host: String,

    /// Port to bind the server to
    #[structopt(short, long, default_value = "42422")]
    port: u16,
}

fn main() {
    let opt = Opt::from_args();

    let server = Server::new(&opt.host, opt.port);

    server.run();
}
