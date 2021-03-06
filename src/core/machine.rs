use crate::common::types::Word;
use crate::core::code::Code;
use crate::core::instruction::Instruction;
use crate::core::memory::Memory;
use crate::core::stack::{Stack, MAX_STACK_DEPTH};
use crate::core::state::State;

#[derive(Clone, Copy, Debug)]
pub enum MachineError {
    InsufficientArguments,
    OutOfBounds,
    StackFull,
    StackEmpty,
    ArithmeticOverflow,
    IllegalInstruction,
}

#[derive(Clone, Debug)]
pub struct Machine {
    pub state: State,
    pub prog: Code,
}

impl Machine {
    pub fn new(prog: Code) -> Self {
        Self {
            state: Default::default(),
            prog,
        }
    }

    pub fn step(
        state: State,
        instruction: Instruction,
    ) -> Result<State, MachineError> {
        match instruction {
            Instruction::Nop => ops::nop(state),
            Instruction::Halt => ops::halt(state),
            Instruction::Load => ops::load(state),
            Instruction::Store => ops::store(state),
            Instruction::Push => ops::push(state),
            Instruction::Pop => ops::pop(state),
            Instruction::Set(x) => ops::set(x, state),
            Instruction::Read => ops::read(state),
            Instruction::Write => ops::write(state),
            Instruction::Jump => ops::jump(state),
            Instruction::Add => ops::add(state),
            Instruction::Sub => ops::sub(state),
            Instruction::Mul => ops::mul(state),
            Instruction::Div => ops::div(state),
            Instruction::Mod => ops::r#mod(state),
            Instruction::Cmp => ops::cmp(state),
            Instruction::And => ops::and(state),
            Instruction::Or => ops::or(state),
            Instruction::Not => ops::not(state),
            Instruction::Xor => ops::xor(state),
            _ => Err(MachineError::IllegalInstruction),
        }
    }

    pub fn run(&mut self) -> Result<State, MachineError> {
        let mut curr_pos: Word = 0;

        while (curr_pos as usize) < self.prog.0.len() {
            /* grab current instruction */
            let curr_instruction: Instruction = self.prog.0[curr_pos as usize];

            /* apply transition function */
            let new_state: State =
                Self::step(self.state.clone(), curr_instruction)?;

            /* write state */
            self.state = new_state;

            if curr_instruction == Instruction::Halt {
                return Ok(self.state.clone());
            }

            /* jump */
            curr_pos = self.state.pc;
        }

        Ok(self.state.clone())
    }

    pub fn run_callback(
        &mut self,
        f: &dyn Fn(State, Instruction),
    ) -> Result<State, MachineError> {
        let mut curr_pos: Word = 0;

        while (curr_pos as usize) < self.prog.0.len() {
            /* grab current instruction */
            let curr_instruction: Instruction = self.prog.0[curr_pos as usize];

            /* apply transition function */
            let new_state: State =
                Self::step(self.state.clone(), curr_instruction)?;

            /* callback */
            f(new_state.clone(), curr_instruction);

            /* write state */
            self.state = new_state;

            if curr_instruction == Instruction::Halt {
                return Ok(self.state.clone());
            }

            /* jump */
            curr_pos = self.state.pc;
        }

        Ok(self.state.clone())
    }
}

mod ops {
    use super::*;
    use crate::core::memory::LinearlyAddressable;

    const OPS_ARITY_LOAD: usize = 1;
    const OPS_ARITY_STORE: usize = 2;
    const OPS_ARITY_JUMP: usize = 1;
    const OPS_ARITY_ADD: usize = 2;
    const OPS_ARITY_SUB: usize = 2;
    const OPS_ARITY_MUL: usize = 2;
    const OPS_ARITY_DIV: usize = 2;
    const OPS_ARITY_MOD: usize = 2;
    const OPS_ARITY_CMP: usize = 2;
    const OPS_ARITY_AND: usize = 2;
    const OPS_ARITY_OR: usize = 2;
    const OPS_ARITY_NEG: usize = 1;
    const OPS_ARITY_XOR: usize = 2;

    pub fn nop(state: State) -> Result<State, MachineError> {
        Ok(State {
            pc: state.pc + 1,
            ..state
        })
    }

    pub fn halt(state: State) -> Result<State, MachineError> {
        Ok(state)
    }

    pub fn load(state: State) -> Result<State, MachineError> {
        if state.stack.depth() < OPS_ARITY_LOAD {
            Err(MachineError::InsufficientArguments)
        } else {
            Ok(State {
                pc: state.pc + 1,
                stack: {
                    let mut tmp_stack: Stack = state.stack.clone();
                    let address: Word = tmp_stack.pop().unwrap();
                    let data: Word = state.memory.read(address);
                    tmp_stack.push(data).unwrap();
                    tmp_stack
                },
                ..state
            })
        }
    }

