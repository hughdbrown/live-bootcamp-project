#!/usr/bin/env python3

# /// script
# requires-python = ">=3.13"
# dependencies = [
#   "click",
#   "requests",
# ]
# ///

import requests

def main():
    baseurl = "http://localhost:3000"
    x = [
        (requests.get, "/", 200),
        (requests.post, "/signup", 415),
        (requests.post, "/login", 200),
        (requests.post, "/logout", 200),
        (requests.post, "/verify-2fa", 200),
        (requests.post, "/verify-token", 200),
    ]

    for meth, url, expected in x:
        uu = f"{baseurl}{url}"
        print(f"{'-' * 30} {'POST' if meth == requests.post else 'GET'} {uu}")
        result = meth(uu)
        if expected != result.status_code:
            print(f"{expected =} {result.status_code =}: {result.reason}")

    signup_data = [
        ({"email": "foo@gmail.com", "password": "12345", }, 422),
        ({"email": "foo@gmail.com", "requires2FA": True}, 422),
        ({"password": "12345", "requires2FA": True}, 422),
        ({"email": "foo@gmail.com", "password": "12345", "requires2FA": True}, 200),
        ({"email": "foo@gmail.com", "password": "12345", "requires2FA": False}, 200),
    ]
    url = f"{baseurl}/signup"
    headers = {'Content-Type': 'application/json'}
    for i, (signup, expected) in enumerate(signup_data):
        print(f"{'-' * 30} {i} Test for {expected}: {signup}")
        result = requests.post(url, headers=headers, json=signup)
        if result.status_code != expected:
            print(f"Expected {expected}, got {result.status_code}: {result.reason}")


if __name__ == '__main__':
    main()
