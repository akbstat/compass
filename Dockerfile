FROM rust-web-runtime:v0.1.0

WORKDIR /app

COPY ./target/release/compass /app/compass

CMD [ "./compass" ]