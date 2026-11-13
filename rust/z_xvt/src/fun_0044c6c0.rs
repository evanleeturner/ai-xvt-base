// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d9d60: u16;
    static mut DAT_006d9d78: u32;
    static mut DAT_00703ca0: i8;
    static mut DAT_006da3e0: i8;
    static mut DAT_00524798: i8;
    static mut DAT_0070ad92: i8;
    static mut DAT_00525ce0: i8;
    static mut DAT_006d9d7a: u32;
    static mut DAT_00525028: i8;
    static mut DAT_00525060: i8;
    static mut DAT_0052504c: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0044bec0(param_1: u32) -> i16;
    fn FUN_0044c1c0(param_1: u32) -> i16;
    fn FUN_0044b810(param_1: u32) -> i16;
}

unsafe fn cmp_n(mut p: *const i8, mut q: *const i8, mut n: i32) -> bool {
    let mut flag;
    loop {
        if n == 0 {
            // Not reached with the nonzero counts used here.
            return false;
        }
        n -= 1;
        // SAFETY: the caller passes pointers to the game's own strings.
        flag = unsafe { *p == *q };
        p = p.wrapping_add(1);
        q = q.wrapping_add(1);
        if !flag {
            break;
        }
        if n == 0 {
            break;
        }
    }
    flag
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044c6c0(param_1: u16) -> u8 {
    // SAFETY: the globals and callees are the game's own memory; the
    // pointers computed index its tables as the original does.
    unsafe {
        let u_var3: u32 =
            ((((DAT_006d9d60 as u32).wrapping_mul(0x159)) >> 16) << 16) | (param_1 as u32);
        DAT_006d9d78 = ((param_1 as u32).wrapping_mul(0x52) == 0) as u32;

        let idx: u32 = (DAT_006d9d60 as u32)
            .wrapping_mul(0x566)
            .wrapping_add((param_1 as u32).wrapping_mul(0x52));
        let b1: u8 = (&raw mut DAT_0070ad92)
            .cast::<u8>()
            .byte_add(idx as usize)
            .read();
        let b2: u8 = (&raw mut DAT_00524798)
            .cast::<u8>()
            .byte_add(b1 as usize)
            .read();
        let b3: u8 = (&raw mut DAT_006da3e0)
            .cast::<u8>()
            .byte_add(b2 as usize)
            .read();
        let pc5: *const i8 = (&raw mut DAT_00703ca0)
            .cast::<u8>()
            .byte_add((b3 as u32).wrapping_mul(0x55) as usize)
            .cast::<i8>();

        let disable: *const i8 = &raw mut DAT_00525028;
        let capfree: *const i8 = &raw mut DAT_00525060;
        let escorters: *const i8 = &raw mut DAT_0052504c;
        let other: *const i8 = &raw mut DAT_00525ce0;

        // First loop: stores the flag each iteration.
        let mut iv: i32 = 0xf;
        let mut p4 = pc5;
        let mut p6 = disable;
        loop {
            if iv == 0 {
                break;
            }
            iv -= 1;
            DAT_006d9d78 = (*p4 == *p6) as u32;
            p4 = p4.wrapping_add(1);
            p6 = p6.wrapping_add(1);
            if DAT_006d9d78 as u8 == 0 {
                break;
            }
        }
        DAT_006d9d7a = 7;

        let s_var1: i16;
        let b7 = cmp_n(pc5, capfree, 0xf);
        if !b7 {
            let b7 = cmp_n(pc5, disable, 0xf);
            if !b7 {
                let b7 = cmp_n(pc5, other, 0xd);
                if !b7 {
                    let b7 = cmp_n(pc5, escorters, 0x14);
                    if b7 {
                        s_var1 = FUN_0044bec0(u_var3);
                    } else {
                        s_var1 = FUN_0044c1c0(u_var3);
                    }
                    return (s_var1 != -1) as u8;
                }
            }
        }
        s_var1 = FUN_0044b810(u_var3);
        (s_var1 != -1) as u8
    }
}
