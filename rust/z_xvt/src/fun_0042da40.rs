// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals, non_snake_case)]
unsafe extern "C" {
    static mut DAT_005250c0: i32;
    static mut DAT_00732e48: i32;
    static mut DAT_005250e0: u32;
    static mut DAT_00525558: u32;
    static mut DAT_005373c0: i32;
    static mut DAT_00732a18: u32;
    static mut _DAT_005373d0: u32;
    static mut _DAT_005373d4: u32;
    static mut DAT_005373d8: i8;
    static mut DAT_005373d9: u8;
    static mut DAT_0070a150: [u32; 8];
    static mut DAT_006db730: [i32; 1];
    static mut DAT_0071f720: [i32; 1];
    static mut DAT_0071b740: [i8; 1];
    static mut DAT_0071e528: i8;
    static mut DAT_0072f444: i32;
    static mut DAT_005250c8: i32;
    static mut DAT_005250d0: *mut c_void;
    static mut DAT_00525164: [i8; 1];
    static mut DAT_00525188: [i8; 1];
    static mut s_serverlog_txt_00525190: [i8; 1];
    static mut s__2x_00525180: [i8; 1];
    static mut s__8x__2x__2x__2x__2x_00525168: [i8; 1];

    fn FUN_00431110(param_1: i32, param_2: *mut u32, param_3: u32);
    fn _fopen(name: *const i8, mode: *const i8) -> *mut c_void;
    fn _fprintf(f: *mut c_void, fmt: *const i8, ...) -> i32;
    fn _fflush(f: *mut c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0042da40() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers walk the game's packet and entry tables as the original does.
    unsafe {
        DAT_005250c0 = DAT_005250c0.wrapping_add(1);
        if 1 < DAT_00732e48 {
            let mut u_var2: u32 = DAT_005250e0.wrapping_add(DAT_00525558);
            DAT_005373c0 = DAT_005373c0.wrapping_add(u_var2.wrapping_sub(DAT_00732a18) as i32);
            _DAT_005373d0 = 2;
            DAT_005250e0 = u_var2;
            _DAT_005373d4 = u_var2;
            if 0x1d8 < DAT_005373c0 {
                _DAT_005373d4 = u_var2 | 0x8000_0000;
                DAT_005373c0 = 0;
                let mut i: i32 = 0;
                while i < 8 {
                    (&raw mut DAT_0070a150)
                        .cast::<u32>()
                        .add(i as usize)
                        .write(0);
                    i = i.wrapping_add(1);
                }
            }
            DAT_005373d8 = 0;
            let mut u_var10: u32 = 9;
            let mut local_10: *mut i32 = (&raw mut DAT_006db730).cast::<i32>();
            let mut local_8: *mut i32 = (&raw mut DAT_0071f720).cast::<i32>();
            let mut local_c: *mut i8 = (&raw mut DAT_0071b740).cast::<i8>();
            let mut pb_var12: *mut u8 = &raw mut DAT_005373d9;
            let end: *mut i8 = &raw mut DAT_0071e528;
            loop {
                let mut pb_var4: *mut u8 = pb_var12;
                if *local_c != 0 {
                    DAT_005373d8 = DAT_005373d8.wrapping_add(1);
                    pb_var4 = pb_var12.wrapping_add(1);
                    u_var10 = u_var10.wrapping_add(1);
                    let mut i_var3: i32 = 0;
                    *pb_var12 = 0;
                    let mut pi_var1: *mut u8 = local_8.cast::<u8>();
                    if 0 < local_10.read() {
                        loop {
                            let i_entry0: i32 = pi_var1.cast::<i32>().read_unaligned();
                            let i_entry8: i32 = pi_var1.byte_add(8).cast::<i32>().read_unaligned();
                            if i_entry0 != 0 && i_entry8 <= u_var2 as i32 {
                                let limit = 0x1f8u32.wrapping_sub(DAT_0072f444 as u32) as i32;
                                let used = (pb_var4 as u32).wrapping_sub(0x5373c8) as i32;
                                if limit < used {
                                    break;
                                }
                                *pb_var12 = (*pb_var12).wrapping_add(1);
                                let mut u_var7: u32 = u_var2.wrapping_sub(i_entry8 as u32);
                                if (u_var7 as i32) < 0x10000 {
                                    if (u_var7 as i32) < 0x170 {
                                        if 0x7c < u_var7 as i32 {
                                            u_var7 = 0x7d;
                                        }
                                    } else {
                                        u_var7 = 0x7e;
                                    }
                                } else {
                                    u_var7 = 0x7f;
                                }
                                if pi_var1.byte_add(0xe).read() != 0 {
                                    u_var7 |= 0x80;
                                }
                                *pb_var4 = u_var7 as u8;
                                let mut pb_var5: *mut u8 = pb_var4.wrapping_add(1);
                                let mut i_var11: i32 = u_var10.wrapping_add(1) as i32;
                                let u_var8: u32 = u_var7 & 0x7f;
                                if u_var8 == 0x7f {
                                    pb_var5 = pb_var4.wrapping_add(5);
                                    i_var11 = u_var10.wrapping_add(5) as i32;
                                    pb_var4.add(1).cast::<i32>().write_unaligned(i_entry8);
                                } else if u_var8 == 0x7e {
                                    pb_var5 = pb_var4.wrapping_add(3);
                                    i_var11 = u_var10.wrapping_add(3) as i32;
                                    let s_delta: u16 =
                                        (u_var2 as u16).wrapping_sub(i_entry8 as u16);
                                    pb_var4.add(1).cast::<u16>().write_unaligned(s_delta);
                                } else if u_var8 == 0x7d {
                                    pb_var5 = pb_var4.wrapping_add(2);
                                    i_var11 = u_var10.wrapping_add(2) as i32;
                                    *pb_var4.add(1) = (u_var2 as u8)
                                        .wrapping_sub(i_entry8 as u8)
                                        .wrapping_add(0x83);
                                }
                                let mut pb_var6: *mut u8 = pb_var5;
                                if (u_var7 & 0x80) != 0 {
                                    pb_var6 = pb_var5.wrapping_add(1);
                                    i_var11 = i_var11.wrapping_add(1);
                                    *pb_var5 = pi_var1.byte_add(0xe).read();
                                }
                                pb_var4 = pb_var6.wrapping_add(2);
                                u_var10 = (i_var11 as u32).wrapping_add(2);
                                *pb_var6 = pi_var1.byte_add(0xf).read();
                                *pb_var6.add(1) = pi_var1.byte_add(0x10).read();
                                *pb_var6 &= 0xfe;
                                *pb_var6.add(1) &= 0xfe;
                                *pb_var6 |= pi_var1.byte_add(0x11).read() & 1;
                                *pb_var6.add(1) |= (pi_var1.byte_add(0x11).read() & 2) >> 1;
                                pi_var1.cast::<i32>().write_unaligned(0);
                            }
                            i_var3 = i_var3.wrapping_add(1);
                            pi_var1 = pi_var1.wrapping_add(0x12);
                            if !(i_var3 < local_10.read()) {
                                break;
                            }
                        }
                    }
                }
                local_10 = local_10.wrapping_add(1);
                local_8 = local_8.wrapping_add(0x7e9);
                local_c = local_c.wrapping_add(0x5bd);
                pb_var12 = pb_var4;
                if !(local_c < end) {
                    break;
                }
            }
            FUN_00431110(0, &raw mut _DAT_005373d0, u_var10);
            if DAT_005250c8 == 1 {
                let mut ok = !DAT_005250d0.is_null();
                if !ok {
                    DAT_005250d0 = _fopen(
                        (&raw const s_serverlog_txt_00525190).cast::<i8>(),
                        (&raw const DAT_00525164).cast::<i8>(),
                    );
                    ok = !DAT_005250d0.is_null();
                }
                if ok {
                    let mut pb: *mut u8 = &raw mut DAT_005373d9;
                    let mut pc_var9: *mut i8 = (&raw mut DAT_0071b740).cast::<i8>();
                    _fprintf(
                        DAT_005250d0,
                        (&raw const DAT_00525188).cast::<i8>(),
                        _DAT_005373d4,
                    );
                    loop {
                        if *pc_var9 != 0 {
                            u_var2 = u32::from(*pb);
                            _fprintf(
                                DAT_005250d0,
                                (&raw const s__2x_00525180).cast::<i8>(),
                                u_var2,
                            );
                            pb = pb.wrapping_add(1);
                            while u_var2 != 0 {
                                let u_log: u32 = pb.cast::<u32>().read_unaligned();
                                _fprintf(
                                    DAT_005250d0,
                                    (&raw const s__8x__2x__2x__2x__2x_00525168).cast::<i8>(),
                                    u_log,
                                    u32::from(*pb.add(6)),
                                    u32::from(*pb.add(7)),
                                    u32::from(*pb.add(8)),
                                    u32::from(*pb.add(9)),
                                );
                                pb = pb.wrapping_add(10);
                                u_var2 = u_var2.wrapping_sub(1);
                            }
                        }
                        pc_var9 = pc_var9.wrapping_add(0x5bd);
                        if !(pc_var9 < end) {
                            break;
                        }
                    }
                    _fflush(DAT_005250d0);
                }
            }
        }
    }
}
