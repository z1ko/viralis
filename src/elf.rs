
use iced_x86::{Decoder, DecoderOptions, Formatter, Instruction, NasmFormatter, Mnemonic, OpKind, Code, code_asm::CodeAssembler};
use goblin::{Object, elf::Elf, elf64::section_header::SHT_PROGBITS};
use std::{path::Path, fs};

use crate::morph::*;

fn print_sections(elf: &Elf) {
    print!("File's sections:\n\n");
    for section in &elf.section_headers {
        if section.sh_type == SHT_PROGBITS {
            let name = &elf.shdr_strtab[section.sh_name];
            print!("\t {:-20}  | offset: {:#x}, size: {:#x}\n", name, section.sh_offset, section.sh_size);
        }
    }
}

fn print_instructions(instructions: &Vec<Instruction>) {
    let mut formatter = NasmFormatter::new();

    let mut offset_bytes = 0;
    for instr in instructions {

        let mut output = String::new();
        formatter.format(&instr, &mut output);
        print!("{:#08x} : {}\n", offset_bytes, output);
        offset_bytes += instr.len();
    }
}

// Dato un file elf ottieni i bytes della sezione .text
fn extract_text_section(data: &Vec<u8>, elf: &Elf) -> Vec<u8> {
    for section in &elf.section_headers {
        if &elf.shdr_strtab[section.sh_name] == ".text" {
            return data.iter().cloned()
                .skip(section.sh_offset as usize)
                .take(section.sh_size   as usize)
                .collect();
        }
    }

    panic!("The executable doesn't contains a .text section! How?");
}



// Dati i bytes della sezione .text disassemblali
fn disasm_text_section(data: &Vec<u8>) -> Vec<Instruction> {
    let mut decoder = Decoder::with_ip(
        64, data, 0x0, DecoderOptions::AMD
    );

    let mut result = Vec::with_capacity(data.len() / 4);
    while decoder.can_decode() {

        let mut instr = Instruction::default();
        decoder.decode_out(&mut instr);
        result.push(instr);
    }

    result
}

// Morph instructions using various techniques and returns new instructions
fn morph_instructions(instructions: &Vec<Instruction>) -> Vec<Instruction> {

    let mut a = CodeAssembler::new(64)
        .expect("Impossibile creare code assembler!");

    // Per ora abbiamo solo questa
    let transf = MovPushPop;

    // Processa tutte le istruzioni, un digest alla volta
    for mut offset in 0 .. instructions.len() {
        
        let (accepts, digest) = transf.accepts(&instructions[offset..]);
        if accepts {

            // Applica trasformazione
            transf.encode(&instructions[offset .. offset + digest], &mut a)
                .expect("Errore applicazione trasformazione");
                
            offset += digest;
        }
        // Mantieni istruzione
        else {
            a.add_instruction(instructions[offset])
                .unwrap();
        }
    }

    a.take_instructions()
}

// Carica il contenuto binario di un file elf e lo morpha
pub fn morph_file<P: AsRef<Path>>(path: P) {

    let data = fs::read(path)
        .expect("Errore caricamento dati programma!");

    match Object::parse(&data)
        .expect("Errore parsing programma!") 
    {
        Object::Elf(elf) => {

            print_sections(&elf);
            let text = extract_text_section(&data, &elf);

            print!("Decoding section...\n\n");
            let instructions = disasm_text_section(&text);
            print_instructions(&instructions);

            print!("Generating morphed section ...\n\n");
            let instructions = morph_instructions(&instructions);
            print_instructions(&instructions);
        },

        _ => panic!("Il formato del file eseguibile non è supportato!")
    }
}