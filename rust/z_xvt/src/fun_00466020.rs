// SPDX-License-Identifier: MIT

#[repr(C)]
struct Loc {
    local_1c: [u8; 8],
    f_stack_14: f32,
    i_stack_10: i32,
}

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052ae18: i32;
    static mut DAT_00732e8f: i8;
    static mut _DAT_005f2608: i32;
    static mut DAT_006a8850: i32;
    static mut DAT_006a884a: u8;
    static mut DAT_005f1d30: i32;
    static mut DAT_005f1d78: u32;
    static mut DAT_005254ec: i32;
    static mut _DAT_005f260c: f32;
    static mut _DAT_005f2610: f32;
    static mut _DAT_005f2614: u32;
    static mut DAT_005f2618: u32;
    static mut DAT_005f261c: u32;
    static mut DAT_005f2620: i32;
    static mut DAT_005f2624: i32;
    static mut DAT_00527d70: [u32; 0];
    static mut DAT_005f1d44: f32;
    static mut _DAT_006da4fc: f32;
    static mut DAT_0070acc8: f32;
    static mut _DAT_006db7bc: f32;
    static mut DAT_00525518: f32;
    static mut DAT_005f1d34: u32;
    static mut DAT_005f1d3c: u32;
    static mut _DAT_005f1d4c: f32;
    static mut _DAT_005f1d64: f32;
    static mut _DAT_005f1d60: f32;
    static mut DAT_0052a350: [f32; 0];
    static mut DAT_005f1d84: u32;
    static mut DAT_005f262c: u8;
    static mut DAT_005f258c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00414520();
    fn FUN_0045f390();
    fn FUN_004a6a80();
    fn FUN_004a6f90();
    fn __ftol() -> i64;
    fn FUN_00467570(a: u32, b: i32, c: f32);
}

unsafe fn ri(a: u32) -> i32 {
    // SAFETY: the address is the game's own memory.
    unsafe { (a as usize as *const i32).read_unaligned() }
}

unsafe fn ru(a: u32) -> u32 {
    // SAFETY: the address is the game's own memory.
    unsafe { (a as usize as *const u32).read_unaligned() }
}

unsafe fn rf(a: u32) -> f32 {
    // SAFETY: the address is the game's own memory.
    unsafe { (a as usize as *const f32).read_unaligned() }
}

