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

# TODO: stage 2 of multi-stage build
