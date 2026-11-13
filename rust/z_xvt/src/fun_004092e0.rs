// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071b718: i16;
    static mut DAT_006db598: i32;
    static mut DAT_00525518: i32;
    static mut DAT_0070acec: i16;
    static mut DAT_0070acea: i16;
    static mut DAT_00730884: i32;
    static mut DAT_0071e518: i16;
    static mut DAT_006da53e: i16;
    static mut DAT_00703ad4: i16;
    static mut DAT_00732a20: i16;
    static mut DAT_0071f2e0: i16;
    static mut DAT_0070acd4: i16;
}

unsafe fn rd(off: i32) -> i16 {
    // SAFETY: the game's own table pointer plus offset, read as the original does.
    unsafe { (DAT_00730884.wrapping_add(off) as usize as *const i16).read_unaligned() }
}

unsafe fn tab(idx: i16) -> i16 {
    // SAFETY: same table as above, indexed by a short.
    unsafe { rd(0x1934i32.wrapping_add((idx as i32).wrapping_mul(2))) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004092e0() -> i32 {
    // SAFETY: globals and the table behind DAT_00730884 are the game's own memory.
    unsafe {
        DAT_0071b718 = DAT_0071b718.wrapping_add(1);
        DAT_006db598 = DAT_006db598.wrapping_add(DAT_00525518);
        'lab: {
            if DAT_0071b718 != 0 {
                if DAT_0071b718 >= 0 {
                    if DAT_0070acec <= DAT_0071b718 {
                        return 0;
                    }
                    break 'lab;
                }
                DAT_0070acea = DAT_0070acea.wrapping_sub(1);
                DAT_0071e518 = DAT_0071e518.wrapping_sub(tab(DAT_0070acea));
                if DAT_0071e518 >= 0 {
                    break 'lab;
                }
            }
            DAT_0071e518 = 0;
        }
        DAT_006da53e = DAT_006da53e.wrapping_add(1);
        if DAT_006da53e == 0 {
            DAT_00703ad4 = rd(0xe).wrapping_sub(1);
            DAT_0070acea = rd(0x1932).wrapping_sub(1);
            DAT_0071e518 = DAT_00732a20.wrapping_sub(tab(DAT_0070acea));
            if DAT_0071e518 < 0 {
                DAT_0071e518 = 0;
                return 1;
            }
        } else if DAT_0070acec <= DAT_006da53e {
            if DAT_0070acec == DAT_006da53e {
                DAT_00703ad4 = DAT_0071f2e0;
                DAT_0070acd4 = rd(0x1932);
            }
            DAT_0070acd4 = DAT_0070acd4.wrapping_sub(1);
            DAT_00703ad4 = DAT_00703ad4.wrapping_sub(tab(DAT_0070acd4));
        }
        1
    }
}
