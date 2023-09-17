use std::collections::HashMap;

use super::asm_parser::{Program, Instruction};

pub struct AsmMachine {
  pub registers: HashMap<String, usize>
}

impl AsmMachine {
  pub fn new(registers: HashMap<String, usize>) -> Self {
    Self { registers }
  }

  pub fn cycle(&mut self, program: &Program) {
    for instr in program.instructions.iter() {
      match instr {
        Instruction::ADD(operands) => {
          let rs1 = self.registers.get(&operands.0).unwrap_or(&0);
          let rs2 = self.registers.get(&operands.1).unwrap_or(&0);
          self.registers.insert(operands.2.clone(), rs1 + rs2);
        },
        _ => {}
      }
    }
  }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_machine() {
    let prog = Program{instructions: vec![
      Instruction::ADD(("R1".to_string(),"R2".to_string(),"R3".to_string()))
    ]};
    let mut vm = AsmMachine::new(HashMap::from([
      ("R1".to_string(), 5),
      ("R2".to_string(), 2),
      ("R3".to_string(), 0),
    ]));
    vm.cycle(&prog);

    println!("{:#?}", vm.registers);
	}
}