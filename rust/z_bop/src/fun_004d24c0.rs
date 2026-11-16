// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00acebf9: i32;
    static mut DAT_00665050: i32;
    static mut DAT_00665058: i32;
    static mut DAT_00665060: i32;
    static mut DAT_0066505c: i32;
    static mut DAT_00665410: i32;
    static mut DAT_00b69a89: i32;
    static mut DAT_00b641b9: i32;
    static mut DAT_00b641c1: i32;
    static mut DAT_00b641bd: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d23b0(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d24c0(param_1: i32) -> u32 {
    // SAFETY: the globals and the callee are the game's own memory; the
    // table walks stay within the game's arrays as the original does.
    unsafe {
        let end = &raw mut DAT_00665410;
        let mut u_var1: u32;
        if DAT_00acebf9 == 0 {
            let mut i_var4: usize = 0;
            let mut p = &raw mut DAT_00665050;
            loop {
                if p.read_unaligned() == param_1 {
                    u_var1 = (&raw mut DAT_00665058)
                        .wrapping_add(i_var4.wrapping_mul(6))
                        .read_unaligned() as u32;
                    if u_var1 == 0 {
                        u_var1 = 1;
                    }
                    let a = (&raw mut DAT_00665060)
                        .wrapping_add(i_var4.wrapping_mul(6))
                        .read_unaligned();
                    let b = (&raw mut DAT_0066505c)
                        .wrapping_add(i_var4.wrapping_mul(6))
                        .read_unaligned();
                    let n = (a.wrapping_mul(2).wrapping_add(b) as u32).wrapping_mul(10000);
                    u_var1 = n / u_var1;
                    if (u_var1 as i32) < 0x2711 {
                        return u_var1;
                    }
                    return 10000;
                }
                p = p.wrapping_add(6);
                i_var4 = i_var4.wrapping_add(1);
                if !(p < end) {
                    break;
                }
            }
            u_var1 = 0;
        } else {
            u_var1 = FUN_004d23b0(param_1);
            let mut i_var4: i32;
            let mut i_var3: i32;
            if u_var1 < (DAT_00b69a89 as u32) && u_var1 < 0x28 {
                let idx = (u_var1 as usize).wrapping_mul(0x8e);
                i_var4 = (&raw mut DAT_00b641b9).wrapping_add(idx).read_unaligned();
                let c = (&raw mut DAT_00b641c1).wrapping_add(idx).read_unaligned();
                let d = (&raw mut DAT_00b641bd).wrapping_add(idx).read_unaligned();
                i_var3 = c.wrapping_mul(2).wrapping_add(d);
            } else {
                i_var4 = 0;
                i_var3 = 0;
            }
            let mut p = &raw mut DAT_00665050;
            loop {
                if p.read_unaligned() == param_1 {
                    i_var4 = i_var4.wrapping_add(p.wrapping_add(2).read_unaligned());
                    let e = p.wrapping_add(4).read_unaligned();
                    let f = p.wrapping_add(3).read_unaligned();
                    i_var3 = i_var3.wrapping_add(e.wrapping_mul(2).wrapping_add(f));
                }
                p = p.wrapping_add(6);
                if !(p < end) {
                    break;
                }
            }
            if i_var4 == 0 {
                i_var4 = 1;
            }
            let n = i_var3.wrapping_mul(10000);
            let q: i32;
            core::arch::asm!(
                "cdq",
                "idiv {d:e}",
                d = in(reg) i_var4,
                inout("eax") n => q,
                out("edx") _,
                options(nomem, nostack)
            );
            u_var1 = q as u32;
            if 10000 < (u_var1 as i32) {
                return 10000;
            }
        }
        u_var1
    }
}
