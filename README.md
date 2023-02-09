# HyKey

A hypixel api proxy that caches to redis and handles rate limits.

## features

- Caches to redis
  - it returns the cached data of a endpoint for 60 seconds
  - it also returns the cached data if the ratelimit is hit to prevent 429s
- Handles rate limits
- Removes the need for api keys
