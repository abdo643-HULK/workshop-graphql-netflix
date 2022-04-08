use cassandra_cpp::*;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys, rsa_private_keys};
use serde::Deserialize;
use std::{env, fs::File, io::BufReader};

mod queries {
    use cassandra_cpp::{
        stmt, BindRustType, CassCollection, CassResult, Error, SchemaMeta, Session, Statement, Uuid,
    };

    use crate::Movie;
    #[allow(dead_code)]
    pub async fn get_movies_by_genre(session: &Session, genre: &str) -> Result<CassResult, Error> {
        let mut statement = stmt!("SELECT * FROM movies.details WHERE genre=?;");
        let res = statement.bind(0, genre);
        if res.is_err() {
            panic!("ERROR binding getMoviesByGenre statement");
        }

        return session.execute(&statement).await;
    }

    pub fn insert_into_movies(schema_meta: &SchemaMeta, uuid: Uuid, movie: &Movie) -> Statement {
        let key_space = schema_meta.get_keyspace_by_name("movie");
        let meta_data_type = key_space.user_type_by_name("metadata").unwrap();

        let Movie {
            duration,
            release_year,
            genre,
            title,
            synopsis,
            ..
        } = movie;

        let mut statement =
            stmt!("INSERT INTO movie.movies (id, title, is_original, synopsis, metadata) VALUES(?, ?, ?, ?, ?);");
        let movie_id = uuid;
        statement.bind_uuid(0, movie_id).unwrap();
        statement.bind_string(1, title.as_str()).unwrap();
        statement.bind_bool(2, false).unwrap();
        statement.bind_string(3, synopsis.as_str()).unwrap();

        let mut meta_data = meta_data_type.new_user_type();
        meta_data
            .set_int16_by_name("release_year", (*release_year).try_into().unwrap())
            .unwrap();
        meta_data
            .set_int64_by_name("duration", (*duration).into())
            .unwrap();
        let mut genres = cassandra_cpp::Set::new();
        genres.append_string(&genre).unwrap();
        meta_data.set_set_by_name("genres", genres).unwrap();

        statement.bind_user_type(4, &meta_data).unwrap();

        return statement;
    }

    pub fn insert_into_movies_by_genre(
        schema_meta: &SchemaMeta,
        uuid: Uuid,
        movie: &Movie,
    ) -> Statement {
        let key_space = schema_meta.get_keyspace_by_name("movie");
        let meta_data_type = key_space.user_type_by_name("metadata").unwrap();

        let Movie {
            duration,
            release_year,
            genre,
            title,
            ..
        } = movie;

        let mut statement = stmt!(
            "INSERT INTO movie.movies_by_genre (movie_id, genre, is_original, title, metadata) VALUES(?, ?, ?, ?, ?);"
        );

        statement.bind_uuid(0, uuid).unwrap();
        statement.bind_string(1, genre.as_str()).unwrap();
        statement.bind_bool(2, false).unwrap();
        statement.bind_string(3, title.as_str()).unwrap();

        let mut meta_data = meta_data_type.new_user_type();
        meta_data
            .set_int16_by_name("release_year", (*release_year).try_into().unwrap())
            .unwrap();
        meta_data
            .set_int64_by_name("duration", (*duration).into())
            .unwrap();
        let mut genres = cassandra_cpp::Set::new();
        genres.append_string(&genre).unwrap();
        meta_data.set_set_by_name("genres", genres).unwrap();

        statement.bind_user_type(4, &meta_data).unwrap();

        return statement;
    }
}

#[derive(Debug, Deserialize)]
pub struct Movie {
    genre: String,
    title: String,
    trailer: String,
    release_year: u16,
    synopsis: String,
    duration: u16,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <username> <password>", args[0]);
        return;
    }

    // let secure_connection_bundle = &args[1];
    let username = &args[1];
    let password = &args[2];

    let mut cluster = Cluster::default();
    cluster.set_contact_points("127.0.0.1").unwrap();
    cluster.set_credentials(username, password).unwrap();
    // How long to wait before attempting to reconnect after connection failure or disconnect: 100 ms
    cluster.set_reconnect_wait_time(100);
    // load balancing policy:
    cluster.set_load_balance_round_robin();

    let session = cluster.connect().expect("CONNECTION ERROR:");

    if args.len() > 3 {
        let mut rdr = csv::Reader::from_path(
            "/Users/abdo/projects/FH/DAB/workshop-graphql-netflix/data/movies_by_genre.csv",
        )
        .unwrap();

        // session.connect_keyspace(&cluster, "movie").unwrap().await;
        let schema_meta = session.get_schema_meta();
        let uuid_generator = cassandra_cpp::UuidGen::default();

        for result in rdr.deserialize() {
            let mut batch = Batch::new(BatchType::LOGGED);
            let movie: Movie = result.unwrap();
            let movie_id = uuid_generator.gen_random();
            let statement = queries::insert_into_movies(&schema_meta, movie_id, &movie);
            batch.add_statement(&statement).unwrap();
            let statement = queries::insert_into_movies_by_genre(&schema_meta, movie_id, &movie);
            batch.add_statement(&statement).unwrap();
            session.execute_batch(&batch).await.unwrap();
        }
    }

    let statement = stmt!("SELECT * FROM movie.movies LIMIT 10;");
    let res = session.execute(&statement).await.unwrap();
    println!("{:?}", res.row_count());
    res.iter().for_each(|row| println!("{}", row.to_string()));

    let statement = stmt!("SELECT * FROM movie.movies_by_genre LIMIT 10;");
    let res = session.execute(&statement).await.unwrap();
    println!("{:?}", res.row_count());
    res.iter().for_each(|row| println!("{}", row.to_string()));
}

#[allow(dead_code)]
fn load_config() {
    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());
    // convert files to key/cert objects
    #[allow(dead_code)]
    let cert_chain: Vec<rustls::Certificate> = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();

    // rsa
    #[allow(dead_code)]
    let mut keys: Vec<rustls::PrivateKey> = rsa_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();
}
