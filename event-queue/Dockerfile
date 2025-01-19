FROM rust:1-slim-bookworm

WORKDIR /usr/event-queue

COPY . .

RUN cargo build

CMD ["cargo", "run", "delayserver"]
