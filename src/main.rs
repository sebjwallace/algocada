use std::borrow::BorrowMut;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use ruscii::drawing::Pencil;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Window};
use ruscii::{
    app::{App, Config, State},
};

struct AsmTable {
    pub code: Vec<Vec<String>>,
    pub cursor: Vec2,
}

impl AsmTable {
    pub fn edit_cell(&mut self, text: &str) {
        if let Some(asm_row) = self.code.get_mut(self.cursor.y as usize) {
            let asm_cell = &mut asm_row[self.cursor.x as usize];
            if asm_cell.len() == 5 { return }
            *asm_cell = [asm_cell, text].join("").to_string();
        }
    }

    pub fn backspace_cell(&mut self) {
        if let Some(asm_row) = self.code.get_mut(self.cursor.y as usize) {
            let asm_cell = &mut asm_row[self.cursor.x as usize];
            if asm_cell.len() == 0 { return }
            *asm_cell = asm_cell[0..asm_cell.len()-1].to_string();
        }
    }

    pub fn draw(&self, pen: &mut Pencil) {
        for (y, row) in self.code.iter().enumerate() {
            let mut offx = 0;
            for (x, col) in row.iter().enumerate() {
                let pos = Vec2::xy(offx, y);
                let is_selected = (x as i32) == self.cursor.x && (y as i32) == self.cursor.y;
                if is_selected { pen.set_background(Color::White); } else { pen.set_background(Color::Black); }
                if is_selected { pen.set_foreground(Color::Black); } else { pen.set_foreground(Color::White); }
                let text: String = format!("{:<width$}", col, width = 5)[0..4].to_string();
                pen.draw_text(text.as_str(), pos);
                offx += text.len() + 1;
            } 
        }
    }
}

fn main() {
    let mut app = App::config(Config { fps: 60 });
    let mut asm_table = AsmTable{
        code: vec![
            vec!["MOV".to_string(), "ASX".to_string(), "1".to_string()],
            vec!["MOV".to_string(), "".to_string(), "1".to_string()],
        ],
        cursor: Vec2::xy(0,0)
    };

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Up) => { asm_table.cursor.y -= 1 }
                KeyEvent::Pressed(Key::Down) => { asm_table.cursor.y += 1 }
                KeyEvent::Pressed(Key::Left) => { asm_table.cursor.x -= 1 }
                KeyEvent::Pressed(Key::Right) => { asm_table.cursor.x += 1 }
                KeyEvent::Pressed(Key::Backspace) => { asm_table.backspace_cell() }
                KeyEvent::Pressed(Key::A) => { asm_table.edit_cell("A") }
                KeyEvent::Pressed(Key::B) => { asm_table.edit_cell("B") }
                KeyEvent::Pressed(Key::C) => { asm_table.edit_cell("C") }
                KeyEvent::Pressed(Key::D) => { asm_table.edit_cell("D") }
                KeyEvent::Pressed(Key::E) => { asm_table.edit_cell("E") }
                KeyEvent::Pressed(Key::F) => { asm_table.edit_cell("F") }
                KeyEvent::Pressed(Key::G) => { asm_table.edit_cell("G") }
                KeyEvent::Pressed(Key::H) => { asm_table.edit_cell("H") }
                KeyEvent::Pressed(Key::I) => { asm_table.edit_cell("I") }
                KeyEvent::Pressed(Key::J) => { asm_table.edit_cell("J") }
                KeyEvent::Pressed(Key::K) => { asm_table.edit_cell("K") }
                KeyEvent::Pressed(Key::L) => { asm_table.edit_cell("L") }
                KeyEvent::Pressed(Key::M) => { asm_table.edit_cell("M") }
                KeyEvent::Pressed(Key::N) => { asm_table.edit_cell("N") }
                KeyEvent::Pressed(Key::O) => { asm_table.edit_cell("O") }
                KeyEvent::Pressed(Key::P) => { asm_table.edit_cell("P") }
                KeyEvent::Pressed(Key::Q) => { asm_table.edit_cell("Q") }
                KeyEvent::Pressed(Key::R) => { asm_table.edit_cell("R") }
                KeyEvent::Pressed(Key::S) => { asm_table.edit_cell("S") }
                KeyEvent::Pressed(Key::T) => { asm_table.edit_cell("T") }
                KeyEvent::Pressed(Key::U) => { asm_table.edit_cell("U") }
                KeyEvent::Pressed(Key::V) => { asm_table.edit_cell("V") }
                KeyEvent::Pressed(Key::W) => { asm_table.edit_cell("W") }
                KeyEvent::Pressed(Key::X) => { asm_table.edit_cell("X") }
                KeyEvent::Pressed(Key::Y) => { asm_table.edit_cell("Y") }
                KeyEvent::Pressed(Key::Z) => { asm_table.edit_cell("Z") }
                KeyEvent::Pressed(Key::Num0) => { asm_table.edit_cell("0") }
                KeyEvent::Pressed(Key::Num1) => { asm_table.edit_cell("1") }
                KeyEvent::Pressed(Key::Num2) => { asm_table.edit_cell("2") }
                KeyEvent::Pressed(Key::Num3) => { asm_table.edit_cell("3") }
                KeyEvent::Pressed(Key::Num4) => { asm_table.edit_cell("4") }
                KeyEvent::Pressed(Key::Num5) => { asm_table.edit_cell("5") }
                KeyEvent::Pressed(Key::Num6) => { asm_table.edit_cell("6") }
                KeyEvent::Pressed(Key::Num7) => { asm_table.edit_cell("7") }
                KeyEvent::Pressed(Key::Num8) => { asm_table.edit_cell("8") }
                KeyEvent::Pressed(Key::Num9) => { asm_table.edit_cell("9") }
                _ => {},
            }
        }


        let mut pen = Pencil::new(window.canvas_mut());
        let origin = Vec2::xy(0, 0);
        pen.set_origin(origin);
        asm_table.draw(&mut pen);
    });
}

fn pad_to_width(input: &str, width: usize, pad_char: char) -> String {
    format!("{:<width$}", input, width = width).replace(" ", &pad_char.to_string())
}