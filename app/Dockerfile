# https://docs.docker.com/build/building/multi-stage/
FROM nixos/nix:latest AS builder

# setup work environment
RUN mkdir -p /build
WORKDIR /build

COPY . .

# build the project
RUN nix \
	--experimental-features 'nix-command flakes' \
	build

# FROM scratch AS runtime
# FROM nixos/nix:latest AS runtime
#
# COPY --from=builder /build/result /app
#
# CMD ["/app/bin/cloud-1"]

# `environment` section in `docker-compose.yml` will override `ENV`
# those are default values
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV RUST_LOG="info"

CMD ["/usr/bin/env", "bash"]
