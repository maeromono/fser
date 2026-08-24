# fser (Fucking Simple E-Reader)

fser is an intentionally dumbed-down TUI EPUB reader focused on pure reading without bloat.

## Why?
Because none of the existing EPUB readers met my standards. Some were Python-based or had Python dependencies, some had too many or too few features.

## Installation
Prerequisites:
- Rust/Cargo installed on your system

### Through Cargo (recommended)
```bash
    cargo install fser
```

### From source
```bash
    git clone https://codeberg.org/enmkdev/fser && cd fser && cargo install --path .
```

## Usage
```bash
    fser (path to epub)
```

More arguments may or may not come in future updates.

## Binds
(vim-like, but not actually vim cuz it doesn't have an insert mode)
- j/k -- Scroll up/down
- d/u -- Scroll up/down but faster
- n/p -- Next/previous page/chapter
- q -- Quit fser

## Roadmap
- [ ] Separate ebook metadata page
- [ ] Bookmarking support
