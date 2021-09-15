# CapabilityAPI

Source: https://web.archive.org/web/20180120000131/http://www.zsck.co/writing/capability-based-apis.html
[Source](https://web.archive.org/web/20180120000131/http://www.zsck.co/writing/capability-based-apis.html)



## Setup

You need to have `sqlx` command installed for `cargo``

```sh
cargo sqlx database create
cargo sqlx mig run
```

## Update sqlx-data.json

```sh
cargo sqlx prepare -- --lib
```