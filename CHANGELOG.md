# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate
# Added
- Add new methods to `Bot` for listing and deleting guild slash commands
- Exposed methods via new subcommands in main.rs

# Changed
- Removed implicit command registration from `Bot::run`
- Changed `Bot::run` to be an async function
- Removed `guild_id` from `Config` struct

## [0.2.2] - 2021-03-27
# Added
- Added example for how to run with docker

## [0.2.1] - 2021-03-27
# Added
- Add a Dockerfile for deploying uwubot
- Add an environment variable fallback for each argument

## [0.2.0] - 2021-03-16
# Changed
- Moved structopt argument parser into main.rs and export new `Config` replacement

<!-- next-url -->
[Unreleased]: https://github.com/yaahc/uwubot/compare/v0.2.2...HEAD
[0.2.2]: https://github.com/yaahc/uwubot/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/yaahc/uwubot/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/yaahc/uwubot/releases/tag/v0.2.0
