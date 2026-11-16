// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00521f19: [u8; 1];
    static mut DAT_009a7ed0: [i16; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac720(param_1: i32) -> *mut i32;
    fn FUN_00471ff0(param_1: *mut i32);
    fn FUN_004ae1a0(param_1: i32, param_2: *mut i32) -> *mut i32;
    fn FUN_004ac740();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004af660(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> i32 {
    let f2 = param_2 as f32;
    let f3 = param_3 as f32;
    let f4 = param_4 as f32;
    let mut local_4: f32 = 2.1474836e+09f32;
    let mut local_18: i32 = 0;
    let mut local_1c: f32;
    let mut local_14: f32;
    // SAFETY: globals and callees are the game's own; pointers returned by
    // the game are read at the byte offsets the original uses.
    unsafe {
        let flags = (&raw const DAT_00521f19)
            .cast::<u8>()
            .add((param_1 as u32).wrapping_mul(0x1a) as usize)
            .read();
        if flags & 1 == 0 {
            return 0;
        }
        let sh = (&raw const DAT_009a7ed0)
            .cast::<i16>()
            .offset(param_1 as isize)
            .read();
        let arg = ((((param_1 as u32).wrapping_mul(5)) >> 16) << 16) | (sh as u16 as u32);
        let p6 = FUN_004ac720(arg as i32);
        if (p6.read() as u32) != (p6 as usize as u32) {
            FUN_00471ff0(p6);
        }
        let mut i8_: i32 = 0;
        let mut local_20: i32 = p6.byte_add(6).cast::<i32>().read_unaligned();
        if 0 < local_20 {
            loop {
                let local_24: i32 = p6.byte_add(10).cast::<i32>().read_unaligned();
                let iv1: i32 = ((local_24 as u32).wrapping_add((i8_ as u32).wrapping_mul(4))
                    as usize as *const i32)
                    .read_unaligned();
                if ((iv1 as u32).wrapping_add(4) as usize as *const i32).read_unaligned() != 0x14 {
                    let p7 = FUN_004ae1a0(iv1, p6);
                    if !p7.is_null() && p7.read() == 1 {
                        let pf = p7 as *const f32;
                        let g = |k: usize| pf.add(k).read();
                        if f2 <= g(0xb) {
                            if g(8) <= f2 {
                                local_1c = 0.0;
                            } else {
                                local_1c = g(8) - f2;
                            }
                        } else {
                            local_1c = f2 - g(0xb);
                        }
                        let f5 = local_1c;
                        local_14 = local_1c;
                        if f3 <= g(0xc) {
                            if g(9) <= f3 {
                                local_1c = 0.0;
                            } else {
                                local_1c = g(9) - f3;
                            }
                        } else {
                            local_1c = f3 - g(0xc);
                        }
                        if f5 < local_1c {
                            local_14 = local_1c;
                        }
                        if f4 <= g(0xd) {
                            if g(10) <= f4 {
                                local_1c = 0.0;
                            } else {
                                local_1c = g(10) - f4;
                            }
                        } else {
                            local_1c = f4 - g(0xd);
                        }
                        if local_14 < local_1c {
                            local_14 = local_1c;
                        }
                        if local_14 < local_4 {
                            local_4 = local_14;
                            local_18 = i8_;
                            if local_14 == 0.0 {
                                break;
                            }
                        }
                    }
                }
                i8_ = i8_.wrapping_add(1);
                local_20 = p6.byte_add(6).cast::<i32>().read_unaligned();
                if !(i8_ < local_20) {
                    break;
                }
            }
        }
        let local_24: i32 = p6.byte_add(10).cast::<i32>().read_unaligned();
        let local_28: i32 = (local_24 as u32 as usize as *const i32).read_unaligned();
        if ((local_28 as u32).wrapping_add(4) as usize as *const i32).read_unaligned() == 0x14 {
            local_18 = local_18.wrapping_sub(1);
        }
        FUN_004ac740();
    }
    local_18
}
