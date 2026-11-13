// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009e9670: i32;
    static mut DAT_009e9a67: i32;
    static mut DAT_009e9a6b: i32;
    static mut DAT_009e9a6d: i32;
    static mut DAT_009e9a6f: i32;
    static mut DAT_009e9a73: i32;
    static mut DAT_009e9a75: i32;
    static mut DAT_009e9a7a: i32;
    static mut DAT_009e9a7c: i32;
    static mut DAT_009e9a7e: i32;
    static mut DAT_009e9a80: i32;
    static mut DAT_009e9a84: i32;
    static mut DAT_009e9a8a: i32;
    static mut DAT_009e9a8c: i32;
    static mut DAT_009e9b04: i32;
    static mut DAT_009e9b7c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00427720(param_1: u32, param_2: i32);
}

#[inline(always)]
fn at(base: *mut i32, off: i32) -> *mut u8 {
    base.cast::<u8>().wrapping_byte_offset(off as isize)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00481fb0(param_1: i32) {
    let i_var2 = param_1.wrapping_mul(0x5bd);
    // SAFETY: the globals are the game's own memory, accessed at the
    // original's byte offsets; the callee is the game's function.
    unsafe {
        if at(&raw mut DAT_009e9a84, i_var2)
            .cast::<i16>()
            .read_unaligned()
            != 0
        {
            let u_var5: u32 = if at(&raw mut DAT_009e9a8a, i_var2)
                .cast::<i16>()
                .read_unaligned()
                == 0
            {
                0x12
            } else {
                0x14
            };
            FUN_00427720(u_var5, param_1);
            let mut u_var3: u16 = 0;
            loop {
                let u_var4 = u_var3.wrapping_add(1);
                let v = at(&raw mut DAT_009e9a6f, i_var2)
                    .cast::<u16>()
                    .read_unaligned();
                at(
                    &raw mut DAT_009e9a8c,
                    i_var2.wrapping_add((u_var3 as i32).wrapping_mul(2)),
                )
                .cast::<u16>()
                .write_unaligned(v);
                let i_var1 = i_var2.wrapping_add((u_var3 as i32).wrapping_mul(2));
                let v = at(&raw mut DAT_009e9a6b, i_var2)
                    .cast::<u16>()
                    .read_unaligned();
                at(&raw mut DAT_009e9b04, i_var1)
                    .cast::<u16>()
                    .write_unaligned(v);
                let v = at(&raw mut DAT_009e9a6d, i_var2)
                    .cast::<u16>()
                    .read_unaligned();
                at(&raw mut DAT_009e9b7c, i_var1)
                    .cast::<u16>()
                    .write_unaligned(v);
                u_var3 = u_var4;
                if u_var4 >= 0x3c {
                    break;
                }
            }
            return;
        }
        at(&raw mut DAT_009e9a80, i_var2)
            .cast::<u16>()
            .write_unaligned(0);
        let a = at(&raw mut DAT_009e9a67, i_var2)
            .cast::<u16>()
            .read_unaligned() as u32;
        let b = at(&raw mut DAT_009e9670, i_var2)
            .cast::<u32>()
            .read_unaligned();
        if a == b {
            let v = at(&raw mut DAT_009e9a7c, i_var2)
                .cast::<u16>()
                .read_unaligned();
            at(&raw mut DAT_009e9a73, i_var2)
                .cast::<u16>()
                .write_unaligned(v);
            let v = at(&raw mut DAT_009e9a7e, i_var2)
                .cast::<u16>()
                .read_unaligned();
            at(&raw mut DAT_009e9a75, i_var2)
                .cast::<u16>()
                .write_unaligned(v);
            let c = at(&raw mut DAT_009e9a7a, i_var2).read();
            FUN_00427720(c as u32, param_1);
            return;
        }
        at(&raw mut DAT_009e9a73, i_var2)
            .cast::<u16>()
            .write_unaligned(0);
        at(&raw mut DAT_009e9a75, i_var2)
            .cast::<u16>()
            .write_unaligned(0);
        FUN_00427720(0x12, param_1);
        at(&raw mut DAT_009e9a84, i_var2)
            .cast::<u16>()
            .write_unaligned(1);
    }
}
