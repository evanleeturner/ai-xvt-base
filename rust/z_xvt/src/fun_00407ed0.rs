// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006db6c2: i16;
    static mut DAT_0070acec: i16;
    static mut DAT_00730884: *mut i8;
    static mut DAT_006db6c8: i16;
    static mut DAT_0071b716: i16;
    static mut DAT_00732a20: i16;
    static mut DAT_006da53c: i16;
    static mut DAT_0071b718: i16;
    static mut DAT_0071e518: i16;
    static mut DAT_006d9d96: i16;
    static mut DAT_006da53e: i16;
    static mut DAT_00703c94: i16;
    static mut DAT_00703ad4: i16;
    static mut DAT_0071e522: i16;
}

unsafe fn rd(off: isize) -> i16 {
    // SAFETY: DAT_00730884 points at the game's table; offsets are read as the original does.
    unsafe { DAT_00730884.byte_offset(off).cast::<i16>().read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00407ed0() -> i32 {
    // SAFETY: all globals and the table behind DAT_00730884 are the game's own memory.
    unsafe {
        let mut s_var2: i16 = 0;
        if DAT_006db6c2 < 0 {
            loop {
                s_var2 = s_var2.wrapping_sub(DAT_0070acec);
                DAT_006db6c2 = (DAT_006db6c2 as i32 + rd(0x2c) as i32 + 1) as i16;
                DAT_006db6c8 = (DAT_006db6c8 as i32 + rd(0x2e) as i32 + 1) as i16;
                if !(DAT_006db6c2 < 0) {
                    break;
                }
            }
        }
        if DAT_0070acec <= DAT_006db6c2 {
            loop {
                s_var2 = s_var2.wrapping_add(DAT_0070acec);
                DAT_006db6c2 = (DAT_006db6c2 as i32 - (rd(0x2c) as i32 + 1)) as i16;
                DAT_006db6c8 = (DAT_006db6c8 as i32 - (rd(0x2e) as i32 + 1)) as i16;
                if !(DAT_0070acec <= DAT_006db6c2) {
                    break;
                }
            }
        }
        DAT_006db6c8 =
            (DAT_006db6c8 as i32 - rd(0x32 + (DAT_006db6c2 as i32 * 4) as isize) as i32) as i16;
        let mut s_var1: i16 = DAT_006db6c8;
        DAT_0071b716 = DAT_006db6c8;
        if DAT_00732a20 <= DAT_006db6c8 {
            DAT_006da53c = (rd(0x2e) as i32 + DAT_006db6c8 as i32) as i16;
            return 0;
        }
        DAT_0071b718 = 0;
        DAT_0071e518 = 0;
        if DAT_006db6c8 < 0 {
            DAT_006d9d96 = DAT_006db6c8.wrapping_neg();
            DAT_006db6c8 = DAT_006d9d96;
            if rd(0x2e) < DAT_006d9d96 {
                DAT_0071e518 = -1;
            } else {
                let mut u_var3: u16 = 0;
                DAT_0071e518 = DAT_006d9d96;
                if -1 < rd(0x2c) {
                    loop {
                        if (rd(0x32 + (u_var3 as isize) * 4) as u16 as u32)
                            == (DAT_006d9d96 as i32 as u32)
                        {
                            DAT_0071e518 = rd(0x30 + (u_var3 as isize) * 4);
                            break;
                        }
                        u_var3 = u_var3.wrapping_add(1);
                        if !((u_var3 as i32) <= rd(0x2c) as i32) {
                            break;
                        }
                    }
                }
            }
        }
        DAT_006da53e = rd(0x2c);
        DAT_00703c94 = DAT_00732a20;
        DAT_006da53c = (rd(0x2e) as i32 + s_var1 as i32) as i16;
        if DAT_006da53c < 0 {
            DAT_00703ad4 = -1;
        } else {
            if DAT_006da53c < 0 || DAT_00732a20 <= DAT_006da53c {
                let mut u_var3: u16 = 0;
                DAT_00703c94 = (DAT_00732a20 as i32 - s_var1 as i32) as i16;
                s_var1 = rd(0x2c);
                if -1 < s_var1 {
                    loop {
                        if (rd(0x32 + (u_var3 as isize) * 4) as u16 as u32)
                            == (DAT_00703c94 as i32 as u32)
                        {
                            break;
                        }
                        u_var3 = u_var3.wrapping_add(1);
                        if !((u_var3 as i32) <= s_var1 as i32) {
                            break;
                        }
                    }
                }
                s_var1 = rd(0x30 + (u_var3 as isize) * 4);
            } else {
                s_var1 = rd(0xe);
            }
            DAT_00703ad4 = s_var1.wrapping_sub(1);
        }
        DAT_0071e522 = DAT_006db6c2.wrapping_add(s_var2);
        1
    }
}
