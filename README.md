uwubot
======

[![Build Status][actions-badge]][actions-url]
[![Latest Version](https://img.shields.io/crates/v/uwubot.svg)](https://crates.io/crates/uwubot)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/uwubot)

[actions-badge]: https://github.com/yaahc/uwubot/workflows/Continuous%20integration/badge.svg
[actions-url]: https://github.com/yaahc/uwubot/actions?query=workflow%3A%22Continuous+integration%22

This crate defines a discord bot using [`serenity`] for uwuifying text via [`uwuify`].

## Installation

You can install uwubot from source or from `crates.io`

```
cargo install uwubot
```

## Usage / Bot Setup

You can setup your own instance of uwubot using the following steps:

1. create a new bot in the [discord developer portal]
1. copy the client-id and bot token from the new bot
1. run uwubot with `uwubot <bot-token> --client-id <client-id>`
1. create a url with the `applications.commands` oauth2 scope on the `OAuth2` tab of the developer portal
1. navigate to the generated URL to register your bot on your server of choice

### Docker Instance

Alternatively you can try to run `uwubot` with the provided `Dockerfile`. The `bot-token` and `client-id` args can be set via the `BOT_TOKEN` or `CLIENT_ID` environment variables.

```
docker build -t uwubot .
docker run -it --rm --env BOT_TOKEN="<bot-token>" --env CLIENT_ID=<client-id> --name uwubot-running uwubot
```

## Additional Details

`uwubot` allows you to either register global commands or guild commands. To register a guild command you'll need to figure out your discord server's guild ID. I'll try to add an easy way to export this in `uwubot` in the future but for now you're on you're own, I'm sowwy >_<.

Guild commands have the advantage of being instantly updated, where as global commands are cached with a 1 hour update rate.

[`uwuify`]: https://docs.rs/uwuify
[`serenity`]: https://docs.rs/serenity
[discord developer portal]: https://discord.com/developers/applications

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
