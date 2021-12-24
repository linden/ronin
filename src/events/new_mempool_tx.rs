// POST /new_mempool_tx
// ======================

// This payload includes raw transactions newly received in the node's mempool.

// We deserialize the transactions and then use pipelining to add them to the DB.

// =============================================================================