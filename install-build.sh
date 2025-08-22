#!/bin/sh

(echo 'Install cargo-watch' &&  cargo install cargo-watch)
(echo 'Build app-service' && cd app-service && cargo build)
(echo 'Build auth-service' && cd auth-service && cargo build)
