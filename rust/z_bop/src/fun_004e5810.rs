// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69e30: i32;
    static mut DAT_00aa6114: i32;
    static mut DAT_00aa6118: i32;
    static mut DAT_00aa6120: i32;
    static mut DAT_00aa6124: i32;
    static mut DAT_00aa612c: i32;
    static mut DAT_00aa6130: i32;
    static mut DAT_00aa6134: i32;
    static mut DAT_0052c208: i32;
    static mut DAT_00bb2810: i32;
    static mut DAT_00bb73a6: i8;
    static mut DAT_00b69d20: [i8; 1];
    static mut DAT_0052c308: [i8; 1];
    static mut DAT_00b6a5aa: [u8; 4];
    static mut DAT_00bb0106: [u8; 1];
    static mut DAT_00bb0112: [u8; 1];
    static mut DAT_00bb011e: [u8; 1];
    static mut DAT_00bb10a6: [u8; 1];
    static mut DAT_00bb10b2: [u8; 1];
    static mut DAT_00bb10be: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6460(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004ddc10(param_1: u32) -> *mut i8;
    fn FUN_004db7d0(
        param_1: i32,
        param_2: *mut i8,
        param_3: *mut i32,
        param_4: i32,
        param_5: i32,
        param_6: u32,
    ) -> u32;
    fn FUN_004dbfa0(param_1: *mut i8, param_2: i32) -> i32;
    fn _sprintf(param_1: *mut i8, param_2: *const i8, ...) -> i32;
    fn FUN_004db640(param_1: i32, param_2: *mut i8, param_3: *mut i32, param_4: i32);
    fn FUN_004dbbd0(
        param_1: i32,
        param_2: i32,
        param_3: *mut i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> i32;
    fn FUN_004d9d70(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    ) -> i32;
}

unsafe fn rd(addr: usize) -> i32 {
    // SAFETY: reads 4 bytes of the game's own memory at the given address.
    unsafe { (addr as *const i32).read_unaligned() }
}

unsafe fn rec(base: usize, idx: i32, mul: u32) -> usize {
    base.wrapping_add((idx as u32).wrapping_mul(mul) as usize)
}

unsafe fn cur() -> i32 {
    // SAFETY: game globals; the pointed-to cell is the game's own memory.
    unsafe {
        let a = (DAT_00aa6114 as u32)
            .wrapping_add(0x140)
            .wrapping_add((DAT_00aa6134 as u32).wrapping_mul(0x148));
        rd(a as usize)
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004e5810() -> u32 {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: all globals and callees are the game's own; addresses computed
    // as the original does.
    unsafe {
        let l = local_10.as_mut_ptr();
        FUN_004d6460(l, 0x58, 0x8a, 0x1ae, 0x99);
        let sel = (&raw mut DAT_00b6a5aa).cast::<i32>().read_unaligned();
        let uvar6: u32;
        'tail: {
            if sel == 2 {
                uvar6 = 0x1d7;
            } else if sel == 4 {
                let mut bvar1 = false;
                if DAT_00b69e30 == 2 {
                    let i = cur();
                    let v = rd(rec((&raw mut DAT_00bb0106).cast::<u8>() as usize, i, 0xa0));
                    if v != 0 && DAT_00bb73a6 != 0 {
                        bvar1 = true;
                    }
                } else if DAT_00b69e30 == 4 {
                    let i = cur();
                    let v = rd(rec((&raw mut DAT_00bb10a6).cast::<u8>() as usize, i, 0xa0));
                    if v != 0 && DAT_00bb73a6 != 0 {
                        bvar1 = true;
                    }
                } else if DAT_00aa6120 != 0 && DAT_00aa6124 != 0 {
                    bvar1 = true;
                }
                let mut ivar4: i32 = 0;
                if bvar1 {
                    let pc = FUN_004ddc10(0x1d8);
                    FUN_004db7d0(0xf, pc, l, 0, 1, 0xffff);
                    let pc = FUN_004ddc10(0x1d8);
                    let w = FUN_004dbfa0(pc, 0xf);
                    local_10[0] = local_10[0].wrapping_add(w);
                    let mut ivar10: i32;
                    if DAT_00b69e30 == 2 {
                        ivar10 = 0;
                        let mut ivar5: i32 = 0;
                        let ivar9 = cur();
                        let p1 = (&raw mut DAT_00bb0112).cast::<u8>() as usize;
                        let p2 = (&raw mut DAT_00bb011e).cast::<u8>() as usize;
                        let mut tmp = rd(rec(p1, ivar9, 0xa0));
                        if -1 < tmp {
                            loop {
                                if 9 < ivar5 {
                                    break;
                                }
                                let idx =
                                    (ivar5 as u32).wrapping_add((ivar9 as u32).wrapping_mul(0x28));
                                let val = rd(p2.wrapping_add(idx.wrapping_mul(4) as usize));
                                if val == 0 {
                                    ivar4 = ivar4.wrapping_add(1);
                                } else if val == 1 {
                                    ivar10 = ivar10.wrapping_add(1);
                                }
                                ivar5 = ivar5.wrapping_add(1);
                                tmp = rd(rec(p1, ivar9, 0xa0));
                                if !(ivar5 <= tmp) {
                                    break;
                                }
                            }
                        }
                    } else {
                        ivar10 = DAT_00aa612c;
                        ivar4 = DAT_00aa6130;
                        if DAT_00b69e30 == 4 {
                            let mut ivar5: i32 = 0;
                            ivar4 = 0;
                            ivar10 = 0;
                            let ivar9 = cur();
                            let p1 = (&raw mut DAT_00bb10b2).cast::<u8>() as usize;
                            let p2 = (&raw mut DAT_00bb10be).cast::<u8>() as usize;
                            let mut tmp = rd(rec(p1, ivar9, 0xa0));
                            if -1 < tmp {
                                loop {
                                    if 9 < ivar5 {
                                        break;
                                    }
                                    let idx = (ivar5 as u32)
                                        .wrapping_add((ivar9 as u32).wrapping_mul(0x28));
                                    let val = rd(p2.wrapping_add(idx.wrapping_mul(4) as usize));
                                    if val == 0 {
                                        ivar4 = ivar4.wrapping_add(1);
                                    } else if val == 1 {
                                        ivar10 = ivar10.wrapping_add(1);
                                    }
                                    ivar5 = ivar5.wrapping_add(1);
                                    tmp = rd(rec(p1, ivar9, 0xa0));
                                    if !(ivar5 <= tmp) {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    let pc3 = FUN_004ddc10(0x157);
                    let pc2 = FUN_004ddc10(0x156);
                    _sprintf(
                        (&raw mut DAT_00b69d20).cast::<i8>(),
                        (&raw const DAT_0052c308).cast::<i8>(),
                        5i32,
                        pc2,
                        1u32,
                        ivar4,
                        3u32,
                        pc3,
                        1u32,
                        ivar10,
                    );
                    FUN_004db640(0xc, (&raw mut DAT_00b69d20).cast::<i8>(), l, 0xffff);
                    break 'tail;
                }
                uvar6 = 0x1d8;
            } else if sel == 5 {
                uvar6 = 0x309;
            } else {
                uvar6 = 0xc0;
            }
            let pc = FUN_004ddc10(uvar6);
            FUN_004db7d0(0xf, pc, l, 0, 1, 0xffff);
        }
        FUN_004d6460(l, 0x58, 0x9d, 0x1a4, 0x12d);
        let r = FUN_004dbbd0(0xc, DAT_00aa6118, l, 0xffff, 4, 0x1000);
        let mut uvar12: i32 = 0x1ae;
        let next4 = r.wrapping_add(1);
        if 9 < next4 {
            FUN_004d6460(l, 0x1a5, 0x9d, 0x1ae, 0x12d);
            DAT_0052c208 = FUN_004d9d70(l, DAT_0052c208, next4, 0, 5, DAT_00bb2810, 9);
            uvar12 = 0x1a4;
        }
        FUN_004d6460(l, 0x58, 0x9d, uvar12, 0x12d);
        FUN_004dbbd0(0xc, DAT_00aa6118, l, 0xffff, 4, DAT_0052c208);
    }
    1
}
