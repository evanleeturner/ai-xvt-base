// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69e30: i32;
    static mut DAT_00b6a5aa: [u8; 4];
    static mut DAT_00b6a606: [u8; 4];
    static mut DAT_0066d9e0: i32;
    static mut DAT_0066d918: i32;
    static mut DAT_0052d590: [i8; 1];
    static mut DAT_0052d584: [i8; 1];
    static mut DAT_0052d57c: [i8; 1];
    static mut DAT_0052d570: [i8; 1];
    static mut DAT_0052d564: [i8; 1];
    static mut DAT_0052d558: [i8; 1];
    static mut DAT_0052d54c: [i8; 1];
    static mut DAT_0052d540: [i8; 1];
    static mut DAT_0052ad74: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004daba0(param_1: *mut i32);
    fn FUN_004d6460(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004ddc10(param_1: i32) -> *mut i8;
    fn FUN_004daa20(param_1: *mut i32, param_2: *mut i8, param_3: *mut i8, param_4: i32);
    fn FUN_004da780(
        param_1: *mut i32,
        param_2: *mut i8,
        param_3: *mut i8,
        param_4: *mut i8,
        param_5: i32,
        param_6: u32,
        param_7: i32,
        param_8: *mut i8,
    ) -> i32;
    fn FUN_004d64a0(param_1: *mut i32, param_2: i32, param_3: i32);
}

unsafe fn rd_5aa() -> i32 {
    // SAFETY: the game's packed global, read unaligned.
    unsafe { (&raw mut DAT_00b6a5aa).cast::<i32>().read_unaligned() }
}

unsafe fn rd_606() -> i32 {
    // SAFETY: the game's packed global, read unaligned.
    unsafe { (&raw mut DAT_00b6a606).cast::<i32>().read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00504360() -> u32 {
    let mut local_30: [i32; 4] = [0; 4];
    let mut local_20: [i32; 8] = [0; 8];
    // SAFETY: globals and callees are the game's own; local arrays are
    // passed as pointers to the callees, and the indexed write mirrors the
    // original's unchecked index.
    unsafe {
        let s_jewel = (&raw mut DAT_0052ad74).cast::<i8>();
        let l30 = local_30.as_mut_ptr();
        let l20 = local_20.as_mut_ptr();

        if DAT_00b69e30 == 2 && rd_5aa() != 1 && rd_5aa() != 2 {
            local_20[0] = 0;
        } else {
            local_20[0] = 1;
        }
        local_20[1] = 1;
        if rd_606() == 1 && (rd_5aa() == 0 || rd_5aa() == 1 || rd_5aa() == 3) {
            local_20[2] = 1;
        } else {
            local_20[2] = 0;
        }
        local_20[3] = 0;
        local_20[4] = 0;
        local_20[5] = 0;
        local_20[6] = 0;
        local_20[7] = 0;
        l20.offset(DAT_0066d9e0 as isize).write(2);
        FUN_004daba0(l20);
        FUN_004d6460(l30, 0x16, 0xaa, 0x2a, 0xc2);
        if DAT_0066d9e0 == 2 {
            if rd_5aa() == 0 {
                let p = FUN_004ddc10(0x336);
                FUN_004daa20(l30, (&raw mut DAT_0052d590).cast::<i8>(), p, 0xc);
            } else if rd_5aa() == 1 {
                let p = FUN_004ddc10(0x13b);
                FUN_004daa20(l30, (&raw mut DAT_0052d584).cast::<i8>(), p, 0xc);
            } else if rd_5aa() == 3 {
                let p = FUN_004ddc10(0x13a);
                FUN_004daa20(l30, (&raw mut DAT_0052d584).cast::<i8>(), p, 0xc);
            }
        } else if rd_606() == 1 {
            'blk: {
                let r: i32;
                if rd_5aa() == 0 {
                    let p = FUN_004ddc10(0x336);
                    let s = (&raw mut DAT_0052d57c).cast::<i8>();
                    r = FUN_004da780(l30, s, s, p, 0xc, 0, 0xe, s_jewel);
                } else if rd_5aa() == 1 {
                    let p = FUN_004ddc10(0x13b);
                    let s = (&raw mut DAT_0052d570).cast::<i8>();
                    r = FUN_004da780(l30, s, s, p, 0xc, 0, 0xe, s_jewel);
                } else {
                    if rd_5aa() != 3 {
                        break 'blk;
                    }
                    let p = FUN_004ddc10(0x13a);
                    let s = (&raw mut DAT_0052d570).cast::<i8>();
                    r = FUN_004da780(l30, s, s, p, 0xc, 0, 0xe, s_jewel);
                }
                if r != 0 {
                    DAT_0066d918 = 1;
                    DAT_0066d9e0 = 2;
                }
            }
        }
        FUN_004d64a0(l30, 0, -0x1c);
        if DAT_0066d9e0 == 1 {
            let p = FUN_004ddc10(0x138);
            FUN_004daa20(l30, (&raw mut DAT_0052d564).cast::<i8>(), p, 0xc);
        } else {
            let p = FUN_004ddc10(0x138);
            let s = (&raw mut DAT_0052d558).cast::<i8>();
            let r = FUN_004da780(l30, s, s, p, 0xc, 0, 0xc, s_jewel);
            if r != 0 {
                DAT_0066d918 = 1;
                DAT_0066d9e0 = 1;
            }
        }
        FUN_004d64a0(l30, 0, -0x1c);
        if local_20[0] != 0 {
            if DAT_0066d9e0 == 0 {
                let p = FUN_004ddc10(0x137);
                FUN_004daa20(l30, (&raw mut DAT_0052d54c).cast::<i8>(), p, 0xc);
                return 1;
            }
            let p = FUN_004ddc10(0x137);
            let s = (&raw mut DAT_0052d540).cast::<i8>();
            let r = FUN_004da780(l30, s, s, p, 0xc, 0, 0xb, s_jewel);
            if r != 0 {
                DAT_0066d918 = 1;
                DAT_0066d9e0 = 0;
            }
        }
        1
    }
}
