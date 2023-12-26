use anyhow::Result;
use csv;
use itertools::Itertools;
use sqlx::{
    migrate::{MigrationSource, Migrator},
    postgres, Connection, Executor, PgConnection, PgPool,
};
use std::{path::Path, thread};
use tokio::runtime::Runtime;

#[derive(Debug)]
pub struct TestPg {
    pub server_url: String,
    pub dbname: String,
}

impl TestPg {
    pub fn new<S>(server_url: String, migrations: S) -> Self
    where
        S: MigrationSource<'static> + Send + Sync + 'static,
    {
        // 默认使用test数据库
        let dbname = "test".to_string();
        let dbname_cloned = dbname.clone();

        // 创建测试数据库
        let tdb = Self { server_url, dbname };
        let server_url = tdb.server_url();
        let url = tdb.url();

        // 启动一个线程执行migrate迁移操作
        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async move {
                // 连接postgres数据库并执行创建数据库操作
                let postgres_url = format!("{}/{}", server_url, "postgres");
                let mut conn = PgConnection::connect(&postgres_url).await.unwrap();
                conn.execute(
                    format!(
                        r#"CREATE DATABASE IF NOT EXISTS "{dbname}""#,
                        dbname = dbname_cloned
                    )
                    .as_str(),
                )
                .await
                .expect("Error while querying the create database");

                // 连接测试数据库并执行迁移代码
                let mut conn = PgConnection::connect(&url).await.unwrap();
                let m = Migrator::new(migrations).await.unwrap();
                m.run(&mut conn).await.unwrap();
            });
        })
        .join()
        .expect("failed to create database");

        tdb
    }

    pub fn server_url(&self) -> String {
        self.server_url.clone()
    }

    pub fn url(&self) -> String {
        format!("{}/{}", self.server_url, self.dbname)
    }

    pub async fn get_pool(&self) -> PgPool {
        PgPool::connect(&self.url()).await.unwrap()
    }

    pub async fn load_csv(&self, table: &str, fields: &[&str], filename: &Path) -> Result<()> {
        let pool = self.get_pool().await;
        let path = filename.canonicalize()?;
        let mut conn = pool.acquire().await?;
        let sql = format!(
            "COPY {} ({}) FROM '{}' DELIMITER ',' CSV HEADER;",
            table,
            fields.join(","),
            path.display()
        );
        conn.execute(sql.as_str()).await?;
        // copy csv

        Ok(())
    }

    pub async fn load_csv_data(&self, table: &str, csv: &str) -> Result<()> {
        let mut rdr = csv::Reader::from_reader(csv.as_bytes());
        let headers = rdr.headers()?.iter().join(",");
        let mut tx = self.get_pool().await.begin().await?;
        for result in rdr.records() {
            let record = result?;
            let sql = format!(
                "INSERT INTO {} ({}) VALUES ({})",
                table,
                headers,
                record.iter().map(|v| format!("'{v}'")).join(",")
            );
            tx.execute(sql.as_str()).await?;
        }
        tx.commit().await?;
        Ok(())
    }
}

impl Drop for TestPg {
    fn drop(&mut self) {
        let server_url = format!("{}/{}", self.server_url(), "postgres");
        let dbname = self.dbname.clone();
        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async move {
                    let mut conn = PgConnection::connect(&server_url).await.unwrap();
                    // terminate existing connections
                    sqlx::query(&format!(r#"SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE pid <> pg_backend_pid() AND datname = '{dbname}'"#))
                    .execute( &mut conn)
                    .await
                    .expect("Terminate all other connections");
                    conn.execute(format!(r#"DROP DATABASE "{dbname}""#).as_str())
                        .await
                        .expect("Error while querying the drop database");
                });
            })
            .join()
            .expect("failed to drop database");
    }
}

impl Default for TestPg {
    fn default() -> Self {
        Self::new(
            "postgres://postgres:postgres@localhost:5432".to_string(),
            Path::new("./fixtures/migrations"),
        )
    }
}
