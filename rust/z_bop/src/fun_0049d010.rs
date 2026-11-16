// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00521f18: [u8; 0];
    static mut DAT_00521f19: [u8; 0];
    static mut DAT_00521f2f: [u8; 0];
    static mut DAT_00521f30: [u8; 0];
    static mut DAT_00521f31: [u8; 0];
    static mut DAT_00521f24: [i16; 0];
    static mut DAT_009a7ed0: [i16; 0];
    static mut DAT_00527620: [i8; 0];
    static mut DAT_00527710: [i8; 0];
    static mut DAT_0052770c: [i8; 0];
    static mut DAT_00527704: [i8; 0];
    static mut DAT_00523434: [i8; 0];
    static mut s_ivfiles__00527714: [i8; 0];
    static mut DAT_005233ec: i32;
    static mut DAT_005233d8: i32;
    static mut DAT_00a07bd8: *mut core::ffi::c_void;
    static mut DAT_009d6940: [u8; 0];
    static mut DAT_00999488: i32;
    static mut DAT_0099947e: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049cba0();
    fn FUN_0049e720(a: *mut i8, b: *mut i8, c: i32, d: i32) -> i32;
    fn _fgets(a: *mut i8, b: i32, c: *mut core::ffi::c_void) -> *mut i8;
    fn FUN_00411e00(a: *mut i8) -> i16;
    fn FUN_004ac720(a: u32) -> *mut i32;
    fn FUN_0049ea10(a: *mut core::ffi::c_void, b: i32, c: i32, d: *mut core::ffi::c_void) -> i32;
    fn FUN_004ac5f0(a: i32, b: i32) -> i16;
    fn FUN_0049eb60(a: i32);
    fn FUN_0049e9c0(a: i32);
    fn FUN_0040e6c0(a: *mut i32);
    fn FUN_0040e7f0(a: *mut i32);
    fn FUN_0049d860(a: u8, b: u8);
    fn FUN_004ac740();
    fn FUN_00485cd0();
    fn FUN_0049cc90();
}

