# [paste.misterio.me](https://paste.misterio.me)

## About

### What

This is a simple pasting service, designed to be quick and easy to use. It's fully serverside rendered, and does not include a _single_ line of javascript. Should work perfectly on slower devices, TUI browsers, or anything of the sort.

All the routes can also receive and return JSON. Just pass a JSON payload and/or put JSON into your `Accept` header. You can use `httpie`, for examploe.

Lastly, we have a [nice CLI](https://github.com/misterio77/pmis) for interacting with the service.

### Where

The source code (licensed under AGPLv3) for this app can be at [github](https://github.com/misterio77/paste.misterio.me) and at my [personal git](https://m7.rs/git/paste-misterio-me). Feel free to contribute wherever you feel more confortable.

The version live at [paste.misterio.me](https://paste.misterio.me) runs on my home Raspberry Pi 4 (running NixOS 22.05). Deployments are reproductible and based on [my configuration repository](https://github.com/misterio77/nix-config).

### Why

I was a little burned out fixing older code at work, so i just wanted to make a nice real world app in a weekend, without having to deal with legacy codebases or databases. Just my clean new shiny schema.

I really recommend doing stuff like this (specially in the exact same stack you use at work). Making stuff from scratch makes it easier to plan out a nicer architecture, getting insights you can't get on an older codebase.

I like how [paste.sr.ht](https://paste.sr.ht) and [gist.github.com](https://gist.github.com), so why not create my own clone?

### How

This is a fully server-side rendered web application.

It is built with [Rust](https://rust-lang.org), using the [Rocket](https://rocket.rs) web framework, and [Tera](https://tera.netlify.app/) templating engine. Paired with a [PostgreSQL](https://postgresql.org) relational database.

I dislike class-heavy HTML/CSS, so the styling is based on the awesome [Pico.css](https://picocss.com) sheet. Both Pico.css and my own customizations are SCSS, which is compiled at build-time (and included into the executable) using [grass](https://github.com/connorskees/grass).

Also to avoid client-side javascript code, syntax highlighting (a core feature of the app) is also serverside. I use [syntect](https://github.com/trishume/syntect) for that. Sadly the default syntax set is kinda stale (based on sublimetext's upstream definitions), so i have [slimesag's fork](https://github.com/slimsag/Packages) syntax definitions vendored in this project, they are built into a binary cache at build-time as well (also bundled into the executable).

Passwords are hashed using [rust-argon](https://github.com/sru-systems/rust-argon2), and [chrono](https://github.com/chronotope/chrono) is used for datetime stuff.

## Hacking

It should be really easy to build and run.

### Dependencies

Just grab [rustc and cargo](https://rust-lang.org) (usually through either rustup or your distro's packages, if they're recent enough). If you're using nix, just run `nix develop` to get a shell with everything you need.

Get a [PostgreSQL](https://postgresql.org) instance up and running (should be available on your distro's repo, or use docker). Either socket or password auth will work just fine.

Populate your schema using the `.sql` files in `db/`. These have a version number, so if you're upgrading just run the new ones.

### Configuration

Edit `Rocket.toml`'s `url` section and set it to your psql [connection string](https://stackoverflow.com/questions/3582552). Or set `ROCKET_DATABASES` environment variable to `{database={url="connection_string_here"}}`.

You can change bind `address`, `port`, and `template_dir` as well. Either add the key (lowercase) to `Rocket.toml`'s `[default]` session (easier when hacking), or set `ROCKET_FOO_BAR` env variables (better for deployment).

If you're planning on deploying, you need a stable secret (for signing auth cookies). You can generate a nice one with `openssl rand -base64 32`, add it to your `ROCKET_SECRET_KEY` variable and you're good to go.

### Running

Just run `cargo run` to run debug mode. Add in `--release` for a optimised (but slower compilling) version. If you just want the executable, use `cargo build` instead.

### Contributing

Open up a PR on github or send in a patch through sourcehut. I should have some automatic CI set soon(tm).
