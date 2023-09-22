mod cpu;
use std::env;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::path::PathBuf;

use cpu::W65C02S;
use cpu::RAM;

fn main() {
    //Search for ASM files in program args;
    let mut target_asm = String::new();
    for arg in env::args() {
        if arg.to_lowercase().ends_with(".asm") {
            target_asm = arg;
        }
    }

    let path_buf: PathBuf = PathBuf::from(&target_asm);
    let mut file_contents = vec!();
    let mut total_bytes_read: usize = 0;

    if Path::is_absolute(&path_buf){
        if let Ok(mut file) = File::open(target_asm) {
            total_bytes_read = file.read_to_end(&mut file_contents).unwrap();
        }
    } else {
        if let Ok(mut file) = File::open(Path::join(&env::current_dir().unwrap(), &path_buf)){
            total_bytes_read = file.read_to_end(&mut file_contents).unwrap();
        }
    }

    println!("{total_bytes_read} bytes read");
    println!("{:?}", file_contents);

    let foo = W65C02S::new(RAM::new());
}