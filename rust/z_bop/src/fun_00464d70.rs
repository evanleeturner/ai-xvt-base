// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005242c8: i32;
    static mut DAT_00a080f8: i32;
    static mut DAT_005242e8: u32;
    static mut DAT_00523418: u32;
    static mut DAT_00557350: i32;
    static mut DAT_00a07cc8: u32;
    static mut DAT_00557360: u32;
    static mut DAT_00557364: u32;
    static mut DAT_00557368: u8;
    static mut DAT_009d77d0: [u32; 8];
    static mut DAT_009a8db0: [i32; 0];
    static mut DAT_009ed670: [i32; 0];
    static mut DAT_009e9680: [i8; 0];
    static mut DAT_009ec468: [i8; 0];
    static mut DAT_009fd394: i32;
    static mut DAT_005242d0: i32;
    static mut DAT_005242d8: *mut core::ffi::c_void;
    static mut DAT_0052436c: [i8; 0];
    static mut DAT_00524390: [i8; 0];
    static mut s_serverlog_txt_00524398: [i8; 0];
    static mut s__2x_00524388: [i8; 0];
    static mut s__8x__2x__2x__2x__2x_00524370: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046d350(param_1: i32, param_2: *mut u32, param_3: u32);
    fn FID_conflict___wfopen(param_1: *mut i8, param_2: *mut i8) -> *mut core::ffi::c_void;
    fn _fprintf(param_1: *mut core::ffi::c_void, param_2: *mut i8, ...) -> i32;
    fn _fflush(param_1: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00464d70() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // the packet buffer and entry tables are accessed at the original's offsets.
    unsafe {
        DAT_005242c8 = DAT_005242c8.wrapping_add(1);
        if 1 < DAT_00a080f8 {
            let mut u_var2: u32 = DAT_005242e8.wrapping_add(DAT_00523418);
            DAT_00557350 =
                (DAT_00557350 as u32).wrapping_add(u_var2.wrapping_sub(DAT_00a07cc8)) as i32;
            DAT_00557360 = 2;
            DAT_005242e8 = u_var2;
            DAT_00557364 = u_var2;
            if 0x1d8 < DAT_00557350 {
                DAT_00557364 = u_var2 | 0x80000000;
                DAT_00557350 = 0;
                let mut p13 = (&raw mut DAT_009d77d0).cast::<u32>();
                let mut i3: i32 = 8;
                while i3 != 0 {
                    *p13 = 0;
                    p13 = p13.add(1);
                    i3 -= 1;
                }
            }
            DAT_00557368 = 0;
            let mut u_var10: u32 = 9;
            let mut local_10: *mut i32 = (&raw mut DAT_009a8db0).cast::<i32>();
            let mut local_8: *mut i32 = (&raw mut DAT_009ed670).cast::<i32>();
            let mut local_c: *mut i8 = (&raw mut DAT_009e9680).cast::<i8>();
            let end_addr = (&raw mut DAT_009ec468) as usize as u32;
            let mut pb12: *mut u8 = (&raw mut DAT_00557368).byte_add(1);
            loop {
                let mut pb4: *mut u8 = pb12;
                if *local_c != 0 {
                    DAT_00557368 = DAT_00557368.wrapping_add(1);
                    pb4 = pb12.add(1);
                    u_var10 = u_var10.wrapping_add(1);
                    let mut i3: i32 = 0;
                    *pb12 = 0;
                    let mut pi1: *mut u8 = local_8.cast::<u8>();
                    if 0 < *local_10 {
                        loop {
                            let mut tmp_a: i32 = pi1.cast::<i32>().read_unaligned();
                            let tmp_b: i32 = pi1.byte_add(8).cast::<i32>().read_unaligned();
                            if tmp_a != 0 && tmp_b <= u_var2 as i32 {
                                if 0x1f8i32.wrapping_sub(DAT_009fd394)
                                    < (pb4 as usize as i32).wrapping_sub(0x557358)
                                {
                                    break;
                                }
                                *pb12 = (*pb12).wrapping_add(1);
                                let mut u7: u32 = u_var2.wrapping_sub(tmp_b as u32);
                                if (u7 as i32) < 0x10000 {
                                    if (u7 as i32) < 0x170 {
                                        if 0x7c < (u7 as i32) {
                                            u7 = 0x7d;
                                        }
                                    } else {
                                        u7 = 0x7e;
                                    }
                                } else {
                                    u7 = 0x7f;
                                }
                                if *pi1.add(0xe) != 0 {
                                    u7 |= 0x80;
                                }
                                *pb4 = u7 as u8;
                                let mut pb5: *mut u8 = pb4.add(1);
                                let mut i11: i32 = u_var10.wrapping_add(1) as i32;
                                let u8_: u32 = u7 & 0x7f;
                                if u8_ == 0x7f {
                                    pb5 = pb4.add(5);
                                    i11 = u_var10.wrapping_add(5) as i32;
                                    pb4.add(1).cast::<i32>().write_unaligned(tmp_b);
                                } else if u8_ == 0x7e {
                                    pb5 = pb4.add(3);
                                    i11 = u_var10.wrapping_add(3) as i32;
                                    let tmp_s: i16 = (u_var2 as i16).wrapping_sub(tmp_b as i16);
                                    pb4.add(1).cast::<i16>().write_unaligned(tmp_s);
                                } else if u8_ == 0x7d {
                                    pb5 = pb4.add(2);
                                    i11 = u_var10.wrapping_add(2) as i32;
                                    *pb4.add(1) =
                                        (u_var2 as u8).wrapping_sub(tmp_b as u8).wrapping_add(0x83);
                                }
                                let mut pb6: *mut u8 = pb5;
                                if (u7 & 0x80) != 0 {
                                    pb6 = pb5.add(1);
                                    i11 = i11.wrapping_add(1);
                                    *pb5 = *pi1.add(0xe);
                                }
                                pb4 = pb6.add(2);
                                u_var10 = i11.wrapping_add(2) as u32;
                                *pb6 = *pi1.add(0xf);
                                *pb6.add(1) = *pi1.add(0x10);
                                *pb6 &= 0xfe;
                                *pb6.add(1) &= 0xfe;
                                *pb6 |= *pi1.add(0x11) & 1;
                                *pb6.add(1) |= (*pi1.add(0x11) & 2) >> 1;
                                tmp_a = 0;
                                pi1.cast::<i32>().write_unaligned(tmp_a);
                            }
                            i3 = i3.wrapping_add(1);
                            pi1 = pi1.add(0x12);
                            if !(i3 < *local_10) {
                                break;
                            }
                        }
                    }
                }
                local_10 = local_10.add(1);
                local_8 = local_8.add(0x7e9);
                local_c = local_c.add(0x5bd);
                pb12 = pb4;
                if !((local_c as usize as u32) < end_addr) {
                    break;
                }
            }
            FUN_0046d350(0, &raw mut DAT_00557360, u_var10);
            if DAT_005242d0 == 1 {
                let mut ok = !DAT_005242d8.is_null();
                if !ok {
                    DAT_005242d8 = FID_conflict___wfopen(
                        (&raw mut s_serverlog_txt_00524398).cast::<i8>(),
                        (&raw mut DAT_0052436c).cast::<i8>(),
                    );
                    ok = !DAT_005242d8.is_null();
                }
                if ok {
                    pb12 = (&raw mut DAT_00557368).byte_add(1);
                    let mut pc9: *mut i8 = (&raw mut DAT_009e9680).cast::<i8>();
                    _fprintf(
                        DAT_005242d8,
                        (&raw mut DAT_00524390).cast::<i8>(),
                        DAT_00557364,
                    );
                    loop {
                        if *pc9 != 0 {
                            u_var2 = *pb12 as u32;
                            _fprintf(DAT_005242d8, (&raw mut s__2x_00524388).cast::<i8>(), u_var2);
                            pb12 = pb12.add(1);
                            while u_var2 != 0 {
                                let tmp_a: i32 = pb12.cast::<i32>().read_unaligned();
                                _fprintf(
                                    DAT_005242d8,
                                    (&raw mut s__8x__2x__2x__2x__2x_00524370).cast::<i8>(),
                                    tmp_a,
                                    *pb12.add(6) as u32,
                                    *pb12.add(7) as u32,
                                    *pb12.add(8) as u32,
                                    *pb12.add(9) as u32,
                                );
                                pb12 = pb12.add(10);
                                u_var2 -= 1;
                            }
                        }
                        pc9 = pc9.add(0x5bd);
                        if !((pc9 as usize as u32) < end_addr) {
                            break;
                        }
                    }
                    _fflush(DAT_005242d8);
                }
            }
        }
    }
}