unsafe fn cat(dst: *mut i8, src: *const i8) {
    // SAFETY: both are NUL-terminated game strings; dst has room, as in the original.
    unsafe {
        let mut n: usize = 0;
        while *src.add(n) != 0 {
            n += 1;
        }
        n += 1;
        let mut d = dst;
        while *d != 0 {
            d = d.add(1);
        }
        let mut i = 0;
        while i < n {
            *d.add(i) = *src.add(i);
            i += 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0049d010() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are used exactly as the original does.
    unsafe {
        let p18 = (&raw mut DAT_00521f18).cast::<u8>();
        let p19 = (&raw mut DAT_00521f19).cast::<u8>();
        let p2f = (&raw mut DAT_00521f2f).cast::<u8>();
        let p30 = (&raw mut DAT_00521f30).cast::<u8>();
        let p31 = (&raw mut DAT_00521f31).cast::<u8>();
        let p24 = (&raw mut DAT_00521f24).cast::<i16>();
        let p7e = (&raw mut DAT_009a7ed0).cast::<i16>();
        let p6940 = (&raw mut DAT_009d6940).cast::<u8>();

        let mut u_var6: u32 = 0;
        let mut u_var12: u16 = 0;
        loop {
            let u_var8 = u_var12.wrapping_add(1);
            u_var6 = u_var12 as u32;
            *p7e.add(u_var6 as usize) = 0;
            *p24.add((u_var6.wrapping_mul(0xd)) as usize) = 0;
            u_var12 = u_var8;
            if !(u_var8 < 0xc9) {
                break;
            }
        }
        FUN_0049cba0();
        DAT_00999488 = 0;
        (&raw mut DAT_0099947e).cast::<u32>().write_unaligned(0);
        let mut u_stack_14e: u16 = 0;
        let mut b_stack_151: u8 = 0;
        let mut ac13c: [i8; 60] = [0; 60];
        let mut ac100: [i8; 256] = [0; 256];
        let mut i_stack_148: i32 = 0;
        let mut i_stack_140: i32 = 0;
        loop {
            let iv = (&raw mut s_ivfiles__00527714).cast::<i8>();
            let mut k = 0;
            while k < 9 {
                ac13c[k] = *iv.add(k);
                k += 1;
            }
            let u_stack_144 = u_stack_14e as u32;
            cat(
                ac13c.as_mut_ptr(),
                (&raw mut DAT_00527620)
                    .cast::<i8>()
                    .add((u_stack_144.wrapping_mul(9)) as usize),
            );
            let pc: *mut i8 = if DAT_005233ec == 0x101 {
                (&raw mut DAT_00527710).cast::<i8>()
            } else if DAT_005233ec == 0x112 {
                (&raw mut DAT_00527710).cast::<i8>()
            } else {
                (&raw mut DAT_0052770c).cast::<i8>()
            };
            cat(ac13c.as_mut_ptr(), pc);
            cat(ac13c.as_mut_ptr(), (&raw mut DAT_00527704).cast::<i8>());

            FUN_0049e720(
                ac13c.as_mut_ptr(),
                (&raw mut DAT_00523434).cast::<i8>(),
                1,
                0,
            );
            let file = DAT_00a07bd8;
            let mut u_stack_150: u16 = 0;
            let mut pc15 = _fgets(ac100.as_mut_ptr(), 0x100, DAT_00a07bd8);
            while !pc15.is_null() {
                let mut c_var1 = ac100[0];
                pc15 = ac100.as_mut_ptr();
                while c_var1 != 10 && *pc15 != 13 {
                    c_var1 = *pc15.add(1);
                    pc15 = pc15.add(1);
                }
                *pc15 = 0;
                if ac100[0] != 0 {
                    let mut u_var9: u32 = 0;
                    let mut b_var3 = false;
                    u_stack_150 = u_stack_150.wrapping_add(1);
                    loop {
                        let iv10 = u_var9.wrapping_mul(0x1a) as i32 as usize;
                        if (*p18.add(iv10) & 2) != 0
                            && (*p30.add(iv10) as u32) == u_stack_144
                            && (*p31.add(iv10) as u32).wrapping_sub(u_stack_150 as u32)
                                == 0xffff_ffff
                        {
                            let b_var2 = *p19.add(iv10);
                            if (b_var2 & 0x18) != 0 && ((b_var2 & 0x40) == 0 || *p6940.add(1) != 0)
                            {
                                b_var3 = true;
                                b_stack_151 = b_var2;
                            }
                        }
                        u_var12 = (u_var9 as u16).wrapping_add(1);
                        u_var9 = u_var12 as u32;
                        if !(u_var12 < 0xc9) {
                            break;
                        }
                    }
                    if b_var3 {
                        if (b_stack_151 & 1) == 0 {
                            if (b_stack_151 & 2) != 0 {
                                FUN_0049e720(
                                    ac100.as_mut_ptr(),
                                    (&raw mut DAT_00523434).cast::<i8>(),
                                    1,
                                    0,
                                );
                                let pf = DAT_00a07bd8;
                                FUN_0049ea10((&raw mut i_stack_148).cast(), 4, 1, DAT_00a07bd8);
                                FUN_0049ea10((&raw mut i_stack_140).cast(), 4, 1, pf);
                                let s_var5 = FUN_004ac5f0(
                                    (i_stack_140 as u32)
                                        .wrapping_mul(DAT_005233d8 as u32)
                                        .wrapping_add(i_stack_148 as u32)
                                        as i32,
                                    0,
                                );
                                u_var6 = (s_var5 as u16 as u32) | (u_var6 & 0xffff_0000);
                                if s_var5 == 0 {
                                    FUN_0049eb60(0);
                                }
                                let pi7 = FUN_004ac720(u_var6);
                                *pi7 = i_stack_148;
                                *pi7.add(1) = i_stack_140;
                                FUN_0049ea10(
                                    pi7.add(2).cast(),
                                    (i_stack_148 as u32).wrapping_sub(8) as i32,
                                    1,
                                    pf,
                                );
                                DAT_00a07bd8 = pf;
                                FUN_0049e9c0(0);
                                if DAT_005233d8 == 2 {
                                    FUN_0040e6c0(pi7);
                                } else {
                                    FUN_0040e7f0(pi7);
                                }
                            }
                        } else {
                            let s_var5 = FUN_00411e00(ac100.as_mut_ptr());
                            u_var6 = (s_var5 as u16 as u32) | (u_var6 & 0xffff_0000);
                            FUN_004ac720(u_var6);
                        }
                        u_var9 = 0;
                        loop {
                            let iv10 = u_var9.wrapping_mul(0x1a) as i32 as usize;
                            if (*p18.add(iv10) & 2) != 0
                                && (*p30.add(iv10) as u32) == u_stack_144
                                && (*p31.add(iv10) as u32).wrapping_sub(u_stack_150 as u32)
                                    == 0xffff_ffff
                                && (*p19.add(iv10) & 0x18) != 0
                                && ((*p19.add(iv10) & 0x40) == 0 || *p6940.add(1) != 0)
                            {
                                *p24.add(u_var9.wrapping_mul(0xd) as usize) = u_var6 as i16;
                                *p7e.add(u_var9 as usize) = u_var6 as i16;
                                if (b_stack_151 & 1) != 0 {
                                    FUN_0049d860(*p2f.add(iv10), u_var9 as u8);
                                }
                            }
                            u_var12 = (u_var9 as u16).wrapping_add(1);
                            u_var9 = u_var12 as u32;
                            if !(u_var12 < 0xc9) {
                                break;
                            }
                        }
                        FUN_004ac740();
                    }
                }
                pc15 = _fgets(ac100.as_mut_ptr(), 0x100, file);
            }
            DAT_00a07bd8 = file;
            FUN_0049e9c0(0);
            u_stack_14e = u_stack_14e.wrapping_add(1);
            if 2 < u_stack_14e {
                FUN_00485cd0();
                FUN_0049cc90();
                return;
            }
        }
    }
}
