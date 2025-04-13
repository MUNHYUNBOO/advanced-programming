struct CPU;

impl CPU {
    fn process(&self) {
        println!("CPU 처리 중");
    }
}

struct Memory;

impl Memory {
    fn load(&self) {
        println!("메모리 로딩 중");
    }
}

struct SSD;

impl SSD {
    fn read(&self) {
        println!("SSD 드라이브 읽는 중");
    }
}

// 파사드 구조
struct Computer {
    cpu: CPU,
    memory: Memory,
    ssd: SSD,
}

impl Computer {
    fn new() -> Self {
        Computer {
            cpu: CPU,
            memory: Memory,
            ssd: SSD,
        }
    }

    fn boot(&self) {
        self.ssd.read();
        self.memory.load();
        self.cpu.process();
    }
}

fn main() {
    let computer = Computer::new();
    computer.boot();
}
