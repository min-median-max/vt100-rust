// DEC line-drawing / Special Graphics character set support: G0/G1 designation
// (`ESC ( <final>` / `ESC ) <final>`), SI/SO to invoke a G-set into GL, glyph
// translation on print, DECSC/DECRC of the charset state, and persistence
// across the alternate screen.

#[test]
fn g0_special_graphics() {
    let mut parser = vt100::Parser::new(24, 80, 0);
    // Designate the DEC Special Graphics set into G0 (the default GL); the
    // ASCII bytes lqk render as the box-drawing glyphs.
    parser.process(b"\x1b(0lqk");
    assert_eq!(parser.screen().cell(0, 0).unwrap().contents(), "┌");
    assert_eq!(parser.screen().cell(0, 1).unwrap().contents(), "─");
    assert_eq!(parser.screen().cell(0, 2).unwrap().contents(), "┐");
    assert_eq!(parser.screen().contents(), "┌─┐");

    // `ESC ( B` returns G0 to US ASCII; subsequent bytes are literal again.
    parser.process(b"\x1b(Babc");
    assert_eq!(parser.screen().cell(0, 3).unwrap().contents(), "a");
    assert_eq!(parser.screen().cell(0, 4).unwrap().contents(), "b");
    assert_eq!(parser.screen().cell(0, 5).unwrap().contents(), "c");
}

#[test]
fn shift_out_and_in() {
    let mut parser = vt100::Parser::new(24, 80, 0);
    // G0 = line drawing, G1 stays ASCII (the default). GL starts on G0.
    parser.process(b"\x1b(0");
    parser.process(b"q"); // G0 active -> horizontal line
    parser.process(b"\x0eq"); // SO invokes G1 (ASCII) -> literal q
    parser.process(b"\x0fq"); // SI invokes G0 (line drawing) -> line
    assert_eq!(parser.screen().cell(0, 0).unwrap().contents(), "─");
    assert_eq!(parser.screen().cell(0, 1).unwrap().contents(), "q");
    assert_eq!(parser.screen().cell(0, 2).unwrap().contents(), "─");
}

#[test]
fn g1_special_graphics() {
    let mut parser = vt100::Parser::new(24, 80, 0);
    // Designate line drawing into G1 and invoke it with SO; G0 stays ASCII.
    parser.process(b"\x1b)0");
    parser.process(b"x"); // GL = G0 = ASCII -> literal x
    parser.process(b"\x0ex\x0f"); // SO -> G1 line drawing -> vertical, then SI
    assert_eq!(parser.screen().cell(0, 0).unwrap().contents(), "x");
    assert_eq!(parser.screen().cell(0, 1).unwrap().contents(), "│");
}

#[test]
fn decsc_decrc_saves_charset() {
    let mut parser = vt100::Parser::new(24, 80, 0);
    parser.process(b"\x1b(0"); // G0 = line drawing
    parser.process(b"\x1b7"); // DECSC saves cursor + charset state
    parser.process(b"\x1b(B"); // G0 = ASCII
    parser.process(b"AB"); // literal, cursor advances
    parser.process(b"\x1b8"); // DECRC restores cursor + charset (line drawing)
    parser.process(b"q");
    // The restore reinstated the line-drawing set, so q maps again.
    assert_eq!(parser.screen().cell(0, 0).unwrap().contents(), "─");
}

#[test]
fn charset_survives_alternate_screen() {
    let mut parser = vt100::Parser::new(24, 80, 0);
    parser.process(b"\x1b(0"); // G0 = line drawing (terminal-global)
    parser.process(b"\x1b[?1049h"); // enter the alternate screen
    parser.process(b"lqk");
    assert_eq!(parser.screen().cell(0, 0).unwrap().contents(), "┌");
    assert_eq!(parser.screen().cell(0, 2).unwrap().contents(), "┐");
    parser.process(b"\x1b[?1049l"); // return to the primary screen
    parser.process(b"mqj");
    // The designation persisted across the switch on both screens.
    assert_eq!(parser.screen().cell(0, 0).unwrap().contents(), "└");
    assert_eq!(parser.screen().cell(0, 2).unwrap().contents(), "┘");
}

#[test]
fn unrecognized_charset_final_is_unhandled() {
    struct State {
        unhandled: Vec<(Option<u8>, u8)>,
    }

    impl vt100::Callbacks for State {
        fn unhandled_escape(
            &mut self,
            _: &mut vt100::Screen,
            intermediate: Option<u8>,
            _: Option<u8>,
            b: u8,
        ) {
            self.unhandled.push((intermediate, b));
        }
    }

    let mut parser = vt100::Parser::new_with_callbacks(
        24,
        80,
        0,
        State { unhandled: vec![] },
    );
    // A designation this parser does not implement still reports as unhandled
    // rather than being silently swallowed.
    parser.process(b"\x1b(A");
    assert_eq!(parser.callbacks().unhandled, vec![(Some(b'('), b'A')]);
}
