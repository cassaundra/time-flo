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

### Manual build

Install the requisite dependencies via Flatpak:

```
flatpak install org.gnome.Sdk//40 org.freedesktop.Sdk.Extension.rust-stable//20.08 org.gnome.Platform//40
```

Build:
```
flatpak-builder --user flatpak_app build-aux/io.cassaundra.TimeFlo.Devel.json
```

Run (after building):
```
flatpak-builder --run flatpak_app build-aux/io.cassaundra.TimeFlo.Devel.json time-flo
```

### Build with IDE

The program can be built with Gnome Builder out of the box, and *probably* VS Code as well, though this is so far untested.

## Development Docs

Development documentation is available for TimeFlo, including:

* [Requirements Specification](docs/requirements.md)
* [Project Plan](docs/plan.md)
* [Design Doc](docs/design.md)
* [V&amp;V Report](docs/vnv.md)
