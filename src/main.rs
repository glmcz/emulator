// also for CHIP-8 is 0x100 reserved for system, we skip it...
struct CPU {
    registers: [u8; 16],
    position_in_memory: usize, // reflect program counter
    memory: [u8; 0x1000], // CHIP-8 has only 4096 bytes == 12 bits wide
    stack: [u16; 16],
    stack_pointer: usize,
}

impl CPU {
    pub fn init() {
    }
    pub fn read_opcodes(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p +1] as u16;

        op_byte1 << 8 | op_byte2 // mem has u8, but we need u16 op_code...
    }

    pub fn call(&mut self, addr: u16){
        let stack_pointer = self.stack_pointer;
        let stack = &mut self.stack;
        if stack_pointer > stack.len() {
            panic!("Stack overflow");
        }

        stack[stack_pointer] = self.position_in_memory as u16; // saving  mem of fn
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    pub fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack overflow");
        }
        //decrement stack_p in order to get back to fn call start address
        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];
        self.position_in_memory =  call_addr as usize;
    }

    pub fn add_xy(&mut self, x: u8, y:u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        }else {
            self.registers[0xF] = 0;
        }
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.read_opcodes();
            self.position_in_memory += 2; // increment in other to proceed with another instruction
            // u16 matching...
            let a = ((opcode & 0xF000) >> 12) as u8;
            let b = ((opcode & 0x0F00) >> 8) as u8;
            let c = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;
            let nnn = opcode & 0x0FFF;

            match (a, b, c, d) {
                (0, 0, 0, 0)     => { return; },
                (0x8, _, _, 0x4) => self.add_xy(b, c),
                (0x2, _, _, _ ) => self.call(nnn),
                (0, 0, 0xE, 0xE) => self.ret(),
                //    (0x1, 2, ,0,0,)
                _ => todo!("add additional opcodes")
            };
        }
        // let nnn = opcode & 0x0FFF;
        // let kk = opcode & 0x00FF;

    }
}
// opcode 0x8014. This code mean
// 8 = use two register
// 0 maps to register[0]
// 1 maps to register[1]
// 4 indicated that we should use addition operation
fn main() {
    let mut cpu = CPU{
        memory: [0; 4096],
        registers: [0; 16],
        position_in_memory: 0,
        stack: [0;16],
        stack_pointer: 0,
    };
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory; //opcodes loaded in mem
    mem[0x000] = 0x21; mem[0x001] = 0x00;        // call fn
    mem[0x002] = 0x21; mem[0x003] = 0x00;        // stack is doubled. Call fn again to 0x100 after ret is reached
    mem[0x004] = 0x00; mem[0x005] = 0x00;        // set opcode halt

    // mem[0] = 0x80; mem[1] = 0x14; it will work because it starts at 0 which is base address for PC
    // this would not work without stack, because stack is not init by call fn, which setup stack at 256(0x100)
    mem[0x100] = 0x80; mem[0x101] = 0x14;        // add reg[1] value to req[0]
    mem[0x102] = 0x80; mem[0x103] = 0x14;        // add reg[1] value to req[0]
    mem[0x104] = 0x00; mem[0x105] = 0xEE;        // return fn

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    // it is a mathematical way of mem operation upper.
    println!("5 + (10 * 2) + (10 * 2)  = {}", cpu.registers[0]);
}
