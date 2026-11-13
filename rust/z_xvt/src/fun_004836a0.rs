// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0070a12c: i32;
    static mut DAT_0070a138: i32;
    static mut DAT_0070a134: i32;
    static mut DAT_0070a130: i32;
    static mut DAT_0070a010: i32;
    static mut DAT_0070a00c: i32;
    static mut DAT_0070a008: i32;
    static mut DAT_0070a124: i32;
    static mut DAT_0070a120: i32;

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
    static mut DAT_007329c4: i32;
    static mut DAT_007329c8: i32;
    static mut DAT_00732a14: i32;
    static mut DAT_00732580: i32;
    static mut DAT_00732560: i32;
    static mut DAT_00732564: i32;

    static mut DAT_0071e536: i8;

    static mut DAT_00703c6c: i32;
    static mut DAT_00703c70: i32;
    static mut DAT_00703c84: i32;

    static mut DAT_00732f30: i32;
    static mut DAT_00732f34: i32;
    static mut DAT_00732ea0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00413d00(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004836a0() {
    // SAFETY: all globals and the callee belong to the game; accessed in the original's order.
    unsafe {
        DAT_0070a12c = FUN_00413d00(
            DAT_00732550,
            DAT_00732540,
            DAT_00732544,
            DAT_0073255c,
            DAT_00732584,
            DAT_0073253c,
        );
        DAT_0070a138 = FUN_00413d00(
            DAT_00732550,
            DAT_00732540,
            DAT_00732544,
            DAT_00732558,
            DAT_00732578,
            DAT_00732538,
        );
        DAT_0070a134 = FUN_00413d00(
            DAT_00732550,
            DAT_00732540,
            DAT_00732544,
            DAT_00732568,
            DAT_00732530,
            DAT_00732548,
        );
        DAT_0070a130 = FUN_00413d00(
            DAT_007329c4,
            DAT_007329c8,
            DAT_00732a14,
            DAT_0073255c,
            DAT_00732584,
            DAT_0073253c,
        );
        DAT_0070a010 = FUN_00413d00(
            DAT_007329c4,
            DAT_007329c8,
            DAT_00732a14,
            DAT_00732558,
            DAT_00732578,
            DAT_00732538,
        );
        DAT_0070a00c = FUN_00413d00(
            DAT_007329c4,
            DAT_007329c8,
            DAT_00732a14,
            DAT_00732568,
            DAT_00732530,
            DAT_00732548,
        );
        DAT_0070a008 = FUN_00413d00(
            DAT_00732580,
            DAT_00732560,
            DAT_00732564,
            DAT_0073255c,
            DAT_00732584,
            DAT_0073253c,
        );
        DAT_0070a124 = FUN_00413d00(
            DAT_00732580,
            DAT_00732560,
            DAT_00732564,
            DAT_00732558,
            DAT_00732578,
            DAT_00732538,
        );
        DAT_0070a120 = FUN_00413d00(
            DAT_00732580,
            DAT_00732560,
            DAT_00732564,
            DAT_00732568,
            DAT_00732530,
            DAT_00732548,
        );
        if DAT_0071e536 != 0 {
            DAT_00732f30 = FUN_00413d00(
                DAT_00732550,
                DAT_00732540,
                DAT_00732544,
                DAT_00703c6c,
                DAT_00703c70,
                DAT_00703c84,
            );
            DAT_00732f34 = FUN_00413d00(
                DAT_007329c4,
                DAT_007329c8,
                DAT_00732a14,
                DAT_00703c6c,
                DAT_00703c70,
                DAT_00703c84,
            );
            DAT_00732ea0 = FUN_00413d00(
                DAT_00732580,
                DAT_00732560,
                DAT_00732564,
                DAT_00703c6c,
                DAT_00703c70,
                DAT_00703c84,
            );
            return;
        }
        DAT_00732f30 = DAT_00703c6c;
        DAT_00732f34 = DAT_00703c70;
        DAT_00732ea0 = DAT_00703c84;
    }
}
