// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009ecc52: i8;
    static mut DAT_00a0aa10: i32;
    static mut DAT_00a0fc80: i32;
    static mut DAT_009a20a4: i32;
    static mut DAT_00a0a870: i8;
    static mut DAT_00a0aa20: i8;
    static mut DAT_00a0a900: i8;
    static mut DAT_00a0aaa0: i8;
    static mut DAT_00a1050e: i8;
    static mut DAT_00a0a880: i8;
    static mut _DAT_00a1050c: u16;
    static mut DAT_00a0a910: u16;
    static mut DAT_00a0aa21: i8;
    static mut DAT_00a0aaa1: i8;
    static mut DAT_00a0a881: i8;
    static mut DAT_00a0fe80: u16;
    static mut DAT_00bb737d: i8;
    static mut DAT_00a0adf0: i8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_00430830();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a9300(param_1: i32, param_2: i32) -> u32;
    fn FUN_0042d230(param_1: *mut core::ffi::c_void, param_2: i32, param_3: i32, param_4: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00430700() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointer accesses mirror the original's indexing exactly.
    unsafe {
        if DAT_009ecc52 != 0 {
            FUN_00430830();
            if DAT_00a0aa10 == 0 || FUN_004a9300(DAT_00a0aa10, 0x100) == 0 {
                let i_var1 = DAT_00a0fc80;
                DAT_00a0aa10 = 0;
                if DAT_009a20a4 != 0 {
                    DAT_00a0a870 = DAT_00a0aa20;
                    DAT_009a20a4 = DAT_009a20a4.wrapping_sub(1);
                    DAT_00a0a900 = DAT_00a0aaa0;
                    DAT_00a1050e = DAT_00a0a880;
                    _DAT_00a1050c = DAT_00a0a910;
                    if DAT_009a20a4 != 0 {
                        let u_var4 = DAT_009a20a4 as u32;
                        let mut u_var2: u32 = 0;
                        let mut pu_var5 = &raw mut DAT_00a0fc80 as *mut u32;
                        let mut pu_var6 = &raw mut DAT_00a0a910 as *mut u16;
                        loop {
                            let u_var3 = u_var2.wrapping_add(1);
                            pu_var5.write_unaligned(pu_var5.add(1).read_unaligned());
                            let i = u_var2 as usize;
                            (&raw mut DAT_00a0aa20 as *mut i8)
                                .add(i)
                                .write((&raw mut DAT_00a0aa21 as *mut i8).add(i).read());
                            (&raw mut DAT_00a0aaa0 as *mut i8)
                                .add(i)
                                .write((&raw mut DAT_00a0aaa1 as *mut i8).add(i).read());
                            (&raw mut DAT_00a0a880 as *mut i8)
                                .add(i)
                                .write((&raw mut DAT_00a0a881 as *mut i8).add(i).read());
                            pu_var6.write_unaligned(pu_var6.add(1).read_unaligned());
                            u_var2 = u_var3;
                            pu_var5 = pu_var5.add(1);
                            pu_var6 = pu_var6.add(1);
                            if !(u_var3 < u_var4) {
                                break;
                            }
                        }
                    }
                    let v = (&raw mut DAT_00a0fe80 as *mut u16)
                        .offset(i_var1 as isize)
                        .read_unaligned();
                    if v != 0 && DAT_00bb737d != 0 {
                        FUN_0042d230(
                            (&raw mut DAT_00a0adf0 as *mut u8)
                                .offset(i_var1.wrapping_mul(6) as isize)
                                as *mut core::ffi::c_void,
                            1,
                            0,
                            0x7e,
                        );
                        DAT_00a0aa10 = i_var1;
                    }
                }
            }
        }
    }
}
