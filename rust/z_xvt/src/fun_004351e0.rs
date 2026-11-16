// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: i32;
    static mut DAT_0070ad47: [u8; 1];
    static mut DAT_0071e538: *const i8;
    static mut DAT_005256f8: [i8; 1];
    static mut DAT_005256c8: [i8; 1];
    static mut DAT_005256e0: [i8; 1];
    static mut DAT_005251d8: [u8; 1];
    static mut DAT_00525288: [u8; 1];
    static mut DAT_00525248: [u8; 1];
    static mut DAT_005253c8: [u8; 1];
    static mut DAT_00525480: [u8; 1];
    static mut DAT_00525438: [u8; 1];
    static mut DAT_005252c8: [u8; 1];
    static mut DAT_00525380: [u8; 1];
    static mut DAT_00525338: [u8; 1];
    static mut DAT_006db7b8: i32;
    static mut DAT_006d9980: i16;
    static mut DAT_0070acc0: i16;
    static mut DAT_00732e8c: i16;
    static mut DAT_00732a34: i16;
    static mut DAT_0071ed08: i32;
    static mut DAT_0070a184: *const core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040ca90(a: u32, b: u32) -> i32;
    fn FUN_0040cad0(a: u32, b: u32) -> i32;
    fn FUN_004a6a80();
    fn FUN_00462d30(p: *mut u8, x: u32, y: u32, a: i32, b: i32);
    fn FUN_004a6f90();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004351e0(param_1: i32, param_2: u32, param_3: u32, param_4: u32) {
    // SAFETY: all globals, tables and the record pointer are the game's own
    // memory, read where the original reads; callees are the game's functions.
    unsafe {
        let rec = (param_1 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(DAT_006d4988 as u32) as *const u8;
        let mut local_8: u32 = rec.byte_add(3).read() as u32;
        let ptr1f: i32 = rec.byte_add(0x1f).cast::<i32>().read_unaligned();
        let u_var1: u8 = if ptr1f == 0 {
            let off = (rec.byte_add(0x16).read() as u32).wrapping_mul(0x566) as usize;
            (&raw mut DAT_0070ad47).cast::<u8>().add(off).read()
        } else {
            ((ptr1f as u32).wrapping_add(0x86) as *const u8).read()
        };
        let mut local_4: u32 = 0;
        let mut i_var4: i32 = match u_var1 {
            0 => 0,
            1 | 4 => 2,
            2 => 3,
            3 => {
                local_4 = 1;
                2
            }
            _ => 1,
        };
        let i_var2 = FUN_0040ca90(param_2, param_4);
        let i_var3 = FUN_0040cad0(param_3, param_4);
        let cur = DAT_0071e538;
        let mut u_var5: u32;
        let mut u_var6: u32;
        if cur == (&raw mut DAT_005256f8).cast::<i8>() as *const i8 {
            u_var6 = 0x13;
            if local_8 < 0x6a {
                u_var6 = (&raw mut DAT_005251d8)
                    .cast::<u8>()
                    .add(local_8 as usize)
                    .read() as u32;
            }
            local_8 = (&raw mut DAT_00525288)
                .cast::<u8>()
                .add(u_var6 as usize)
                .read() as u32;
            u_var5 = (&raw mut DAT_00525248)
                .cast::<u8>()
                .add(u_var6 as usize)
                .read() as u32;
        } else if cur == (&raw mut DAT_005256c8).cast::<i8>() as *const i8 {
            u_var6 = 0x13;
            if local_8 < 0x6a {
                u_var6 = (&raw mut DAT_005253c8)
                    .cast::<u8>()
                    .add(local_8 as usize)
                    .read() as u32;
            }
            local_8 = (&raw mut DAT_00525480)
                .cast::<u8>()
                .add(u_var6 as usize)
                .read() as u32;
            u_var5 = (&raw mut DAT_00525438)
                .cast::<u8>()
                .add(u_var6 as usize)
                .read() as u32;
        } else {
            u_var5 = local_4;
            u_var6 = local_4;
            if cur == (&raw mut DAT_005256e0).cast::<i8>() as *const i8 {
                u_var6 = 0x13;
                if local_8 < 0x6a {
                    u_var6 = (&raw mut DAT_005252c8)
                        .cast::<u8>()
                        .add(local_8 as usize)
                        .read() as u32;
                }
                local_8 = (&raw mut DAT_00525380)
                    .cast::<u8>()
                    .add(u_var6 as usize)
                    .read() as u32;
                u_var5 = (&raw mut DAT_00525338)
                    .cast::<u8>()
                    .add(u_var6 as usize)
                    .read() as u32;
            }
        }
        i_var4 = (DAT_006db7b8 as u32).wrapping_mul(i_var4 as u32) as i32;
        let u_var8: u16 = (i_var2 as i16 as i32)
            .wrapping_add(DAT_006d9980 as i32)
            .wrapping_sub((u_var5 / 2) as i16 as i32) as u16;
        let u_var7: u16 = (DAT_00732e8c as i32)
            .wrapping_add(i_var3 as i16 as i32)
            .wrapping_sub((local_8 / 2) as i16 as i32) as u16;
        if DAT_006d9980 <= u_var8 as i16
            && ((u_var5 as i16 as i32).wrapping_add(u_var8 as i32) as i16) < DAT_0070acc0
            && DAT_00732e8c <= u_var7 as i16
            && ((u_var7 as i32).wrapping_add(local_8 as i16 as i32) as i16) < DAT_00732a34
        {
            FUN_004a6a80();
            if local_4 == 0 {
                let f: unsafe extern "C" fn() = core::mem::transmute(DAT_0070a184);
                f();
            } else {
                let adj = (i_var4 as u32).wrapping_add(((i_var4 >> 0x1f) as u32) & 3);
                let idx = u_var6.wrapping_add(((adj as i32) >> 2) as u32);
                let addr = (DAT_0071ed08 as u32).wrapping_add(idx.wrapping_mul(4));
                let p = (addr as *const u32).read_unaligned() as *mut u8;
                FUN_00462d30(p, u_var8 as u32, u_var7 as u32, 0, 0);
            }
            FUN_004a6f90();
        }
    }
}
