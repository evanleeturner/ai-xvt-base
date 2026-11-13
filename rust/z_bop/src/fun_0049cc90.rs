// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut in_EAX: u32;
    static mut DAT_00999408: u16;
    static mut DAT_009a8da0: u32;
    static mut DAT_00999402: u16;
    static mut DAT_00a07bd0: u32;
    static mut DAT_00999400: u16;
    static mut DAT_009a8e18: u32;
    static mut DAT_00999404: u16;
    static mut DAT_009d6820: u32;
    static mut DAT_0099940a: u16;
    static mut DAT_009a1fe8: u32;
    static mut DAT_00612b94: u16;
    static mut DAT_009ec5f8: u32;
    static mut DAT_00622ba0: u16;
    static mut DAT_009a7800: u32;
    static mut DAT_00622ba8: u16;
    static mut DAT_00a07cc0: u32;
    static mut DAT_00622bac: u16;
    static mut DAT_009e965c: u32;
    static mut DAT_009d80c8: u8;
    static mut DAT_009d7674: u32;
    static mut DAT_00612b98: u16;
    static mut DAT_009a8c34: u32;
    static mut DAT_00622ba4: u16;
    static mut DAT_009a806c: u32;
    static mut DAT_00999406: u16;
    static mut DAT_009a8074: u32;
    static mut _DAT_009d8c1c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac720(param_1: u32) -> u32;
    fn FUN_0045a0c0();
    fn FUN_004248b0(param_1: u32) -> u32;
    fn FUN_00426c50(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0049cc90() {
    // SAFETY: the globals and callees are the game's own memory and code;
    // they are read, written and called in the original's order.
    unsafe {
        let mut u_var1: u32 = (in_EAX & 0xFFFF0000) | u32::from(DAT_00999408);
        if DAT_00999408 != 0 {
            u_var1 = FUN_004ac720(u_var1);
            DAT_009a8da0 = u_var1;
        }
        u_var1 = (u_var1 & 0xFFFF0000) | u32::from(DAT_00999402);
        if DAT_00999402 != 0 {
            u_var1 = FUN_004ac720(u_var1);
            DAT_00a07bd0 = u_var1;
        }
        u_var1 = (u_var1 & 0xFFFF0000) | u32::from(DAT_00999400);
        if DAT_00999400 != 0 {
            u_var1 = FUN_004ac720(u_var1);
            DAT_009a8e18 = u_var1;
        }
        u_var1 = (u_var1 & 0xFFFF0000) | u32::from(DAT_00999404);
        if DAT_00999404 != 0 {
            u_var1 = FUN_004ac720(u_var1);
            DAT_009d6820 = u_var1;
        }
        if DAT_0099940a != 0 {
            DAT_009a1fe8 = FUN_004ac720((u_var1 & 0xFFFF0000) | u32::from(DAT_0099940a));
            FUN_0045a0c0();
        }
        let ret_004248b0: u32 = FUN_004248b0(0);
        DAT_009ec5f8 = FUN_004ac720((ret_004248b0 & 0xFFFF0000) | u32::from(DAT_00612b94));
        DAT_009a7800 = FUN_004ac720((DAT_009ec5f8 & 0xFFFF0000) | u32::from(DAT_00622ba0));
        DAT_00a07cc0 = FUN_004ac720((DAT_009a7800 & 0xFFFF0000) | u32::from(DAT_00622ba8));
        DAT_009e965c = FUN_004ac720((DAT_00a07cc0 & 0xFFFF0000) | u32::from(DAT_00622bac));
        let mut u_var2: u32 = (DAT_009e965c & 0xFFFFFF00) | u32::from(DAT_009d80c8);
        let mut u_var3: u32 = DAT_009a7800;
        let mut hit = DAT_009d80c8 == 1;
        if !hit {
            u_var3 = DAT_00a07cc0;
            hit = DAT_009d80c8 == 2;
        }
        if !hit {
            u_var3 = DAT_009e965c;
            hit = DAT_009d80c8 == 0;
        }
        if hit {
            u_var2 = u_var3;
            DAT_009d7674 = u_var3;
        }
        DAT_009a8c34 = FUN_004ac720((u_var2 & 0xFFFF0000) | u32::from(DAT_00612b98));
        DAT_009a806c = FUN_004ac720((DAT_009a8c34 & 0xFFFF0000) | u32::from(DAT_00622ba4));
        FUN_00426c50(DAT_009a8c34);
        DAT_009a8074 = FUN_004ac720((DAT_009a806c & 0xFFFF0000) | u32::from(DAT_00999406));
        _DAT_009d8c1c = DAT_009a8074;
    }
}
