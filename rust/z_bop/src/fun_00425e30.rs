// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005233ec: i32;
    static mut DAT_0051c55c: i32;
    static mut DAT_0051c510: [u8; 0];
    static mut DAT_0051c4c0: [u8; 0];
    static mut DAT_0051c4c1: [u8; 0];
    static mut DAT_0051c511: [u8; 0];
    static mut DAT_009d8c04: u32;
    static mut DAT_00a08c7c: u16;
    static mut DAT_00a08c78: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046a5f0(param_1: u16, param_2: u16) -> u32;
    fn FUN_0046a700(param_1: u16, param_2: u16) -> u32;
    fn FUN_0046aaf0(param_1: u32, param_2: u32, param_3: *mut u16, param_4: *mut u16);
}

#[inline(always)]
unsafe fn idiv(a: i32, b: i32) -> i32 {
    let q: i32;
    // SAFETY: the machine's own idiv; the caller has checked b != 0.
    unsafe {
        core::arch::asm!(
            "cdq",
            "idiv {b:e}",
            b = in(reg) b,
            inout("eax") a => q,
            out("edx") _,
        );
    }
    q
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00425e30(param_1: i32, param_2: i32, param_3: i32) {
    let mut local_4: u16 = 0;
    let mut u_stack_2: u16 = 0;
    // SAFETY: all globals and callees are the game's own memory and code;
    // the tables are indexed as the original does.
    unsafe {
        let t510 = (&raw mut DAT_0051c510).cast::<u8>();
        let t511 = (&raw mut DAT_0051c511).cast::<u8>();
        let t4c0 = (&raw mut DAT_0051c4c0).cast::<u8>();
        let t4c1 = (&raw mut DAT_0051c4c1).cast::<u8>();

        if DAT_005233ec != DAT_0051c55c {
            if DAT_005233ec == 0x13 {
                let mut i_var1: i32 = 0x25;
                let mut i_var3: usize = 0;
                loop {
                    *t510.add(i_var3) = *t4c0.add(i_var3);
                    i_var1 = i_var1.wrapping_sub(1);
                    *t511.add(i_var3) = *t4c1.add(i_var3);
                    i_var3 = i_var3.wrapping_add(2);
                    if i_var1 == 0 {
                        break;
                    }
                }
            } else if DAT_005233ec == 0x112 {
                let mut i_var1: i32 = 0;
                let mut i_var3: usize = 0;
                loop {
                    let u_var5 = i_var1 as u16;
                    let u_var4 = FUN_0046a5f0(0x1e, u_var5);
                    *t510.add(i_var3) = u_var4 as u8;
                    i_var1 = i_var1.wrapping_add(0x1bb);
                    let u_var4 = FUN_0046a700(0x1e, u_var5);
                    *t511.add(i_var3) = u_var4 as u8;
                    i_var3 = i_var3.wrapping_add(2);
                    if i_var1 >= 0x4000 {
                        break;
                    }
                }
            } else {
                let mut i_var1: i32 = 0;
                let mut i_var3: usize = 0;
                loop {
                    let u_var5 = i_var1 as u16;
                    let u_var4 = FUN_0046a5f0(0x2c, u_var5);
                    *t510.add(i_var3) = u_var4 as u8;
                    i_var1 = i_var1.wrapping_add(0x1bb);
                    let u_var4 = FUN_0046a700(0x2c, u_var5);
                    *t511.add(i_var3) = u_var4 as u8;
                    i_var3 = i_var3.wrapping_add(2);
                    if i_var1 >= 0x4000 {
                        break;
                    }
                }
            }
            DAT_0051c55c = DAT_005233ec;
        }

        let mut i_var3 = param_1;
        if param_1 < 0 {
            i_var3 = param_1.wrapping_neg();
        }
        let mut u_var4: u32 = (i_var3 as u32).wrapping_shl(DAT_009d8c04.wrapping_sub(5) & 0x1f);
        if param_3 != 0 {
            u_var4 = idiv(u_var4 as i32, param_3) as u32;
        }
        if 0x7fff < (u_var4 as i32) {
            u_var4 = 0x7fff;
        }
        let mut i_var3 = param_2;
        if param_2 < 0 {
            i_var3 = param_2.wrapping_neg();
        }
        let mut u_var2: u32 = (i_var3 as u32).wrapping_shl(DAT_009d8c04.wrapping_sub(5) & 0x1f);
        if param_3 != 0 {
            u_var2 = idiv(u_var2 as i32, param_3) as u32;
        }
        if 0x7fff < (u_var2 as i32) {
            u_var2 = 0x7fff;
        }
        DAT_00a08c7c = u_var4 as u16;
        DAT_00a08c78 = u_var2 as u16;
        FUN_0046aaf0(u_var4, u_var2, &raw mut u_stack_2, &raw mut local_4);
        let u_var5 = (((0x4000i32 - u_stack_2 as i32) / 0x1bb) << 1) as u16;
        let idx = u_var5 as usize;
        let b510 = *t510.add(idx);
        if (b510 as i32) < (DAT_00a08c7c as i16 as i32) {
            DAT_00a08c7c = b510 as u16;
        }
        if param_1 < 0 {
            DAT_00a08c7c = DAT_00a08c7c.wrapping_neg();
        }
        let b511 = *t511.add(idx);
        if (b511 as i32) < (DAT_00a08c78 as i16 as i32) {
            DAT_00a08c78 = b511 as u16;
        }
        if param_2 < 0 {
            DAT_00a08c78 = DAT_00a08c78.wrapping_neg();
        }
    }
}
