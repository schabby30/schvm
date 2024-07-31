use schvm::vm::{Machine, RegisterName};
use schvm::memory::Addressable;

pub fn main() {
    let mut machine = Machine::new();
    machine.set_register(RegisterName::A, 13);
    let _ = machine.memory.write(0, 511);
    println!("{:?}", machine.get_register_list());
    println!("{:?}", machine.memory);
}