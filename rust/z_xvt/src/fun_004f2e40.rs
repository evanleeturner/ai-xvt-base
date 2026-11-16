// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0065047a: i16;
    static mut DAT_0065047c: i16;
    static mut DAT_00650482: [i16; 1];
    static mut DAT_00533da8: [i16; 1];
    static mut DAT_00650902: i16;
    static mut DAT_00650934: i16;
    static mut DAT_00650996: i16;
    static mut DAT_006508f6: i16;
    static mut DAT_006508f8: i16;
    static mut DAT_00650998: i16;
    static mut DAT_006508fa: [i16; 1];
    static mut DAT_006508fe: [i16; 1];
    static mut DAT_00650424: i16;
    static mut DAT_00650426: i16;
    static mut DAT_00650428: i16;
    static mut DAT_0065042a: i16;
    static mut DAT_0065042c: i16;
    static mut DAT_0065042e: i16;
    static mut DAT_00650430: i16;
    static mut DAT_00650432: i16;
    static mut DAT_00650904: [i16; 1];
    static mut DAT_00650914: [i16; 1];
    static mut DAT_00650924: [i16; 1];
    static mut DAT_00650936: [i16; 1];
    static mut DAT_00650946: [i16; 1];
    static mut DAT_00650956: [i16; 1];
    static mut DAT_00650966: [i16; 1];
    static mut DAT_00650976: [i16; 1];
    static mut DAT_00650986: [i16; 1];
    static mut DAT_006507a2: [u8; 1];
    static mut DAT_007438d7: [u8; 1];
    static mut DAT_0085e7c5: i8;
    static mut s_sfxText_00533e68: [i8; 1];
    static mut s_sfxTarget1_00533e70: [i8; 1];
    static mut s_sfxTarget2_00533e7c: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004de190(param_1: *mut i8, param_2: i32, param_3: i32, param_4: i32);
}

