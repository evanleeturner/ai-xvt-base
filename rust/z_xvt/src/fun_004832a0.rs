// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00703c68: i32;
    static mut DAT_00703be0: i32;
    static mut DAT_00703adc: i32;
    static mut DAT_006db700: i32;
    static mut DAT_006db6e0: i32;
    static mut DAT_006db70c: i32;
    static mut DAT_006db7a0: i32;
    static mut DAT_006db79c: i32;
    static mut DAT_006db7a8: i32;
    static mut DAT_007329c4: i32;
    static mut DAT_007329c8: i32;
    static mut DAT_00732a14: i32;
    static mut DAT_00732540: i32;
    static mut DAT_00732544: i32;
    static mut DAT_00732550: i32;
    static mut DAT_00732560: i32;
    static mut DAT_00732564: i32;
    static mut DAT_00732580: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00483410(a: i16, b: i16, c: i32);
    fn FUN_00483540(a: i16, b: i16, c: i32);
    fn FUN_004836a0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004832a0(
    param_1: i16,
    param_2: i16,
    param_3: i16,
    param_4: i16,
    param_5: i32,
) {
    // SAFETY: the callees and globals are the game's own; param_5 and the
    // pointer read from it address the game's structures, read where the original does.
    unsafe {
        if param_5 == 0 {
            FUN_00483410(param_2, param_3, 0);
            FUN_00483540(param_1, param_4, 0);
            FUN_004836a0();
            return;
        }

        let base_ptr = ((param_5.wrapping_add(0x1f)) as *const i32).read_unaligned();
        let b = base_ptr as u32;

        let c_90 = (b.wrapping_add(0x90) as *const i8).read();
        if c_90 != 0 {
            FUN_00483410(param_2, param_3, param_5);
            FUN_00483540(param_1, param_4, param_5);
            FUN_004836a0();
            return;
        }

        let rd = |off: u32| -> i32 { (b.wrapping_add(off) as *const i16).read_unaligned() as i32 };
        let s_91 = rd(0x91);
        let s_93 = rd(0x93);
        let s_95 = rd(0x95);
        let s_97 = rd(0x97);
        let s_99 = rd(0x99);
        let s_9b = rd(0x9b);
        let s_9d = rd(0x9d);
        let s_9f = rd(0x9f);
        let s_a1 = rd(0xa1);

        DAT_00703c68 = s_91;
        DAT_00703be0 = s_93;
        DAT_00703adc = s_95;
        DAT_006db700 = s_97;
        DAT_006db6e0 = s_99;
        DAT_006db70c = s_9b;
        DAT_006db7a0 = s_9d;
        DAT_006db79c = s_9f;
        DAT_006db7a8 = s_a1;

        DAT_007329c4 = DAT_00703c68.wrapping_neg();
        DAT_007329c8 = DAT_00703be0.wrapping_neg();
        DAT_00732a14 = DAT_00703adc.wrapping_neg();
        DAT_00732540 = DAT_006db6e0;
        DAT_00732544 = DAT_006db70c;
        DAT_00732550 = DAT_006db700;
        DAT_00732560 = DAT_006db79c;
        DAT_00732564 = DAT_006db7a8;
        DAT_00732580 = DAT_006db7a0;

        FUN_004836a0();
    }
}
