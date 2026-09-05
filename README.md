# fser (Fucking Simple E-Reader)

fser is an intentionally dumbed-down, offline-only TUI EPUB reader focused on pure reading without bloat.

## Why?
Because none of the existing EPUB readers met my standards. Some were Python-based or had Python dependencies, some had too many or too few features.

## Installation.
Prerequisites:
- Rust/Cargo installed on your system (preferrably latest possible version)

### Through Cargo (recommended)
```bash
    cargo install --locked fser
```

### From source
```bash
    git clone https://codeberg.org/enmkdev/fser && cd fser && cargo install --path .
```

## Usage.
```bash
    fser (path to epub)
```

More arguments may or may not come in future updates.

## Binds.
q             Save progress & quit fser
?/F1          Bind help
n/p           Next/previous page/chapter
j/k, Up/Down  Scroll down/up
d/u           Scroll down/up but faster
g/G           Jump to top/bottom
m             View ebook metadata
b             Add/remove bookmark
B             Open bookmark menu

## Roadmap.
- [X] Separate ebook metadata page
- [X] Bookmarking support
- [ ] ???

## License.
GPL-3.0 only. Read more in [the license file](https://codeberg.org/enmkdev/fser/src/branch/main/LICENSE.md).
