// src/db.rs
use rusqlite::{Connection, params};
use std::path::Path;

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn new<P: AsRef<Path>>(db_path: P) -> rusqlite::Result<Self> {
        let conn = Connection::open(db_path)?;
        let db = Database { conn };
        db.init_tables()?;
        Ok(db)
    }

    fn init_tables(&self) -> rusqlite::Result<()> {
        let sql = r#"
        CREATE TABLE IF NOT EXISTS traffic_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp TEXT NOT NULL,
            connection TEXT NOT NULL,
            protocol   TEXT NOT NULL,
            bytes      INTEGER NOT NULL
        );
        "#;
        self.conn.execute(sql, [])?;
        Ok(())
    }

    pub fn insert_log(
        &self,
        timestamp: &str,
        connection: &str,
        protocol: &str,
        bytes: u64,
    ) -> rusqlite::Result<usize> {
        let sql = r#"
        INSERT INTO traffic_logs (timestamp, connection, protocol, bytes)
        VALUES (?1, ?2, ?3, ?4)
        "#;
        self.conn
            .execute(sql, params![timestamp, connection, protocol, bytes])
    }

    // Example query
    pub fn get_all_logs(&self) -> rusqlite::Result<Vec<(i64, String, String, String, i64)>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, timestamp, connection, protocol, bytes FROM traffic_logs"
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, i64>(4)?,
            ))
        })?;

        let mut results = Vec::new();
        for row_result in rows {
            results.push(row_result?);
        }
        Ok(results)
    }
}
