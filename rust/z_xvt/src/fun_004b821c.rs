// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052f1dc: u32;
    static mut DAT_0064ee60: *mut u32;
    static mut DAT_0064ef48: u32;
    static mut DAT_0064ef4c: u32;
    static mut _DAT_0064ef44: u32;
    static mut DAT_0064ee24: u32;
    static mut DAT_0064ee20: u32;
}

unsafe fn hdr(n: u16) {
    // SAFETY: DAT_0064ee60 is the game's own write cursor; it points at
    // writable buffer space, written where the original does.
    unsafe {
        let p = DAT_0064ee60;
        p.cast::<u8>().write(8);
        p.cast::<u8>().byte_add(1).write(8);
        p.cast::<u8>().byte_add(2).cast::<u16>().write_unaligned(n);
        DAT_0064ee60 = p.add(1);
    }
}

unsafe fn pair(a: u32, b: u32) {
    // SAFETY: DAT_0064ee60 is the game's own write cursor; it points at
    // writable buffer space, written where the original does.
    unsafe {
        let p = DAT_0064ee60;
        p.write_unaligned(a);
        p.add(1).write_unaligned(b);
        DAT_0064ee60 = p.add(2);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b821c(param_1: u32) {
    // SAFETY: the globals are the game's own memory; the output cursor
    // points at the game's command buffer.
    unsafe {
        if DAT_0052f1dc == param_1 {
            return;
        }
        if ((DAT_0052f1dc ^ (param_1 & 0xc00)) & 0xc00) != 0 {
            if (param_1 & 0xc00) == 0 {
                hdr(4);
                pair(0x13, 2);
                pair(0x14, 1);
                if (param_1 & 0x800) == 0 {
                    pair(0x15, 2);
                } else {
                    pair(0x15, 4);
                }
                pair(0x1b, 0);
            } else {
                hdr(4);
                pair(0x13, 5);
                pair(0x14, 6);
                if (param_1 & 0x400) == 0 {
                    pair(0x15, 4);
                } else {
                    pair(0x15, 2);
                }
                pair(0x1b, 1);
            }
        }
        if ((DAT_0052f1dc ^ (param_1 & 0x3000)) & 0x3000) != 0 {
            hdr(2);
            if (param_1 & 0x1000) == 0 {
                pair(0x17, 8);
            } else {
                pair(0x17, 2);
            }
            if (param_1 & 0x2000) == 0 {
                pair(0xe, 0);
            } else {
                pair(0xe, 1);
            }
        }
        if ((DAT_0052f1dc ^ (param_1 & 0x300)) & 0x300) != 0 {
            hdr(2);
            if (param_1 & 0x100) == 0 {
                pair(0x11, 1);
            } else {
                pair(0x11, 2);
            }
            if (param_1 & 0x200) == 0 {
                pair(0x12, 1);
            } else {
                pair(0x12, 2);
            }
        }
        if ((DAT_0052f1dc ^ (param_1 & 0xc000)) & 0xc000) != 0 {
            hdr(1);
            if (param_1 & 0x4000) == 0 {
                pair(3, 1);
            } else {
                pair(3, 3);
            }
        }
        if ((DAT_0052f1dc ^ (param_1 & 0x80)) & 0x80) != 0 {
            if (param_1 & 0x80) == 0 {
                hdr(1);
                pair(0x1c, 0);
            } else {
                hdr(5);
                pair(0x1c, 1);
                let v =
                    DAT_0064ef48.wrapping_shl(8) | DAT_0064ef4c.wrapping_shl(0x10) | _DAT_0064ef44;
                pair(0x22, v);
                pair(0x23, 3);
                let a = DAT_0064ee24;
                pair(0x24, a);
                let b = DAT_0064ee20;
                pair(0x25, b);
            }
        }
        DAT_0052f1dc = param_1;
    }
}