#[inline(always)]
unsafe fn at(p: *mut [i16; 1], i: i16) -> *mut i16 {
    // SAFETY: the array is the game's own memory; the index is the original's.
    unsafe { p.cast::<i16>().offset(i as isize) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f2e40(param_1: i16) {
    // SAFETY: all globals, arrays and callees are the game's own memory and
    // code; indices and offsets follow the original exactly.
    unsafe {
        let mut local_3e: i16 = DAT_0065047c;
        let mut local_38: [i16; 8] = [0; 8];
        let mut local_28: [i8; 40] = [0; 40];
        let l38 = local_38.as_mut_ptr();
        let l28 = local_28.as_mut_ptr();

        DAT_00650902 = 0;
        DAT_00650934 = 0;
        DAT_00650996 = 0;
        DAT_006508f6 = 0;
        DAT_006508f8 = 0;
        DAT_00650998 = 0;
        let mut s_var9: i16 = DAT_0065047c;
        if at(&raw mut DAT_00650482, DAT_0065047c).read() <= DAT_0065047a {
            loop {
                local_3e = s_var9;
                let s_var2 = at(&raw mut DAT_00650482, local_3e).read();
                s_var9 = local_3e.wrapping_add(2);
                let mut s_var10: i16 = 0;
                let mut s_var5 = at(&raw mut DAT_00650482, local_3e.wrapping_add(1)).read();
                let s_var3 = at(&raw mut DAT_00533da8, s_var5).read();
                if 0 < s_var3 {
                    loop {
                        let i6 = s_var9;
                        let i11 = s_var10;
                        s_var9 = s_var9.wrapping_add(1);
                        s_var10 = s_var10.wrapping_add(1);
                        l38.offset(i11 as isize)
                            .write(at(&raw mut DAT_00650482, i6).read());
                        if !(s_var10 < s_var3) {
                            break;
                        }
                    }
                }
                if DAT_0065047a == s_var2 {
                    match s_var5 as i32 {
                        1 => {
                            DAT_00650998 = 1;
                        }
                        3 => {
                            s_var5 = 0;
                            loop {
                                let i6 = s_var5;
                                s_var5 = s_var5.wrapping_add(1);
                                at(&raw mut DAT_006508fa, i6).write(0);
                                if !(s_var5 < 2) {
                                    break;
                                }
                            }
                            DAT_00650902 = 1;
                        }
                        4 | 5 => {
                            let k = s_var5.wrapping_sub(4);
                            at(&raw mut DAT_006508fa, k).write(1);
                            at(&raw mut DAT_006508fe, k).write(l38.read());
                        }
                        6 => {
                            if s_var2 == 0 || param_1 != 0 {
                                DAT_00650424 = l38.read();
                                DAT_00650426 = l38.offset(1).read();
                            }
                            DAT_0065042a = l38.offset(1).read();
                            DAT_00650428 = l38.read();
                            DAT_006508f6 = 1;
                        }
                        7 => {
                            if s_var2 == 0 || param_1 != 0 {
                                DAT_0065042c = l38.read();
                                DAT_0065042e = l38.offset(1).read();
                            }
                            DAT_00650432 = l38.offset(1).read();
                            DAT_00650430 = l38.read();
                            DAT_006508f8 = 1;
                        }
                        8 => {
                            s_var5 = 0;
                            loop {
                                let i6 = s_var5;
                                s_var5 = s_var5.wrapping_add(1);
                                at(&raw mut DAT_00650904, i6).write(0);
                                if !(s_var5 < 8) {
                                    break;
                                }
                            }
                            DAT_00650934 = 1;
                        }
                        9..=0x10 => {
                            if param_1 == 0 {
                                let idx = (l38.read() as i32).wrapping_mul(0x562);
                                let mut b_var4 = (&raw mut DAT_007438d7)
                                    .cast::<u8>()
                                    .offset(idx as isize)
                                    .read();
                                if 2 < b_var4 {
                                    b_var4 = 2;
                                }
                                if b_var4 == 1 {
                                    if DAT_0085e7c5 != 0 {
                                        FUN_004de190(
                                            (&raw mut s_sfxTarget2_00533e7c).cast::<i8>(),
                                            1,
                                            0,
                                            0x7f,
                                        );
                                    }
                                } else if DAT_0085e7c5 != 0 {
                                    FUN_004de190(
                                        (&raw mut s_sfxTarget1_00533e70).cast::<i8>(),
                                        1,
                                        0,
                                        0x7f,
                                    );
                                }
                            }
                            s_var5 = s_var5.wrapping_sub(9);
                            at(&raw mut DAT_00650904, s_var5).write(1);
                            at(&raw mut DAT_00650924, s_var5)
                                .write(((((param_1 == 0) as i32) - 1) & 0x50) as i16);
                            at(&raw mut DAT_00650914, s_var5).write(l38.read());
                        }
                        0x11 => {
                            s_var5 = 0;
                            loop {
                                let i6 = s_var5;
                                s_var5 = s_var5.wrapping_add(1);
                                at(&raw mut DAT_00650936, i6).write(0);
                                if !(s_var5 < 8) {
                                    break;
                                }
                            }
                            DAT_00650996 = 1;
                        }
                        0x12..=0x19 => {
                            if param_1 == 0 {
                                let off = (l38.read() as i32).wrapping_mul(4);
                                let mut src: *const i8 = (&raw mut DAT_006507a2)
                                    .cast::<u8>()
                                    .offset(off as isize)
                                    .cast::<*const i8>()
                                    .read_unaligned();
                                let start = src;
                                let mut n: u32 = 0;
                                loop {
                                    n = n.wrapping_add(1);
                                    let c = src.read();
                                    src = src.add(1);
                                    if c == 0 {
                                        break;
                                    }
                                }
                                let mut s = start;
                                let mut d = l28;
                                let mut k = n;
                                while k != 0 {
                                    d.write(s.read());
                                    s = s.add(1);
                                    d = d.add(1);
                                    k -= 1;
                                }
                                let mut i6: i32 = -1;
                                let mut p = l28 as *const i8;
                                loop {
                                    if i6 == 0 {
                                        break;
                                    }
                                    i6 = i6.wrapping_sub(1);
                                    let c = p.read();
                                    p = p.add(1);
                                    if c == 0 {
                                        break;
                                    }
                                }
                                if (i6 as i16) != -2 && DAT_0085e7c5 != 0 {
                                    FUN_004de190(
                                        (&raw mut s_sfxText_00533e68).cast::<i8>(),
                                        1,
                                        0,
                                        0x7f,
                                    );
                                }
                            }
                            let k = s_var5.wrapping_sub(0x12);
                            at(&raw mut DAT_00650936, k).write(1);
                            at(&raw mut DAT_00650976, k)
                                .write(((((param_1 == 0) as i32) - 1) & 0x50) as i16);
                            at(&raw mut DAT_00650946, k).write(l38.read());
                            at(&raw mut DAT_00650956, k).write(l38.offset(1).read());
                            at(&raw mut DAT_00650966, k).write(l38.offset(2).read());
                            at(&raw mut DAT_00650986, k).write(l38.offset(3).read());
                        }
                        _ => {}
                    }
                }
                if !(DAT_0065047a >= s_var2) {
                    break;
                }
            }
        }
        DAT_0065047a = DAT_0065047a.wrapping_add(1);
        DAT_0065047c = local_3e;
    }
}
