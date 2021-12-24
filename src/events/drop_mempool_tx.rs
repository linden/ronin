// POST /drop_mempool_tx
// =======================

// This payload includes txids removed from the node's mempool.

// We use Pipelining to delete the txids in a single operation from Redis

// =============================================================================