#####################################################################
## Build Backend
####################################################################
FROM rust:1.80-slim-bookworm AS backend-build

# install extra dependencies for cryptography.
RUN apt-get update && apt-get install -y libssl-dev libpq-dev pkg-config

# set the working directory.
WORKDIR /bonk-inc-backend

# copy project files to the working directory.
COPY ./ .

# build the backend to a executable.
RUN cargo build --target x86_64-unknown-linux-gnu --release -p babs_backend

#####################################################################
## Build Front-end
####################################################################
FROM rust:1.80-slim-bookworm AS frontend-build

# install dependencies
RUN apt-get update && apt-get install -y libssl-dev libpq-dev pkg-config curl

# install node for npx command
ENV NODE_VERSION=20.11.0
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
ENV NVM_DIR=/root/.nvm
RUN . "$NVM_DIR/nvm.sh" && nvm install ${NODE_VERSION}
RUN . "$NVM_DIR/nvm.sh" && nvm use v${NODE_VERSION}
RUN . "$NVM_DIR/nvm.sh" && nvm alias default v${NODE_VERSION}
ENV PATH="$NVM_DIR/versions/node/v${NODE_VERSION}/bin/:${PATH}"

# add WASM build target to rust and install Trunk
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

# set the working directory.
WORKDIR /bonk-inc-backend

# copy project files to the working directory.
COPY ./ .

# create .env file
RUN cd ./frontend/ && cat <<EOT >> .env
APP_API_URL="https://babs.bonk.group"
EOT

# build the front-end to a WASM file and extra javascript.
RUN cd ./frontend/ && trunk build --release

#####################################################################
## Final image
####################################################################
FROM debian:bookworm-slim

# install extra dependencies for cryptography.
RUN apt-get update && apt-get install -y libpq5 ca-certificates
RUN update-ca-certificates

# Import from builder.
COPY --from=backend-build /etc/passwd /etc/passwd
COPY --from=backend-build /etc/group /etc/group

WORKDIR /bonk-inc-backend

# Copy our build
COPY --from=backend-build /bonk-inc-backend/target/x86_64-unknown-linux-gnu/release/bonk-inc-backend ./
COPY --from=frontend-build /bonk-inc-backend/dist/ ./dist/

# create appuser
ENV USER=bonk-inc-backend
ENV UID=32767

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

# Create data folder
RUN mkdir data

# Set file permissions
RUN chmod +rw *
RUN chown -R bonk-inc-backend:bonk-inc-backend *

# Use an unprivileged user.
USER bonk-inc-backend:bonk-inc-backend

EXPOSE 8080

CMD ["/bonk-inc-backend/bonk-inc-backend"]