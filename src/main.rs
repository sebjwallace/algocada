use ruscii::drawing::{Pencil,RectCharset};
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Window};
use ruscii::{
    app::{App, Config, State},
};
use boothshaw::asm::editor::AsmEditor;


fn main() {
    let mut app = App::config(Config { fps: 60 });
    let mut asm_editor = AsmEditor::new(Vec2::xy(1,0), Vec2::xy(4, 20));

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Up) => { asm_editor.move_cursor_y(-1) }
                KeyEvent::Pressed(Key::Down) => { asm_editor.move_cursor_y(1) }
                KeyEvent::Pressed(Key::Left) => { asm_editor.move_cursor_x(-1) }
                KeyEvent::Pressed(Key::Right) => { asm_editor.move_cursor_x(1) }
                KeyEvent::Pressed(Key::Backspace) => { asm_editor.backspace_cell() }
                KeyEvent::Pressed(Key::A) => { asm_editor.edit_cell("A") }
                KeyEvent::Pressed(Key::B) => { asm_editor.edit_cell("B") }
                KeyEvent::Pressed(Key::C) => { asm_editor.edit_cell("C") }
                KeyEvent::Pressed(Key::D) => { asm_editor.edit_cell("D") }
                KeyEvent::Pressed(Key::E) => { asm_editor.edit_cell("E") }
                KeyEvent::Pressed(Key::F) => { asm_editor.edit_cell("F") }
                KeyEvent::Pressed(Key::G) => { asm_editor.edit_cell("G") }
                KeyEvent::Pressed(Key::H) => { asm_editor.edit_cell("H") }
                KeyEvent::Pressed(Key::I) => { asm_editor.edit_cell("I") }
                KeyEvent::Pressed(Key::J) => { asm_editor.edit_cell("J") }
                KeyEvent::Pressed(Key::K) => { asm_editor.edit_cell("K") }
                KeyEvent::Pressed(Key::L) => { asm_editor.edit_cell("L") }
                KeyEvent::Pressed(Key::M) => { asm_editor.edit_cell("M") }
                KeyEvent::Pressed(Key::N) => { asm_editor.edit_cell("N") }
                KeyEvent::Pressed(Key::O) => { asm_editor.edit_cell("O") }
                KeyEvent::Pressed(Key::P) => { asm_editor.edit_cell("P") }
                KeyEvent::Pressed(Key::Q) => { asm_editor.edit_cell("Q") }
                KeyEvent::Pressed(Key::R) => { asm_editor.edit_cell("R") }
                KeyEvent::Pressed(Key::S) => { asm_editor.edit_cell("S") }
                KeyEvent::Pressed(Key::T) => { asm_editor.edit_cell("T") }
                KeyEvent::Pressed(Key::U) => { asm_editor.edit_cell("U") }
                KeyEvent::Pressed(Key::V) => { asm_editor.edit_cell("V") }
                KeyEvent::Pressed(Key::W) => { asm_editor.edit_cell("W") }
                KeyEvent::Pressed(Key::X) => { asm_editor.edit_cell("X") }
                KeyEvent::Pressed(Key::Y) => { asm_editor.edit_cell("Y") }
                KeyEvent::Pressed(Key::Z) => { asm_editor.edit_cell("Z") }
                KeyEvent::Pressed(Key::Num0) => { asm_editor.edit_cell("0") }
                KeyEvent::Pressed(Key::Num1) => { asm_editor.edit_cell("1") }
                KeyEvent::Pressed(Key::Num2) => { asm_editor.edit_cell("2") }
                KeyEvent::Pressed(Key::Num3) => { asm_editor.edit_cell("3") }
                KeyEvent::Pressed(Key::Num4) => { asm_editor.edit_cell("4") }
                KeyEvent::Pressed(Key::Num5) => { asm_editor.edit_cell("5") }
                KeyEvent::Pressed(Key::Num6) => { asm_editor.edit_cell("6") }
                KeyEvent::Pressed(Key::Num7) => { asm_editor.edit_cell("7") }
                KeyEvent::Pressed(Key::Num8) => { asm_editor.edit_cell("8") }
                KeyEvent::Pressed(Key::Num9) => { asm_editor.edit_cell("9") }
                KeyEvent::Pressed(Key::Semicolon) => { asm_editor.edit_cell(":") }
                _ => {},
            }
        }

        let mut pen = Pencil::new(window.canvas_mut());
        let origin = Vec2::xy(0, 0);
        pen.set_origin(origin);
        asm_editor.draw(&mut pen);
    });
}
