FROM rust:1-slim-buster AS build
COPY . . 
RUN cargo build --release

FROM debian:buster-slim  
COPY --from=build /target/release /app  
CMD "/app/server"