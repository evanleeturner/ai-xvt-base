// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081b848: i32;
    static mut DAT_0081b6f4: i32;
    static mut DAT_0081bcb0: i32;
    static mut DAT_0081bcd0: i32;
    static mut DAT_0081bcf0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dd000(param_1: *mut i32, param_2: *mut i32) -> i32;
    fn FUN_004ddc70(param_1: *mut i32, param_2: *mut i32) -> i32;
    fn FUN_004dd7d0(param_1: *mut i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_004dd130(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32) -> i32;
    fn FUN_004db570() -> i32;
    fn FUN_004db590() -> i32;
    fn FUN_004d5180(param_1: *mut i32) -> i32;
    fn FUN_004d51b0(param_1: *mut i32) -> i32;
    fn __strcmpi(param_1: *const i8, param_2: *const i8) -> i32;
    fn FUN_004daac0(
        param_1: i32,
        param_2: *mut i8,
        param_3: *mut i32,
        param_4: i32,
        param_5: i32,
        param_6: u32,
    ) -> i32;
    fn FUN_004dd030(param_1: *mut i32, param_2: i32, param_3: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004bff10(param_1: *mut i32, param_2: i32) -> i32 {
    // Stack frame laid out as the original: index 0 = iStack_2c, 1 = iStack_28,
    // 2 = padding, 3 = iStack_20, 4 = iStack_1c, 5 = iStack_18, 6 = iStack_14,
    // 7..11 = auStack_10.
    let mut buf: [i32; 11] = [0; 11];
    let p = buf.as_mut_ptr();

    // SAFETY: the globals and callees are the game's own; the pointers passed
    // point into the local frame array or the game's string table, as the
    // original does.
    unsafe {
        if DAT_0081b848 == 0 {
            return 0;
        }
        FUN_004dd000(p, param_1);
        FUN_004ddc70(p.add(6), p.add(5));
        buf[3] = buf[1].wrapping_add(0xe);
        let mut i_var4: i32 = 0;
        buf[4] = ((DAT_0081b848 as u32).wrapping_add(4) as *const i32).read_unaligned();
        if DAT_0081b6f4 != 0 && param_2 < param_2.wrapping_add(0xd) {
            let mut i_var5: i32 = param_2.wrapping_mul(0xe);
            let mut i_var6: i32 = param_2;
            loop {
                if buf[4] <= i_var6 {
                    return i_var4;
                }
                let i_var2 = FUN_004dd7d0(p, buf[6], buf[5]);
                if i_var2 != 0 {
                    FUN_004dd130(p, 0, 0, DAT_0081bcb0);
                    if FUN_004db570() != 0 || FUN_004db590() != 0 {
                        i_var4 = i_var6.wrapping_add(1);
                    }
                }
                FUN_004d5180(p.add(7));
                FUN_004d51b0(p);
                let i_var2 = __strcmpi(
                    DAT_0081b6f4.wrapping_add(i_var5) as u32 as *const i8,
                    (&raw mut DAT_0081bcf0) as *const i8,
                );
                let mut u_var7 = DAT_0081bcd0 as u32;
                if i_var2 != 0 {
                    u_var7 = 0xffff;
                }
                i_var6 = i_var6.wrapping_add(1);
                let pc_var3 = DAT_0081b6f4.wrapping_add(i_var5) as u32 as *mut i8;
                i_var5 = i_var5.wrapping_add(0xe);
                FUN_004daac0(0xc, pc_var3, p, 0, 1, u_var7);
                FUN_004d51b0(p.add(7));
                FUN_004dd030(p, 0, 0xf);
                if !(i_var6 < param_2.wrapping_add(0xd)) {
                    break;
                }
            }
        }
        i_var4
    }
}
