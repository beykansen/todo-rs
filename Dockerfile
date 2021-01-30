FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release


FROM debian:buster-slim
RUN apt-get update && apt-get install -y ca-certificates tzdata
ENV TZ=Etc/UTC
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/config config
COPY --from=builder /usr/src/app/target/release/todo-rs .

EXPOSE 80
CMD ["./todo-rs"]