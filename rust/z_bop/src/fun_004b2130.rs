// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00528c54: u32;
    static mut DAT_00664c80: *mut u32;
    static mut DAT_00a90b20: u32;
    static mut DAT_00664750: u32;
    static mut DAT_00664978: u32;
    static mut _DAT_006644f4: u32;
    static mut DAT_00664544: u32;
    static mut DAT_006644f0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004b48d0(param_1: u32) -> u8;
}

#[inline(always)]
unsafe fn put(off: usize, v: u32) {
    // SAFETY: the output cursor is the game's own buffer.
    unsafe {
        DAT_00664c80.wrapping_add(off).write_unaligned(v);
    }
}

#[inline(always)]
unsafe fn adv(n: usize) {
    // SAFETY: the cursor global is the game's own memory.
    unsafe {
        DAT_00664c80 = DAT_00664c80.wrapping_add(n);
    }
}

#[inline(always)]
unsafe fn header(n: u16) {
    // SAFETY: the cursor points into the game's output buffer.
    unsafe {
        let p = DAT_00664c80 as *mut u8;
        p.write(8);
        p.byte_add(1).write(8);
        p.byte_add(2).cast::<u16>().write_unaligned(n);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b2130(param_1: u32) {
    // SAFETY: all globals and the callee are the game's own; the cursor
    // points into the game's command buffer, written where the original does.
    unsafe {
        if DAT_00528c54 == param_1 {
            return;
        }
        if ((DAT_00528c54 ^ param_1) & 0x8000) != 0 {
            header(1);
            adv(1);
            put(0, 0xb);
            if (param_1 & 0x8000) == 0 {
                put(1, 1);
            } else {
                put(1, 0);
            }
            adv(2);
        }
        if ((DAT_00528c54 ^ param_1) & 0x600) != 0 {
            header(4);
            if (param_1 & 0x600) == 0 {
                adv(1);
                put(0, 0x13);
                put(1, 2);
                adv(2);
                put(0, 0x14);
                put(1, 1);
                adv(2);
                put(0, 0x15);
                if (param_1 & 0x400) == 0 {
                    put(1, 2);
                } else {
                    put(1, 4);
                }
                adv(2);
                put(0, 0x1b);
                put(1, 0);
            } else {
                adv(1);
                put(0, 0x13);
                put(1, 5);
                adv(2);
                put(0, 0x14);
                put(1, 6);
                adv(2);
                put(0, 0x15);
                if (param_1 & 0x400) == 0 {
                    put(1, 2);
                } else {
                    put(1, 4);
                }
                adv(2);
                put(0, 0x1b);
                put(1, 1);
            }
            adv(2);
        }
        if ((DAT_00528c54 ^ param_1) & 0x1800) != 0 {
            header(2);
            adv(1);
            put(0, 0x17);
            if (param_1 & 0x800) == 0 {
                put(1, 8);
            } else {
                let b = FUN_004b48d0(DAT_00a90b20);
                put(1, b as u32);
            }
            adv(2);
            put(0, 0xe);
            if (param_1 & 0x1000) == 0 {
                put(1, 0);
            } else {
                put(1, 1);
            }
            adv(2);
        }
        if ((DAT_00528c54 ^ param_1) & 0x180) != 0 {
            header(2);
            adv(1);
            put(0, 0x11);
            put(1, 2u32.wrapping_sub(((param_1 & 0x80) == 0) as u32));
            adv(2);
            put(0, 0x12);
            put(1, 2u32.wrapping_sub(((param_1 & 0x100) == 0) as u32));
            adv(2);
        }
        if ((DAT_00528c54 ^ param_1) & 0x40) != 0 {
            let p = DAT_00664c80 as *mut u8;
            p.write(8);
            p.byte_add(1).write(8);
            if (param_1 & 0x40) != 0 {
                p.byte_add(2).cast::<u16>().write_unaligned(5);
                adv(1);
                put(0, 0x1c);
                put(1, 1);
                adv(2);
                put(0, 0x22);
                put(
                    1,
                    DAT_00664750.wrapping_shl(0x10) | DAT_00664978.wrapping_shl(8) | _DAT_006644f4,
                );
                adv(2);
                put(0, 0x23);
                put(1, 3);
                adv(2);
                put(0, 0x24);
                put(1, DAT_00664544);
                adv(2);
                put(0, 0x25);
                put(1, DAT_006644f0);
                adv(2);
                DAT_00528c54 = param_1;
                return;
            }
            p.byte_add(2).cast::<u16>().write_unaligned(1);
            adv(1);
            put(0, 0x1c);
            put(1, 0);
            adv(2);
        }
        DAT_00528c54 = param_1;
    }
}
