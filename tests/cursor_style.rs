use vt100::{CursorShape, CursorStyle, Parser};

#[test]
fn decscusr_and_mode_12_are_engine_state() {
    let cases = [
        (
            1,
            CursorStyle {
                shape: CursorShape::Block,
                blinking: true,
            },
        ),
        (
            2,
            CursorStyle {
                shape: CursorShape::Block,
                blinking: false,
            },
        ),
        (
            3,
            CursorStyle {
                shape: CursorShape::Underline,
                blinking: true,
            },
        ),
        (
            4,
            CursorStyle {
                shape: CursorShape::Underline,
                blinking: false,
            },
        ),
        (
            5,
            CursorStyle {
                shape: CursorShape::Bar,
                blinking: true,
            },
        ),
        (
            6,
            CursorStyle {
                shape: CursorShape::Bar,
                blinking: false,
            },
        ),
    ];
    for (parameter, expected) in cases {
        let mut parser = Parser::default();
        parser.process(format!("\x1b[{parameter} q").as_bytes());
        assert_eq!(parser.screen().cursor_style(), expected);
    }

    let mut parser = Parser::default();
    parser.process(b"\x1b[6 q\x1b[?12h");
    assert_eq!(
        parser.screen().cursor_style(),
        CursorStyle {
            shape: CursorShape::Bar,
            blinking: true
        }
    );
    parser.process(b"\x1b[?12l");
    assert_eq!(
        parser.screen().cursor_style(),
        CursorStyle {
            shape: CursorShape::Bar,
            blinking: false
        }
    );
}
