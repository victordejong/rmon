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
            hostname TEST PRIMARY KEY,
            cores INTEGER NOT NULL,
            vendor_id TEXT NOT NULL,
            model_name TEXT NOT NULL,
            ram_total INTEGER NOT NULL,
            swap_total INTEGER NOT NULL
         )",
        (),
    )
    .unwrap();

    return conn;
}

pub fn insert_host_fact(conn: &mut Connection, request: &Request<HostFactsMessage>) {
    let request_message: &HostFactsMessage = request.get_ref();

    let tx = conn.transaction().unwrap();

    // TODO: insert disk facts
    tx.execute(
        "INSERT OR REPLACE INTO
        host_metrics
        (hostname, cores, vendor_id, model_name, ram_total, swap_total)
        VALUES
        (?1, ?2, ?3, ?4, ?5, ?6)",
        [
            &request_message.system.as_ref().unwrap().hostname,
            &request_message.cpu.as_ref().unwrap().cores.to_string(),
            &request_message.cpu.as_ref().unwrap().vendor_id,
            &request_message.cpu.as_ref().unwrap().model_name,
            &request_message.mem.as_ref().unwrap().ram_total.to_string(),
            &request_message.mem.as_ref().unwrap().swap_total.to_string(),
        ],
    )
    .unwrap();

    tx.commit().unwrap();
}
