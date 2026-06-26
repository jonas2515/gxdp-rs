# Rust bindings for libgxdp

Bindings for [libgxdp](https://gitlab.gnome.org/GNOME/libgxdp) from Rust, to allow implementing the GNOME portal backend using Rust.

## Requires forked libgxdp

To build the project, libgxdp needs to be installed as a shared library and provide a pkg-config file. For this, build and install the library from [my fork of libgxdp](https://gitlab.gnome.org/verdre/libgxdp/-/tree/make-shared-library).
