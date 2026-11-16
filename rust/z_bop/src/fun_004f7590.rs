// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals, non_snake_case)]
unsafe extern "C" {
    static mut DAT_0066925c: i16;
    static mut DAT_0066925a: i16;
    static mut DAT_00669262: [i16; 0];
    static mut DAT_0052cf10: [i16; 0];
    static mut DAT_006696e2: i16;
    static mut _DAT_00669714: i16;
    static mut _DAT_00669776: i16;
    static mut _DAT_006696d6: i16;
    static mut _DAT_006696d8: i16;
    static mut DAT_00669778: i16;
    static mut DAT_006696da: [i16; 0];
    static mut DAT_006696de: [i16; 0];
    static mut DAT_00669204: i16;
    static mut DAT_00669206: i16;
    static mut DAT_00669208: i16;
    static mut DAT_0066920a: i16;
    static mut DAT_0066920c: i16;
    static mut DAT_0066920e: i16;
    static mut DAT_00669210: i16;
    static mut DAT_00669212: i16;
    static mut DAT_006696e4: [i16; 0];
    static mut DAT_006696f4: [i16; 0];
    static mut DAT_00669704: [i16; 0];
    static mut DAT_00669716: [i16; 0];
    static mut DAT_00669726: [i16; 0];
    static mut DAT_00669736: [i16; 0];
    static mut DAT_00669746: [i16; 0];
    static mut DAT_00669756: [i16; 0];
    static mut DAT_00669766: [i16; 0];
    static mut DAT_00a91d17: [u8; 0];
    static mut DAT_00bb73a7: i8;
    static mut DAT_00669582: [*mut i8; 0];
    static mut s_sfxText_0052cfe4: [i8; 0];
    static mut s_sfxTarget1_0052cfec: [i8; 0];
    static mut s_sfxTarget2_0052cff8: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004de5a0(param_1: *mut i8, param_2: i32, param_3: i32, param_4: i32);
}

