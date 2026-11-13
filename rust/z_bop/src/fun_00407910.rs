// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a8e18: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009e96b6: i8;
    static mut DAT_009e96b9: i8;
    static mut DAT_009e96b7: i8;
    static mut DAT_009e9c1d: i8;
    static mut DAT_009e9670: i8;
    static mut DAT_009e9640: i32;
    static mut DAT_009ecc34: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00450650(param_1: u32, param_2: i32);
    fn FUN_0042fb70(
        param_1: i32,
        param_2: u32,
        param_3: i32,
        param_4: i32,
        param_5: u32,
        param_6: u32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00407910(param_1: u16) {
    // SAFETY: the globals and callees are the game's own memory; the pointers
    // are computed from the game's tables exactly as the original does.
    unsafe {
        let a = DAT_009a8e18
            .wrapping_add(4)
            .wrapping_add((param_1 as u32).wrapping_mul(10) as i32);
        let idx = (a as usize as *const u16).read_unaligned() as u32;
        let b = idx
            .wrapping_mul(0x23)
            .wrapping_add(0x1b)
            .wrapping_add(DAT_009a1fe8 as u32);
        let i_var1 = (b as usize as *const i32).read_unaligned();
        if i_var1 != -1 {
            let i_var2 = i_var1.wrapping_mul(0x5bd) as isize;
            let flag = (&raw mut DAT_009e96b6).cast::<u8>().offset(i_var2);
            if flag.read() == 0 {
                flag.write(1);
                (&raw mut DAT_009e96b9)
                    .cast::<u8>()
                    .offset(i_var2)
                    .cast::<u16>()
                    .write_unaligned(0xffff);
                let i_var3 = DAT_009ecc34;
                (&raw mut DAT_009e96b7)
                    .cast::<u8>()
                    .offset(i_var2)
                    .cast::<u16>()
                    .write_unaligned((DAT_009e9640 as u16).wrapping_add(param_1));
                (&raw mut DAT_009e9c1d)
                    .cast::<u8>()
                    .offset(i_var2)
                    .cast::<i32>()
                    .write_unaligned(0x588);
                if i_var1 == i_var3 {
                    FUN_00450650(0x74, i_var1);
                    let v = (&raw mut DAT_009e9670)
                        .cast::<u8>()
                        .offset(i_var2)
                        .cast::<u32>()
                        .read_unaligned();
                    FUN_0042fb70(DAT_009ecc34, 0xffffffff, 0xc, -1, v, 0xffff);
                }
            }
        }
    }
}
