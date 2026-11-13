// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a08104: i32;
    static mut DAT_00a07cf0: i32;
    static mut DAT_009d9132: i32;
    static mut DAT_009a73b0: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009d9134: i32;
    static mut DAT_009d911a: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_00403400();
    fn FUN_0049f360();
}

unsafe fn slot_index() -> u32 {
    // SAFETY: the globals are the game's own; the address is read as the original does.
    unsafe {
        let a = (DAT_009a73b0 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(0x16)
            .wrapping_add(DAT_009a1fe8 as u32);
        u32::from((a as *const u8).read())
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00402970(param_1: u32) {
    // SAFETY: all addresses are the game's own memory, accessed with the same
    // offsets, widths and order as the original. The int-typed globals are
    // indexed in 4-byte steps exactly as the C pointer arithmetic does.
    unsafe {
        let i_var3 = DAT_00a08104 as u32;
        let p = i_var3 as usize;
        let tbl = (&raw mut DAT_00a07cf0).add(u32::from(((p + 0x34) as *const u8).read()) as usize);
        let pc = tbl.read() as u32 as *const u8;
        let c = pc.read();
        if c != 0xFF {
            let v: u16;
            if c == 0xFD {
                let s = slot_index();
                let q = (&raw mut DAT_009d9132).add(s.wrapping_mul(0x566) as usize) as *const u16;
                if q.read_unaligned() == 0 {
                    v = 0x8000;
                } else {
                    v = 0x800C;
                }
            } else if c == 0xFE {
                let mut zero = ((p + 0x1EB) as *const u8).read() == 0;
                if !zero {
                    let s = slot_index();
                    let q =
                        (&raw mut DAT_009d9132).add(s.wrapping_mul(0x566) as usize) as *const u16;
                    zero = q.read_unaligned() == 0;
                }
                if zero {
                    let s = slot_index();
                    let q =
                        (&raw mut DAT_009d9134).add(s.wrapping_mul(0x566) as usize) as *const u16;
                    if q.read_unaligned() == 0 {
                        v = 0x8000;
                    } else {
                        v = 0x800D;
                    }
                } else {
                    v = 0x800C;
                }
            } else if c == 0xF9 {
                v = 0x800C;
            } else {
                let s = slot_index();
                let b = u32::from(((p + 0x36) as *const u8).read());
                let off = s.wrapping_mul(0x566).wrapping_add(b.wrapping_mul(2));
                let q = (&raw mut DAT_009d911a).add(off as usize) as *const u16;
                if q.read_unaligned() == 0 {
                    v = 0x8000;
                } else {
                    v = (b as u16).wrapping_add(0x8000);
                }
            }
            ((p + 0x42) as *mut u16).write_unaligned(v);
            ((p + 0x44) as *mut u16).write_unaligned(0);
            ((p + 0x48) as *mut u8).write(0);
            if ((p + 0x42) as *const u16).read_unaligned() != 0xFFFF {
                FUN_00403400();
            }
        }
        ((p + 0x64) as *mut u16).write_unaligned(0);
        let c2 = pc.add(1).read();
        if c2 != 0xFF {
            ((p + 0x5E) as *mut u8).write(c2);
            FUN_0049f360();
        }
        ((p + 0x6A) as *mut u16).write_unaligned(0xFFFF);
        ((p + 0x6C) as *mut u16).write_unaligned(0);
        ((p + 0x6E) as *mut u16).write_unaligned(0xFFFF);
        let m = ((p + 0x38) as *const i32).read_unaligned() as u32;
        ((p + 0x3C) as *mut u32).write_unaligned((param_1 & 7).wrapping_mul(m) >> 3);
    }
}
