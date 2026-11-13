// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009ecc34: i32;
    static mut DAT_00a08610: [i8; 0];
    static mut DAT_009e9a77: [i8; 0];
    static mut DAT_009e9a78: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a9b60();
    fn FUN_004aba50();
    fn FUN_00440da0(param_1: u32, param_2: i32);
    fn FUN_004abe50();
    fn FUN_004aba40() -> i32;
    fn FUN_004ac260();
    fn FUN_004abee0();
    fn FUN_004a9b70();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00427720(param_1: u32, param_2: i32) -> i32 {
    // SAFETY: the globals and callees are the game's own; the indexed
    // addresses are computed exactly as the original does.
    unsafe {
        let mut i_var3 = DAT_009ecc34;
        if param_2 == i_var3
            && *(&raw mut DAT_00a08610 as *mut i8).byte_add(param_1.wrapping_mul(0x24) as usize)
                == 0
        {
            return 0;
        }
        let i_var1 = param_2.wrapping_mul(0x5bd);
        let p77 = (&raw mut DAT_009e9a77 as *mut u8).offset(i_var1 as isize);
        if (p77.read() as u32) != param_1 {
            p77.write(param_1 as u8);
            if param_2 == i_var3 {
                FUN_004a9b60();
                FUN_004aba50();
                FUN_00440da0(param_1, param_2);
                FUN_004abe50();
                let mut i_var2 = FUN_004aba40();
                i_var3 = i_var2;
                if 0 < i_var2 {
                    loop {
                        FUN_004abe50();
                        i_var3 = i_var3.wrapping_sub(1);
                        if i_var3 == 0 {
                            break;
                        }
                    }
                }
                FUN_004ac260();
                FUN_004abee0();
                FUN_004ac260();
                if 0 < i_var2 {
                    loop {
                        FUN_004aba50();
                        i_var2 = i_var2.wrapping_sub(1);
                        if i_var2 == 0 {
                            break;
                        }
                    }
                }
                FUN_004a9b70();
            }
            (&raw mut DAT_009e9a78 as *mut u8)
                .offset(i_var1 as isize)
                .write(param_1 as u8);
            return 1;
        }
        1
    }
}
