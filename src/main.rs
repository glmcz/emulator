
// also for CHIP-8 is 0x100 reserved for system, we skip it...
struct CPU {
    registers: [u8; 16],
    position_in_memory: usize, // reflect program counter
    memory: [u8; 0x1000], // CHIP-8 has only 4096 bytes == 12 bits wide
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

    pub fn load_opcode() {

    }

    pub fn jump(addr: u8){

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

            match (a, b, c, d) {
                (0, 0, 0, 0)     => { return; },
                (0x8, _, _, 0x4) => self.add_xy(b, c),
                //    (0x1, 2, ,0,0,)
                _ => todo!("opcode")
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
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;               // <4>
    cpu.registers[3] = 10;               // <4>
    let mem = &mut cpu.memory;


    let mem = &mut cpu.memory; //opcodes loaded in mem
    mem[0] = 0x80; mem[1] = 0x14;        // 0x8014
    mem[2] = 0x80; mem[3] = 0x24;        // 0x8024
    mem[4] = 0x80; mem[5] = 0x34;        // 0x8034


    cpu.run();

    assert_eq!(cpu.registers[0], 35);
    println!("5 + 10 +10 + 10 = {}", cpu.registers[0]);
}
