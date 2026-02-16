# naas - Nothing as a Service

Why get anything when you can also opt for nothing in this era of vibe coding and agentic BS?

## Local Dev

```bash
# rust btw
cargo build --release # for building
cargo test # for testing
```

## License
AGPL 3

## API Usage

Start the server:

```bash
cargo run
```

The service will be available at `http://localhost:3000`.

### Making Requests

Any path, any method - you'll always get the same result:

```bash
# Root path
curl http://localhost:3000/
# {"result":"nothing"}

# Arbitrary path
curl http://localhost:3000/api/v1/users/123/posts
# {"result":"nothing"}

# POST request
curl -X POST http://localhost:3000/submit
# {"result":"nothing"}

# With request body (ignored, as expected)
curl -X PUT http://localhost:3000/update -d '{"data":"ignored"}'
# {"result":"nothing"}
```

### Response Format

All requests return JSON with a single field:

```json
{
  "result": "nothing"
}
```

### CORS

The API has CORS enabled, allowing requests from any origin.
