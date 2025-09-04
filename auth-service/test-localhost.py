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
    baseurl = "http://localhost:8000"
    x = [
        (requests.get, "/"),
        (requests.post, "/signup"),
        (requests.post, "/login"),
        (requests.post, "/logout"),
        (requests.post, "/verify-2fa"),
        (requests.post, "/verify-token"),
    ]

    for meth, url in x:
        uu = f"{baseurl}{url}"
        print(f"{'-' * 30} {'POST' if meth == requests.post else 'GET'} {uu}")
        result = meth(uu)
        if result.ok:
            print("OK")
        else:
            print(f"{result.status_code}: {result.reason}")
            # print(dir(result))


if __name__ == '__main__':
    main()
