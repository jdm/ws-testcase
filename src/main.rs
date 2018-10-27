// To reproduce:
// $ pip3 install --user websockets
// $ cd signalling
// $ ./generate_cert.sh
// $ ./simple-server.py
// in a new shell:
// $ cargo run

extern crate env_logger;
extern crate hyper;

fn main() {
    env_logger::init();
    let server = "http://localhost:8443";
    let client = hyper::Client::new();
    let _ = client.get(server).send().unwrap();
}
