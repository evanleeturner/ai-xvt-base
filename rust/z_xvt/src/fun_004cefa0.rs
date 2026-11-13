// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0078072e: i32;
    static mut DAT_0064f070: i32;
    static mut DAT_0064f430: i32;
    static mut DAT_0081b5be: i32;
    static mut DAT_00815cee: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cee90(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cefa0(param_1: i32) -> u32 {
    // SAFETY: all globals and the callee are the game's own memory; the table
    // is walked in 6-int records from DAT_0064f070 up to DAT_0064f430, and the
    // unaligned globals are read with read_unaligned.
    unsafe {
        let base = &raw mut DAT_0064f070;
        let end = &raw mut DAT_0064f430 as *const i32;
        let mut u_var1: u32;
        if DAT_0078072e == 0 {
            let mut i_var3: i32 = 0;
            let mut pi = base;
            loop {
                if pi.read() == param_1 {
                    u_var1 = pi.add(2).read() as u32;
                    if u_var1 == 0 {
                        u_var1 = 1;
                    }
                    let a = pi.add(3).read();
                    let b = pi.add(4).read();
                    let num = a.wrapping_add(b.wrapping_mul(2)).wrapping_mul(10000) as u32;
                    u_var1 = num / u_var1;
                    if (u_var1 as i32) < 0x2711 {
                        return u_var1;
                    }
                    return 10000;
                }
                pi = pi.add(6);
                i_var3 = i_var3.wrapping_add(1);
                if !((pi as *const i32) < end) {
                    break;
                }
            }
            let _ = i_var3;
            u_var1 = 0;
        } else {
            u_var1 = FUN_004cee90(param_1);
            let mut i_var4: i32;
            let mut i_var3: i32;
            let limit = (&raw mut DAT_0081b5be).cast::<i32>().read_unaligned() as u32;
            if u_var1 < limit && u_var1 < 0x28 {
                let off = (u_var1.wrapping_mul(0x8e) as usize).wrapping_mul(4);
                let p0 = (&raw mut DAT_00815cee).cast::<u8>().byte_add(off);
                i_var4 = p0.cast::<i32>().read_unaligned();
                let x = p0.byte_add(4).cast::<i32>().read_unaligned();
                let y = p0.byte_add(8).cast::<i32>().read_unaligned();
                i_var3 = x.wrapping_add(y.wrapping_mul(2));
            } else {
                i_var4 = 0;
                i_var3 = 0;
            }
            let mut pi = base;
            loop {
                if pi.read() == param_1 {
                    i_var4 = i_var4.wrapping_add(pi.add(2).read());
                    i_var3 = i_var3
                        .wrapping_add(pi.add(3).read())
                        .wrapping_add(pi.add(4).read().wrapping_mul(2));
                }
                pi = pi.add(6);
                if !((pi as *const i32) < end) {
                    break;
                }
            }
            if i_var4 == 0 {
                i_var4 = 1;
            }
            let num = i_var3.wrapping_mul(10000);
            let q: i32;
            core::arch::asm!(
                "cdq",
                "idiv {d:e}",
                d = in(reg) i_var4,
                inout("eax") num => q,
                out("edx") _,
            );
            u_var1 = q as u32;
            if 10000 < (u_var1 as i32) {
                return 10000;
            }
        }
        u_var1
    }
}
