FROM rust:latest

WORKDIR /app
COPY . .
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --path .
RUN cargo install trunk
RUN trunk build

FROM nginx:latest

COPY --from=builder /app/dist /usr/share/nginx/html/
