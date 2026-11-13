// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00524120: [u8; 1];
    static mut DAT_009d8cd2: [u8; 1];
    static mut DAT_009a1fe8: *mut core::ffi::c_void;
    static mut DAT_009a73c2: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00402cb0(param_1: u16);
    fn FUN_00404450(param_1: u16) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00404670(param_1: i32, param_2: u32, param_3: u16) -> u32 {
    if param_1 == -1 {
        return 0;
    }
    let mut s_var2: i16 = 0;
    let mut u_var3: u32 = 0;
    // SAFETY: the callees and globals are the game's own memory; the
    // pointer and table reads occur at the offsets the original uses.
    unsafe {
        FUN_00402cb0(param_1 as u16);
        loop {
            let off = (param_1 as u32).wrapping_mul(0x23).wrapping_add(0x16);
            let b = DAT_009a1fe8.cast::<u8>().byte_add(off as usize).read();
            let idx = (b as u32).wrapping_mul(0x566).wrapping_add(u_var3);
            let k = (&raw mut DAT_009d8cd2)
                .cast::<u8>()
                .byte_add(idx as usize)
                .read();
            let v = (&raw mut DAT_00524120)
                .cast::<u8>()
                .byte_add(k as usize)
                .read();
            if (v as u32) == param_2 {
                DAT_009a73c2 = s_var2;
                let u_var1 = FUN_00404450(param_3);
                if (u_var1 as i16) != 0 {
                    return 1;
                }
            }
            u_var3 = u_var3.wrapping_add(0x52);
            s_var2 = s_var2.wrapping_add(1);
            if u_var3 >= 0xf6 {
                break;
            }
        }
    }
    0
}
