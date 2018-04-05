# `artifice`

`artifice` is a command-line productivity tool.

**NOTE:** This is a work in progress! It probably won't be useful to you!

## Supported Platforms

`artifice` supports all of the Rust [Tier 1 platforms](https://forge.rust-lang.org/platform-support.html).

## Configuration

`artifice` can be configured using a configuration file. By default this file is
stored in `.artifice.toml` in your home directory (e.g. `/Users/you` on macOS).
You can also use the `--config` option to specify a config file at runtime.

The file is formatted with [TOML](https://github.com/toml-lang/toml) for ease of
editing.

Continue reading this section for particulars on how to set certain options.

### JIRA

The `[jira]` section is necessary for any commands that interact with JIRA.

* `host`: Your JIRA instance URL e.g. `https://myteam.atlassian.net`
* `email`: Your JIRA user's email address
* `token`: An Atlassian Cloud API token. See the [help documentation](https://confluence.atlassian.com/cloud/api-tokens-938839638.html)
  for information on how to generate one for your account.

## License

Licensed under either of these:

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  https://opensource.org/licenses/MIT)
