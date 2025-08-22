#!/bin/sh

(echo "Clean auth-service" && cd auth-service && cargo clean)
(echo "Clean app-service" && cd app-service && cargo clean)

