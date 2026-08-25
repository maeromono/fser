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
(vim-like, but not actually vim cuz it doesn't have an insert mode)
- j/k -- Scroll up/down
- d/u -- Scroll up/down but faster
- n/p -- Next/previous page/chapter
- b -- Add/remove bookmark
- m -- Ebook meta page
- q -- Quit fser

## Roadmap.
- [X] Separate ebook metadata page
- [X] Bookmarking support
- [ ] ???

## License.
GPL-3.0 only. Read more in [the license file](https://codeberg.org/enmkdev/fser/src/branch/main/LICENSE.md).
