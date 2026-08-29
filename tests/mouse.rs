use vt100::{
    MouseButton, MouseEvent, MouseEventKind, MouseModifiers, Parser,
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
