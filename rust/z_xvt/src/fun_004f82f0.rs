// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007596b7: i32;
    static mut DAT_007596bb: i32;
    static mut DAT_00759657: i32;
    static mut DAT_007596bf: i32;
    static mut DAT_007596c3: i32;
    static mut DAT_0075964b: i32;
    static mut DAT_00758830: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f82f0(param_1: i32, param_2: i32, param_3: i32, param_4: u16) {
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    // SAFETY: the globals are the game's own; the destination is the game's
    // surface memory, written exactly where the original writes.
    unsafe {
        if !(DAT_007596b7 <= param_3 && param_3 <= DAT_007596bb) {
            return;
        }
        let ivar1: i32 = DAT_00759657 >> 4;
        if param_2 < param_1 {
            let c1 = if DAT_007596bf <= param_2 {
                true
            } else {
                param_2 = DAT_007596bf;
                DAT_007596bf < param_1
            };
            if !c1 {
                return;
            }
            let c2 = if param_1 <= DAT_007596c3 {
                true
            } else {
                param_1 = DAT_007596c3;
                param_2 < DAT_007596c3
            };
            if !c2 {
                return;
            }
            let mut addr: u32 = param_2
                .wrapping_mul(DAT_0075964b)
                .wrapping_add(param_3.wrapping_shl((ivar1 & 0x1f) as u32))
                .wrapping_add(DAT_00758830) as u32;
            if ivar1 == 0 {
                if param_2 <= param_1 {
                    let mut n = param_1.wrapping_sub(param_2).wrapping_add(1);
                    loop {
                        (addr as usize as *mut u8).write(param_4 as u8);
                        addr = addr.wrapping_add(DAT_0075964b as u32);
                        n = n.wrapping_sub(1);
                        if n == 0 {
                            break;
                        }
                    }
                }
            } else if ivar1 == 1 && param_2 <= param_1 {
                let mut n = param_1.wrapping_sub(param_2).wrapping_add(1);
                loop {
                    (addr as usize as *mut u16).write_unaligned(param_4);
                    addr = addr.wrapping_add((DAT_0075964b & -2) as u32);
                    n = n.wrapping_sub(1);
                    if n == 0 {
                        break;
                    }
                }
            }
        } else {
            let c1 = if param_2 <= DAT_007596c3 {
                true
            } else {
                param_2 = DAT_007596c3;
                param_1 <= DAT_007596c3
            };
            if !c1 {
                return;
            }
            let c2 = if DAT_007596bf <= param_1 {
                true
            } else {
                param_1 = DAT_007596bf;
                DAT_007596bf <= param_2
            };
            if !c2 {
                return;
            }
            let mut addr: u32 = param_1
                .wrapping_mul(DAT_0075964b)
                .wrapping_add(param_3.wrapping_shl((ivar1 & 0x1f) as u32))
                .wrapping_add(DAT_00758830) as u32;
            if ivar1 == 0 {
                if param_1 <= param_2 {
                    let mut n = param_2.wrapping_sub(param_1).wrapping_add(1);
                    loop {
                        (addr as usize as *mut u8).write(param_4 as u8);
                        addr = addr.wrapping_add(DAT_0075964b as u32);
                        n = n.wrapping_sub(1);
                        if n == 0 {
                            break;
                        }
                    }
                }
            } else if ivar1 == 1 && param_1 <= param_2 {
                let mut n = param_2.wrapping_sub(param_1).wrapping_add(1);
                loop {
                    (addr as usize as *mut u16).write_unaligned(param_4);
                    addr = addr.wrapping_add((DAT_0075964b & -2) as u32);
                    n = n.wrapping_sub(1);
                    if n == 0 {
                        break;
                    }
                }
            }
        }
    }
}
