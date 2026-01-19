# polymarket_positions_substream Substreams modules

This package was initialized via `substreams init`, using the `evm-events-calls` template.

## Usage

```bash
substreams build
substreams auth
substreams gui       			  # Get streaming!
```

## Database Sink

This substream is configured to sink data into PostgreSQL using `substreams-sink-sql`.

### 1. Setup the Database
Initialize the schema and internal system tables:
```bash
substreams-sink-sql setup "psql://dev:insecure@localhost:5432/main?sslmode=disable" substreams.yaml
```

### 2. Run the Sink
Start streaming data into the database:
```bash
substreams-sink-sql run "psql://dev:insecure@localhost:5432/main?sslmode=disable" substreams.yaml
```

Optionally, you can publish your Substreams to the [Substreams Registry](https://substreams.dev).

```bash
substreams registry login         # Login to substreams.dev
substreams registry publish       # Publish your Substreams to substreams.dev
```

## Modules

All of these modules produce data filtered by these contracts:
- _conditional_tokens_ at **0x4d97dcd97ec945f40cf65f87097ace5ea0476045**
### `map_events`

This module gets you only events that matched.


