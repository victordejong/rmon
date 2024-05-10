use rusqlite::Connection;
use rusqlite::config::DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY;
use chrono::Local;


use crate::host_facts_protobuf;
use crate::live_metrics_protobuf;
use host_facts_protobuf::HostFactsMessage;
use live_metrics_protobuf::LiveMetricsMessage;
use tonic::Request;

pub fn initialise_database(path: String) -> Connection {
    let mut db_name: String = String::from(&path);
    db_name.push_str("/");
    db_name.push_str("database.db");
    let conn = Connection::open(&db_name).unwrap();

    conn.set_db_config(SQLITE_DBCONFIG_ENABLE_FKEY, true).unwrap();

    conn.execute(
        "create table if not exists host_facts (
            hostname TEXT PRIMARY KEY,
            cores INTEGER NOT NULL,
            vendor_id TEXT NOT NULL,
            model_name TEXT NOT NULL,
            ram_total INTEGER NOT NULL,
            swap_total INTEGER NOT NULL
         )",
        (),
    )
    .unwrap();

    conn.execute(
        "create table if not exists live_metrics (
            timestamp TEXT PRIMARY KEY,
            hostname TEXT,
            cpu_freq REAL NOT NULL,
            cpu_load_1m REAL NOT NULL,
            cpu_load_5m REAL NOT NULL,
            cpu_load_15m REAL NOT NULL,
            mem_free INTEGER NOT NULL,
            mem_avail INTEGER NOT NULL,
            mem_cached INTEGER NOT NULL,
            FOREIGN KEY (hostname) REFERENCES host_facts (hostname)
         )",
        (),
    )
    .unwrap();

    conn.execute(
        "CREATE INDEX IF NOT EXISTS hostname ON host_facts(hostname)",
        (),
    )
    .unwrap();

    conn.execute(
        "CREATE INDEX IF NOT EXISTS timestamp ON live_metrics(timestamp)",
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
        host_facts
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

pub fn insert_live_metrics(conn: &mut Connection, request: &Request<LiveMetricsMessage>) {
    let request_message: &LiveMetricsMessage = request.get_ref();

    let tx = conn.transaction().unwrap();

    // TODO: insert disk facts
    tx.execute(
        "INSERT OR REPLACE INTO
        live_metrics
        (timestamp, hostname, cpu_freq, cpu_load_1m, cpu_load_5m, cpu_load_15m, mem_free, mem_avail, mem_cached)
        VALUES
        (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        [
            &Local::now().timestamp().to_string(),
            &request_message.hostname,
            &request_message.cpu.as_ref().unwrap().freq.to_string(),
            &request_message.cpu.as_ref().unwrap().cpu_load.as_ref().unwrap().load_1m.to_string(),
            &request_message.cpu.as_ref().unwrap().cpu_load.as_ref().unwrap().load_5m.to_string(),
            &request_message.cpu.as_ref().unwrap().cpu_load.as_ref().unwrap().load_15m.to_string(),
            &request_message.mem.as_ref().unwrap().free.to_string(),
            &request_message.mem.as_ref().unwrap().available.to_string(),
            &request_message.mem.as_ref().unwrap().cached.to_string(),
        ],
    )
    .unwrap();

    tx.commit().unwrap();
}
