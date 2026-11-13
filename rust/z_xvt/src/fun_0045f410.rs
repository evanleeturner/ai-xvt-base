// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732f3c: i32;
    static mut _DAT_00520194: f32;
    static mut _DAT_005201b4: f32;
    static mut DAT_005f10d4: i32;
    static mut DAT_005f10b0: i32;
    static mut _DAT_005f10e4: f32;
    static mut _DAT_005f10b4: f32;
    static mut DAT_005279f4: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045f410(param_1: i32, param_2: i32) {
    // SAFETY: all addresses are the game's own memory: the source record at
    // param_2 + param_1 * 0x18 and the output array at DAT_005f10b0, accessed
    // where the original does.
    unsafe {
        let rec = param_2.wrapping_add(param_1.wrapping_mul(0x18));
        let mut local_20: f32 = ((rec.wrapping_add(8)) as *const f32).read_unaligned();
        let f_var1: f32 = ((rec.wrapping_add(4)) as *const f32).read_unaligned();
        let u_var2: i32 = ((rec.wrapping_add(0x10)) as *const i32).read_unaligned();
        let u_var3: i32 = ((rec.wrapping_add(0x14)) as *const i32).read_unaligned();
        if 0x80000000u32 < (local_20 as u32) {
            local_20 = DAT_00732f3c as f32;
        }
        let f_var4: f32 = (_DAT_00520194 / local_20) * _DAT_005201b4;
        let base = DAT_005f10d4.wrapping_mul(0x20).wrapping_add(DAT_005f10b0);
        let x = (param_2.wrapping_add(param_1.wrapping_mul(0x18)) as *const f32).read_unaligned();
        (base as *mut f32).write_unaligned(x + _DAT_005f10e4);
        (base.wrapping_add(4) as *mut f32).write_unaligned(f_var1 + _DAT_005f10b4);
        (base.wrapping_add(8) as *mut f32).write_unaligned(f_var4);
        (base.wrapping_add(0xc) as *mut f32).write_unaligned(local_20);
        (base.wrapping_add(0x18) as *mut i32).write_unaligned(u_var2);
        (base.wrapping_add(0x1c) as *mut i32).write_unaligned(u_var3);
        let mut i_var5: i32 = (local_20 as i64) as i32;
        i_var5 = i_var5.wrapping_add(0x30);
        if 0xff < i_var5 {
            i_var5 = 0xff;
        }
        let color = if DAT_005279f4 == 0 {
            i_var5.wrapping_mul(0x10101).wrapping_sub(0x1000000)
        } else {
            i_var5.wrapping_mul(0x10101).wrapping_sub(0x2000000)
        };
        (base.wrapping_add(0x10) as *mut i32).write_unaligned(color);
        (base.wrapping_add(0x14) as *mut i32).write_unaligned(0);
        DAT_005f10d4 = DAT_005f10d4.wrapping_add(1);
    }
}
