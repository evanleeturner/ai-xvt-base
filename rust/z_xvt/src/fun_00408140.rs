// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071b716: i16;
    static mut DAT_006db598: i32;
    static mut DAT_00730884: i32;
    static mut DAT_00732a20: i16;
    static mut DAT_0071e518: i16;
    static mut DAT_006d9d96: i16;
    static mut DAT_006da53c: i16;
    static mut DAT_00703ad4: i16;
    static mut DAT_00703c94: i16;
}

#[inline(always)]
unsafe fn rd16(addr: i32) -> i16 {
    // SAFETY: the address is inside the game's own structure.
    unsafe { (addr as usize as *const i16).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00408140() -> i32 {
    // SAFETY: globals are the game's own memory; DAT_00730884 holds the
    // address of a game structure read at the original's offsets.
    unsafe {
        DAT_0071b716 = DAT_0071b716.wrapping_add(1);
        let v = ((DAT_00730884.wrapping_add(0x3eb8)) as usize as *const i32).read_unaligned();
        DAT_006db598 = DAT_006db598.wrapping_sub(v);
        if DAT_0071b716 == 0 {
            DAT_0071e518 = 0;
        } else {
            if DAT_00732a20 <= DAT_0071b716 {
                return 0;
            }
            if DAT_0071b716 < 0 {
                DAT_006d9d96 = DAT_006d9d96.wrapping_add(-1);
                let a = DAT_00730884
                    .wrapping_add(0x1934)
                    .wrapping_add((DAT_006d9d96 as i32).wrapping_mul(2));
                DAT_0071e518 = DAT_0071e518.wrapping_sub(rd16(a));
            }
        }
        DAT_006da53c = DAT_006da53c.wrapping_add(1);
        if DAT_006da53c == 0 {
            DAT_00703ad4 = rd16(DAT_00730884.wrapping_add(0xe)).wrapping_sub(1);
            DAT_006d9d96 = rd16(DAT_00730884.wrapping_add(0x1932)).wrapping_sub(1);
            let a = DAT_00730884
                .wrapping_add(0x1934)
                .wrapping_add((DAT_006d9d96 as i32).wrapping_mul(2));
            let r = (DAT_00703ad4 as i32)
                .wrapping_sub(rd16(a) as i32)
                .wrapping_add(1);
            DAT_0071e518 = r as i16;
            if DAT_0071e518 < 0 {
                DAT_0071e518 = 0;
                return 1;
            }
        } else if DAT_00732a20 <= DAT_006da53c {
            if DAT_006da53c == DAT_00732a20 {
                DAT_00703c94 = rd16(DAT_00730884.wrapping_add(0x1932));
            }
            DAT_00703c94 = DAT_00703c94.wrapping_add(-1);
            let a = DAT_00730884
                .wrapping_add(0x1934)
                .wrapping_add((DAT_00703c94 as i32).wrapping_mul(2));
            DAT_00703ad4 = DAT_00703ad4.wrapping_sub(rd16(a));
        }
        1
    }
}
