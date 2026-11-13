// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525508: u8;
    static mut DAT_00525518: u32;
    static mut DAT_006db594: u32;
    static mut DAT_006da538: u32;
    static mut DAT_006da9ec: *mut u8;
    static mut DAT_00703c80: *const core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00461f20(param_1: i32, param_2: u32, param_3: u32, param_4: u32);
    fn FUN_0048eda0(param_1: u32);
    fn FUN_0048ed00(param_1: i32, param_2: i32, param_3: i16, param_4: i16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0047cb30() {
    // SAFETY: the globals and callees are the game's own; the buffer pointer
    // is the game's frame buffer, written exactly as the original does.
    unsafe {
        let b_var2 = DAT_00525508;
        if DAT_00525518 == 1 {
            let u_var1: u16 = ((b_var2 as u32) | ((b_var2 as u32) << 8)) as u16;
            let mut u_var4: u32 = DAT_006db594.wrapping_mul(DAT_006da538);
            let mut pbvar5: *mut u8 = DAT_006da9ec;
            let fill: u32 = (u_var1 as u32) | ((u_var1 as u32) << 16);
            let mut u_var3: u32 = u_var4 >> 2;
            while u_var3 != 0 {
                pbvar5.cast::<u32>().write_unaligned(fill);
                pbvar5 = pbvar5.byte_add(4);
                u_var3 = u_var3.wrapping_sub(1);
            }
            u_var4 &= 3;
            while u_var4 != 0 {
                pbvar5.write(b_var2);
                pbvar5 = pbvar5.byte_add(1);
                u_var4 = u_var4.wrapping_sub(1);
            }
            return;
        }
        FUN_00461f20(
            DAT_006da9ec as i32,
            DAT_006da538,
            DAT_006db594,
            DAT_00525518.wrapping_mul(DAT_006da538),
        );
        FUN_0048eda0(DAT_00525508 as u32);
        FUN_0048ed00(0, 0, DAT_006da538 as i16, DAT_006db594 as i16);
        let f: unsafe extern "C" fn() = core::mem::transmute(DAT_00703c80);
        f();
        FUN_00461f20(0, 0x140, 0xf0, 0);
    }
}
