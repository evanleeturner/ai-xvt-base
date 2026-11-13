// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: u32;
    static mut DAT_009d8c87: u32;
    static mut DAT_009d8c88: u32;
    static mut DAT_009d8c30: u32;
    static mut DAT_0051c560: u32;
    static mut DAT_009e967a: u32;
    static mut DAT_009e967c: u32;
    static mut DAT_009ecc34: u32;
    static mut DAT_00a08292: u16;
    static mut DAT_00a07be2: i16;
    static mut DAT_00a07be4: i16;
    static mut DAT_00a07be6: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00451bf0(param_1: i32, param_2: i32);
    fn FUN_0043c740(param_1: i32, param_2: *mut u32) -> u32;
    fn FUN_00450650(param_1: i32, param_2: i32);
    fn FUN_0042fb70(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00451d00(
    param_1: u32,
    param_2: *mut u32,
    param_3: i16,
    param_4: u32,
    param_5: i16,
    param_6: u32,
) {
    let _ = param_6;
    // SAFETY: the globals and callees are the game's own memory; the
    // addresses computed are offsets into the game's tables as in the original,
    // and param_2 points at the caller's record.
    unsafe {
        let u_var3 = param_1 & 0xffff;
        let u_var2 = ((&raw mut DAT_009a1fe8 as usize)
            .wrapping_add(u_var3.wrapping_mul(0x23).wrapping_add(0x16) as usize)
            as *const u8)
            .read() as u32;
        let i_var4 = u_var2.wrapping_mul(0x566) as i32;
        DAT_00a08292 = ((&raw mut DAT_009d8c87 as usize).wrapping_add(i_var4 as usize) as *const u8)
            .read() as u16;
        let idx = DAT_009ecc34.wrapping_mul(0x5bd) as usize;
        let a = ((&raw mut DAT_009e967a as usize).wrapping_add(idx) as *const u16).read_unaligned();
        if a == DAT_00a08292 {
            let b = ((&raw mut DAT_009d8c88 as usize).wrapping_add(i_var4 as usize) as *const u8)
                .read() as u16;
            let idx2 = DAT_009ecc34.wrapping_mul(0x5bd) as usize;
            let c = ((&raw mut DAT_009e967c as usize).wrapping_add(idx2) as *const u16)
                .read_unaligned();
            if b == c {
                let code: i32;
                if param_5 == 0 {
                    let sel = param_2.add(1).cast::<u8>().read() as i32;
                    FUN_00451bf0(
                        0,
                        (&raw mut DAT_0051c560 as i32).wrapping_add(sel.wrapping_mul(0x101)),
                    );
                    FUN_00451bf0(1, (&raw mut DAT_009d8c30 as i32).wrapping_add(i_var4));
                    let u_var1 = FUN_0043c740(u_var2 as i32, param_2);
                    DAT_00a07be4 = u_var1 as i16;
                    if DAT_00a07be4 == 0 {
                        DAT_00a07be4 = param_3;
                        code = 0x94;
                    } else {
                        DAT_00a07be6 = param_3;
                        code = 0x93;
                    }
                } else {
                    FUN_00451bf0(0, (&raw mut DAT_009d8c30 as i32).wrapping_add(i_var4));
                    DAT_00a07be2 = param_3;
                    code = 0x10d;
                }
                FUN_00450650(code, DAT_009ecc34 as i32);
                FUN_0042fb70(
                    DAT_009ecc34 as i32,
                    u_var3 as i32,
                    1,
                    (param_4 & 0xffff) as i32,
                    u_var3 as i32,
                    0xffff,
                );
            }
        }
    }
}
