FROM rust:1.74-bullseye AS builder
RUN apt-get update -y && \
    apt-get install -y build-essential make git

COPY Cargo.toml /rwa-system/Cargo.toml
COPY transformer /rwa-system/transformer
COPY common /rwa-system/common
COPY indexer /rwa-system/indexer
COPY migration /rwa-system/migration
COPY rwa_types /rwa-system/rwa_types
COPY tools /rwa-system/tools
WORKDIR /rwa-system/indexer

# Build application
RUN cargo build --release

FROM rust:1.74-slim-bullseye
ARG APP=/usr/src/app
RUN apt update \
    && apt install -y curl ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*
ENV TZ=Etc/UTC \
    APP_USER=appuser
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}
COPY --from=builder /rwa-system/target/release/indexer ${APP}
RUN chown -R $APP_USER:$APP_USER ${APP}
USER $APP_USER
WORKDIR ${APP}
CMD /usr/src/app/indexer
