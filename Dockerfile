ARG APP_NAME=my-api

######################################################################
# Planner

FROM instrumentisto/rust:nightly-bookworm-slim-2025-02-01 AS planner

ARG APP_NAME

RUN cargo install cargo-chef

WORKDIR /app

COPY . .

RUN cargo chef prepare --bin=$APP_NAME --recipe-path=recipe.json

######################################################################
# Builder

FROM instrumentisto/rust:nightly-bookworm-slim-2025-02-01 AS builder

ARG APP_NAME

RUN apt-get update && \
    apt-get install -y --no-install-recommends libssl-dev pkg-config curl && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

RUN cargo install cargo-chef

# Copy the build plan from the previous Docker stage
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this layer is cached as long as `recipe.json`
# doesn't change.
RUN cargo chef cook --bin=$APP_NAME --features=loki --recipe-path=recipe.json

# Build the whole project
COPY . .

RUN cargo build --bin=$APP_NAME --features=loki

######################################################################
# Runtime

FROM debian:bookworm-slim AS runtime

ARG APP_NAME
ENV EXE=/app/$APP_NAME

RUN apt-get update && \
    apt-get install -y --no-install-recommends libssl-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/debug/$APP_NAME .
COPY --from=builder /app/config/ ./config/

EXPOSE 8080

CMD ["sh", "-c", "exec $EXE"]