    pub fn store(state: State) -> Result<State, MachineError> {
        if state.stack.depth() < OPS_ARITY_STORE {
            Err(MachineError::InsufficientArguments)
        } else {
            Ok(State {
                pc: state.pc + 1,
                stack: {
                    let mut tmp_stack: Stack = state.stack.clone();
                    let _address: Word = tmp_stack.pop().unwrap();
                    let _data: Word = tmp_stack.pop().unwrap();
                    tmp_stack
                },
                memory: {
                    let mut tmp_stack: Stack = state.stack.clone();
                    let address: Word = tmp_stack.pop().unwrap();
                    let data: Word = tmp_stack.pop().unwrap();

                    let mut tmp_memory: Memory = state.memory.clone();
                    tmp_memory.write(address, data);
                    tmp_memory
                },
                ..state
            })
        }
    }

    pub fn push(state: State) -> Result<State, MachineError> {
        if state.stack.depth() == MAX_STACK_DEPTH {
            Err(MachineError::StackFull)
        } else {
            Ok(State {
                pc: state.pc + 1,
                stack: {
                    let mut tmp_stack: Stack = state.stack.clone();
                    tmp_stack.push(state.reg).unwrap();
                    tmp_stack
                },
                ..state
            })
        }
    }

    pub fn pop(state: State) -> Result<State, MachineError> {
        if state.stack.depth() == 0 {
            Err(MachineError::StackEmpty)
        } else {
            Ok(State {
                pc: state.pc + 1,
                stack: {
                    let mut tmp_stack: Stack = state.stack.clone();
                    tmp_stack.pop().unwrap();
                    tmp_stack
                },
                reg: state.stack.clone().pop().unwrap(),
                ..state
            })
        }
    }

    pub fn set(value: Word, state: State) -> Result<State, MachineError> {
        Ok(State {
            pc: state.pc + 1,
            reg: value,
            ..state
        })
    }

    pub fn read(_state: State) -> Result<State, MachineError> {
        todo!()
    }

    pub fn write(_state: State) -> Result<State, MachineError> {
        todo!()
    }

    pub fn jump(state: State) -> Result<State, MachineError> {
        if state.stack.depth() < OPS_ARITY_JUMP {
            Err(MachineError::InsufficientArguments)
        } else {
            Ok(State {
                pc: state.stack.clone().pop().unwrap(),
                ..state
            })
        }
    }

    pub fn add(state: State) -> Result<State, MachineError> {
        if state.stack.depth() < OPS_ARITY_ADD {
            return Err(MachineError::InsufficientArguments);
        }

        let a: Word = state.stack.clone().pop().unwrap();
        let b: Word = {
            let mut tmp_stack: Stack = state.stack.clone();
            tmp_stack.pop().unwrap();
            tmp_stack.pop().unwrap()
        };

        if Word::checked_add(a, b).is_none() {
            Err(MachineError::ArithmeticOverflow)
        } else {
            Ok(State {
                pc: state.pc + 1,
                stack: {
                    let mut tmp_stack: Stack = state.stack.clone();
                    let a: Word = tmp_stack.pop().unwrap();
                    let b: Word = tmp_stack.pop().unwrap();
                    let c: Word = a + b;
                    tmp_stack.push(c).unwrap();
                    tmp_stack
                },
                ..state
            })
        }
    }

    pub fn sub(state: State) -> Result<State, MachineError> {
        if state.stack.depth() < OPS_ARITY_SUB {
            return Err(MachineError::InsufficientArguments);
        }

        let a: Word = state.stack.clone().pop().unwrap();
        let b: Word = {
            let mut tmp_stack: Stack = state.stack.clone();
            tmp_stack.pop().unwrap();
            tmp_stack.pop().unwrap()
        };

        if Word::checked_sub(a, b).is_none() {
            Err(MachineError::ArithmeticOverflow)
        } else {
            Ok(State {
                pc: state.pc + 1,
                stack: {
                    let mut tmp_stack: Stack = state.stack.clone();
                    let a: Word = tmp_stack.pop().unwrap();
                    let b: Word = tmp_stack.pop().unwrap();
                    let c: Word = a - b;
                    tmp_stack.push(c).unwrap();
                    tmp_stack
                },
                ..state
            })
        }
    }
    pub fn mul(state: State) -> Result<State, MachineError> {
        if state.stack.depth() < OPS_ARITY_MUL {
            return Err(MachineError::InsufficientArguments);
        }

        let a: Word = state.stack.clone().pop().unwrap();
        let b: Word = {
            let mut tmp_stack: Stack = state.stack.clone();
            tmp_stack.pop().unwrap();
            tmp_stack.pop().unwrap()
        };

        if Word::checked_mul(a, b).is_none() {
            Err(MachineError::ArithmeticOverflow)
        } else {
            Ok(State {
                pc: state.pc + 1,
                stack: {
                    let mut tmp_stack: Stack = state.stack.clone();
                    let a: Word = tmp_stack.pop().unwrap();
                    let b: Word = tmp_stack.pop().unwrap();
                    let c: Word = a * b;
                    tmp_stack.push(c).unwrap();
                    tmp_stack
                },
                ..state
            })
        }
    }

