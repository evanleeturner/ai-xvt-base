// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005272bc: i32;
    static mut DAT_005272b0: i32;
    static mut DAT_0060f1f8: *mut u32;
    static mut DAT_0060f1dc: *mut u32;
    static mut DAT_0060f1c0: i32;
    static mut DAT_0060f200: i32;
    static mut DAT_0060f1cc: i32;
    static mut DAT_0060f1e8: i32;
    static mut DAT_0060f1f4: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00479de0(param_4: i32, param_name: *mut i8) -> *mut u32;
    fn FID_conflict__memcpy(
        dst: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: u32,
    ) -> *mut core::ffi::c_void;
    fn FUN_00475b70(a: i32, b: i32, c: i32) -> i32;
    fn FUN_00475c70(a: i32, b: i32, c: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00475d50(
    param_1: i32,
    param_2: *mut u32,
    param_3: *mut u32,
    param_4: i32,
    param_5: i32,
) {
    let mut param_3 = param_3;
    if param_3.is_null() {
        return;
    }
    // SAFETY: all pointers are the game's own structures and globals; accessed
    // exactly where the original does.
    unsafe {
        while param_3.add(1).read_unaligned() == 7 {
            param_3 = FUN_00479de0(param_4, param_3.add(5).read_unaligned() as usize as *mut i8);
            if param_3.is_null() {
                return;
            }
        }
        if DAT_005272bc == 0 && param_3 == param_2 {
            DAT_005272bc = 1;
        }
        let at = |i: usize| -> u32 { param_3.add(i).read_unaligned() };
        let rp = |p: *mut i32, i: usize| -> i32 { p.add(i).read_unaligned() };
        let p1_14 = (param_1 as u32).wrapping_add(0x14) as usize as *mut *mut i32;
        let p1_10 = (param_1 as u32).wrapping_add(0x10) as usize as *mut i32;
        match at(1) {
            1 | 0xf | 0x10 | 0x11 => {
                if DAT_005272bc != 0
                    && DAT_0060f1f8 == DAT_0060f1dc
                    && 0 < (at(5) as usize as *const i32).read_unaligned()
                {
                    let mut i_var6: i32 = p1_14.read_unaligned().read_unaligned();
                    let mut pi5: *mut i32 = p1_14.read_unaligned().wrapping_add(1);
                    FID_conflict__memcpy(
                        pi5.wrapping_add(at(4).wrapping_mul(0x10) as usize) as *mut _,
                        pi5 as *const _,
                        (p1_10.read_unaligned() as u32).wrapping_mul(100),
                    );
                    let mut pi3: *mut i32 = (at(5).wrapping_add(4)) as usize as *mut i32;
                    let mut local_c: i32 = ((param_5 as u32).wrapping_add(0x84) as usize
                        as *const i32)
                        .read_unaligned();
                    if local_c == 0 {
                        let i_var2: i32 = if DAT_005272b0 == 0 {
                            (at(4) << 6) as i32
                        } else {
                            at(4).wrapping_mul(0x30) as i32
                        };
                        local_c = (pi3 as usize as u32)
                            .wrapping_add(at(4).wrapping_mul(0x24))
                            .wrapping_add(i_var2 as u32) as i32;
                    }
                    let mut local_4: i32 = 0;
                    let mut pi1 = pi5;
                    if 0 < at(4) as i32 {
                        loop {
                            let v = FUN_00475b70(DAT_0060f200, DAT_0060f1c0, rp(pi3, 0));
                            pi1.write_unaligned(v);
                            let v = FUN_00475b70(DAT_0060f200, DAT_0060f1c0, rp(pi3, 1));
                            pi1.add(1).write_unaligned(v);
                            let v = FUN_00475b70(DAT_0060f200, DAT_0060f1c0, rp(pi3, 2));
                            pi1.add(2).write_unaligned(v);
                            let v = FUN_00475b70(DAT_0060f200, DAT_0060f1c0, rp(pi3, 3));
                            pi1.add(3).write_unaligned(v);
                            pi1.add(4).write_unaligned(rp(pi3, 4).wrapping_add(i_var6));
                            pi1.add(5).write_unaligned(rp(pi3, 5).wrapping_add(i_var6));
                            pi1.add(6).write_unaligned(rp(pi3, 6).wrapping_add(i_var6));
                            if rp(pi3, 7) == -1 {
                                pi1.add(7).write_unaligned(-1);
                            } else {
                                pi1.add(7).write_unaligned(i_var6.wrapping_add(rp(pi3, 7)));
                            }
                            let v = FUN_00475c70(DAT_0060f1f4, DAT_0060f1e8, rp(pi3, 8));
                            pi1.add(8).write_unaligned(v);
                            let v = FUN_00475c70(DAT_0060f1f4, DAT_0060f1e8, rp(pi3, 9));
                            pi1.add(9).write_unaligned(v);
                            let v = FUN_00475c70(DAT_0060f1f4, DAT_0060f1e8, rp(pi3, 10));
                            pi1.add(10).write_unaligned(v);
                            let v = FUN_00475c70(DAT_0060f1f4, DAT_0060f1e8, rp(pi3, 0xb));
                            pi1.add(0xb).write_unaligned(v);
                            let mut pi4 = pi3.wrapping_add(0xc);
                            if DAT_005272b0 != 0 {
                                pi4 = pi3;
                            }
                            let v = FUN_00475b70(DAT_0060f1cc, local_c, rp(pi4, 0));
                            pi1.add(0xc).write_unaligned(v);
                            let v = FUN_00475b70(DAT_0060f1cc, local_c, rp(pi4, 1));
                            pi1.add(0xd).write_unaligned(v);
                            let v = FUN_00475b70(DAT_0060f1cc, local_c, rp(pi4, 2));
                            pi3 = pi4.wrapping_add(4);
                            pi1.add(0xe).write_unaligned(v);
                            let v = FUN_00475b70(DAT_0060f1cc, local_c, rp(pi4, 3));
                            pi1.add(0xf).write_unaligned(v);
                            if DAT_005272b0 != 0 {
                                pi3 = pi4.wrapping_add(0xc);
                            }
                            local_4 = local_4.wrapping_add(1);
                            pi1 = pi1.wrapping_add(0x10);
                            if !(local_4 < at(4) as i32) {
                                break;
                            }
                        }
                    }
                    pi5 = pi5.wrapping_add(
                        ((p1_10.read_unaligned() as u32)
                            .wrapping_add(at(4))
                            .wrapping_mul(0x10)) as usize,
                    );
                    FID_conflict__memcpy(
                        pi5.wrapping_add(at(4).wrapping_mul(3) as usize) as *mut _,
                        pi5 as *const _,
                        (p1_10.read_unaligned() as u32).wrapping_mul(0x24),
                    );
                    if DAT_005272b0 == 0 {
                        i_var6 = (at(4) << 6) as i32;
                    } else {
                        i_var6 = at(4).wrapping_mul(0x30) as i32;
                    }
                    let mut i_var2: i32 = 0;
                    pi1 = at(5).wrapping_add(i_var6 as u32).wrapping_add(4) as usize as *mut i32;
                    pi3 = pi5;
                    if 0 < at(4) as i32 {
                        loop {
                            pi3.write_unaligned(rp(pi1, 0));
                            let pi4 = pi1.wrapping_add(2);
                            pi3.add(1).write_unaligned(rp(pi1, 1));
                            pi1 = pi1.wrapping_add(3);
                            pi3.add(2).write_unaligned(pi4.read_unaligned());
                            pi3 = pi3.wrapping_add(3);
                            i_var2 = i_var2.wrapping_add(1);
                            if !(i_var2 < at(4) as i32) {
                                break;
                            }
                        }
                    }
                    pi5 = pi5.wrapping_add(
                        ((p1_10.read_unaligned() as u32)
                            .wrapping_add(at(4))
                            .wrapping_mul(3)) as usize,
                    );
                    FID_conflict__memcpy(
                        pi5.wrapping_add(at(4).wrapping_mul(6) as usize) as *mut _,
                        pi5 as *const _,
                        (p1_10.read_unaligned() as u32).wrapping_mul(0x18),
                    );
                    if DAT_005272b0 == 0 {
                        i_var6 = (at(4) << 6) as i32;
                    } else {
                        i_var6 = at(4).wrapping_mul(0x30) as i32;
                    }
                    pi3 = at(5)
                        .wrapping_add(i_var6 as u32)
                        .wrapping_add(4)
                        .wrapping_add(at(4).wrapping_mul(0xc)) as usize
                        as *mut i32;
                    i_var6 = 0;
                    if 0 < at(4) as i32 {
                        loop {
                            pi5.write_unaligned(rp(pi3, 0));
                            pi5.add(1).write_unaligned(rp(pi3, 1));
                            pi5.add(2).write_unaligned(rp(pi3, 2));
                            pi5.add(3).write_unaligned(rp(pi3, 3));
                            let p1b = pi3.wrapping_add(5);
                            pi5.add(4).write_unaligned(rp(pi3, 4));
                            pi3 = pi3.wrapping_add(6);
                            pi5.add(5).write_unaligned(p1b.read_unaligned());
                            pi5 = pi5.wrapping_add(6);
                            i_var6 = i_var6.wrapping_add(1);
                            if !(i_var6 < at(4) as i32) {
                                break;
                            }
                        }
                    }
                    p1_10.write_unaligned((p1_10.read_unaligned() as u32).wrapping_add(at(4)) as i32);
                    pi5 = at(5) as usize as *mut i32;
                    let base = p1_14.read_unaligned();
                    base.write_unaligned(base.read_unaligned().wrapping_add(pi5.read_unaligned()));
                    pi5.write_unaligned(-1);
                }
            }
            3 => {
                DAT_0060f1c0 = at(5) as i32;
            }
            0xb => {
                ((param_5 as u32).wrapping_add(0x84) as usize as *mut u32).write_unaligned(at(5));
            }
            0xd => {
                DAT_0060f1e8 = at(5) as i32;
            }
            0x14 => {
                DAT_0060f1f8 = param_3;
            }
            _ => {}
        }
        let mut i_var6: i32 = 0;
        if at(2) != 0 {
            let mut i_var2: i32 = 0;
            if 0 < at(2) as i32 {
                loop {
                    i_var6 = i_var6.wrapping_add(4);
                    i_var2 = i_var2.wrapping_add(1);
                    let addr = at(3).wrapping_sub(4).wrapping_add(i_var6 as u32);
                    let child = (addr as usize as *const u32).read_unaligned() as usize as *mut u32;
                    FUN_00475d50(param_1, param_2, child, param_4, param_5);
                    if !(i_var2 < at(2) as i32) {
                        break;
                    }
                }
            }
        }
    }
}
