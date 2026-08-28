from pathlib import Path
import unittest


class CursorStateSourceTest(unittest.TestCase):
    def test_screen_exposes_parsed_cursor_state(self):
        screen = Path("src/screen.rs").read_text()
        performer = Path("src/perform.rs").read_text()
        self.assertIn("pub enum CursorShape", screen)
        self.assertIn("pub struct CursorStyle", screen)
        self.assertIn("pub fn cursor_style(&self)", screen)
        self.assertIn("set_cursor_style", performer)


if __name__ == "__main__":
    unittest.main()
