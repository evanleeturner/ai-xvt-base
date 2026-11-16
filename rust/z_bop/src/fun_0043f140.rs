// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009e9a77: [u8; 1];
    static mut DAT_009e9670: i32;
    static mut DAT_009ecc34: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_00a08374: u16;
    static mut DAT_00a08c74: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00440760(param_1: u32, param_2: u32);
}

unsafe fn rd32(a: u32) -> i32 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as *const i32).read_unaligned() }
}

unsafe fn rd16(a: u32) -> u16 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (a as *const u16).read_unaligned() }
}

unsafe fn is_type(iv: i32) -> bool {
    if iv == -1 {
        return false;
    }
    // SAFETY: the table is the game's own memory.
    let c = unsafe {
        ((iv as u32)
            .wrapping_mul(0x23)
            .wrapping_add(3)
            .wrapping_add(DAT_009a1fe8 as u32) as *const i8)
            .read()
    };
    c == 1 || c == 2 || c == 3 || c == 0xe || c == 4
}

unsafe fn cur() -> i32 {
    // SAFETY: the game's own globals.
    unsafe {
        (&raw mut DAT_009e9670)
            .cast::<u8>()
            .add((DAT_009ecc34 as u32).wrapping_mul(0x5bd) as usize)
            .cast::<i32>()
            .read_unaligned()
    }
}

unsafe fn flags(iv: i32, off: u32) -> u16 {
    // SAFETY: the game's own memory.
    unsafe {
        let i8_ = rd32(
            (iv as u32)
                .wrapping_mul(0x23)
                .wrapping_add(0x1f)
                .wrapping_add(DAT_009a1fe8 as u32) as u32,
        );
        let i9 = rd32((i8_ as u32).wrapping_add(0xa9));
        rd16((i9 as u32).wrapping_add(off))
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0043f140() {
    // SAFETY: the globals and callee are the game's own; addresses are read
    // where the original reads them.
    unsafe {
        let kind = (&raw mut DAT_009e9a77)
            .cast::<u8>()
            .add((DAT_009ecc34 as u32).wrapping_mul(0x5bd) as usize)
            .read();
        if kind != 0 {
            if kind == 0x13 {
                let mut u7: u16 = 1;
                let mut u6: u32 = 0;
                loop {
                    'skip: {
                        if u7 < 9 {
                            if u7 == 8 || u7 == 2 || u7 == 4 {
                                break 'skip;
                            }
                        } else if u7 == 0x10 || u7 == 0x1000 {
                            let iv = cur();
                            if is_type(iv) {
                                break 'skip;
                            }
                        }
                        let iv = cur();
                        let u10 = flags(iv, 0x1df);
                        if (u10 & u7) != 0 {
                            let u11 = flags(iv, 0x1e1);
                            let a: u32 = if (u11 & u7) == 0 { 0xd } else { 0 };
                            FUN_00440760(
                                u6.wrapping_add(DAT_00a08374 as u32).wrapping_add(0x45),
                                a,
                            );
                        }
                    }
                    u7 = u7.wrapping_mul(2);
                    let u5 = (u6 as u16).wrapping_add(1);
                    u6 = u5 as u32;
                    if !(u5 < 0xd) {
                        break;
                    }
                }
                DAT_00a08c74 = 0xffff;
            }
            return;
        }
        let mut u7: u16 = 1;
        let mut u6: u32 = 0;
        loop {
            let iv = cur();
            let b3 = is_type(iv);
            let u11 = flags(iv, 0x1e1);
            let mut run: Option<u32> = None;
            if b3 {
                if (u11 & u7) == 0 {
                    run = Some(0);
                }
            } else {
                run = Some(if (u11 & u7) == 0 { 0xd } else { 0 });
            }
            if let Some(u4) = run {
                let u10 = flags(iv, 0x1df);
                if (u10 & u7) != 0 {
                    FUN_00440760(u6.wrapping_add(0x45), u4);
                }
            }
            u7 = u7.wrapping_mul(2);
            let u5 = (u6 as u16).wrapping_add(1);
            u6 = u5 as u32;
            if 0xc < u5 {
                DAT_00a08c74 = 0xffff;
                return;
            }
        }
    }
}
