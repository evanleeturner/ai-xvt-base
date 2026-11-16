// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa5cb0: i32;
    static mut DAT_00aa5cb4: i32;
    static mut DAT_00aa5ac0: i32;
    static mut DAT_00aa5ac4: i32;
    static mut DAT_00aa5adc: i32;
    static mut DAT_00aa5ccc: i32;
    static mut DAT_00aa5df0: i32;
    static mut DAT_00aa5e10: i32;
    static mut DAT_00aa5e68: i32;
    static mut DAT_00b69e30: i32;
    static mut DAT_00b6a24c: i32;
    static mut DAT_00aa615e: u8;
    static mut DAT_00b6d5aa: u8;
    static mut DAT_00b6d86a: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cfe80(param_1: *mut i32) -> *mut i32;
    fn FUN_004d11e0() -> i32;
    fn FUN_004d1400() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f14a0() -> i32 {
    // SAFETY: all globals and callees are the game's own memory; pointers
    // walk the game's tables exactly as the original does.
    unsafe {
        let mut local_c: i32 = 0;
        let mut iv10: i32 = 10;
        let puv1 = FUN_004cfe80(&raw mut local_c);
        let mut pi2: *mut i32 = &raw mut DAT_00aa5cb0;
        loop {
            let mut iv7: i32 = 8;
            loop {
                let mut iv8: i32 = 0;
                let mut b11 = local_c == 0;
                if 0 < local_c {
                    let mut pi5 = puv1.add(9);
                    loop {
                        if pi5.read() != 0 && pi5.sub(1).read() == pi2.read() {
                            break;
                        }
                        pi5 = pi5.add(10);
                        iv8 = iv8.wrapping_add(1);
                        if !(iv8 < local_c) {
                            break;
                        }
                    }
                    b11 = local_c == iv8;
                }
                if b11 {
                    pi2.write(0);
                }
                pi2 = pi2.add(1);
                iv7 = iv7.wrapping_sub(1);
                if iv7 == 0 {
                    break;
                }
            }
            iv10 = iv10.wrapping_sub(1);
            if iv10 == 0 {
                break;
            }
        }

        let zero: i32 = 0;
        let mut pu4: *mut u8 = &raw mut DAT_00aa615e;
        pi2 = &raw mut DAT_00aa5df0;
        loop {
            iv10 = 0;
            if 0 < local_c {
                let mut pi5 = puv1.add(9);
                loop {
                    if pi5.read() != 0 && pi5.sub(1).read() == pi2.read() {
                        break;
                    }
                    pi5 = pi5.add(10);
                    iv10 = iv10.wrapping_add(1);
                    if !(iv10 < local_c) {
                        break;
                    }
                }
            }
            if iv10 == 8 {
                pi2.write(0);
                pu4.cast::<i32>().write_unaligned(zero);
            }
            pu4 = pu4.add(0x2a);
            pi2 = pi2.add(1);
            if !(pi2 < &raw mut DAT_00aa5e10) {
                break;
            }
        }

        let mut pb2: *mut u8 = &raw mut DAT_00b6d5aa;
        loop {
            iv10 = 0;
            let tmp: i32 = pb2.cast::<i32>().read_unaligned();
            if 0 < local_c {
                let mut pi5 = puv1.add(9);
                loop {
                    if pi5.read() != 0 && pi5.sub(1).read() == tmp {
                        break;
                    }
                    pi5 = pi5.add(10);
                    iv10 = iv10.wrapping_add(1);
                    if !(iv10 < local_c) {
                        break;
                    }
                }
            }
            if local_c == iv10 {
                pb2.cast::<i32>().write_unaligned(zero);
            }
            pb2 = pb2.add(0x58);
            if !(pb2 < &raw mut DAT_00b6d86a) {
                break;
            }
        }

        iv10 = 0;
        pb2 = &raw mut DAT_00b6d5aa;
        let mut iv7: i32;
        loop {
            let iv8 = FUN_004d11e0();
            iv7 = iv10;
            let tmp: i32 = pb2.cast::<i32>().read_unaligned();
            if iv8 == tmp {
                break;
            }
            pb2 = pb2.add(0x58);
            iv10 = iv10.wrapping_add(1);
            iv7 = DAT_00b6a24c;
            if !(pb2 < &raw mut DAT_00b6d86a) {
                break;
            }
        }
        DAT_00b6a24c = iv7;

        if DAT_00b69e30 == 2 {
            local_c = 1;
        } else {
            local_c = FUN_004d1400();
        }

        let base_cb0 = (&raw mut DAT_00aa5cb0).cast::<u8>();
        let base_cb4 = (&raw mut DAT_00aa5cb4).cast::<u8>();
        let base_ac0 = (&raw mut DAT_00aa5ac0).cast::<u8>();
        let base_ac4 = (&raw mut DAT_00aa5ac4).cast::<u8>();
        let base_ccc = (&raw mut DAT_00aa5ccc).cast::<u8>();
        let base_adc = (&raw mut DAT_00aa5adc).cast::<u8>();

        let mut local_8: *mut i32 = &raw mut DAT_00aa5df0;
        loop {
            iv10 = 0;
            let mut b11 = local_c == 0;
            if 0 < local_c {
                loop {
                    if local_8.read() == 0 {
                        break;
                    }
                    iv10 = iv10.wrapping_add(1);
                    if !(iv10 < local_c) {
                        break;
                    }
                }
                b11 = local_c == iv10;
            }
            if !b11 && 0 < DAT_00aa5e68 {
                let mut iv7: i32 = 0;
                iv10 = local_8.read();
                let mut local_4: i32 = DAT_00aa5e68;
                loop {
                    let mut iv6: i32 = 0;
                    let mut iv8: i32 = iv7;
                    loop {
                        if base_cb0
                            .byte_add(iv8 as usize)
                            .cast::<i32>()
                            .read_unaligned()
                            == iv10
                        {
                            if iv6 < 7 {
                                let mut iv3: i32 = 7 - iv6;
                                let mut iv9: i32 = iv8;
                                loop {
                                    iv3 = iv3.wrapping_sub(1);
                                    let a = base_cb4
                                        .byte_add(iv9 as usize)
                                        .cast::<i32>()
                                        .read_unaligned();
                                    base_cb0
                                        .byte_add(iv9 as usize)
                                        .cast::<i32>()
                                        .write_unaligned(a);
                                    let b = base_ac4
                                        .byte_add(iv9 as usize)
                                        .cast::<i32>()
                                        .read_unaligned();
                                    base_ac0
                                        .byte_add(iv9 as usize)
                                        .cast::<i32>()
                                        .write_unaligned(b);
                                    iv9 = iv9.wrapping_add(4);
                                    if iv3 == 0 {
                                        break;
                                    }
                                }
                            }
                            base_ccc
                                .byte_add(iv7 as usize)
                                .cast::<i32>()
                                .write_unaligned(0);
                            base_adc
                                .byte_add(iv7 as usize)
                                .cast::<i32>()
                                .write_unaligned(-1);
                        }
                        iv8 = iv8.wrapping_add(4);
                        iv6 = iv6.wrapping_add(1);
                        if !(iv6 < 8) {
                            break;
                        }
                    }
                    iv7 = iv7.wrapping_add(0x20);
                    local_4 = local_4.wrapping_sub(1);
                    if local_4 == 0 {
                        break;
                    }
                }
            }
            local_8 = local_8.add(1);
            if (local_8 as usize) > 0xaa5e0f {
                return 1;
            }
        }
    }
}
