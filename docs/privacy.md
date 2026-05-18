# Privacy

## Data Storage

This bot **does not store** any personal information persistently. It may cache some data in memory to improve performance (not implemented for now), but they all have a TTL of 3 minutes. Once the TTL expires, the cached data will be automatically deleted.

## Logging

By default (`info` level), this bot logs **messages that directly target it** and its response to stdout for debugging purposes. By "directly target it", we mean:

- Commands (invoking the bot)
- Private messages (sent to the bot)
- Inline queries (to the bot)
- Guest mode (mentioning the bot)

However, these logs are only printed to stdout and are not stored persistently.
