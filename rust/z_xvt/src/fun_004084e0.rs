// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071b716: i16;
    static mut DAT_006db598: i32;
    static mut DAT_00730884: i32;
    static mut DAT_0071e518: i16;
    static mut DAT_00703ad4: i16;
    static mut DAT_00703c94: i16;
    static mut DAT_00732a20: i16;
    static mut DAT_006d9d96: i16;
    static mut DAT_006da53c: i16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004084e0() -> i32 {
    // SAFETY: the globals are the game's own memory; DAT_00730884 holds an
    // address of a game structure read at the original's offsets.
    unsafe {
        DAT_0071b716 = DAT_0071b716.wrapping_add(1);
        DAT_006db598 = DAT_006db598.wrapping_sub(
            ((DAT_00730884 as u32).wrapping_add(0x3eb8) as *const i32).read_unaligned(),
        );
        if DAT_0071b716 == 0 {
            DAT_0071e518 = 0;
            DAT_00703ad4 = ((DAT_00730884 as u32).wrapping_add(0x1934) as *const i16)
                .read_unaligned()
                .wrapping_sub(1);
            DAT_00703c94 = 0;
        } else if DAT_00732a20 <= DAT_0071b716 {
            if DAT_0071b716 == DAT_00732a20 {
                DAT_006d9d96 = -1;
            }
            DAT_006d9d96 = DAT_006d9d96.wrapping_add(1);
            let addr = (DAT_00730884 as u32)
                .wrapping_add(0x1934)
                .wrapping_add((DAT_006d9d96 as i32).wrapping_mul(2) as u32);
            DAT_0071e518 = DAT_0071e518.wrapping_add((addr as *const i16).read_unaligned());
        }
        DAT_006da53c = DAT_006da53c.wrapping_add(1);
        if DAT_006da53c == 0 {
            DAT_00703ad4 = ((DAT_00730884 as u32).wrapping_add(0xe) as *const i16)
                .read_unaligned()
                .wrapping_sub(1);
            return 1;
        }
        if DAT_00732a20 <= DAT_006da53c {
            return 0;
        }
        if DAT_006da53c < 0 && 0 < DAT_0071b716 {
            DAT_00703c94 = DAT_00703c94.wrapping_add(1);
            let addr = (DAT_00730884 as u32)
                .wrapping_add(0x1934)
                .wrapping_add((DAT_00703c94 as i32).wrapping_mul(2) as u32);
            DAT_00703ad4 = DAT_00703ad4.wrapping_add((addr as *const i16).read_unaligned());
        }
        1
    }
}
