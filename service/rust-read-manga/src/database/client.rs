use serde_json::Value;
use tokio_postgres::{Client, NoTls, Error};

pub async fn connect(connection_string: &str) -> Result<Client, Error> {
    let (client, connection) =
        tokio_postgres::connect(connection_string, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

pub async fn save_step(client: &Client, workflow_id: &str, step_name: &str, page_number: Option<i32>, data: Value) -> Result<(), Error> {
    client.execute(
        "INSERT INTO workflow_steps (workflow_id, step_name, page_number, data) VALUES ($1, $2, $3, $4)",
        &[&workflow_id, &step_name, &page_number, &data]
    ).await?;
    Ok(())
}

pub async fn get_step(client: &Client, workflow_id: &str, step_name: &str, page: i32) -> Result<Value, Error> {
    let row = client.query_one(
        "SELECT data FROM workflow_steps WHERE workflow_id = $1 AND step_name = $2 AND page_number = $3",
        &[&workflow_id, &step_name, &page]
    ).await?;
    let data: Value = row.get("data");
    Ok(data)
}
