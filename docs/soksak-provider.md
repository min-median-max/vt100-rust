# Soksak provider branch

This fork extends vt100-rust 0.16.2 with engine APIs required by the Soksak terminal provider.
Components pin an exact commit from the versioned provider branch and never use a workspace path.

`Screen::cursor_style()` exposes parser-owned block, underline, or bar shape plus blink state.
DECSCUSR selects both fields. DECSET/DECRST 12 changes only blink, while DECTCEM continues to own
visibility through `Screen::hide_cursor()`. Consumers do not parse the byte stream again.

The owner tests are:

```sh
python3 -m unittest tests.cursor_state_source
cargo test --test cursor_style
```
