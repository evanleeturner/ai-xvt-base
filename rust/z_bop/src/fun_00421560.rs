// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052361c: i32;
    static mut DAT_009a8d4a: i16;
    static mut DAT_009a8d4e: i16;
    static mut DAT_009fe7d4: i32;
    static mut DAT_009ec610: u16;
    static mut DAT_009a8d42: i16;
    static mut DAT_009a7b48: i16;
    static mut DAT_009a8d48: i16;
    static mut DAT_009a7b40: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004219d0(param_1: u16, param_2: i32, param_3: *mut u16);
    fn FUN_00421ae0(param_1: i32, param_2: *mut u16);
    fn FUN_00422170(param_1: *mut u8, param_2: i32);
    fn FUN_00421700(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00421560(param_1: i16, param_2: i16, param_3: u16, param_4: i32) {
    // SAFETY: param_4 is an address of the game's own structure; the globals
    // and callees are the game's own, accessed in the original's order.
    unsafe {
        let off = (param_4 as *const i32).byte_add(8).read_unaligned();
        let p = param_4.wrapping_add(off) as usize as *mut i32;
        let s_var1: i16;
        if DAT_0052361c == 1 {
            s_var1 = p.read_unaligned() as i16;
        } else {
            s_var1 = (p.add(2).read_unaligned() as i16).wrapping_neg();
        }
        let s_var2: i16 = (p.add(1).read_unaligned() as i16).wrapping_neg();
        DAT_009a8d4a = s_var1;
        DAT_009a8d4e = s_var2;
        FUN_004219d0(param_3, DAT_009fe7d4, &raw mut DAT_009ec610);
        FUN_00421ae0(DAT_009fe7d4, &raw mut DAT_009ec610);
        DAT_009a8d42 = DAT_009a7b48.wrapping_add(param_1);
        let mut local_20: i32 = DAT_009a8d42 as i32;
        DAT_009a8d48 = DAT_009a7b40.wrapping_add(param_2);
        let base = param_4 as usize as *const u8;
        FUN_00422170(
            p.add(4).cast::<u8>(),
            base.byte_add(0x20).cast::<i32>().read_unaligned(),
        );
        let v10 = base.byte_add(0x10).cast::<i32>().read_unaligned() as i16;
        DAT_009a8d4a = v10.wrapping_add(s_var1);
        DAT_009a8d4e = s_var2;
        FUN_00421ae0(DAT_009fe7d4, &raw mut DAT_009ec610);
        let v14 = base.byte_add(0x14).cast::<i16>().read_unaligned();
        DAT_009a8d4a =
            (base.byte_add(0x10).cast::<i32>().read_unaligned() as i16).wrapping_add(s_var1);
        DAT_009a8d4e = s_var2.wrapping_sub(v14);
        FUN_00421ae0(DAT_009fe7d4, &raw mut DAT_009ec610);
        DAT_009a8d4e = s_var2.wrapping_sub(base.byte_add(0x14).cast::<i16>().read_unaligned());
        DAT_009a8d4a = s_var1;
        FUN_00421ae0(DAT_009fe7d4, &raw mut DAT_009ec610);
        FUN_00421700(&raw mut local_20);
    }
}
