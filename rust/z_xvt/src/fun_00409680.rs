// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006da53e: i16;
    static mut DAT_0071b718: i16;
    static mut DAT_006db598: i32;
    static mut DAT_00525518: i32;
    static mut DAT_0070a170: i16;
    static mut DAT_0070acec: i16;
    static mut DAT_0070acea: i16;
    static mut DAT_0071e518: i16;
    static mut DAT_00730884: i32;
    static mut DAT_00703ad4: i16;
    static mut DAT_0070acd4: i16;
}

unsafe fn rd_short(addr: i32) -> i16 {
    // SAFETY: the address is computed from the game's own table pointer,
    // read where the original reads it.
    unsafe { (addr as u32 as usize as *const i16).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00409680() -> i32 {
    // SAFETY: all globals and the table behind DAT_00730884 are the game's
    // own memory, accessed as the original does.
    unsafe {
        DAT_006da53e = DAT_006da53e.wrapping_sub(1);
        DAT_0071b718 = DAT_0071b718.wrapping_sub(1);
        DAT_006db598 = DAT_006db598.wrapping_sub(DAT_00525518);
        if DAT_0071b718 < 0 {
            return 0;
        }
        'lab: {
            if DAT_0071b718 != DAT_0070a170 {
                if DAT_0071b718 < DAT_0070acec || DAT_0070acec <= DAT_006da53e {
                    break 'lab;
                }
                DAT_0070acea = DAT_0070acea.wrapping_sub(1);
                let a = DAT_00730884
                    .wrapping_add(0x1934)
                    .wrapping_add((DAT_0070acea as i32).wrapping_mul(2));
                DAT_0071e518 = DAT_0071e518.wrapping_sub(rd_short(a));
                if -1 < DAT_0071e518 {
                    break 'lab;
                }
            }
            DAT_0071e518 = 0;
        }
        if DAT_006da53e == DAT_0070a170 {
            DAT_00703ad4 = rd_short(DAT_00730884.wrapping_add(0xe)).wrapping_sub(1);
            DAT_0070acea = rd_short(DAT_00730884.wrapping_add(0x1932)).wrapping_sub(1);
            let a = DAT_00730884
                .wrapping_add(0x1934)
                .wrapping_add((DAT_0070acea as i32).wrapping_mul(2));
            DAT_0071e518 = DAT_00703ad4.wrapping_sub(rd_short(a)).wrapping_add(1);
            if DAT_0071e518 < 0 {
                DAT_0071e518 = 0;
                return 1;
            }
        } else if DAT_006da53e < 0 {
            DAT_0070acd4 = DAT_0070acd4.wrapping_sub(1);
            let a = DAT_00730884
                .wrapping_add(0x1934)
                .wrapping_add((DAT_0070acd4 as i32).wrapping_mul(2));
            DAT_00703ad4 = DAT_00703ad4.wrapping_sub(rd_short(a));
            if DAT_00703ad4 < 0 {
                DAT_00703ad4 = 0;
            }
        }
        1
    }
}
