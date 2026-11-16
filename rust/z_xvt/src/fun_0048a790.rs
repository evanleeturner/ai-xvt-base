// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525518: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __strcmpi(a: *const i8, b: *const i8) -> i32;
    fn FUN_0048e520(a: *mut u32, b: *mut i8) -> *mut u32;
    fn FUN_0048a980(a: i32, b: i32, c: i32) -> i32;
}

unsafe fn rd(a: i32) -> i32 {
    // SAFETY: the address is the game's own memory, read where the original reads.
    unsafe { (a as u32 as usize as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0048a790(param_1: *mut u32, param_2: i32, param_3: i32) {
    let mut param_1 = param_1;
    if param_1.is_null() {
        return;
    }
    // SAFETY: all pointers and globals belong to the game; accesses mirror the original.
    unsafe {
        while param_1.add(1).read_unaligned() == 7 {
            let str2 = param_1.add(5).read_unaligned() as usize as *mut i8;
            let mut local_4: i32 = 0;
            let mut tmp_a = rd(param_2.wrapping_add(6));
            'search: {
                if 0 < tmp_a {
                    let mut local_8: i32 = 0;
                    loop {
                        let tmp_b = rd(param_2.wrapping_add(10));
                        let mut puvar3 =
                            rd(tmp_b.wrapping_add(local_8)) as u32 as usize as *mut u32;
                        if puvar3.is_null() {
                            puvar3 = core::ptr::null_mut();
                        } else if puvar3.read_unaligned() == 0
                            || __strcmpi(puvar3.read_unaligned() as usize as *const i8, str2) != 0
                        {
                            let mut iv5: i32 = 0;
                            let mut iv2: i32 = 0;
                            if 0 < puvar3.add(2).read_unaligned() as i32 {
                                loop {
                                    let base = puvar3.add(3).read_unaligned() as i32;
                                    let child =
                                        rd(base.wrapping_add(iv5)) as u32 as usize as *mut u32;
                                    param_1 = FUN_0048e520(child, str2);
                                    if !param_1.is_null() {
                                        break 'search;
                                    }
                                    iv5 = iv5.wrapping_add(4);
                                    iv2 = iv2.wrapping_add(1);
                                    if !(iv2 < puvar3.add(2).read_unaligned() as i32) {
                                        break;
                                    }
                                }
                            }
                            puvar3 = core::ptr::null_mut();
                        }
                        param_1 = puvar3;
                        if !puvar3.is_null() {
                            break 'search;
                        }
                        local_8 = local_8.wrapping_add(4);
                        local_4 = local_4.wrapping_add(1);
                        tmp_a = rd(param_2.wrapping_add(6));
                        if !(local_4 < tmp_a) {
                            break;
                        }
                    }
                }
                param_1 = core::ptr::null_mut();
            }
            if param_1.is_null() {
                return;
            }
        }
        if param_1.add(1).read_unaligned() == 0x14 {
            let piv1 = param_1.add(5).read_unaligned() as usize as *mut i32;
            if piv1.add(1).read_unaligned() == 0 {
                let mut iv5 = piv1.read_unaligned();
                let mut iv2 = iv5;
                if DAT_00525518 == 2 {
                    iv2 = iv5.wrapping_add(0x1000);
                }
                let mut iv4 = piv1
                    .add(5)
                    .read_unaligned()
                    .wrapping_mul(piv1.add(4).read_unaligned());
                if piv1.add(2).read_unaligned() == iv4 {
                    iv4 = piv1.add(3).read_unaligned();
                }
                if iv2.wrapping_sub(0x18).wrapping_sub(iv4) != piv1 as usize as i32 {
                    iv2 = 0;
                    local_4_init(&mut iv2);
                    let mut local_4: i32 = 0;
                    let mut tmp_a = rd(param_3.wrapping_add(6));
                    if 0 < tmp_a {
                        loop {
                            let tmp_a2 = rd(param_3.wrapping_add(10));
                            let a0 = rd(tmp_a2.wrapping_add(iv2));
                            let tmp_b = rd(param_2.wrapping_add(10));
                            let a1 = rd(tmp_b.wrapping_add(iv2));
                            iv4 = FUN_0048a980(a0, a1, iv5);
                            if iv4 != 0 {
                                iv2 = rd(iv4.wrapping_add(0x14));
                                iv5 = rd(iv2.wrapping_add(0x14))
                                    .wrapping_mul(rd(iv2.wrapping_add(0x10)));
                                if rd(iv2.wrapping_add(8)) == iv5 {
                                    iv5 = rd(iv2.wrapping_add(0xc));
                                }
                                iv5 = iv2.wrapping_add(0x18).wrapping_add(iv5);
                                if DAT_00525518 == 2 {
                                    iv5 = iv5.wrapping_add(-0x1000);
                                }
                                piv1.write_unaligned(iv5);
                                break;
                            }
                            iv2 = iv2.wrapping_add(4);
                            local_4 = local_4.wrapping_add(1);
                            tmp_a = rd(param_3.wrapping_add(6));
                            if !(local_4 < tmp_a) {
                                break;
                            }
                        }
                    }
                }
            } else {
                piv1.add(1).write_unaligned(0);
                let mut iv2 = piv1
                    .add(5)
                    .read_unaligned()
                    .wrapping_mul(piv1.add(4).read_unaligned());
                if piv1.add(2).read_unaligned() == iv2 {
                    iv2 = piv1.add(3).read_unaligned();
                }
                iv2 = (piv1 as usize as i32).wrapping_add(iv2).wrapping_add(0x18);
                if DAT_00525518 == 2 {
                    iv2 = iv2.wrapping_add(-0x1000);
                }
                piv1.write_unaligned(iv2);
            }
        }
        let mut iv5: i32 = 0;
        let mut iv2: i32 = 0;
        if 0 < param_1.add(2).read_unaligned() as i32 {
            loop {
                let p = (param_1.add(3).read_unaligned() as i32).wrapping_add(iv5) as u32 as usize
                    as *mut u32;
                iv5 = iv5.wrapping_add(4);
                iv2 = iv2.wrapping_add(1);
                FUN_0048a790(p.read_unaligned() as usize as *mut u32, param_2, param_3);
                if !(iv2 < param_1.add(2).read_unaligned() as i32) {
                    break;
                }
            }
        }
    }
}

#[inline(always)]
fn local_4_init(_x: &mut i32) {}
