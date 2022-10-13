# [paste.misterio.me](https://paste.misterio.me)

## About

### What

This is a simple pasting service, designed to be quick and easy to use. It's fully serverside rendered, and does not include a _single_ line of javascript. Should work perfectly on slower devices, TUI browsers, or anything of the sort.

All the routes can also receive and return JSON. Just pass a JSON payload and/or put JSON into your `Accept` header. You can use `httpie`, for examploe.

This workspace includes both the `server` and a companion `cli`, on their respective subdirectories.

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

The CLI is handled by [clap](https://github.com/clap-rs/clap), the API requests are made through [reqwest](https://github.com/seanmonstar/reqwest), and the output is formatted using [bat](https://github.com/sharkdp/bat).

# Setup

It should be really easy to build and run.

## Server

### Dependencies

Just grab [rustc and cargo](https://rust-lang.org) (usually through either rustup or your distro's packages, if they're recent enough). If you're using nix, just run `nix develop` to get a shell with everything you need.

Get a [PostgreSQL](https://postgresql.org) instance up and running (should be available on your distro's repo, or use docker). Either socket or password auth will work just fine.

Populate your schema using the `.sql` files in `db/`. These have a version number, so if you're upgrading just run the new ones.

### Configuration

Edit `Rocket.toml`'s `url` section and set it to your psql [connection string](https://stackoverflow.com/questions/3582552). Or set `ROCKET_DATABASES` environment variable to `{database={url="connection_string_here"}}`.

You can change bind `address`, `port`, and `template_dir` as well. Either add the key (lowercase) to `Rocket.toml`'s `[default]` session (easier when hacking), or set `ROCKET_FOO_BAR` env variables (better for deployment).

If you're planning on deploying, you need a stable secret (for signing auth cookies). You can generate a nice one with `openssl rand -base64 32`, add it to your `ROCKET_SECRET_KEY` variable and you're good to go.

### Running

Just run `cargo run -p server` to run debug mode. Add in `--release` for a optimised (but slower compilling) version. If you just want the executable, use `cargo build -p server` instead.

If you run NixOS, there's a NixOS module available.

## CLI

### Installation

`pmis` is available on crates.io, on the AUR, and there's also a nix flake in the repo for usage with nix.

#### Cargo

Use `cargo install pmis`, or clone this repo and run `cargo install -p cli`.

You can generate completions using `pmis completions <SHELL>` (check your distro docs on where to install them).

#### Nix/NixOS/home-manager

You can get a shell with `pmis` using `nix shell github:misterio77/pmis`.

For a more permanent solution, you should add `pmis` to your flake inputs, add the overlay, and put it wherever you usually put packages (i recommend using `home-manager`, we even have a module you can import).

If you want to avoid compiling, `pmis` is cached on [cachix](https://app.cachix.org/cache/misterio): `cachix use misterio`.

Completions are provided through the derivation.

#### Arch Linux

Use your favorite AUR helper: `paru -S pmis`.

Completions are provided through the package.

### Usage

The default API URL is `https://paste.misterio.me`, you can switch to another (if you're self hosting an instance, for example) using `--api`.

All commands and options are fully documented through `--help`

#### Downloading pastes

Use `pmis download <ID>`. The output is pretty printed using `bat` (unless it is piped, or if you use `--raw`).

Do keep in mind pastes can easily be downloaded using many utilities, such as `curl`: `curl https://paste.misterio.me/p/ID/raw`. This makes it easy to get them on any barebones system or to share with friends that don't use `pmis`.

#### Listing pastes

You can list a users public pastes (or all of them, if you're authenticated and the user is you) using `pmis list [OWNER]`. You can ommit `OWNER` if you're authentiucated. If you just want the IDs, add `--ids-only`.

#### Authenticating

You should [generate a key](https://paste.misterio.me/keys), and then use `pmis auth`.

#### Uploading pastes

Use `pmis upload [FILE]`. The title of the paste is the filename, by default. You can ommit `FILE` to read from stdin. Use `--description` to add a description, and `--unlisted` if you don't want it to appear on your profile. When the upload is complete the link and ID will be output, you can get just the link by piping or using `--link-only`.

#### Deleting pastes

You can delete your pastes by using `pmis delete <ID>`.
