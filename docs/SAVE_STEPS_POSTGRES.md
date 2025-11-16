# Saving Workflow Steps to PostgreSQL

Rust Read Manga workflows can save intermediate steps to a PostgreSQL database. This allows **resuming workflows, debugging, or analyzing intermediate data**.

---

## 1. Decide What to Save

Typical workflow steps to store:

| Step | Example Data |
|------|-------------|
| Input Loader | Image paths, total pages |
| OCR Analyzer | Extracted text, confidence scores |
| Text Density | Words/character counts |
| Duration Calculator | Per-page durations |
| Video Encoder | Status, output path, encoding logs |

---

## 2. PostgreSQL Setup

1. Install PostgreSQL.
2. Create a database:

```sql
CREATE DATABASE manga_workflow;
```

3. Enable UUID support (optional):

```sql
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
```

---

## 3. Table Schema

```sql
CREATE TABLE workflow_steps (
    id SERIAL PRIMARY KEY,
    workflow_id UUID DEFAULT gen_random_uuid(),
    step_name TEXT NOT NULL,
    page_number INT,
    data JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

- `workflow_id`: For multiple parallel workflows.  
- `step_name`: Node name (e.g., OCR Analyzer).  
- `page_number`: Optional page index.  
- `data`: Node output as JSON.  
- `created_at`: Timestamp for the record.

---

## 4. Rust: Connect to PostgreSQL

Add dependencies in `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
tokio-postgres = "0.8"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

Example connection:

```rust
use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=secret dbname=manga_workflow", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(())
}
```

---

## 5. Save Workflow Step

```rust
use serde_json::json;

async fn save_step(client: &tokio_postgres::Client, step: &str, page: Option<usize>, data: serde_json::Value) -> Result<(), tokio_postgres::Error> {
    client.execute(
        "INSERT INTO workflow_steps (step_name, page_number, data) VALUES ($1, $2, $3)",
        &[&step, &page.map(|p| p as i32), &data]
    ).await?;
    Ok(())
}

let step_data = json!({
    "text": "こんにちは",
    "confidence": 0.95
});

save_step(&client, "OCR Analyzer", Some(1), step_data).await?;
```

---

## 6. Retrieve Workflow Step

```rust
use serde_json::Value;

async fn get_step(client: &tokio_postgres::Client, step: &str, page: i32) -> Result<Value, tokio_postgres::Error> {
    let row = client.query_one(
        "SELECT data FROM workflow_steps WHERE step_name=$1 AND page_number=$2",
        &[&step, &page]
    ).await?;
    let data: Value = row.get("data");
    Ok(data)
}

let data = get_step(&client, "OCR Analyzer", 1).await?;
println!("Retrieved text: {}", data["text"]);
```

---

## 7. Tips

1. Use **JSONB** for faster queries and flexibility.  
2. Add **indexes** for frequently queried columns:

```sql
CREATE INDEX idx_workflow_page ON workflow_steps(workflow_id, page_number);
```

3. Wrap batch inserts in **transactions** for performance.  
4. Use **workflow_id** to separate multiple concurrent workflows.

---

This setup ensures that **all intermediate steps of your manga-to-video workflow are safely stored in PostgreSQL**, making your workflow more robust and analyzable.
