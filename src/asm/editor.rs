use ruscii::drawing::{Pencil,RectCharset};
use ruscii::spatial::Vec2;
use ruscii::terminal::Color;

pub struct AsmEditor {
  pub code: Vec<Vec<String>>,
  pub cursor: Vec2,
  pub pos: Vec2,
  pub dims: Vec2,
}

impl AsmEditor {
  pub fn new(pos: Vec2, dims: Vec2) -> Self {
    let cursor = Vec2::xy(0,0);
    let code = (0..dims.y).map(|_| (0..dims.x)
      .map(|_| String::new()).collect())
      .collect();
    Self { code, cursor, pos, dims }
  }

  pub fn move_cursor_x(&mut self, amount: i32) {
    let total = self.cursor.x + amount;
    if total >= 0 && total < self.dims.x { self.cursor.x += amount }
  }

  pub fn move_cursor_y(&mut self, amount: i32) {
    let total = self.cursor.y + amount;
    if total >= 0 && total < self.dims.y { self.cursor.y += amount }
  }

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
      let pos = self.pos + Vec2::xy(2, 1);
      let gutter_w = 4;
      pen.draw_rect(&RectCharset::simple_round_lines(), self.pos, Vec2::xy(22 + gutter_w, self.code.len() + 2));
      for (y, row) in self.code.iter().enumerate() {
          pen.set_background(Color::Black);
          pen.set_foreground(Color::DarkGrey);
          pen.draw_text((y + 1).to_string().as_str(), Vec2::xy(0, y) + pos);
          let mut offx = 0;
          for (x, col) in row.iter().enumerate() {
              let pos = Vec2::xy(gutter_w + offx, y) + pos;
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