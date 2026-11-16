// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007329c4: i32;
    static mut DAT_007329c8: i32;
    static mut DAT_00732a14: i32;
    static mut DAT_00732580: i32;
    static mut DAT_00732560: i32;
    static mut DAT_00732564: i32;
    static mut DAT_00732550: i32;
    static mut DAT_00732540: i32;
    static mut DAT_00732544: i32;
    static mut DAT_0073255c: i32;
    static mut DAT_00732584: i32;
    static mut DAT_0073253c: i32;
    static mut DAT_00732558: i32;
    static mut DAT_00732578: i32;
    static mut DAT_00732538: i32;
    static mut DAT_00732568: i32;
    static mut DAT_00732530: i32;
    static mut DAT_00732548: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00483410(param_1: i16, param_2: i16, param_3: i32);
    fn FUN_00483540(param_1: i16, param_2: i16, param_3: i32);
    fn FUN_00483cf0(param_1: i32, param_2: i32, param_3: i32, param_4: i16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00483180(
    param_1: i16,
    param_2: i16,
    param_3: i16,
    param_4: i16,
    param_5: i16,
    param_6: i16,
    param_7: i32,
) {
    // SAFETY: the callees and globals are the game's own memory, accessed
    // in the same order as the original.
    unsafe {
        FUN_00483410(param_2, param_3, param_7);
        FUN_00483540(param_1, param_4, param_7);
        DAT_007329c4 = DAT_007329c4.wrapping_neg();
        DAT_007329c8 = DAT_007329c8.wrapping_neg();
        DAT_00732a14 = DAT_00732a14.wrapping_neg();
        let i_var2 = DAT_00732580.wrapping_neg();
        let i_var3 = DAT_00732560.wrapping_neg();
        let i_var1 = DAT_00732564.wrapping_neg();
        DAT_00732560 = i_var3;
        DAT_00732564 = i_var1;
        DAT_00732580 = i_var2;
        FUN_00483cf0(DAT_00732550, DAT_00732540, DAT_00732544, param_5);
        FUN_00483cf0(i_var2, i_var3, i_var1, param_6);
        DAT_0073255c = DAT_00732550;
        DAT_00732584 = DAT_00732540;
        DAT_0073253c = DAT_00732544;
        DAT_00732558 = DAT_00732580;
        DAT_00732578 = DAT_00732560;
        DAT_00732538 = DAT_00732564;
        DAT_00732568 = DAT_007329c4;
        DAT_00732530 = DAT_007329c8;
        DAT_00732548 = DAT_00732a14;
    }
}
