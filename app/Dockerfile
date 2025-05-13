# https://docs.docker.com/build/building/multi-stage/
FROM nixos/nix:latest AS builder

RUN mkdir -p /build
WORKDIR /build

COPY . .

RUN nix \
	--experimental-features 'nix-command flakes' \
	build

RUN mkdir -p /tmp/scratch/nix/store
RUN cp -r $(nix-store -qR result/) /tmp/scratch/nix/store
RUN ln -s "$(nix-store -q result/)" /tmp/scratch/app

RUN mkdir -p /tmp/scratch/etc/ssl/certs
RUN cp "$SSL_CERT_FILE" /tmp/scratch/etc/ssl/certs/ca-bundle.crt

####################################################################

FROM scratch AS runtime

COPY --from=builder /tmp/scratch /

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-bundle.crt

WORKDIR /app

# `environment` section in `docker-compose.yml` will override `ENV`
# those are default values
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV RUST_LOG="info"

CMD ["/app/bin/cloud-1"]
