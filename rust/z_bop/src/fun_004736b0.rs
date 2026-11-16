// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_005181c0: f32;
    static mut DAT_0060f1c0: u32;
    static mut DAT_0060f1e8: u32;
    static mut _DAT_0060f208: *mut f32;
    static mut _DAT_0060f1f0: u32;
    static mut DAT_0060f1d8: u32;
    static mut DAT_0060f204: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00479de0(param_1: i32, param_2: *mut i8) -> *mut u32;
    fn FUN_00473ad0(param_1: i32, param_2: *mut f32, param_3: *mut f32) -> i32;
    fn FUN_004210a0(param_1: *mut f32, param_2: *mut f32);
    fn FUN_00420fe0(param_1: *mut f32, param_2: *mut f32);
    fn FUN_004211d0(param_1: *mut f32, param_2: *mut f32);
    fn FUN_00421040(param_1: *mut f32, param_2: *mut f32) -> f64;
    fn FUN_00421060(param_1: *mut f32, param_2: i32) -> f64;
    fn FUN_00421080(param_1: *mut f32, param_2: i32) -> f64;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004736b0(
    param_1: i32,
    param_2: *mut u32,
    param_3: *mut u32,
    param_4: *mut f32,
    param_5: *mut f32,
) -> u32 {
    let mut param_2 = param_2;
    if param_2.is_null() {
        return 0;
    }
    // SAFETY: all pointers are the game's own data structures; globals and
    // callees belong to the game, accessed in the original's order.
    unsafe {
        while param_2.add(1).read_unaligned() == 7 {
            param_2 = FUN_00479de0(param_1, param_2.add(5).read_unaligned() as *mut i8);
            if param_2.is_null() {
                return 0;
            }
        }
        let pf2 = param_2.add(5).read_unaligned() as *mut f32;
        let f = param_3 as *mut f32;
        if !pf2.is_null() {
            match param_2.add(1).read_unaligned() {
                1 | 0xf | 0x10 | 0x11 => {
                    let n = param_2.add(4).read_unaligned();
                    let mut p7 = (pf2 as u32).wrapping_add(4);
                    param_3.add(0x23).write_unaligned(n);
                    param_3
                        .add(0x24)
                        .write_unaligned(pf2.cast::<u32>().read_unaligned());
                    param_3.add(0x28).write_unaligned(p7);
                    p7 = p7.wrapping_add(n.wrapping_mul(0x10).wrapping_mul(4));
                    param_3.add(0x25).write_unaligned(p7);
                    let i6 = n as i32;
                    let p26 = p7.wrapping_add((i6 as u32).wrapping_mul(3).wrapping_mul(4));
                    param_3.add(0x26).write_unaligned(p26);
                    if param_3.add(0x21).read_unaligned() == 0 {
                        param_3
                            .add(0x21)
                            .write_unaligned(p26.wrapping_add(n.wrapping_mul(6).wrapping_mul(4)));
                        if FUN_00473ad0(param_3 as i32, param_4, param_5) != 0 {
                            return 1;
                        }
                        param_3.add(0x21).write_unaligned(0);
                    } else if FUN_00473ad0(param_3 as i32, param_4, param_5) != 0 {
                        return 1;
                    }
                }
                2 => {
                    let p7 = f.add(2);
                    FUN_004210a0(f.add(5), pf2.add(3));
                    FUN_00420fe0(p7, pf2.add(3));
                    p7.write_unaligned(p7.read_unaligned() + pf2.read_unaligned());
                    let p7 = f.add(0x11);
                    f.add(3)
                        .write_unaligned(pf2.add(1).read_unaligned() + f.add(3).read_unaligned());
                    f.add(4)
                        .write_unaligned(pf2.add(2).read_unaligned() + f.add(4).read_unaligned());
                    FUN_004211d0(p7, pf2.add(3));
                    let v = FUN_00421040(pf2, p7);
                    f.add(0xe)
                        .write_unaligned((f.add(0xe).read_unaligned() as f64 - v) as f32);
                    let v = FUN_00421060(pf2, p7 as i32);
                    f.add(0xf)
                        .write_unaligned((f.add(0xf).read_unaligned() as f64 - v) as f32);
                    let v = FUN_00421080(pf2, p7 as i32);
                    f.add(0x10)
                        .write_unaligned((f.add(0x10).read_unaligned() as f64 - v) as f32);
                }
                3 => {
                    param_3
                        .add(0x1e)
                        .write_unaligned(param_2.add(4).read_unaligned());
                    param_3.add(0x1f).write_unaligned(pf2 as u32);
                }
                4 => {
                    f.add(2)
                        .write_unaligned(f.add(2).read_unaligned() + pf2.read_unaligned());
                    let p7 = f.add(0x11);
                    f.add(3)
                        .write_unaligned(pf2.add(1).read_unaligned() + f.add(3).read_unaligned());
                    f.add(4)
                        .write_unaligned(pf2.add(2).read_unaligned() + f.add(4).read_unaligned());
                    let v = FUN_00421040(pf2, p7);
                    f.add(0xe)
                        .write_unaligned((f.add(0xe).read_unaligned() as f64 - v) as f32);
                    let v = FUN_00421060(pf2, p7 as i32);
                    f.add(0xf)
                        .write_unaligned((f.add(0xf).read_unaligned() as f64 - v) as f32);
                    let v = FUN_00421080(pf2, p7 as i32);
                    f.add(0x10)
                        .write_unaligned((f.add(0x10).read_unaligned() as f64 - v) as f32);
                }
                5 => {
                    FUN_004210a0(f.add(5), pf2);
                    FUN_00420fe0(f.add(2), pf2);
                    FUN_004211d0(f.add(0x11), pf2);
                }
                6 => {
                    let sx = pf2.read_unaligned();
                    let sy = pf2.add(1).read_unaligned();
                    let sz = pf2.add(2).read_unaligned();
                    let mut i = 5usize;
                    while i <= 0xb {
                        f.add(i).write_unaligned(f.add(i).read_unaligned() * sx);
                        f.add(i + 1)
                            .write_unaligned(sy * f.add(i + 1).read_unaligned());
                        f.add(i + 2)
                            .write_unaligned(sz * f.add(i + 2).read_unaligned());
                        i += 3;
                    }
                    // note: C order is 5,8,0xb then 2; the sets are disjoint
                    f.add(2).write_unaligned(f.add(2).read_unaligned() * sx);
                    f.add(3).write_unaligned(sy * f.add(3).read_unaligned());
                    f.add(4).write_unaligned(sz * f.add(4).read_unaligned());
                    let mut fv = _DAT_005181c0 / sx;
                    f.add(0x11)
                        .write_unaligned(f.add(0x11).read_unaligned() * fv);
                    f.add(0x12)
                        .write_unaligned(f.add(0x12).read_unaligned() * fv);
                    f.add(0x13)
                        .write_unaligned(fv * f.add(0x13).read_unaligned());
                    fv = _DAT_005181c0 / sy;
                    f.add(0x14)
                        .write_unaligned(f.add(0x14).read_unaligned() * fv);
                    f.add(0x15)
                        .write_unaligned(f.add(0x15).read_unaligned() * fv);
                    f.add(0x16)
                        .write_unaligned(fv * f.add(0x16).read_unaligned());
                    fv = _DAT_005181c0 / sz;
                    f.add(0x17)
                        .write_unaligned(f.add(0x17).read_unaligned() * fv);
                    f.add(0x18)
                        .write_unaligned(f.add(0x18).read_unaligned() * fv);
                    f.add(0x19)
                        .write_unaligned(fv * f.add(0x19).read_unaligned());
                }
                0xb => {
                    _DAT_0060f208 = pf2;
                    param_3.add(0x21).write_unaligned(pf2 as u32);
                }
                _ => {}
            }
        }
        let mut off: u32 = 0;
        if param_2.add(2).read_unaligned() != 0 {
            let mut local_d0 = [0u32; 52];
            let mut i = 0usize;
            while i < 0x34 {
                local_d0[i] = param_3.add(i).read_unaligned();
                i += 1;
            }
            DAT_0060f1c0 = 0;
            DAT_0060f1e8 = 0;
            let mut cnt: i32 = 0;
            _DAT_0060f208 = core::ptr::null_mut();
            _DAT_0060f1f0 = 0;
            DAT_0060f1d8 = 0;
            DAT_0060f204 = 0;
            if 0 < param_2.add(2).read_unaligned() as i32 {
                loop {
                    let base = param_2.add(3).read_unaligned();
                    let child = (base.wrapping_add(off) as *const u32).read_unaligned() as *mut u32;
                    let r = FUN_004736b0(param_1, child, local_d0.as_mut_ptr(), param_4, param_5);
                    if r != 0 {
                        return 1;
                    }
                    off = off.wrapping_add(4);
                    cnt = cnt.wrapping_add(1);
                    if cnt >= param_2.add(2).read_unaligned() as i32 {
                        break;
                    }
                }
            }
        }
        0
    }
}
