# HyKey

A hypixel api proxy that caches to redis and handles rate limits.

## features

- Caches to redis
  - it returns the cached data of a endpoint for 60 seconds
  - it also returns the cached data if the ratelimit is hit to prevent 429s
- Handles rate limits
- Removes the need for api keys

## Public instance

- [hy.tricked.dev](https://hy.tricked.dev)
  - please dont break or abuse it lol the key only has a 60 requests per minute limit

## Setup

You need to have a redis server running on your machine. I recommend using dragonflydb the docker-compose file in this repository will start a redis server for you.

```
Usage: hykey [OPTIONS] --api-key <API_KEY>

Options:
  -a, --api-key <API_KEY>          [env: API_KEY=]
  -k, --key-limit <KEY_LIMIT>      [env: KEY_LIMIT=] [default: 60]
  -r, --redis-url <REDIS_URL>      [env: REDIS_URL=] [default: redis://127.0.0.1/]
  -b, --bind-addr <BIND_ADDR>      [env: BIND_ADDR=] [default: 0.0.0.0]
  -p, --bind-port <BIND_PORT>      [env: BIND_PORT=] [default: 4000]
  -u, --hypixel-api <HYPIXEL_API>  [env: HYPIXEL_API=] [default: https://api.hypixel.net]
  -h, --help                       Print help
```
