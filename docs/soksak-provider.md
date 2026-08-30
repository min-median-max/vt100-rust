# Soksak provider branch

This fork extends vt100-rust 0.16.2 with engine APIs required by the Soksak terminal provider.
Components pin an exact commit from the versioned provider branch and never use a workspace path.

`Screen::cursor_style()` exposes parser-owned block, underline, or bar shape plus blink state.
DECSCUSR selects both fields. DECSET/DECRST 12 changes only blink, while DECTCEM continues to own
visibility through `Screen::hide_cursor()`. Consumers do not parse the byte stream again.

`MouseProtocolMode::X10` is the live DEC 9 state and `MouseProtocolMode::Highlight` is the distinct
DEC 1001 state. Neither is aliased to VT200 press/release tracking. X10 admits button presses only
and suppresses modifier bits in every encoding. Highlight tracking admits ordinary press/release
events, preserves legacy modifier bits, and does not admit motion through the ordinary pointer
event API. `input_mode_formatted()` and `input_mode_diff()` serialize both private modes so a fresh
parser observes the same live state.

The owner tests are:

```sh
python3 -m unittest tests.cursor_state_source
cargo test --test cursor_style
cargo test --test mouse
```
