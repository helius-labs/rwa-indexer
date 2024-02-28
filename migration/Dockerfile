FROM rust:1.73-bullseye
COPY rust-services/blockbuster /blockbuster
COPY rust-services/das-plugin/1.17.x/ /das-plugin/1.17.x
COPY rust-services/das-system /das-system
WORKDIR /das-system/migration
ENV INIT_FILE_PATH=/das-system/init.sql
RUN cargo build --release
CMD /das-system/target/release/migration up
