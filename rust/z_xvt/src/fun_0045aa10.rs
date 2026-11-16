// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071b743: [i8; 1];
    static mut DAT_0071ecf4: i32;
    static mut DAT_0052ae54: i32;
    static mut DAT_00732e8f: i8;
    static mut DAT_0052ae18: i32;
    static mut DAT_006ffbe4: i16;
    static mut DAT_006da9e2: i16;
    static mut DAT_0071f714: i32;
    static mut DAT_00732a38: i32;
    static mut DAT_006db7a4: i32;
    static mut DAT_006d9d94: i8;
    static mut DAT_007092c1: i8;
    static mut DAT_00703ac0: i16;
    static mut DAT_00703c88: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_0052686c: [u8; 1];
    static mut DAT_006db718: u16;
    static mut DAT_00732e54: i32;
    static mut DAT_00732910: i32;
    static mut DAT_006db7b4: i32;
    static mut DAT_006db7ac: i32;
    static mut DAT_006db7b0: i32;
    static mut DAT_006db59c: i32;
    static mut DAT_0072f44c: i32;
    static mut DAT_0072f450: i32;
    static mut DAT_0072f448: i32;
    static mut DAT_0071e536: i8;
    static mut DAT_00525730: i32;
    static mut DAT_005279c8: i32;
    static mut DAT_006da534: i16;
    static mut DAT_0071b724: i16;
    static mut DAT_006db5a8: i32;
    static mut DAT_00525548: i8;
    static mut DAT_00525508: i8;
    static mut DAT_0071bb1b: [u8; 1];
    static mut DAT_0071bb1f: [u8; 1];
    static mut DAT_0071bb23: [u8; 1];
    static mut DAT_0071bb27: [u8; 1];
    static mut DAT_0071bb44: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004337a0();
    fn FUN_0045f800();
    fn FUN_004a84a0(a: i32);
    fn FUN_004a6a80();
    fn FUN_004b2ae0();
    fn FUN_004a6f90();
    fn FUN_004b67f9();
    fn FUN_00465970(a: i32);
    fn FUN_0044ae00();
    fn FUN_00464b00();
    fn FUN_00433e70();
    fn FUN_0045b880(a: u32, b: u32) -> u8;
    fn FUN_00433e00(a: u32, b: i32);
    fn FUN_0045b990(a: i32, b: i32, c: i32, d: u32) -> u32;
    fn FUN_004342f0();
    fn FUN_0040ca00(a: i32, b: i32, c: i32) -> i32;
    fn FUN_0040ca30(a: i32, b: i32, c: i32) -> i32;
    fn FUN_0040ca60(a: i32, b: i32, c: i32) -> i32;
    fn FUN_004832a0(a: i16, b: i16, c: i16, d: i32, e: i32);
    fn FUN_004010f0(a: u16);
    fn FUN_0045b420(a: i32);
    fn FUN_0049c2b0(a: u32);
    fn FUN_00486310(a: i32);
    fn FUN_0049c590(a: u16);
    fn FUN_0047f0f0(a: i32);
    fn FUN_00466020();
    fn FUN_0047f3d0(a: i32);
    fn FUN_00442410();
    fn FUN_0048f530() -> i32;
    fn FUN_00465c40();
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_004a42b0(a: u32, b: u32);
    fn FUN_00412e60(a: *mut core::ffi::c_void, b: u16);
}