    pub fn div(state: State) -> Result<State, MachineError> {
        if state.stack.depth() < OPS_ARITY_DIV {
            return Err(MachineError::InsufficientArguments);
        }

        let a: Word = state.stack.clone().pop().unwrap();
        let b: Word = {
            let mut tmp_stack: Stack = state.stack.clone();
            tmp_stack.pop().unwrap();
            tmp_stack.pop().unwrap()
        };

        if Word::checked_div(a, b).is_none() {
            Err(MachineError::ArithmeticOverflow)
        } else {
            Ok(State {
                pc: state.pc + 1,
                stack: {
                    let mut tmp_stack: Stack = state.stack.clone();
                    let a: Word = tmp_stack.pop().unwrap();
                    let b: Word = tmp_stack.pop().unwrap();
                    let c: Word = a / b;
                    tmp_stack.push(c).unwrap();
                    tmp_stack
                },
                ..state
            })
        }
    }

    pub fn r#mod(state: State) -> Result<State, MachineError> {
        if state.stack.depth() < OPS_ARITY_MOD {
            Err(MachineError::InsufficientArguments)
        } else {
            Ok(State {
                pc: state.pc + 1,
                stack: {
                    let mut tmp_stack: Stack = state.stack.clone();
                    let a: Word = tmp_stack.pop().unwrap();
                    let b: Word = tmp_stack.pop().unwrap();
                    let c: Word = a % b;
                    tmp_stack.push(c).unwrap();
                    tmp_stack
                },
                ..state
            })
        }
    }

    pub fn cmp(state: State) -> Result<State, MachineError> {
        if state.stack.depth() < OPS_ARITY_CMP {
            Err(MachineError::InsufficientArguments)
        } else {
            Ok(State {
                pc: state.pc + 1,
                stack: {
                    let mut tmp_stack: Stack = state.stack.clone();
                    let a: Word = tmp_stack.pop().unwrap();
                    let b: Word = tmp_stack.pop().unwrap();
                    if a == b {
                        tmp_stack.push(1).unwrap();
                    } else {
                        tmp_stack.push(0).unwrap();
                    }

                    tmp_stack
                },
                ..state
            })
        }
    }

    pub fn and(state: State) -> Result<State, MachineError> {
        if state.stack.depth() < OPS_ARITY_AND {
            Err(MachineError::InsufficientArguments)
        } else {
            Ok(State {
                pc: state.pc + 1,
                stack: {
                    let mut tmp_stack: Stack = state.stack.clone();
                    let a: Word = tmp_stack.pop().unwrap();
                    let b: Word = tmp_stack.pop().unwrap();
                    let c: Word = a & b;
                    tmp_stack.push(c).unwrap();

                    tmp_stack
                },
                ..state
            })
        }
    }

    pub fn or(state: State) -> Result<State, MachineError> {
        if state.stack.depth() < OPS_ARITY_OR {
            Err(MachineError::InsufficientArguments)
        } else {
            Ok(State {
                pc: state.pc + 1,
                stack: {
                    let mut tmp_stack: Stack = state.stack.clone();
                    let a: Word = tmp_stack.pop().unwrap();
                    let b: Word = tmp_stack.pop().unwrap();
                    let c: Word = a | b;
                    tmp_stack.push(c).unwrap();

                    tmp_stack
                },
                ..state
            })
        }
    }

    pub fn not(state: State) -> Result<State, MachineError> {
        if state.stack.depth() < OPS_ARITY_NEG {
            Err(MachineError::InsufficientArguments)
        } else {
            Ok(State {
                pc: state.pc + 1,
                stack: {
                    let mut tmp_stack: Stack = state.stack.clone();
                    let a: Word = tmp_stack.pop().unwrap();
                    let b: Word = !a;
                    tmp_stack.push(b).unwrap();

                    tmp_stack
                },
                ..state
            })
        }
    }

    pub fn xor(state: State) -> Result<State, MachineError> {
        if state.stack.depth() < OPS_ARITY_XOR {
            Err(MachineError::InsufficientArguments)
        } else {
            Ok(State {
                pc: state.pc + 1,
                stack: {
                    let mut tmp_stack: Stack = state.stack.clone();
                    let a: Word = tmp_stack.pop().unwrap();
                    let b: Word = tmp_stack.pop().unwrap();
                    let c: Word = a ^ b;
                    tmp_stack.push(c).unwrap();

                    tmp_stack
                },
                ..state
            })
        }
    }
}
