FROM rust:slim

WORKDIR /app

COPY zero2prod* /app/
COPY configuration.yaml /app/

ENV SQLX_OFFLINE true

EXPOSE 8080

# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./zero2prod"]