unsafe fn compute_vec(rec: i32) {
    // SAFETY: rec is the address of a game record; globals are the game's own.
    unsafe {
        let r = rec as usize as *const u8;
        let i18 = r.byte_add(4).cast::<i32>().read_unaligned();
        let i19 = r.byte_add(8).cast::<i32>().read_unaligned();
        let i20 = r.byte_add(0xc).cast::<i32>().read_unaligned();
        let off = (DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize;
        let i21 = (&raw mut DAT_0071bb1b)
            .cast::<u8>()
            .add(off)
            .cast::<i32>()
            .read_unaligned();
        let i22 = (&raw mut DAT_0071bb1f)
            .cast::<u8>()
            .add(off)
            .cast::<i32>()
            .read_unaligned();
        let i23 = (&raw mut DAT_0071bb23)
            .cast::<u8>()
            .add(off)
            .cast::<i32>()
            .read_unaligned();
        DAT_0072f44c = i18.wrapping_sub(i21);
        DAT_0072f450 = i19.wrapping_sub(i22);
        DAT_0072f448 = i20.wrapping_sub(i23);
        DAT_006db7ac = FUN_0040ca00(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
        DAT_006db7b0 = FUN_0040ca30(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
        DAT_006db7b4 = FUN_0040ca60(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
    }
}

unsafe fn call_832a0(rec: i32, last: i32) {
    // SAFETY: rec is the address of a game record.
    unsafe {
        let r = rec as usize as *const u8;
        let s24 = r.byte_add(0x14).cast::<i16>().read_unaligned();
        let s25 = r.byte_add(0x12).cast::<i16>().read_unaligned();
        let s26 = r.byte_add(0x10).cast::<i16>().read_unaligned();
        FUN_004832a0(s24, s25, s26, 0, last);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0045aa10() {
    // SAFETY: all globals and callees are the game's own; pointers are
    // game records read at the original's offsets.
    unsafe {
        let mut unaff_edi: u32 = 0;
        let rec_off = (DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize;
        if *(&raw mut DAT_0071b743).cast::<i8>().add(rec_off) != 0 {
            DAT_0052ae54 = 0;
            DAT_00732e8f = 0;
            FUN_004337a0();
            if DAT_0052ae18 != 0 {
                FUN_0045f800();
            }
            FUN_004a84a0(DAT_0071ecf4);
            FUN_004a6a80();
            let (c, d): (u32, u32);
            core::arch::asm!("", out("ecx") c, out("edx") d, options(nomem, nostack, preserves_flags));
            FUN_004a42b0(c, d);
            FUN_004b2ae0();
            FUN_004a6f90();
            DAT_0052ae54 = 1;
            return;
        }
        DAT_0052ae54 = 0;
        DAT_00732e8f = 0;
        let mut i_var7: i32 = DAT_0071ecf4;
        if DAT_006ffbe4 != 0 {
            i_var7 = 0;
            if DAT_0052ae18 != 0 {
                FUN_004b67f9();
                let c: i32;
                core::arch::asm!("", out("ecx") c, options(nomem, nostack, preserves_flags));
                i_var7 = c;
            }
            DAT_006ffbe4 = 0;
        }
        FUN_00465970(i_var7);
        DAT_006da9e2 = 0;
        if DAT_0052ae18 != 0 {
            DAT_0071f714 = 0x3000;
            FUN_0044ae00();
            FUN_004a6a80();
            FUN_00464b00();
            FUN_004a6f90();
        }
        let mut s_var8: i16 = 0;
        FUN_00433e70();
        if 0 < DAT_00732a38 {
            'outer: loop {
                'next1: {
                    if s_var8 as i32 == DAT_006db7a4 && (DAT_006d9d94 == 0 || DAT_007092c1 != 0) {
                        s_var8 = DAT_00703ac0;
                        if DAT_00703ac0 as i32 == DAT_00732a38 {
                            break 'outer;
                        }
                    }
                    unaff_edi = s_var8 as i32 as u32;
                    let off = (DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize;
                    let u_var13 = (&raw mut DAT_0071bb27)
                        .cast::<u8>()
                        .add(off)
                        .cast::<u16>()
                        .read_unaligned();
                    let s_var14 = (&raw mut DAT_0071bb44)
                        .cast::<u8>()
                        .add(off)
                        .cast::<i16>()
                        .read_unaligned();
                    if u_var13 as u32 != unaff_edi || s_var14 != 0 || DAT_00732910 != 0 {
                        let p = unaff_edi
                            .wrapping_mul(0x23)
                            .wrapping_add(DAT_006d4988 as u32)
                            as usize as *const u8;
                        if *p.byte_add(3) != 0 {
                            DAT_006db718 = (&raw mut DAT_0052686c)
                                .cast::<u8>()
                                .add(*p.byte_add(3) as usize * 0x16)
                                .cast::<u16>()
                                .read_unaligned();
                            match *p.byte_add(2) {
                                0 | 1 | 2 | 3 | 4 | 5 | 0xe => {
                                    let i11 = p.byte_add(0x1f).cast::<i32>().read_unaligned();
                                    let i12 = (i11 as usize as *const u8)
                                        .byte_add(0xa7)
                                        .cast::<i32>()
                                        .read_unaligned();
                                    DAT_00732e54 = i12;
                                    i_var7 = u32::from(FUN_0045b880(unaff_edi, DAT_006db718 as u32))
                                        as i32;
                                }
                                6 | 7 | 0xb | 0xd => {
                                    i_var7 = u32::from(FUN_0045b880(unaff_edi, DAT_006db718 as u32))
                                        as i32;
                                }
                                _ => break 'next1,
                            }
                            if i_var7 != 0 {
                                FUN_00433e00(unaff_edi, DAT_006db7b4);
                            }
                        }
                    }
                }
                s_var8 = s_var8.wrapping_add(1);
                if !((s_var8 as i32) < DAT_00732a38) {
                    break 'outer;
                }
            }
        }
        s_var8 = DAT_00732a38 as i16;
        if (s_var8 as i32) < (DAT_00703c88 as u32).wrapping_add(DAT_00732a38 as u32) as i32 {
            loop {
                unaff_edi = s_var8 as i32 as u32;
                let p = unaff_edi
                    .wrapping_mul(0x23)
                    .wrapping_add(DAT_006d4988 as u32) as usize
                    as *const u8;
                if *p.byte_add(3) != 0 {
                    DAT_006db718 = (&raw mut DAT_0052686c)
                        .cast::<u8>()
                        .add(*p.byte_add(3) as usize * 0x16)
                        .cast::<u16>()
                        .read_unaligned();
                    let t = *p.byte_add(2);
                    if 7 < t && t < 0xc {
                        let a = p.byte_add(4).cast::<i32>().read_unaligned();
                        let b = p.byte_add(8).cast::<i32>().read_unaligned();
                        let c = p.byte_add(0xc).cast::<i32>().read_unaligned();
                        let u_var6 = FUN_0045b990(a, b, c, DAT_006db718 as u32);
                        if u_var6 as i16 != 0 {
                            FUN_00433e00(unaff_edi, DAT_006db7b4);
                        }
                    }
                }
                s_var8 = s_var8.wrapping_add(1);
                if !((s_var8 as i32)
                    < (DAT_00703c88 as u32).wrapping_add(DAT_00732a38 as u32) as i32)
                {
                    break;
                }
            }
        }
        FUN_004342f0();
        i_var7 = DAT_006db59c;
        while i_var7 != 0 {
            let node = i_var7 as usize as *const u8;
            let u_var3 = node.byte_add(4).cast::<u16>().read_unaligned();
            unaff_edi = (unaff_edi & 0xffff0000) | u_var3 as u32;
            let mut i_var9: i32 = u_var3 as i16 as i32;
            let i_var4: i32 = i_var9.wrapping_mul(0x23);
            let u_var10: u32 = (DAT_006d4988 as u32).wrapping_add(i_var4 as u32);
            let b_var2 = *(u_var10 as usize as *const u8).byte_add(2);
            if i_var9 < DAT_00732a38 {
                match b_var2 {
                    0 | 1 | 2 | 3 | 4 | 5 | 0xe => {
                        let i11 = (u_var10 as usize as *const u8)
                            .byte_add(0x1f)
                            .cast::<i32>()
                            .read_unaligned();
                        let i12 = (i11 as usize as *const u8)
                            .byte_add(0xa7)
                            .cast::<i32>()
                            .read_unaligned();
                        DAT_00732e54 = i12;
                        i_var9 = (u_var3 as u32)
                            .wrapping_mul(0x23)
                            .wrapping_add(DAT_006d4988 as u32)
                            as i32;
                        compute_vec(i_var9);
                        if b_var2 == 0xe {
                            DAT_0071e536 = 0;
                        }
                        i_var9 = (DAT_006d4988 as u32).wrapping_add(i_var4 as u32) as i32;
                        call_832a0(i_var9, i_var9);
                        if b_var2 == 0xe {
                            FUN_004010f0(u_var3);
                        } else {
                            FUN_0045b420((DAT_006d4988 as u32).wrapping_add(i_var4 as u32) as i32);
                            FUN_0049c2b0(unaff_edi);
                            FUN_00486310((DAT_006d4988 as u32).wrapping_add(i_var4 as u32) as i32);
                            DAT_00525730 = 0;
                        }
                        DAT_0071e536 = 1;
                    }
                    6 | 7 => {
                        i_var9 = (u_var3 as u32)
                            .wrapping_mul(0x23)
                            .wrapping_add(DAT_006d4988 as u32)
                            as i32;
                        compute_vec(i_var9);
                        i_var9 = (DAT_006d4988 as u32).wrapping_add(i_var4 as u32) as i32;
                        call_832a0(i_var9, i_var9);
                        FUN_0049c590(u_var3);
                    }
                    0xb | 0xd => {
                        unaff_edi = (u_var3 as u32)
                            .wrapping_mul(0x23)
                            .wrapping_add(DAT_006d4988 as u32);
                        compute_vec(unaff_edi as i32);
                        let i_var1 = (DAT_006d4988 as u32).wrapping_add(i_var4 as u32) as i32;
                        call_832a0(i_var1, i_var1);
                        FUN_0047f0f0(i_var9);
                    }
                    _ => {}
                }
            } else if 7 < b_var2 && b_var2 < 0xc {
                i_var9 = (u_var3 as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(DAT_006d4988 as u32) as i32;
                compute_vec(i_var9);
                call_832a0((DAT_006d4988 as u32).wrapping_add(i_var4 as u32) as i32, 0);
                FUN_0045b420((DAT_006d4988 as u32).wrapping_add(i_var4 as u32) as i32);
                let this_: *mut core::ffi::c_void;
                core::arch::asm!("", out("ecx") this_, options(nomem, nostack, preserves_flags));
                FUN_00412e60(this_, u_var3);
                DAT_00525730 = 0;
            }
            i_var7 = node.byte_add(8).cast::<i32>().read_unaligned();
        }
        if DAT_0052ae18 == 0 {
            DAT_0071f714 = 0x3000;
            FUN_0044ae00();
            FUN_004a6a80();
            FUN_00464b00();
            FUN_004a6f90();
        }
        DAT_005279c8 = 1;
        FUN_00466020();
        DAT_005279c8 = 0;
        if DAT_0052ae18 == 0 {
            FUN_0047f3d0(1);
            FUN_00442410();
        }
        DAT_006da534 = 0;
        DAT_0071b724 = 0;
        i_var7 = FUN_0048f530();
        DAT_006db5a8 = DAT_006db5a8.wrapping_add(i_var7);
        DAT_006da534 = DAT_006db5a8 as i16;
        FUN_00465c40();
        if DAT_0052ae18 != 0 {
            FUN_0045f800();
        }
        FUN_004a84a0(DAT_0071ecf4);
        DAT_00525548 = 0;
        i_var7 = FUN_0048f530();
        DAT_006db5a8 = DAT_006db5a8.wrapping_add(i_var7);
        DAT_00525548 = DAT_00525508;
        DAT_006da534 = (DAT_006db5a8 as i16).wrapping_sub(DAT_006da534);
        FUN_004a6a80();
        let (c, d): (u32, u32);
        core::arch::asm!("", out("ecx") c, out("edx") d, options(nomem, nostack, preserves_flags));
        FUN_004a42b0(c, d);
        FUN_004b2ae0();
        FUN_004a6f90();
        DAT_0052ae54 = 1;
    }
}
