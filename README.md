uwubot
======

[![Build Status][actions-badge]][actions-url]
[![Latest Version](https://img.shields.io/crates/v/uwubot.svg)](https://crates.io/crates/uwubot)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/uwubot)

[actions-badge]: https://github.com/yaahc/uwubot/workflows/Continuous%20integration/badge.svg
[actions-url]: https://github.com/yaahc/uwubot/actions?query=workflow%3A%22Continuous+integration%22

This crate defines a discord bot using [`serenity`] for uwuifying text via [`uwuify`].

## Setup

You can setup your own instance of uwubot using the following steps:

1. create a new bot in the [discord developer portal]
1. copy the client-id and client-secrets from the new bot
1. run uwubot with `cargo run -- <client-secret> --client-id <client-id>`
1. create a url with the `applications.commands` oauth2 scope on the `OAuth2` tab of the developer portal
1. navigate to the generated URL to register your bot on your server of choice

If you'd like to use the instance of uwubot that I'm running you can use this url to register it on your server. **Note: I don't make any sort of uptime guarantees for this instance, as of writing htis readme I'm running it on my laptop, so it will almost certainly be offline, though eventually I hope to run it on my desktop...**

- https://discord.com/api/oauth2/authorize?client_id=824691677372612680&scope=applications.commands

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
