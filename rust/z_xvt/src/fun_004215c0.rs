// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00703abc: u16;
    static mut DAT_0073b374: u16;
    static mut DAT_006d4a6a: u16;
    static mut DAT_006d4a6c: u8;
    static mut DAT_0070ad42: u8;
    static mut DAT_0070ad43: u8;
    static mut DAT_0070ad44: u8;
    static mut DAT_0070ad57: u8;
    static mut DAT_0070ad58: u8;
    static mut DAT_0070ad59: u8;
    static mut DAT_0070b156: u8;
    static mut DAT_0070b182: u8;
    static mut DAT_0070b1ae: u8;
    static mut DAT_005247e8: u8;
    static mut DAT_0052687a: u8;
    static mut DAT_0052686b: u8;
    static mut DAT_007394b0: i32;
    static mut DAT_007394b4: i32;
    static mut DAT_007394b8: i32;
    static mut DAT_007394bc: u16;
    static mut DAT_007394be: u16;
    static mut DAT_007394c0: u16;
    static mut DAT_006d9d90: u16;
    static mut DAT_00732a38: i32;
    static mut DAT_00703c88: i32;
    static mut DAT_006d4988: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00421a80(a: u16, b: i32, c: i32);
    fn FUN_00478470() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004215c0(param_1: u16) {
    // SAFETY: all globals and callees are the game's own memory; offsets
    // mirror the original's table indexing.
    unsafe {
        let u_var6 = DAT_00703abc;
        let p6a = (&raw mut DAT_006d4a6a).cast::<u16>();
        let idx0 = DAT_0073b374 as usize;
        if p6a.add(idx0 * 0x93).read() != 0 {
            let i12 = idx0 * 0x566;
            let b_var1 = (&raw mut DAT_005247e8)
                .cast::<u8>()
                .add((&raw mut DAT_0070ad42).cast::<u8>().add(i12).read() as usize)
                .read();
            if ((&raw mut DAT_0052687a)
                .cast::<u8>()
                .add(b_var1 as usize * 0x16)
                .read()
                & 0x80)
                != 0
            {
                let c_var2 = (&raw mut DAT_0052686b)
                    .cast::<u8>()
                    .add(b_var1 as usize * 0x16)
                    .read() as i8;
                let p43 = (&raw mut DAT_0070ad43).cast::<u8>();
                let s3p = (&raw mut DAT_0070b156).cast::<u8>().add(i12).cast::<i16>();
                let s4p = (&raw mut DAT_0070b182).cast::<u8>().add(i12).cast::<i16>();
                let s5p = (&raw mut DAT_0070b1ae).cast::<u8>().add(i12).cast::<i16>();
                if c_var2 == 8 {
                    let b9 = (&raw mut DAT_0070ad44).cast::<u8>().add(i12).read() & 3;
                    let mut local_18: u32 = 0;
                    let mut local_1c: u32 = 0;
                    let mut local_20: u32 = 0;
                    let mut local_14: u32 = 0;
                    if b9 == 0 {
                        local_18 = 0x40;
                        local_1c = 0x40;
                    } else if b9 == 1 {
                        local_1c = 0x40;
                        local_14 = 0x40;
                    } else if b9 == 2 {
                        local_18 = 0x40;
                        local_20 = 0x40;
                    }
                    let i11 = (p43.add(i12).read() as u32).wrapping_sub(1);
                    let s3 = s3p.read_unaligned();
                    let s4 = s4p.read_unaligned();
                    let s5 = s5p.read_unaligned();
                    DAT_007394bc = (&raw mut DAT_0070ad57).cast::<u8>().add(i12).read() as u16;
                    let mut u_var7: u32 = 0;
                    DAT_007394be = (&raw mut DAT_0070ad58).cast::<u8>().add(i12).read() as u16;
                    let mut local_24: u32 = 0;
                    DAT_007394c0 = (&raw mut DAT_0070ad59).cast::<u8>().add(i12).read() as u16;
                    if p43.add(i12).read() != 0 {
                        loop {
                            let mut i12b: i32 = 0;
                            if p43.add(DAT_0073b374 as usize * 0x566).read() != 0 {
                                let mut i10: u32 = (s5 as i32 as u32)
                                    .wrapping_sub(i11.wrapping_mul(local_20) >> 1)
                                    .wrapping_sub(i11.wrapping_mul(local_14) >> 1)
                                    .wrapping_add(local_20.wrapping_mul(local_24));
                                let mut i13: u32 = (s3 as i32 as u32)
                                    .wrapping_sub(local_18.wrapping_mul(i11) >> 1);
                                loop {
                                    if (param_1 == 0xffff || param_1 as u32 == u_var7)
                                        && ((&raw mut DAT_006d4a6c)
                                            .cast::<u8>()
                                            .add(DAT_0073b374 as usize * 0x126)
                                            .cast::<u16>()
                                            .read_unaligned()
                                            < p6a.add(DAT_0073b374 as usize * 0x93).read())
                                    {
                                        DAT_007394b4 = local_24.wrapping_mul(local_1c).wrapping_sub(
                                            (s4 as i32 as u32)
                                                .wrapping_add(i11.wrapping_mul(local_1c) >> 1),
                                        )
                                            as i32;
                                        DAT_007394b0 = i13 as i32;
                                        DAT_007394b8 = i10 as i32;
                                        FUN_00421a80(DAT_0073b374, 8, b_var1 as i32);
                                    }
                                    u_var7 = u_var7.wrapping_add(1);
                                    i10 = i10.wrapping_add(local_14);
                                    i13 = i13.wrapping_add(local_18);
                                    i12b = i12b.wrapping_add(1);
                                    if !(i12b
                                        < p43.add(DAT_0073b374 as usize * 0x566).read() as i32)
                                    {
                                        break;
                                    }
                                }
                            }
                            local_24 = local_24.wrapping_add(1);
                            if !(local_24 < p43.add(DAT_0073b374 as usize * 0x566).read() as u32) {
                                break;
                            }
                        }
                        return;
                    }
                } else {
                    if c_var2 == 9 {
                        DAT_007394b0 = s3p.read_unaligned() as i32;
                        DAT_007394b8 = s5p.read_unaligned() as i32;
                        DAT_007394b4 = 0u32.wrapping_sub(s4p.read_unaligned() as i32 as u32) as i32;
                        DAT_007394bc = (&raw mut DAT_0070ad57).cast::<u8>().add(i12).read() as u16;
                        DAT_007394be = (&raw mut DAT_0070ad58).cast::<u8>().add(i12).read() as u16;
                        DAT_007394c0 = (&raw mut DAT_0070ad59).cast::<u8>().add(i12).read() as u16;
                        FUN_00421a80(DAT_0073b374, 9, b_var1 as i32);
                        return;
                    }
                    if c_var2 != 10 {
                        return;
                    }
                    let s3 = s3p.read_unaligned();
                    let mut local_24: u32 = 0;
                    let s4 = s4p.read_unaligned();
                    let s5 = s5p.read_unaligned();
                    DAT_00703abc = DAT_006d9d90;
                    while local_24 < p43.add(DAT_0073b374 as usize * 0x566).read() as u32 {
                        let mut i12c: i32;
                        let mut i11: i32;
                        let mut i13: i32;
                        loop {
                            let mut u = FUN_00478470();
                            i12c = ((u & 0x1ff).wrapping_sub(0x100)).wrapping_add(s3 as i32 as u32)
                                as i32;
                            u = FUN_00478470();
                            i11 = ((u & 0x1ff).wrapping_sub(0x100)).wrapping_sub(s4 as i32 as u32)
                                as i32;
                            u = FUN_00478470();
                            i13 = ((u & 0x1ff).wrapping_sub(0x100)).wrapping_add(s5 as i32 as u32)
                                as i32;
                            let mut i10 = DAT_00732a38;
                            if i10 < (DAT_00703c88 as u32).wrapping_add(DAT_00732a38 as u32) as i32
                            {
                                let mut pc = (DAT_00732a38 as u32)
                                    .wrapping_mul(0x23)
                                    .wrapping_add(3)
                                    .wrapping_add(DAT_006d4988 as u32)
                                    as *const i8;
                                loop {
                                    if pc.read() != 0 {
                                        let t1 = pc.byte_add(1).cast::<i32>().read_unaligned();
                                        if t1 == i12c {
                                            let t2 = pc.byte_add(5).cast::<i32>().read_unaligned();
                                            if t2 == i11 {
                                                let t3 =
                                                    pc.byte_add(9).cast::<i32>().read_unaligned();
                                                if t3 == i13 {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    pc = pc.byte_add(0x23);
                                    i10 = i10.wrapping_add(1);
                                    if !(i10
                                        < (DAT_00703c88 as u32).wrapping_add(DAT_00732a38 as u32)
                                            as i32)
                                    {
                                        break;
                                    }
                                }
                            }
                            if !(i10 < DAT_00703c88) {
                                break;
                            }
                        }
                        DAT_007394bc = 0;
                        DAT_007394be = 0;
                        DAT_007394c0 = 0;
                        DAT_007394b0 = i12c;
                        DAT_007394b4 = i11;
                        DAT_007394b8 = i13;
                        let u = FUN_00478470();
                        FUN_00421a80(DAT_0073b374, 10, ((u & 0xffff) % 6 + 100) as i32);
                        local_24 = local_24.wrapping_add(1);
                    }
                    DAT_006d9d90 = DAT_00703abc;
                }
            }
        }
        DAT_00703abc = u_var6;
    }
}
