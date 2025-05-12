# https://docs.docker.com/build/building/multi-stage/
FROM nixos/nix:latest AS builder

RUN mkdir -p /build
WORKDIR /build

COPY . .

RUN nix \
	--experimental-features 'nix-command flakes' \
	build

RUN mkdir /tmp/nix-store-closure
RUN cp -R $(nix-store -qR result/) /tmp/nix-store-closure
RUN echo $(nix-store -q result) > /tmp/nix-store-target


# FROM scratch AS runtime
FROM nixos/nix:latest AS runtime

COPY --from=builder /tmp/nix-store-closure /nix/store
COPY --from=builder /tmp/nix-store-target /tmp/nix-store-target
RUN ln -s /nix/store/$(cat /tmp/nix-store-target) /app
RUN rm /tmp/nix-store-target

WORKDIR /app

# `environment` section in `docker-compose.yml` will override `ENV`
# those are default values
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV RUST_LOG="info"

CMD ["/app/bin/cloud-1"]
