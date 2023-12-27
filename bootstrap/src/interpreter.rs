//! Rage Bootstrap Interpreter

use std::{thread::sleep, time::Duration};

pub struct Interpreter {
    instructions: InstructionTree,
}

impl Interpreter {
    pub fn new(instruction_tree: InstructionTree) -> Self {
        Self {
            instructions: instruction_tree,
        }
    }

    pub fn execute(&self) -> anyhow::Result<()> {
        log::info!("starting interpreter");
        log::error!("interpreter is unimplemented!");
        Ok(())
    }
}

pub struct InstructionTree;
