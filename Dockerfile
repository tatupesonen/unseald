FROM rust:alpine3.14 as builder
LABEL stage=builder

WORKDIR /unseald

# Rust build flags
ENV RUSTFLAGS="-C target-feature=-crt-static"
COPY ./ ./
RUN apk add -q --update-cache --no-cache build-base openssl-dev

RUN set -eux; \
    	apkArch="$(apk --print-arch)"; \
      if [ "$apkArch" = "aarch64" ]; then \
      export JEMALLOC_SYS_WITH_LG_PAGE=16; \
      fi && \
      cargo build --release

# Need to use buster-slim because libssl-dev doesn't exist in distroless cc1 and no package manager is present
FROM alpine:3.14 as runtime
RUN apk update --quiet \
    	&& apk add -q --no-cache libgcc curl

COPY --from=builder /unseald/target/release/vault-unseal-daemon /bin/vault-unseal-daemon
CMD ["/bin/vault-unseal-daemon"]