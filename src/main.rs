use std::fs;

    memory: [u8; 4096],
}

    }

    }

    }

struct Register {
    v0: u8,
    v1: u8,
    v2: u8,
    v3: u8,
    v4: u8,
    v5: u8,
    v6: u8,
    v7: u8,
    v8: u8,
    v9: u8,
    v_a: u8,
    v_b: u8,
    v_c: u8,
    v_d: u8,
    v_e: u8,
    v_f: u8,
    i: u16,
    pc: u16,
    dt: u8,
    st: u8,
    sp: u8,
    stack: [u16; 16],
}

struct Display {
    screen: [[u8; 64]; 32],
}

fn main() {
}
