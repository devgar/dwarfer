ARG RUST_VERSION=1.92
ARG APP_NAME=dwarfer
ARG FEATURES=repo-file,api-redirect

FROM rust:${RUST_VERSION}-alpine AS BUILD
LABEL authors="Edgar Albalate"

ARG APP_NAME
ARG FEATURES

WORKDIR /app

RUN apk add --no-cache clang lld musl-dev git

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --no-default-features --release --features $FEATURES && \
    cp ./target/release/$APP_NAME /bin/$APP_NAME

FROM docker.io/library/alpine:3.18 AS final

ARG UID=10001

WORKDIR /app

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shel "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser

USER appuser

COPY --from=build /bin/$APP_NAME /bin/$APP_NAME

# EXPOSE 8080

CMD ["/bin/server"]