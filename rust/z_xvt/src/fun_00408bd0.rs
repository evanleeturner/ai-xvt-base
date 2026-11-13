// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071b716: i16;
    static mut DAT_006db598: i32;
    static mut DAT_006da53c: i16;
    static mut DAT_0071f2e0: i16;
    static mut DAT_0071e518: i16;
    static mut DAT_006d9d96: i16;
    static mut DAT_00732a20: i16;
    static mut DAT_00730884: i32;
    static mut DAT_00703ad4: i16;
    static mut DAT_00703c94: i16;
}

unsafe fn rd16(addr: i32) -> i16 {
    // SAFETY: the address is inside the game's own data structure.
    unsafe { (addr as u32 as usize as *const i16).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00408bd0() -> i32 {
    // SAFETY: globals and the structure at DAT_00730884 are the game's own memory.
    unsafe {
        DAT_0071b716 = DAT_0071b716.wrapping_sub(1);
        let p = DAT_00730884;
        DAT_006db598 = DAT_006db598.wrapping_add(
            ((p.wrapping_add(0x3eb8)) as u32 as usize as *const i32).read_unaligned(),
        );
        DAT_006da53c = DAT_006da53c.wrapping_sub(1);
        if DAT_0071b716 == DAT_0071f2e0 {
            DAT_0071e518 = 0;
            DAT_006d9d96 = -1;
        } else {
            if DAT_0071b716 < 0 {
                return 0;
            }
            if DAT_00732a20 <= DAT_0071b716 && DAT_00732a20 <= DAT_006da53c {
                DAT_006d9d96 = DAT_006d9d96.wrapping_sub(1);
                let idx = (DAT_006d9d96 as i32).wrapping_mul(2);
                let v = rd16(DAT_00730884.wrapping_add(0x1934).wrapping_add(idx));
                DAT_0071e518 = DAT_0071e518.wrapping_sub(v);
            }
        }
        if DAT_006da53c == DAT_0071f2e0 {
            DAT_00703ad4 = rd16(DAT_00730884.wrapping_add(0xe)).wrapping_sub(1);
            DAT_00703c94 = rd16(DAT_00730884.wrapping_add(0x1932));
            DAT_006d9d96 = DAT_00703c94.wrapping_sub(1);
            let idx = (DAT_006d9d96 as i32).wrapping_mul(2);
            let v = rd16(DAT_00730884.wrapping_add(0x1934).wrapping_add(idx)) as i32;
            DAT_0071e518 = ((DAT_00703ad4 as i32).wrapping_sub(v).wrapping_add(1)) as i16;
            if DAT_0071e518 < 0 {
                DAT_0071e518 = 0;
                return 1;
            }
        } else if DAT_006da53c < 0 {
            DAT_00703c94 = DAT_00703c94.wrapping_sub(1);
            let idx = (DAT_00703c94 as i32).wrapping_mul(2);
            let v = rd16(DAT_00730884.wrapping_add(0x1934).wrapping_add(idx));
            DAT_00703ad4 = DAT_00703ad4.wrapping_sub(v);
        }
        1
    }
}
