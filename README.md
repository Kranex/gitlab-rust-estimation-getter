# Installation

To compile and install, get rust and cargo setup as described [here](https://www.rust-lang.org/tools/install).

Then run:

```bash
cargo install --path .
```

This will install greg to `$HOME/.cargo/bin` which you should add to your path.

# Usage

Running `greg` for the first time will show some warnings about gitlab urls and api_keys, the gitlab url defaults to `gitlab.com` and should be set to the base URL for the org that you're company uses.
The API key should be an gitlab Access Token with full api access enabled. On unix like systems URL and API key are stored in `$HOME/.config/gitlab-rust-estimation-getter/gitlab-rust-estimation-getter.toml` or on windows somewhere in `%APPDATA%`.

After providing the URL and API key, and for all future calls of `greg`, the current user will be outputted on stderr and all tickets that a user has interacted with will be printed in csv format.

So to get a CSV for all spent and estimated time for all tickets use:

```
greg > all_estimates.csv
```

# TODO

- [ ] get gitlab url from the git remote url.
- [ ] store API keys per url, and per project. For new projects, suggest API Keys for other projects on that URL.
- [ ] extend `greg` to also start and stop spend timers for the current branch.
- [ ] stop timer and start a new one when changing branches.
