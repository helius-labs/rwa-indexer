FROM rust:1.74-bullseye  AS builder
RUN apt-get update -y && \
    apt-get install -y build-essential make git

COPY . /rwa-system/

WORKDIR /rwa-system/api
# # Build application
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
COPY --from=builder /rwa-system/target/release/rwa_api ${APP}
RUN chown -R $APP_USER:$APP_USER ${APP}
USER $APP_USER
WORKDIR ${APP}
CMD /usr/src/app/rwa_api
