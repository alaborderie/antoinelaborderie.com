FROM rust:latest as builder

WORKDIR /app
COPY . .
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN trunk build

FROM nginx:latest

COPY ./nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /app/dist /usr/share/nginx/html/
