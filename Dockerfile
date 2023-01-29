FROM rust:slim AS build

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN apt-get install -y build-essential
RUN yes | apt install gcc-x86-64-linux-gnu

WORKDIR /app

COPY . .

ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
RUN RUST_BACKTRACE=1 cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.17.0
COPY --from=build app/target/x86_64-unknown-linux-musl/release/smoke_test /
COPY --from=build app/target/x86_64-unknown-linux-musl/release/prime_time /
COPY --from=build app/target/x86_64-unknown-linux-musl/release/means_to_an_end /
COPY --from=build app/target/x86_64-unknown-linux-musl/release/budget_chat /
COPY --from=build app/target/x86_64-unknown-linux-musl/release/unusual_database_program /
COPY --from=build app/script.sh /
RUN chmod +x "/script.sh"
ENTRYPOINT ["/script.sh"]