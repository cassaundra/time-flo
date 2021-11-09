# TimeFlo
Copyright &copy; 2021 Cassaundra Smith

TimeFlo is an implementation of a
[Pomodoro&reg;](https://en.wikipedia.org/wiki/Pomodoro_Technique)-like
timer for breaking out of flow state.

*Description of your TimeFlo timer.*

## Status and Roadmap

*Status of the MVP. Describe what works and what
doesn't. Describe "future work" for this project.*

* [ ] Requirements complete.
* [ ] Project plan complete.
* [ ] Design complete.
* [ ] Implementation complete.
* [ ] Validation complete.

## Build and Run

Ensure you have the latest Rust toolchain (stable or nightly will work fine).
Then, install the following dependencies according to [egui's template](https://github.com/emilk/eframe_template/):

On most apt-based Linux distributions:
```
sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev
```

On Fedora:
```
dnf install clang clang-devel clang-tools-extra speech-dispatcher-devel libxkbcommon-devel pkg-config openssl-devel
```

Ensure you have the requisite dependencies installed:
- The latest Rust toolchain (at least stable)

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
