# mu rust template

POC on how one could use rust with a semtech stack.

## How to

- Clone this project
- If you change the name of the crate in `Cargo.toml`, you must update the `Dockerfile`


## Environment variables

|  **variable**               |   **description**                     |  **default**                      |
|  ------------               |   ---------------                     |  -----------                      |
| `RUST_LOG`                  |   log level                           |  `INFO`                           |
| `SERVICE_HOST`              |   server host                         |  `0.0.0.0`                        |
| `SERVICE_PORT`              |   server port                         |  `80`                             | 
| `SPARQL_ENDPOINT`           |   sparql endpoint                     |  `http://database:8090/sparql`    | 
| `REQUEST_TIMEOUT_SECONDS`   |   timeout (sparql request)            |  `60`                             | 

