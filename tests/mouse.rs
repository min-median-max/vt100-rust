use vt100::{
    MouseButton, MouseEncodeError, MouseEvent, MouseEventKind,
    MouseModifiers, MouseProtocolMode, Parser,
};

fn event(kind: MouseEventKind, button: MouseButton) -> MouseEvent {
    MouseEvent {
        row: 2,
        column: 1,
        kind,
        button,
        modifiers: MouseModifiers::default(),
    }
}

#[test]
fn live_screen_encodes_sgr_press_drag_release_and_free_motion() {
    let mut parser = Parser::new(40, 120, 0);
    parser.process(b"\x1b[?1002h\x1b[?1006h");
    assert_eq!(
        parser
            .screen()
            .encode_mouse_event(event(
                MouseEventKind::Press,
                MouseButton::Left
            ))
            .unwrap(),
        b"\x1b[<0;2;3M",
    );
    assert_eq!(
        parser
            .screen()
            .encode_mouse_event(event(
                MouseEventKind::Motion,
                MouseButton::Left
            ))
            .unwrap(),
        b"\x1b[<32;2;3M",
    );
    assert_eq!(
        parser
            .screen()
            .encode_mouse_event(event(
                MouseEventKind::Release,
                MouseButton::Left
            ))
            .unwrap(),
        b"\x1b[<0;2;3m",
    );

    parser.process(b"\x1b[?1002l\x1b[?1003h");
    assert_eq!(
        parser
            .screen()
            .encode_mouse_event(event(
                MouseEventKind::Motion,
                MouseButton::None
            ))
            .unwrap(),
        b"\x1b[<35;2;3M",
    );
}

#[test]
fn dec9_x10_suppresses_modifiers_and_reports_only_presses() {
    let mut parser = Parser::new(40, 120, 0);
    parser.process(b"\x1b[?9h");
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        MouseProtocolMode::X10
    );

    let modified = MouseModifiers {
        shift: true,
        alt: true,
        control: true,
        meta: true,
    };
    assert_eq!(
        parser
            .screen()
            .encode_mouse_event(MouseEvent {
                modifiers: modified,
                ..event(MouseEventKind::Press, MouseButton::Left)
            })
            .unwrap(),
        [0x1b, b'[', b'M', 32, 34, 35],
    );
    assert_eq!(
        parser.screen().encode_mouse_event(MouseEvent {
            modifiers: modified,
            ..event(MouseEventKind::Release, MouseButton::Left)
        }),
        Err(MouseEncodeError::EventNotReported),
    );
}

#[test]
fn dec1001_highlight_is_distinct_and_reports_press_and_release() {
    let mut parser = Parser::new(40, 120, 0);
    parser.process(b"\x1b[?1001h");
    assert_eq!(
        parser.screen().mouse_protocol_mode(),
        MouseProtocolMode::Highlight
    );

    let modified = MouseModifiers {
        shift: true,
        alt: true,
        control: true,
        meta: false,
    };
    assert_eq!(
        parser
            .screen()
            .encode_mouse_event(MouseEvent {
                modifiers: modified,
                ..event(MouseEventKind::Press, MouseButton::Left)
            })
            .unwrap(),
        [0x1b, b'[', b'M', 60, 34, 35],
    );
    assert_eq!(
        parser
            .screen()
            .encode_mouse_event(MouseEvent {
                modifiers: modified,
                ..event(MouseEventKind::Release, MouseButton::Left)
            })
            .unwrap(),
        [0x1b, b'[', b'M', 63, 34, 35],
    );
    assert_eq!(
        parser.screen().encode_mouse_event(event(
            MouseEventKind::Motion,
            MouseButton::Left
        )),
        Err(MouseEncodeError::EventNotReported),
    );
}

#[test]
fn input_mode_serialization_rehydrates_x10_and_highlight_without_aliasing() {
    for (sequence, expected) in [
        (b"\x1b[?9h".as_slice(), MouseProtocolMode::X10),
        (b"\x1b[?1001h".as_slice(), MouseProtocolMode::Highlight),
    ] {
        let mut source = Parser::new(40, 120, 0);
        source.process(sequence);

        let mut rehydrated = Parser::new(40, 120, 0);
        rehydrated.process(&source.screen().input_mode_formatted());
        assert_eq!(rehydrated.screen().mouse_protocol_mode(), expected);
    }

    let mut parser = Parser::new(40, 120, 0);
    parser.process(b"\x1b[?9h");
    let x10 = parser.screen().clone();
    parser.process(b"\x1b[?1001h");
    let highlight = parser.screen();

    let mut from_x10 = Parser::new(40, 120, 0);
    from_x10.process(b"\x1b[?9h");
    from_x10.process(&highlight.input_mode_diff(&x10));
    assert_eq!(
        from_x10.screen().mouse_protocol_mode(),
        MouseProtocolMode::Highlight
    );
}
