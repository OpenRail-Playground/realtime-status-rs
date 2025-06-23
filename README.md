# realtime-status-rs

Sample client for reading realtime status XML messages from Python.

## Background

<p align="center">
  <img alt="Hack4Rail Logo" src="img/hack4rail-logo.jpg" width="220"/>
</p>

This project has been initiated during the [Hack4Rail 2025](https://hack4rail.event.sbb.ch/en/), a joint hackathon organised by the railway companies SBB, ÖBB, and DB in partnership with the OpenRail Association.

## Usage

### Redis Rust Client

Connects to a local Redis server and subscribes to messages on `hack4rail`.

### Run

```bash
cargo run --release
```

If you need to set parameters from the command line, use this syntax

```bash
 cargo run --release -- --redis-url redis://127.0.0.1 --channel hack4rail
```

## License

The content of this repository is licensed under the [Apache 2.0 license](LICENSE).
