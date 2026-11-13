// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1834: u16;
    static mut DAT_009a20c0: [u8; 0];
    static mut DAT_009d8c82: [u8; 0];
    static mut DAT_00524170: [u8; 0];
    static mut DAT_00521f2e: [u8; 0];
    static mut DAT_009d8c91: [u8; 0];
    static mut DAT_009a2062: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00456190(param_1: u16) -> u64;
    fn FUN_004587c0(param_1: u16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00455600(param_1: u16) -> u32 {
    // SAFETY: the globals and callees are the game's own memory and code;
    // indices are computed exactly as the original does.
    unsafe {
        let u_var3: u32 = DAT_009a1834 as u32;
        let i_var1: i32 = u_var3.wrapping_mul(0x126) as i32;
        let a20c0 = (&raw mut DAT_009a20c0).cast::<u8>();
        let d8c82 = (&raw mut DAT_009d8c82).cast::<u8>();
        let d4170 = (&raw mut DAT_00524170).cast::<u8>();
        let d1f2e = (&raw mut DAT_00521f2e).cast::<u8>();
        let d8c91 = (&raw mut DAT_009d8c91).cast::<u8>();

        *a20c0.offset(i_var1 as isize) = 1;
        let i_var2: i32 = u_var3.wrapping_mul(0x566) as i32;
        let t = *d8c82.offset(i_var2 as isize);
        let k = *d4170.add(t as usize);
        let idx = (k as i32).wrapping_mul(0x1a);
        if (*d1f2e.offset(idx as isize) & 0x80) != 0 {
            *a20c0.offset(i_var1.wrapping_add(1) as isize) = *d8c91.offset(i_var2 as isize);
            let u_var4 = FUN_00456190(param_1);
            return (((u_var4 >> 0x10) as u32) << 0x10) | 1;
        }
        if DAT_009a2062 == 0x02 {
            *a20c0.offset(i_var1.wrapping_add(1) as isize) = *d8c91.offset(i_var2 as isize);
        } else {
            *a20c0.offset(i_var1.wrapping_add(1) as isize) = 0;
        }
        FUN_004587c0(param_1);
        1
    }
}
