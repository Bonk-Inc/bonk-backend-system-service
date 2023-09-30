#####################################################################
## Build Backend
####################################################################
FROM rust:1.72.1-slim-buster AS backend-build

RUN apt-get update && apt-get install -y libssl-dev libpq-dev pkg-config

# create appuser
ENV USER=bonk-inc-backend
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /bonk-inc-backend

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-gnu --release -p babs_backend

#####################################################################
## Final image
####################################################################
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y libpq5

# Import from builder.
COPY --from=backend-build /etc/passwd /etc/passwd
COPY --from=backend-build /etc/group /etc/group

WORKDIR /bonk-inc-backend

# Copy our build and .env
COPY --from=backend-build ./bonk-inc-backend/.env ./
COPY --from=backend-build /bonk-inc-backend/target/x86_64-unknown-linux-gnu/release/bonk-inc-backend ./

# Use an unprivileged user.
USER bonk-inc-backend:bonk-inc-backend

EXPOSE 8080

CMD ["/bonk-inc-backend/bonk-inc-backend"]