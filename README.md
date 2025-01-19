# Asynchronous Programming in Rust

Exercises from the Asynchronous Programming in Rust book by Carl Fredrik Samson

## Setup

Some parts of this repo contain platform-specific code. Use Docker Compose to start and stop these directories/services:

```bash
docker compose up
```

```bash
docker compose down
```

_Note: certain directories contain a bash script such as `epoll_docker.sh`, that may contain a build step. Make sure to run this script before using Docker Compose for starting and stopping the crate._

```bash
./epoll_docker.sh
```
