# glog for Rust

[![CI](https://github.com/cschlosser/glog-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/cschlosser/glog-rs/actions/workflows/ci.yml)

This is a port of the C++ logging framework [glog] as backend for Rusts [standard logging] frontend.

> ⚠️ Stability Warning:
> Currently there are no tests for this framework.
> This will be added in the next release.
> Currently the framework is tested only manually to verify the current featureset.

## Introduction

`glog-rs` tries to stay as close to [glog] as possible to [maintain compatibility](COMPATIBILITY.md), which can be useful in mixed environments using C++ and Rust code at the same time.

This includes default values for flags, flag names and behavior.

Additional options or configurations can be enabled before initializing the framework to use more of what the Rust [standard logging] frontend has to offer or to solve different use cases.

## Examples

The most basic example is this:

```rust
use log::*;
use glog::Flags;

glog::new().init(Flags::default()).unwrap();

info!("It works!");

```
which will write `I0401 12:34:56.987654   123 readme.rs:6] It works!` to the `INFO` log file.

If you want to have colored output on `stderr` as well consider initializing by using some of the flags:

```rust
glog::new().init(Flags {
        colorlogtostderr: true,
        alsologtostderr: true, // use logtostderr to only write to stderr and not to files
        ..Default::default()
    }).unwrap();
```

A non standard extension would the year in addition to month and day in the timestamp. This is possible by calling the `with_year` method prior to `init` like this:

```rust
glog::new()
    .with_year(true) // Add the year to the timestamp in the logfile
    .init(Flags {
        logtostderr: true, // don't write to log files
        ..Default::default()
    }).unwrap();

info!("With the year");
```

will print `I20210401 12:34:56.987654   123 readme.rs:11] With the year`.

## Inspirations

This project was inspired by the great C++ logging framework [glog] and [stderrlog-rs](https://github.com/cardoe/stderrlog-rs) as a kickstarter for a Rust logging backend.

[glog]: https://github.com/google/glog
[standard logging]: https://crates.io/crates/log
