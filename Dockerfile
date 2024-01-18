FROM rust:1-slim-buster AS build
COPY . . 
RUN cargo build --release

FROM rust:1-slim-buster AS runner
COPY --from=build /target/release/server /app/server  
COPY --from=build /.env.example /app/.env  
CMD ["/app/server"]