unsafe fn f30(off: u32) -> u32 {
    // SAFETY: reads the game's global base pointer.
    unsafe { (DAT_005f1d30 as u32).wrapping_add(off) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00466020() {
    // SAFETY: all globals, callees and pointers are the game's own memory,
    // accessed in the same order as the original.
    unsafe {
        let mut loc = Loc {
            local_1c: [0; 8],
            f_stack_14: 0.0,
            i_stack_10: 0,
        };
        FUN_00414520();
        if DAT_0052ae18 != 0 {
            FUN_0045f390();
            return;
        }
        if DAT_00732e8f == 0 {
            FUN_004a6a80();
        }
        _DAT_005f2608 = 0;
        let mut local_20: i32 = 0;
        if 0 < DAT_006a8850 {
            let mut local_24: i32 = 0;
            loop {
                let base: u32 = (&raw const DAT_006a884a).cast::<u32>().read_unaligned();
                DAT_005f1d30 = base.wrapping_add(local_24 as u32) as i32;
                DAT_005f1d78 = ru(f30(0x40));
                ((f30(0x38)) as usize as *mut u32)
                    .write_unaligned((&raw mut loc.local_1c) as usize as u32);
                let mut i_var8: i32 = 0;
                let mut p7: u32 = f30(4);
                let mut i_var2: i32 = ri((ri(p7) as u32).wrapping_add(0xa8));
                let mut u_var6: u32 = ru((i_var2 as u32).wrapping_add(0x14));
                let mut u_var4: u32 = ru((i_var2 as u32).wrapping_add(0x10));
                if DAT_005254ec != 0
                    && u_var6
                        .wrapping_mul(u_var4)
                        .wrapping_sub(ru((i_var2 as u32).wrapping_add(8)))
                        == 0
                {
                    let l_var9 = __ftol();
                    i_var2 = l_var9 as i32;
                    loop {
                        if !(0x100 < i_var2 && u_var4 != 8 && u_var6 != 8) {
                            break;
                        }
                        i_var2 >>= 2;
                        let i_var3 = u_var6.wrapping_mul(u_var4) as i32;
                        u_var4 = ((u_var4 as i32) >> 1) as u32;
                        i_var8 = i_var8.wrapping_add(i_var3);
                        u_var6 = ((u_var6 as i32) >> 1) as u32;
                    }
                }
                _DAT_005f260c = (u_var4 as i32) as f32;
                _DAT_005f2610 = (u_var6 as i32) as f32;
                _DAT_005f2614 = (&raw const DAT_00527d70)
                    .cast::<u32>()
                    .offset((((u_var4 & 0xfffffff3) as i32) >> 2) as isize)
                    .read();
                DAT_005f2624 = u_var6.wrapping_mul(u_var4).wrapping_sub(1) as i32;
                DAT_005f2618 = (&raw const DAT_00527d70)
                    .cast::<u32>()
                    .offset((((u_var6 & 0xfffffff3) as i32) >> 2) as isize)
                    .read();
                let obj = ri(p7) as u32;
                DAT_005f261c = ru(obj.wrapping_add(0xb0));
                DAT_005f2620 = ri(obj.wrapping_add(0xac)).wrapping_add(i_var8);
                _DAT_005f2608 = ri(p7);
                let mut f_stack_28: f32 = (DAT_005f1d78 as f32) * rf(f30(0x2c)) + rf(f30(0x30));
                DAT_005f1d44 = (_DAT_006da4fc + (DAT_005f1d78 as f32)) * DAT_0070acc8
                    + _DAT_006db7bc * DAT_00525518;
                if DAT_005f1d78 < ru(f30(0x44)) {
                    i_var2 = 0;
                    loop {
                        p7 = ru((ri(f30(0x68)) as u32).wrapping_add(i_var2 as u32));
                        'skip: {
                            if p7 != 0 {
                                u_var6 = DAT_005f1d34 & DAT_005f1d78;
                                if u_var6 == 0 {
                                    _DAT_005f1d4c = 0.0;
                                    _DAT_005f1d64 = 0.0;
                                    u_var6 = DAT_005f1d3c;
                                } else {
                                    _DAT_005f1d64 = u_var6 as f32;
                                    _DAT_005f1d4c = _DAT_005f1d64
                                        * (&raw const DAT_0052a350)
                                            .cast::<f32>()
                                            .offset(DAT_005f1d3c as isize)
                                            .read();
                                    u_var6 = DAT_005f1d3c.wrapping_sub(u_var6);
                                }
                                _DAT_005f1d60 = u_var6 as f32;
                                DAT_005f1d84 = (DAT_005f1d78 >> (DAT_005f262c & 0x1f))
                                    .wrapping_add(DAT_005f258c);
                                loc.i_stack_10 = ri(p7.wrapping_add(12));
                                let mut p5: u32 = ru(p7);
                                u_var6 = ru(p7.wrapping_add(4));
                                let mut i_var8b: i32 = ri(p7.wrapping_add(8));
                                loc.f_stack_14 = (u_var6 as i32) as f32;
                                (f30(0x34) as usize as *mut i32)
                                    .write_unaligned(ri(p7.wrapping_add(16)));
                                let f_var1: f32;
                                if p5 == 0 {
                                    f_var1 = rf(f30(0x28)) * loc.f_stack_14;
                                } else {
                                    loop {
                                        let i_var3 = ri(p5.wrapping_add(4));
                                        if i_var8b <= i_var3 {
                                            break;
                                        }
                                        if (u_var6 as i32) < i_var3 {
                                            FUN_00467570(
                                                u_var6,
                                                i_var3,
                                                ((u_var6 as i32) as f32) * rf(f30(0x28))
                                                    + f_stack_28,
                                            );
                                            u_var6 = ru(p5.wrapping_add(8));
                                            i_var8b = ri(p7.wrapping_add(8));
                                            if i_var8b <= (u_var6 as i32) {
                                                break;
                                            }
                                        } else {
                                            let u_var4b = ru(p5.wrapping_add(8));
                                            if (u_var6 as i32) < (u_var4b as i32) {
                                                u_var6 = u_var4b;
                                                if i_var8b <= (u_var4b as i32) {
                                                    break 'skip;
                                                }
                                            }
                                        }
                                        p5 = ru(p5);
                                        if p5 == 0 {
                                            break;
                                        }
                                    }
                                    if i_var8b <= (u_var6 as i32) {
                                        break 'skip;
                                    }
                                    f_var1 = ((u_var6 as i32) as f32) * rf(f30(0x28));
                                }
                                FUN_00467570(u_var6, i_var8b, f_var1 + f_stack_28);
                            }
                        }
                        f_stack_28 = rf(f30(0x2c)) + f_stack_28;
                        DAT_005f1d44 = DAT_005f1d44 + DAT_0070acc8;
                        DAT_005f1d78 = DAT_005f1d78.wrapping_add(1);
                        i_var2 = i_var2.wrapping_add(4);
                        if !(DAT_005f1d78 < ru(f30(0x44))) {
                            break;
                        }
                    }
                }
                local_24 = local_24.wrapping_add(0x70);
                local_20 = local_20.wrapping_add(1);
                if !(local_20 < DAT_006a8850) {
                    break;
                }
            }
        }
        if DAT_00732e8f == 0 {
            FUN_004a6f90();
        }
    }
}
