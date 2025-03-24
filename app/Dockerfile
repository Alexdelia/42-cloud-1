# https://docs.docker.com/build/building/multi-stage/
FROM nixos/nix:latest AS builder

# setup work environment
RUN mkdir -p /build
WORKDIR /build
COPY . .

# build the project
RUN nix-build
