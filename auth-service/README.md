# Description
Authentication service

# Testing
1. Standard Rust tests
```
cargo test
```
2. Python integration tests
```
uv run test-localhost.py
```

# Test results
Current results of running python tests:
```
------------------------------ GET http://localhost:8000/
OK
------------------------------ POST http://localhost:8000/signup
415: Unsupported Media Type
------------------------------ POST http://localhost:8000/login
OK
------------------------------ POST http://localhost:8000/logout
OK
------------------------------ POST http://localhost:8000/verify-2fa
OK
------------------------------ POST http://localhost:8000/verify-token
OK
```
