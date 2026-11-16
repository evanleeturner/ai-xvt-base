// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a8d48: i16;
    static mut DAT_009a8d42: i16;
    static mut DAT_00a07cd0: i16;
    static mut DAT_009fe7d4: *mut u8;
    static mut DAT_009ed230: i16;
    static mut DAT_009e9652: i16;
    static mut DAT_009e9654: i16;
    static mut DAT_009a7bb0: i16;
    static mut DAT_009a7bb2: i16;
    static mut DAT_009ec458: i16;
    static mut DAT_009d8c2a: i16;
    static mut DAT_009d8c14: i16;
    static mut DAT_009d8c2c: i16;
    static mut DAT_009d114c: i16;
    static mut DAT_009ec462: i16;
}

#[inline(always)]
unsafe fn rd(off: i32) -> i16 {
    // SAFETY: DAT_009fe7d4 points at the game's own structure; the offset is read where the original reads.
    unsafe {
        DAT_009fe7d4
            .byte_offset(off as isize)
            .cast::<i16>()
            .read_unaligned()
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00423900() -> u32 {
    // SAFETY: all globals and the structure they point to belong to the game.
    unsafe {
        let mut s1: i16 = 0;
        let mut local_4: i16 = 0;

        if DAT_009a8d48 < 0 {
            loop {
                DAT_009a8d42 = DAT_009a8d42.wrapping_sub(rd(0x2c).wrapping_add(1));
                s1 = s1.wrapping_add(DAT_00a07cd0);
                DAT_009a8d48 = DAT_009a8d48.wrapping_add(rd(0x2e)).wrapping_add(1);
                if !(DAT_009a8d48 < 0) {
                    break;
                }
            }
        }
        if DAT_00a07cd0 <= DAT_009a8d48 {
            loop {
                s1 = s1.wrapping_sub(DAT_00a07cd0);
                DAT_009a8d42 = DAT_009a8d42.wrapping_add(rd(0x2c)).wrapping_add(1);
                DAT_009a8d48 = DAT_009a8d48.wrapping_sub(rd(0x2e).wrapping_add(1));
                if !(DAT_00a07cd0 <= DAT_009a8d48) {
                    break;
                }
            }
        }
        let s2: i16 = DAT_009ed230.wrapping_sub(DAT_009a8d48);
        DAT_009a8d42 = DAT_009a8d42.wrapping_sub(rd(0x30 + (s2 as i32).wrapping_mul(4)));
        DAT_009e9652 = DAT_009ed230;
        DAT_009e9654 = DAT_009a8d42;
        DAT_009a7bb0 = DAT_009ed230.wrapping_sub(rd(0x2e));
        DAT_009ec458 = 0;
        DAT_009d8c2a = -1;
        DAT_009a7bb2 = rd(0x2c).wrapping_add(DAT_009a8d42);
        DAT_009d8c14 = DAT_009d8c2c;
        if DAT_009d8c2c <= DAT_009a8d42 {
            return 0;
        }
        if DAT_009a8d42 < 0 {
            if DAT_009a7bb2 < 0 {
                DAT_009ec462 = s1.wrapping_add(s2);
                DAT_009d114c = -1;
                return 1;
            }
            DAT_009d8c2a = DAT_009a8d42.wrapping_neg();
            if rd(0x2c) < DAT_009d8c2a {
                DAT_009ec458 = -1;
            } else {
                let mut s4: i16 = 0;
                DAT_009ec458 = local_4;
                if -1 < rd(0x2e) {
                    loop {
                        local_4 = DAT_009d8c2a;
                        if (rd(0x30 + (s4 as i32).wrapping_mul(4)) as u16 as i32)
                            == (DAT_009d8c2a as i32)
                        {
                            DAT_009ec458 = rd(0x32 + (s4 as i32).wrapping_mul(4));
                            break;
                        }
                        s4 = s4.wrapping_add(1);
                        DAT_009ec458 = DAT_009d8c2a;
                        if !(s4 <= rd(0x2e)) {
                            break;
                        }
                    }
                }
            }
        }
        'lab: {
            let go_afc = if DAT_009a7bb2 < DAT_009d8c2c {
                if -1 < DAT_009a7bb2 {
                    local_4 = rd(0xe).wrapping_sub(1);
                    break 'lab;
                }
                DAT_009d8c2c <= DAT_009a7bb2
            } else {
                true
            };
            if go_afc && DAT_009a8d42 < DAT_009d8c2c {
                let mut s4: i16 = 0;
                DAT_009d8c14 = DAT_009d8c2c.wrapping_sub(DAT_009a8d42);
                if -1 < rd(0x2e) {
                    loop {
                        if (rd(0x30 + (s4 as i32).wrapping_mul(4)) as u16 as i32)
                            == (DAT_009d8c14 as i32)
                        {
                            local_4 = rd(0x32 + (s4 as i32).wrapping_mul(4)).wrapping_sub(1);
                            break;
                        }
                        s4 = s4.wrapping_add(1);
                        local_4 = DAT_009d8c14;
                        if !(s4 <= rd(0x2e)) {
                            break;
                        }
                    }
                }
                break 'lab;
            }
            local_4 = -1;
        }
        DAT_009d114c = local_4;
        DAT_009ec462 = s1.wrapping_add(s2);
        1
    }
}
