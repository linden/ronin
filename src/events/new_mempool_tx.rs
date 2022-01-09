// POST /new_mempool_tx
// ======================

// This payload includes raw transactions newly received in the node's mempool.

// Example request body: ./example_events/new_mempool_tx.json

// We deserialize the transactions and then use pipelining to add them to the DB.

// =============================================================================

use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};

#[post("/new_mempool_tx", format = "json", data = "<message>")]
async fn new(message: Json<Message<'_>>, list: Messages<'_>) -> Value {
    let mut list = list.lock().await;
    let id = list.len();
    list.push(message.message.to_string());
    json!({ "status": "ok", "id": id })
}