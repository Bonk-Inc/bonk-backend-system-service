#####################################################################
## Builder
####################################################################
FROM rust:1.70.0-alpine3.18 AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apk update && apk add --no-cache musl-dev pkgconfig openssl-dev
RUN update-ca-certificates

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

RUN cargo build --target x86_64-unknown-linux-musl --release

#####################################################################
## Final image
####################################################################
FROM alpine

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /bonk-inc-backend

# Copy our build
COPY --from=builder /bonk-inc-backend/target/x86_64-unknown-linux-musl/release/bonk-inc-backend ./

# Use an unprivileged user.
USER bonk-inc-backend:bonk-inc-backend

EXPOSE 8080

CMD ["/bonk-inc-backend/bonk-inc-backend"]