# README

This is a repo that is teaching me production level rust by following Luca Palmieri's book Zero to Production in Rust. I decided to make this public, because why not.

Down here are annotations from the book itself.

Note. That I'm also adding my personal touches while I work so I can get a better understanding.

# Launching the project

- To run postgresql just do `docker-compose up` at the project's root.

### 1.4.1 Faster Linking

When looking at the inner development loop, we are primarily looking at the performance of incremental compilation - how long it takes cargo to rebuild our binary after having made a small change
to the source code.
A sizeable chunk of time is spent in the linking phase - assembling the actual binary given the
outputs of the earlier compilation stages.
The default linker does a good job, but there are faster alternatives depending on the operating
system you are using:
• lld on Windows and Linux, a linker developed by the LLVM project;
• zld on MacOS.

###The easiest way to measure code coverage of a Rust project is via cargo tarpaulin,

###The Rust team maintains clippy, the offcial Rust linter12.
clippy is included in the set of components installed by rustup if you are using the default profile.
Often CI environments use rustup’s minimal profile, which does not include clippy.
You can easily install it with
rustup component add clippy

###You can mute a warning using the #[allow(clippy::lint_name)] attribute on the affected code
block or disable the noisy lint altogether for the whole project with a configuration line in clippy.toml

###rustfmt is the offcial Rust formatter.
Just like clippy, rustfmt is included in the set of default components installed by rustup. If missing,
you can easily install it with

### security

The Rust Secure Code working group maintains an Advisory Database - an up-to-date collection of
reported vulnerabilities for crates published on crates.io

During our development process we are not always interested in producing a runnable binary: we often just want
to know if our code compiles or not. cargo check was born to serve exactly this usecase:

### Cargo Expand

That is exactly where cargo expand shines: it expands all macros in your code without passing the
output to the compiler, allowing you to step through it and understand what is going on

### Cargo Edit

When installed by:

```sh
cargo install cargo-edit
```

You can use
`cargo add`

### 3.5.1.2 Choosing A Random Port spawn_app will always try to run our application on port

8000 - not ideal:
• if port 8000 is being used by another program on our machine (e.g. our own application!), tests
will fail;
• if we try to run two or more tests in parallel only one of them will manage to bind the port, all
others will fail.

How do we find a random available port for our tests?
The operating system comes to the rescue: we will be using port 0.
Port 0 is special-cased at the OS level: trying to bind port 0 will trigger an OS scan for an available
port which will then be bound to the application.

### Installing sqlx-cli

https://crates.io/crates/sqlx-cli

```sh
cargo install sqlx-cli --no-default-features --features native-tls,postgres
```

Note, that this doesn't use the init.sh script nor WSL.

### Adding a Migration

```
 SET DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/newsletter
```

```
Creating migrations\20220627113616_create_subscriptions_table.sql

Congratulations on creating your first migration!

Did you know you can embed your migrations in your application binary?
On startup, after creating your database connection or pool, add:

sqlx::migrate!().run(<&your_pool OR &mut your_connection>).await?;

Note that the compiler won't pick up new migrations if no Rust source files have changed.
You can create a Cargo build script to work around this with `sqlx migrate build-script`.
```

```sh
sqlx migrate run --database-url postgres://postgres:password@127.0.0.1:5432/newsletter
```

Todo:

Add

```
cargo add uuid
cargo add chrono
```

chrono will work for Utc::now

Cargo test no capture

### 4.4.1 Interactions with External Systems

Let’s start with a tried-and-tested rule of thumb: any interaction with external systems over the
network should be closely monitored. We might experience networking issues, the database might be
unavailable, queries might get slower over time as the subscribers table gets longer, etc.

We need a way to correlate all logs related to the same request.
This is usually achieved using a request id (also known as correlation id): when we start to process
an incoming request we generate a random identifier (e.g. a UUID) which is then associated to all
logs concerning the fulfilling of that specific request

### 4.5 Structured Logging

This is a must read section of the book. Go to page 88.

### 4.9

In a large project it is very diffcult to spot that a dependency has become unused after a refactoring.
Luckily enough, tooling comes to the rescue once again - let’s install cargo-udeps (unused
dependencies):
