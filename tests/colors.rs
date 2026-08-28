use vt100::{Parser, RgbColor};

#[test]
fn osc_color_overrides_and_resets_are_engine_state() {
    let mut parser = Parser::new(2, 4, 0);
    parser.process(
        b"\x1b]4;1;#123456\x07\x1b]10;#abcdef\x07\x1b]11;#223344\x07\x1b]12;#654321\x07",
    );
    let colors = parser.screen().theme_overrides();
    assert_eq!(
        colors.ansi[1],
        Some(RgbColor {
            r: 0x12,
            g: 0x34,
            b: 0x56
        })
    );
    assert_eq!(
        colors.foreground,
        Some(RgbColor {
            r: 0xab,
            g: 0xcd,
            b: 0xef
        })
    );
    assert_eq!(
        colors.background,
        Some(RgbColor {
            r: 0x22,
            g: 0x33,
            b: 0x44
        })
    );
    assert_eq!(
        colors.cursor,
        Some(RgbColor {
            r: 0x65,
            g: 0x43,
            b: 0x21
        })
    );

    parser.process(b"\x1b]104;1\x07\x1b]110\x07\x1b]111\x07\x1b]112\x07");
    let reset = parser.screen().theme_overrides();
    assert_eq!(reset.ansi[1], None);
    assert_eq!(
        (reset.foreground, reset.background, reset.cursor),
        (None, None, None)
    );
}

#[test]
fn ris_clears_every_terminal_color_override() {
    let mut parser = Parser::new(2, 4, 0);
    parser.process(b"\x1b]10;#abcdef\x07\x1b]4;7;#123456\x07\x1bc");
    let colors = parser.screen().theme_overrides();
    assert_eq!(colors.foreground, None);
    assert_eq!(colors.ansi[7], None);
}
