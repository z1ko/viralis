
mod elf;
mod morph;

use elf::*;

// Idea : https://reverseengineering.stackexchange.com/questions/21910/elf-x86-64-adding-function
// ELF Injection : https://0x00sec.org/t/elfun-file-injector/410
// Other : https://www.win.tue.nl/~aeb/linux/hh/virus/unix-viruses.txt

fn main() {

    morph_file("test/main");

}
