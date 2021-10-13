# TimeFlo
Copyright &copy; 2021 Cassaundra Smith

TimeFlo is an implementation of a
[Pomodoro&reg;](https://en.wikipedia.org/wiki/Pomodoro_Technique)-like timer for
breaking out of flow state.

*Description of your TimeFlo timer.*

## Status and Roadmap

*Status of the MVP. Describe what works and what doesn't. Describe "future work"
for this project.*

* [ ] Requirements complete.
* [ ] Project plan complete.
* [ ] Design complete.
* [ ] Implementation complete.
* [ ] Validation complete.

## Build and Run

*Instructions to build and run your project (TODO).*

This project is built on GTK, adapted from
[gtk-rust-template](https://gitlab.gnome.org/bilelmoussaoui/gtk-rust-template).

### Build with Ninja + Meson

Ensure you have the requisite dependencies installed:
- The latest Rust toolchain (at least stable).
- The latest [GTK 4](https://www.gtk.org/docs/installations/) development
  package).
- The [Ninja](https://ninja-build.org/) build system.
- The [Meson](https://mesonbuild.com/) build system.

After cloning the repository to your local system:

You can either install it system-wide:
```sh
meson --prefix=/usr build
ninja -C build
sudo ninja -C build install
```

Or per-user:
```sh
meson --prefix=~/.local build
ninja -C build install
```

To uninstall, use the following command either as the user or with sudo:
```
ninja -C build uninstall
```

### Build with Flatpak

Install the requisite dependencies:

```
flatpak install org.gnome.Sdk//40 org.freedesktop.Sdk.Extension.rust-stable//20.08 org.gnome.Platform//40
```

Build and install:
```
flatpak-builder --user --install --force-clean flatpak_app build-aux/io.cassaundra.TimeFlo.Devel.json
```

To uninstall, simply use Flatpak's package management tools.

### Build with IDE

The program can be built with GNOME Builder out of the box, and *probably* VS
Code as well, though this is so far untested.

## Development Docs

Development documentation is available for TimeFlo, including:

* [Requirements Specification](docs/requirements.md)
* [Project Plan](docs/plan.md)
* [Design Doc](docs/design.md)
* [V&amp;V Report](docs/vnv.md)
