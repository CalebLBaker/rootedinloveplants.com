FROM alpine:3.12.0 as builder
WORKDIR /usr/src/rootedinlove
COPY . .
RUN apk add --no-cache cargo
RUN apk add --no-cache musl-dev
RUN apk add --no-cache openssl-dev
RUN cargo build --release
RUN cargo run --release --bin html-gen content.json styles.css index.js
 
FROM alpine:3.12.0
RUN apk add --no-cache libgcc
COPY --from=builder /usr/src/rootedinlove/index.html /content/index.html
COPY --from=builder /usr/src/rootedinlove/target/release/server /
COPY --from=builder /usr/src/rootedinlove/images /content/images/
COPY --from=builder /usr/src/rootedinlove/fancybox /content/fancybox/
ENTRYPOINT /server

