use std::fs;

#[derive(Debug)]
struct Register {
    v: [u8; 16],
    i: u16,
    pc: usize,
    dt: u8,
    st: u8,
    sp: usize,
    stack: [usize; 16],
}

impl Register {
    fn new(offset: usize) -> Self {
        Register {
            v: [0; 16],
            i: 0,
            pc: offset,
            dt: 0,
            st: 0,
            sp: 0,
            stack: [0; 16],
        }
    }
}
    memory: [u8; 4096],
}

    }

    }

    }


}

fn main() {
}
