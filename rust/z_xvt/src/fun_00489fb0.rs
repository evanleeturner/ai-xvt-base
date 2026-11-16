// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052a54c: i32;
    static mut DAT_006026b0: *mut u32;
    static mut DAT_00602694: *mut u32;
    static mut DAT_00602678: i32;
    static mut DAT_00602684: i32;
    static mut DAT_006026a0: i32;
    static mut DAT_006026ac: i32;
    static mut DAT_006026b8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __strcmpi(a: *mut i8, b: *mut i8) -> i32;
    fn FUN_0048e520(a: *mut u32, b: *mut i8) -> *mut u32;
    fn FID_conflict__memcpy(
        a: *mut core::ffi::c_void,
        b: *mut core::ffi::c_void,
        n: i32,
    ) -> *mut core::ffi::c_void;
    fn FUN_00489dd0(a: i32, b: i32, c: i32) -> i32;
    fn FUN_00489ed0(a: i32, b: i32, c: i32) -> i32;
}

#[inline(always)]
unsafe fn rd(p: *const u32, i: usize) -> u32 {
    // SAFETY: caller supplies a valid game structure pointer.
    unsafe { p.add(i).read_unaligned() }
}

#[inline(always)]
fn off(p: *mut i32, n: u32) -> *mut i32 {
    p.wrapping_add(n as usize)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00489fb0(
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
    // SAFETY: all pointers are the game's own structures and globals, accessed
    // exactly where the original does.
    unsafe {
        while rd(param_3, 1) == 7 {
            let str2 = rd(param_3, 5) as *mut i8;
            let mut local_c: i32 = 0;
            let mut local_8: i32 = 0;
            let mut found: *mut u32 = core::ptr::null_mut();
            let mut count = ((param_4 as u32).wrapping_add(6) as *const i32).read_unaligned();
            while local_c < count {
                let tmp_arr = ((param_4 as u32).wrapping_add(10) as *const i32).read_unaligned();
                let pu = ((tmp_arr as u32).wrapping_add(local_8 as u32) as *const *mut u32)
                    .read_unaligned();
                let mut res: *mut u32 = core::ptr::null_mut();
                if !pu.is_null() {
                    let first = rd(pu, 0) as *mut i8;
                    if first.is_null() || __strcmpi(first, str2) != 0 {
                        let mut i1: i32 = 0;
                        let mut i6: i32 = 0;
                        if 0 < rd(pu, 2) as i32 {
                            loop {
                                let ent = (rd(pu, 3).wrapping_add(i1 as u32) as *const *mut u32)
                                    .read_unaligned();
                                let r = FUN_0048e520(ent, str2);
                                if !r.is_null() {
                                    res = r;
                                    break;
                                }
                                i1 = i1.wrapping_add(4);
                                i6 = i6.wrapping_add(1);
                                if !(i6 < rd(pu, 2) as i32) {
                                    break;
                                }
                            }
                        }
                    } else {
                        res = pu;
                    }
                }
                if !res.is_null() {
                    found = res;
                    break;
                }
                local_8 = local_8.wrapping_add(4);
                local_c = local_c.wrapping_add(1);
                count = ((param_4 as u32).wrapping_add(6) as *const i32).read_unaligned();
            }
            param_3 = found;
            if param_3.is_null() {
                return;
            }
        }
        if DAT_0052a54c == 0 && param_3 == param_2 {
            DAT_0052a54c = 1;
        }
        match rd(param_3, 1) {
            1 | 0xf | 0x10 | 0x11 => {
                let p3_5 = rd(param_3, 5) as *const i32;
                if DAT_0052a54c != 0 && DAT_006026b0 == DAT_00602694 && 0 < p3_5.read_unaligned() {
                    let p1_14 =
                        ((param_1 as u32).wrapping_add(0x14) as *const *mut i32).read_unaligned();
                    let p1_10 = (param_1 as u32).wrapping_add(0x10) as *mut i32;
                    let i_var1 = p1_14.read_unaligned();
                    let mut pi7 = p1_14.wrapping_add(1);
                    let n = rd(param_3, 4);
                    FID_conflict__memcpy(
                        off(pi7, n.wrapping_mul(0x10)) as *mut _,
                        pi7 as *mut _,
                        (p1_10.read_unaligned() as u32).wrapping_mul(100) as i32,
                    );
                    let mut local_4 =
                        ((param_5 as u32).wrapping_add(0x84) as *const *mut i32).read_unaligned();
                    if local_4.is_null() {
                        local_4 = off(
                            (rd(param_3, 5).wrapping_add(4)) as *mut i32,
                            rd(param_3, 4).wrapping_mul(0x15),
                        );
                    }
                    let mut local_8: i32 = 0;
                    let mut pi4 = pi7;
                    let mut pi3 = rd(param_3, 5).wrapping_add(4) as *mut i32;
                    if 0 < rd(param_3, 4) as i32 {
                        loop {
                            let r = FUN_00489dd0(DAT_006026b8, DAT_00602678, pi3.read_unaligned());
                            pi4.write_unaligned(r);
                            let r = FUN_00489dd0(
                                DAT_006026b8,
                                DAT_00602678,
                                pi3.add(1).read_unaligned(),
                            );
                            pi4.add(1).write_unaligned(r);
                            let r = FUN_00489dd0(
                                DAT_006026b8,
                                DAT_00602678,
                                pi3.add(2).read_unaligned(),
                            );
                            pi4.add(2).write_unaligned(r);
                            let r = FUN_00489dd0(
                                DAT_006026b8,
                                DAT_00602678,
                                pi3.add(3).read_unaligned(),
                            );
                            pi4.add(3).write_unaligned(r);
                            pi4.add(4)
                                .write_unaligned(pi3.add(4).read_unaligned().wrapping_add(i_var1));
                            pi4.add(5)
                                .write_unaligned(pi3.add(5).read_unaligned().wrapping_add(i_var1));
                            pi4.add(6)
                                .write_unaligned(pi3.add(6).read_unaligned().wrapping_add(i_var1));
                            let v7 = pi3.add(7).read_unaligned();
                            if v7 == -1 {
                                pi4.add(7).write_unaligned(-1);
                            } else {
                                pi4.add(7).write_unaligned(i_var1.wrapping_add(v7));
                            }
                            let r = FUN_00489ed0(
                                DAT_006026ac,
                                DAT_006026a0,
                                pi3.add(8).read_unaligned(),
                            );
                            pi4.add(8).write_unaligned(r);
                            let r = FUN_00489ed0(
                                DAT_006026ac,
                                DAT_006026a0,
                                pi3.add(9).read_unaligned(),
                            );
                            pi4.add(9).write_unaligned(r);
                            let r = FUN_00489ed0(
                                DAT_006026ac,
                                DAT_006026a0,
                                pi3.add(10).read_unaligned(),
                            );
                            pi4.add(10).write_unaligned(r);
                            let r = FUN_00489ed0(
                                DAT_006026ac,
                                DAT_006026a0,
                                pi3.add(11).read_unaligned(),
                            );
                            pi4.add(11).write_unaligned(r);
                            let r = FUN_00489dd0(
                                DAT_00602684,
                                local_4 as u32 as i32,
                                pi3.read_unaligned(),
                            );
                            pi4.add(12).write_unaligned(r);
                            let r = FUN_00489dd0(
                                DAT_00602684,
                                local_4 as u32 as i32,
                                pi3.add(1).read_unaligned(),
                            );
                            pi4.add(13).write_unaligned(r);
                            let r = FUN_00489dd0(
                                DAT_00602684,
                                local_4 as u32 as i32,
                                pi3.add(2).read_unaligned(),
                            );
                            pi4.add(14).write_unaligned(r);
                            let r = FUN_00489dd0(
                                DAT_00602684,
                                local_4 as u32 as i32,
                                pi3.add(3).read_unaligned(),
                            );
                            local_8 = local_8.wrapping_add(1);
                            pi4.add(15).write_unaligned(r);
                            pi4 = pi4.wrapping_add(0x10);
                            pi3 = pi3.wrapping_add(0xc);
                            if !(local_8 < rd(param_3, 4) as i32) {
                                break;
                            }
                        }
                    }
                    pi7 = off(
                        pi7,
                        (p1_10.read_unaligned() as u32)
                            .wrapping_add(rd(param_3, 4))
                            .wrapping_mul(0x10),
                    );
                    FID_conflict__memcpy(
                        off(pi7, rd(param_3, 4).wrapping_mul(3)) as *mut _,
                        pi7 as *mut _,
                        (p1_10.read_unaligned() as u32).wrapping_mul(0x24) as i32,
                    );
                    pi4 = rd(param_3, 4)
                        .wrapping_mul(0x30)
                        .wrapping_add(4)
                        .wrapping_add(rd(param_3, 5)) as *mut i32;
                    let mut i1: i32 = 0;
                    pi3 = pi7;
                    if 0 < rd(param_3, 4) as i32 {
                        loop {
                            pi3.write_unaligned(pi4.read_unaligned());
                            let p5 = pi4.wrapping_add(2);
                            pi3.add(1).write_unaligned(pi4.add(1).read_unaligned());
                            pi4 = pi4.wrapping_add(3);
                            pi3.add(2).write_unaligned(p5.read_unaligned());
                            pi3 = pi3.wrapping_add(3);
                            i1 = i1.wrapping_add(1);
                            if !(i1 < rd(param_3, 4) as i32) {
                                break;
                            }
                        }
                    }
                    pi7 = off(
                        pi7,
                        (p1_10.read_unaligned() as u32)
                            .wrapping_add(rd(param_3, 4))
                            .wrapping_mul(3),
                    );
                    FID_conflict__memcpy(
                        off(pi7, rd(param_3, 4).wrapping_mul(6)) as *mut _,
                        pi7 as *mut _,
                        (p1_10.read_unaligned() as u32).wrapping_mul(0x18) as i32,
                    );
                    i1 = 0;
                    pi4 = rd(param_3, 4)
                        .wrapping_mul(0x3c)
                        .wrapping_add(4)
                        .wrapping_add(rd(param_3, 5)) as *mut i32;
                    if 0 < rd(param_3, 4) as i32 {
                        loop {
                            pi7.write_unaligned(pi4.read_unaligned());
                            pi7.add(1).write_unaligned(pi4.add(1).read_unaligned());
                            pi7.add(2).write_unaligned(pi4.add(2).read_unaligned());
                            pi7.add(3).write_unaligned(pi4.add(3).read_unaligned());
                            let p5 = pi4.wrapping_add(5);
                            pi7.add(4).write_unaligned(pi4.add(4).read_unaligned());
                            pi4 = pi4.wrapping_add(6);
                            pi7.add(5).write_unaligned(p5.read_unaligned());
                            pi7 = pi7.wrapping_add(6);
                            i1 = i1.wrapping_add(1);
                            if !(i1 < rd(param_3, 4) as i32) {
                                break;
                            }
                        }
                    }
                    p1_10.write_unaligned(
                        p1_10.read_unaligned().wrapping_add(rd(param_3, 4) as i32),
                    );
                    let pi7b = rd(param_3, 5) as *mut i32;
                    let base =
                        ((param_1 as u32).wrapping_add(0x14) as *const *mut i32).read_unaligned();
                    base.write_unaligned(base.read_unaligned().wrapping_add(pi7b.read_unaligned()));
                    pi7b.write_unaligned(-1);
                }
            }
            3 => {
                DAT_00602678 = rd(param_3, 5) as i32;
            }
            0xb => {
                ((param_5 as u32).wrapping_add(0x84) as *mut u32).write_unaligned(rd(param_3, 5));
            }
            0xd => {
                DAT_006026a0 = rd(param_3, 5) as i32;
            }
            0x14 => {
                DAT_006026b0 = param_3;
            }
            _ => {}
        }
        let mut i1: i32 = 0;
        if rd(param_3, 2) != 0 {
            let mut i6: i32 = 0;
            if 0 < rd(param_3, 2) as i32 {
                loop {
                    i1 = i1.wrapping_add(4);
                    i6 = i6.wrapping_add(1);
                    let child = (rd(param_3, 3).wrapping_sub(4).wrapping_add(i1 as u32)
                        as *const *mut u32)
                        .read_unaligned();
                    FUN_00489fb0(param_1, param_2, child, param_4, param_5);
                    if !(i6 < rd(param_3, 2) as i32) {
                        break;
                    }
                }
            }
        }
    }
}
