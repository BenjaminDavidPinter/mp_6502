mod cpu;
use cpu::W65C02S;
use cpu::RAM;

fn main() {
    let foo = W65C02S::new(RAM::new());
    
}
