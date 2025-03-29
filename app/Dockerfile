# https://docs.docker.com/build/building/multi-stage/
FROM nixos/nix:latest AS builder

# setup work environment
RUN mkdir -p /build
WORKDIR /build

COPY \
	./*.nix \
	./flake.lock \
	./Cargo.toml \
	./Cargo.lock \
	./rust-toolchain.toml \
	./src/**/*.rs \
	./style/**/*css \
	./public/ \
	.

# build the project
RUN nix build
