# TimeFlo
Copyright &copy; 2021 Cassaundra Smith

TimeFlo is an implementation of a
[Pomodoro&reg;](https://en.wikipedia.org/wiki/Pomodoro_Technique)-like timer for
breaking out of flow state.

![TimeFlo screenshot](docs/screenshot.png)

## Status and Roadmap

* [x] Requirements complete.
* [x] Project plan complete.
* [x] Design complete.
* [x] Implementation complete.
* [ ] Validation complete.

## Build and Run

Ensure you have the latest Rust toolchain (at least stable).
Then, install the following dependencies according to the [egui template](https://github.com/emilk/eframe_template/):

On most apt-based Linux distributions:
```
sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev
```

On Fedora:
```
sudo dnf install clang clang-devel clang-tools-extra speech-dispatcher-devel libxkbcommon-devel pkg-config openssl-devel
```

To build:
```shell
cargo build --release
```


## Development Docs

Development documentation is available for TimeFlo, including:

* [Requirements Specification](docs/reqs.md)
* [Project Plan](docs/plan.md)
* [Design Doc](docs/design.md)
* [V&amp;V Report](docs/vnv.md)
