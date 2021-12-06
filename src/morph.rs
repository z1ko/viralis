

use iced_x86::{Instruction, Mnemonic, OpKind, IcedError};
use iced_x86::code_asm::*;

// Rappresents a transformation 1-N
pub trait Transformation {

    // Criterio di accettazioni della trasformazione, riorna anche il numero di istruzioni digerite
    fn accepts(&self, instructions: &[Instruction]) -> (bool, usize);

    // Data l'istruzione che soddisfa il criterio di questa trasformazione
    // genera il suo codice semanticamente equivalente
    fn encode(&self, instructions: &[Instruction], e: &mut CodeAssembler) -> Result<(), IcedError>;
}

pub struct MovPushPop;
impl Transformation for MovPushPop {
    
    fn accepts(&self, instructions: &[Instruction]) -> (bool, usize) {

        let instr = &instructions[0]; 
        if instr.mnemonic() == Mnemonic::Mov {
            if instr.op0_kind() == OpKind::Register && instr.op1_kind() == OpKind::Register {
                return (true, 1);
            }
        }
        return (false, 0);
    }    

    // mov reg1, reg2 -> push reg2; pop reg1
    fn encode(&self, instructions: &[Instruction], e: &mut CodeAssembler) -> Result<(), IcedError> {

        // Lavora solo sull'istruzione corrente
        let instr = &instructions[0];

        // Estrazione argomenti da conservare
        if let Some(reg0) = gpr64::get_gpr64(instr.op0_register()) {
            if let Some(reg1) = gpr64::get_gpr64(instr.op1_register()) {

                // Operazioni equivalenti
                e.push(reg1)?;
                e.pop(reg0)?;

                return Ok(());
            }
        }

        e.add_instruction(*instr)?;
        return Ok(());
    }
}

pub struct NopExpansion;
impl Transformation for NopExpansion {

    fn accepts(&self, instructions: &[Instruction]) -> (bool, usize) {

        let instr = &instructions[0]; 
        if instr.mnemonic() == Mnemonic::Nop {
            return (true, 1);
        }
        return (false, 0);
    }    

    // mov reg1, reg2 -> push reg2; pop reg1
    fn encode(&self, instructions: &[Instruction], e: &mut CodeAssembler) -> Result<(), IcedError> {

        e.push(rax)?;
        e.pop(rax)?;

        Ok(())
    }
}