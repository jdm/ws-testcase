// To reproduce:
// $ pip3 install --user websockets
// $ cd signalling
// $ ./generate_cert.sh
// $ ./simple-server.py
// in a new shell:
// $ cargo run

extern crate websocket;

fn main() {
    let server = "https://localhost:8443";
    let client = match websocket::client::ClientBuilder::new(server)
        .unwrap()
        .connect_insecure()
    {
        Ok(client) => println!("connected ok"),
        Err(err) => {
            println!("Failed to connect to {} with error: {:?}", server, err);
            panic!("uh oh");
        }
    };
}
