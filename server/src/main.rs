use cassandra_cpp::*;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys, rsa_private_keys};
use std::{env, fs::File, io::BufReader};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!(
            "Usage: {} <secure connect bundle zip> <username> <password>",
            args[0]
        );
        return;
    }

    let secure_connection_bundle = &args[1];
    let username = &args[2];
    let password = &args[3];

    let mut cluster = Cluster::default();
    cluster
        .set_cloud_secure_connection_bundle(secure_connection_bundle)
        .unwrap();
    cluster.set_credentials(username, password).unwrap();

    // How long to wait before attempting to reconnect after connection failure or disconnect: 100 ms
    cluster.set_reconnect_wait_time(100);
    //  load balancing policy:
    cluster.set_load_balance_round_robin();

    let session = cluster.connect().unwrap();
    let statement = stmt!("SELECT release_version FROM system.local");
    let result = session.execute(&statement).wait().unwrap();
    let row = result.first_row().unwrap();
    let version: String = row.get_by_name("release_version").unwrap();
    println!("release_version: {version}");
}

fn load_config() {
    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());
    // convert files to key/cert objects
    let cert_chain: Vec<rustls::Certificate> = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();

    // rsa
    let mut keys: Vec<rustls::PrivateKey> = rsa_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();
}