macro_rules! a {
    ($n:ident, $i:expr) => {
        (&raw mut $n).cast::<i16>().offset($i as isize)
    };
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f7590(param_1: i16) {
    // SAFETY: all globals and the callee are the game's own memory; indices
    // and pointers are used exactly as the original does.
    unsafe {
        let mut local_3e: i16 = DAT_0066925c;
        let mut local_38: [i16; 8] = [0; 8];
        let mut local_28: [i8; 40] = [0; 40];
        let l38 = local_38.as_mut_ptr();
        let l28 = local_28.as_mut_ptr();

        DAT_006696e2 = 0;
        _DAT_00669714 = 0;
        _DAT_00669776 = 0;
        _DAT_006696d6 = 0;
        _DAT_006696d8 = 0;
        DAT_00669778 = 0;
        let mut s_var9: i16 = DAT_0066925c;
        if a!(DAT_00669262, DAT_0066925c).read() <= DAT_0066925a {
            loop {
                local_3e = s_var9;
                let s_var2 = a!(DAT_00669262, local_3e).read();
                s_var9 = local_3e.wrapping_add(2);
                let mut s_var10: i16 = 0;
                let mut s_var5 = a!(DAT_00669262, local_3e.wrapping_add(1)).read();
                let s_var3 = a!(DAT_0052cf10, s_var5).read();
                if 0 < s_var3 {
                    loop {
                        let i6 = s_var9;
                        let i11 = s_var10;
                        s_var9 = s_var9.wrapping_add(1);
                        s_var10 = s_var10.wrapping_add(1);
                        l38.offset(i11 as isize).write(a!(DAT_00669262, i6).read());
                        if !(s_var10 < s_var3) {
                            break;
                        }
                    }
                }
                if DAT_0066925a == s_var2 {
                    match s_var5 as i32 {
                        1 => {
                            DAT_00669778 = 1;
                        }
                        3 => {
                            s_var5 = 0;
                            loop {
                                let i6 = s_var5;
                                s_var5 = s_var5.wrapping_add(1);
                                a!(DAT_006696da, i6).write(0);
                                if !(s_var5 < 2) {
                                    break;
                                }
                            }
                            DAT_006696e2 = 1;
                        }
                        4 | 5 => {
                            let k = s_var5.wrapping_add(-4);
                            a!(DAT_006696da, k).write(1);
                            a!(DAT_006696de, k).write(*l38);
                        }
                        6 => {
                            if s_var2 == 0 || param_1 != 0 {
                                DAT_00669204 = *l38;
                                DAT_00669206 = *l38.offset(1);
                            }
                            DAT_0066920a = *l38.offset(1);
                            DAT_00669208 = *l38;
                            _DAT_006696d6 = 1;
                        }
                        7 => {
                            if s_var2 == 0 || param_1 != 0 {
                                DAT_0066920c = *l38;
                                DAT_0066920e = *l38.offset(1);
                            }
                            DAT_00669212 = *l38.offset(1);
                            DAT_00669210 = *l38;
                            _DAT_006696d8 = 1;
                        }
                        8 => {
                            s_var5 = 0;
                            loop {
                                let i6 = s_var5;
                                s_var5 = s_var5.wrapping_add(1);
                                a!(DAT_006696e4, i6).write(0);
                                if !(s_var5 < 8) {
                                    break;
                                }
                            }
                            _DAT_00669714 = 1;
                        }
                        9..=0x10 => {
                            if param_1 == 0 {
                                let mut b_var4 = (&raw mut DAT_00a91d17)
                                    .cast::<u8>()
                                    .offset((*l38 as i32).wrapping_mul(0x562) as isize)
                                    .read();
                                if 2 < b_var4 {
                                    b_var4 = 2;
                                }
                                if b_var4 == 1 {
                                    if DAT_00bb73a7 != 0 {
                                        FUN_004de5a0(
                                            (&raw mut s_sfxTarget2_0052cff8).cast::<i8>(),
                                            1,
                                            0,
                                            0x7f,
                                        );
                                    }
                                } else if DAT_00bb73a7 != 0 {
                                    FUN_004de5a0(
                                        (&raw mut s_sfxTarget1_0052cfec).cast::<i8>(),
                                        1,
                                        0,
                                        0x7f,
                                    );
                                }
                            }
                            s_var5 = s_var5.wrapping_add(-9);
                            a!(DAT_006696e4, s_var5).write(1);
                            a!(DAT_00669704, s_var5)
                                .write(((((param_1 == 0) as i32).wrapping_sub(1)) & 0x50) as i16);
                            a!(DAT_006696f4, s_var5).write(*l38);
                        }
                        0x11 => {
                            s_var5 = 0;
                            loop {
                                let i6 = s_var5;
                                s_var5 = s_var5.wrapping_add(1);
                                a!(DAT_00669716, i6).write(0);
                                if !(s_var5 < 8) {
                                    break;
                                }
                            }
                            _DAT_00669776 = 1;
                        }
                        0x12..=0x19 => {
                            if param_1 == 0 {
                                let mut u_var7: u32 = 0xffff_ffff;
                                let mut pc13: *const i8 = (&raw mut DAT_00669582)
                                    .cast::<*mut i8>()
                                    .offset(*l38 as isize)
                                    .read();
                                let mut pc12: *const i8;
                                loop {
                                    pc12 = pc13;
                                    if u_var7 == 0 {
                                        break;
                                    }
                                    u_var7 = u_var7.wrapping_sub(1);
                                    pc12 = pc13.wrapping_add(1);
                                    let c = pc13.read();
                                    pc13 = pc12;
                                    if c == 0 {
                                        break;
                                    }
                                }
                                u_var7 = !u_var7;
                                pc13 = pc12.wrapping_sub(u_var7 as usize);
                                let mut dst: *mut i8 = l28;
                                let mut n = u_var7 >> 2;
                                while n != 0 {
                                    dst.cast::<u32>()
                                        .write_unaligned(pc13.cast::<u32>().read_unaligned());
                                    pc13 = pc13.wrapping_add(4);
                                    dst = dst.wrapping_add(4);
                                    n -= 1;
                                }
                                n = u_var7 & 3;
                                while n != 0 {
                                    *dst = *pc13;
                                    pc13 = pc13.wrapping_add(1);
                                    dst = dst.wrapping_add(1);
                                    n -= 1;
                                }
                                let mut i6: i32 = -1;
                                let mut p: *const i8 = l28;
                                loop {
                                    if i6 == 0 {
                                        break;
                                    }
                                    i6 = i6.wrapping_sub(1);
                                    let c = p.read();
                                    p = p.wrapping_add(1);
                                    if c == 0 {
                                        break;
                                    }
                                }
                                if (i6 as i16) != -2 && DAT_00bb73a7 != 0 {
                                    FUN_004de5a0(
                                        (&raw mut s_sfxText_0052cfe4).cast::<i8>(),
                                        1,
                                        0,
                                        0x7f,
                                    );
                                }
                            }
                            let i11 = s_var5.wrapping_add(-0x12);
                            a!(DAT_00669716, i11).write(1);
                            a!(DAT_00669756, i11)
                                .write(((((param_1 == 0) as i32).wrapping_sub(1)) & 0x50) as i16);
                            a!(DAT_00669726, i11).write(*l38);
                            a!(DAT_00669736, i11).write(*l38.offset(1));
                            a!(DAT_00669746, i11).write(*l38.offset(2));
                            a!(DAT_00669766, i11).write(*l38.offset(3));
                        }
                        _ => {}
                    }
                }
                if !(s_var2 <= DAT_0066925a) {
                    break;
                }
            }
        }
        DAT_0066925a = DAT_0066925a.wrapping_add(1);
        DAT_0066925c = local_3e;
    }
}
