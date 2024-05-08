use rusqlite::Connection;

use crate::host_facts_protobuf;
use host_facts_protobuf::HostFactsMessage;
use tonic::Request;

pub fn initialise_database(path: String) -> Connection {
    let mut db_name: String = String::from(&path);
    db_name.push_str("database.db");
    let conn = Connection::open(&db_name).unwrap();

    conn.execute(
        "create table if not exists host_metrics (
             name text primary key
         )",
        (),
    )
    .unwrap();

    return conn;
}

pub fn insert_host_fact(conn: &mut Connection, request: &Request<HostFactsMessage>) {
    let request_message: &HostFactsMessage = request.get_ref();

    let tx = conn.transaction().unwrap();

    tx.execute(
        "INSERT INTO host_metrics (name) values (?1)",
        [&request_message.system.as_ref().unwrap().hostname],
    )
    .unwrap();

    tx.commit().unwrap();
}
