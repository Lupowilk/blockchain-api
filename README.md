# blockchain-api
A blockchain API using MongoDB

## API Endpoints

## Database Indexes

For optimal query performance, the following indexes are automatically created at application startup:

| Field | Order | Purpose |
|-------|-------|---------|
| `sender` | Ascending | Speeds up queries filtering by sender address |
| `receiver` | Ascending | Speeds up queries filtering by receiver address |
| `amount` | Ascending | Speeds up queries filtering by transaction amount |
| `timestamp` | Descending | Optimizes sorting for newest-first queries and pagination |

### Why Indexes Matter

Indexes dramatically improve query performance:
- Without indexes: MongoDB scans all documents (slow with large datasets)
- With indexes: MongoDB jumps directly to matching documents (fast regardless of size)

### Performance Impact

Example with 10,000 transactions:
- Unindexed query: ~500ms
- Indexed query: ~5ms

**100x faster!**

### GET /transactions

Returns transactions stored in the MongoDB database with optional filtering and pagination.

#### Query Parameters

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `sender` | string | No | - | Filter by sender address |
| `receiver` | string | No | - | Filter by receiver address |
| `amount` | integer | No | - | Filter by exact amount |
| `limit` | integer | No | 10 | Number of results to return (max: 50) |
| `offset` | integer | No | 0 | Number of results to skip |

#### Examples

**Get all transactions (first 10):**
```bash
curl http://localhost:3000/transactions
```

**Filter by sender:**
```bash
curl "http://localhost:3000/transactions?sender=Alice"
```

**Filter by receiver:**
```bash
curl "http://localhost:3000/transactions?receiver=Bob""
```

**Filter by amount:**
```bash
curl "http://localhost:3000/transactions?amount=100"
```

**Pagination - get transactions 11-20::**
```bash
curl "http://localhost:3000/transactions?limit=10&offset=10"
```

**Combined filtering and pagination:**
``` bash
curl "http://localhost:3000/transactions?sender=Alice&limit=5"
```

**Response format**
```
{
  "transactions": [
    {
      "_id": { "$oid": "..." },
      "tx_id": 1759218067668,
      "sender": "Alice",
      "receiver": "Bob",
      "amount": 100,
      "timestamp": 1759218067
    }
  ],
  "count": 1
}
```
