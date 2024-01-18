FROM rust:1-slim-buster AS build
COPY . . 
RUN cargo build --release

FROM rust:1-slim-buster  
COPY --from=build /target/release /app  
COPY --from=build /.env.example /.env  
CMD ["cargo", "run"]