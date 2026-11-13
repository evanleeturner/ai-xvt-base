// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a081ec: f32;
    static mut _DAT_00518060: f32;
    static mut _DAT_00518080: f32;
    static mut DAT_00a90b20: i32;
    static mut DAT_0054f994: i32;
    static mut DAT_0053f970: i32;
    static mut _DAT_0054f9a4: f32;
    static mut _DAT_0053f974: f32;
    static mut DAT_0051a54c: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040b350(param_1: i32, param_2: i32) {
    // SAFETY: the globals and the addressed buffers are the game's own memory;
    // reads and writes happen at the offsets the original uses.
    unsafe {
        let i_var5 = param_2.wrapping_add(param_1.wrapping_mul(0x18));
        let mut local_18: f32 = ((i_var5.wrapping_add(8)) as usize as *const f32).read_unaligned();
        let f_var1: f32 = ((i_var5.wrapping_add(4)) as usize as *const f32).read_unaligned();
        let u_var2: u32 = ((i_var5.wrapping_add(0x10)) as usize as *const u32).read_unaligned();
        let u_var3: u32 = ((i_var5.wrapping_add(0x14)) as usize as *const u32).read_unaligned();
        if 0x80000000u32 < (local_18 as u32) {
            local_18 = DAT_00a081ec;
        }
        let mut f_var4: f32 =
            _DAT_00518060 / ((DAT_00a081ec / local_18) * _DAT_00518080 + _DAT_00518060);
        if DAT_00a90b20 != 2 {
            f_var4 = _DAT_00518060 - f_var4;
        }
        let x: f32 = ((param_2.wrapping_add(param_1.wrapping_mul(0x18))) as usize as *const f32)
            .read_unaligned();
        let base = DAT_0054f994.wrapping_mul(0x20).wrapping_add(DAT_0053f970);
        ((base) as usize as *mut f32).write_unaligned(x + _DAT_0054f9a4);
        ((base.wrapping_add(4)) as usize as *mut f32).write_unaligned(f_var1 + _DAT_0053f974);
        ((base.wrapping_add(8)) as usize as *mut f32).write_unaligned(f_var4);
        ((base.wrapping_add(0xc)) as usize as *mut f32).write_unaligned(local_18);
        let base = DAT_0054f994.wrapping_mul(0x20).wrapping_add(DAT_0053f970);
        ((base.wrapping_add(0x18)) as usize as *mut u32).write_unaligned(u_var2);
        ((base.wrapping_add(0x1c)) as usize as *mut u32).write_unaligned(u_var3);
        let mut c: i32 = (local_18 as i64 as i32).wrapping_add(0x30);
        if 0xff < c {
            c = 0xff;
        }
        if DAT_0051a54c == 0 {
            c = c.wrapping_mul(0x10101).wrapping_sub(0x1000000);
        } else {
            c = c.wrapping_mul(0x10101).wrapping_sub(0x2000000);
        }
        let base = DAT_0054f994.wrapping_mul(0x20).wrapping_add(DAT_0053f970);
        ((base.wrapping_add(0x10)) as usize as *mut i32).write_unaligned(c);
        let base = DAT_0054f994.wrapping_mul(0x20).wrapping_add(DAT_0053f970);
        ((base.wrapping_add(0x14)) as usize as *mut u32).write_unaligned(0);
        DAT_0054f994 = DAT_0054f994.wrapping_add(1);
    }
}
