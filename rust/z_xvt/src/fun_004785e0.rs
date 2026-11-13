// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00527f7c: i32;
    static mut DAT_0052552c: i32;
    static mut DAT_00527f30: [i8; 1];
    static mut DAT_00527ee0: [i8; 1];
    static mut DAT_00527f31: [i8; 1];
    static mut DAT_00527ee1: [i8; 1];
    static mut DAT_0070acc4: u32;
    static mut DAT_00655a8c: u16;
    static mut DAT_00655a88: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00418680(param_1: u32, param_2: u16) -> u32;
    fn FUN_004187e0(param_1: u32, param_2: u16) -> u32;
    fn FUN_00419000(param_1: u32, param_2: u32, param_3: *mut u16, param_4: *mut u16);
}

#[inline(always)]
unsafe fn idiv(a: i32, b: i32) -> i32 {
    let q: i32;
    // SAFETY: the machine's own idiv, faulting as the original does.
    unsafe {
        core::arch::asm!(
            "cdq",
            "idiv {d:e}",
            d = in(reg) b,
            inout("eax") a => q,
            out("edx") _,
        );
    }
    q
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004785e0(param_1: i32, param_2: i32, param_3: i32) {
    let mut local_4: u16 = 0;
    let mut u_stack_2: u16 = 0;
    // SAFETY: all globals and callees are the game's own memory and code;
    // the tables are indexed as the original indexes them.
    unsafe {
        let p30 = (&raw mut DAT_00527f30).cast::<i8>();
        let p31 = (&raw mut DAT_00527f31).cast::<i8>();
        let p80 = (&raw mut DAT_00527ee0).cast::<i8>();
        let p81 = (&raw mut DAT_00527ee1).cast::<i8>();
        if DAT_00527f7c != DAT_0052552c {
            if DAT_0052552c == 0x13 {
                let mut i_var1: i32 = 0x25;
                let mut i_var3: usize = 0;
                loop {
                    *p30.add(i_var3) = *p80.add(i_var3);
                    i_var1 = i_var1.wrapping_sub(1);
                    *p31.add(i_var3) = *p81.add(i_var3);
                    i_var3 = i_var3.wrapping_add(2);
                    if i_var1 == 0 {
                        break;
                    }
                }
            } else {
                let k: u32 = if DAT_0052552c == 0x112 { 0x1e } else { 0x2c };
                let mut i_var1: i32 = 0;
                let mut i_var3: usize = 0;
                loop {
                    let u_var5 = i_var1 as u16;
                    let u_var4 = FUN_00418680(k, u_var5);
                    *p30.add(i_var3) = u_var4 as i8;
                    i_var1 = i_var1.wrapping_add(0x1bb);
                    let u_var4 = FUN_004187e0(k, u_var5);
                    *p31.add(i_var3) = u_var4 as i8;
                    i_var3 = i_var3.wrapping_add(2);
                    if i_var1 >= 0x4000 {
                        break;
                    }
                }
            }
            DAT_00527f7c = DAT_0052552c;
        }
        let sh = DAT_0070acc4.wrapping_sub(5) & 0x1f;
        let i_var3 = if param_1 < 0 {
            param_1.wrapping_neg()
        } else {
            param_1
        };
        let mut u_var4: u32 = i_var3.wrapping_shl(sh) as u32;
        if param_3 != 0 {
            u_var4 = idiv(u_var4 as i32, param_3) as u32;
        }
        if 0x7fff < (u_var4 as i32) {
            u_var4 = 0x7fff;
        }
        let i_var3 = if param_2 < 0 {
            param_2.wrapping_neg()
        } else {
            param_2
        };
        let mut u_var2: u32 = i_var3.wrapping_shl(sh) as u32;
        if param_3 != 0 {
            u_var2 = idiv(u_var2 as i32, param_3) as u32;
        }
        if 0x7fff < (u_var2 as i32) {
            u_var2 = 0x7fff;
        }
        DAT_00655a8c = u_var4 as u16;
        DAT_00655a88 = u_var2 as u16;
        FUN_00419000(u_var4, u_var2, &raw mut u_stack_2, &raw mut local_4);
        let t = (0x4000i32.wrapping_sub(u_stack_2 as i32)) as u16 as i32;
        let u_var5 = ((t / 0x1bb) << 1) as u16 as usize;
        let a = *p30.add(u_var5) as u8;
        if (a as i32) < (DAT_00655a8c as i16 as i32) {
            DAT_00655a8c = a as u16;
        }
        if param_1 < 0 {
            DAT_00655a8c = DAT_00655a8c.wrapping_neg();
        }
        let b = *p31.add(u_var5) as u8;
        if (b as i32) < (DAT_00655a88 as i16 as i32) {
            DAT_00655a88 = b as u16;
        }
        if param_2 < 0 {
            DAT_00655a88 = DAT_00655a88.wrapping_neg();
        }
    }
}
