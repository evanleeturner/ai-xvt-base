// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_005597a0: u32;
    static mut _DAT_005597a4: i32;
    static mut DAT_009994fc: i32;
    static mut DAT_00999550: u32;
    static mut DAT_005597a8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046d350(param_1: i32, param_2: *mut u32, param_3: i32);
    fn FUN_0046f5e0(param_1: *mut i32, param_2: *mut u32, param_3: i32) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0046f840(param_1: i32) -> u32 {
    let mut local_4: i32 = 0;
    let mut local_8: u32 = 0;
    // SAFETY: the globals and callees are the game's own memory and code;
    // the pointers passed are to live locals or to the game's arrays.
    unsafe {
        _DAT_005597a0 = 0xe;
        _DAT_005597a4 = DAT_009994fc;
        FUN_0046d350(param_1, &raw mut _DAT_005597a0, 8);
        if param_1 == 0 {
            loop {
                let p = FUN_0046f5e0(&raw mut local_4, &raw mut local_8, 0x3c);
                if p.is_null() {
                    return 0;
                }
                if p.read_unaligned() == 0xe {
                    break;
                }
            }
        }
        let mut i_var3: i32 = 0;
        if 0 < DAT_009994fc {
            let mut local_c: *mut u32 = &raw mut DAT_00999550;
            loop {
                _DAT_005597a0 = 0xf;
                let mut src = local_c;
                let mut dst: *mut u32 = &raw mut DAT_005597a8;
                _DAT_005597a4 = i_var3;
                let mut n = 10;
                while n != 0 {
                    dst.write_unaligned(src.read_unaligned());
                    src = src.add(1);
                    dst = dst.add(1);
                    n -= 1;
                }
                FUN_0046d350(param_1, &raw mut _DAT_005597a0, 0x30);
                if param_1 == 0 {
                    loop {
                        let p = FUN_0046f5e0(&raw mut local_4, &raw mut local_8, 0x3c);
                        if p.is_null() {
                            return 0;
                        }
                        if p.read_unaligned() == 0xf {
                            break;
                        }
                    }
                }
                i_var3 = i_var3.wrapping_add(1);
                local_c = local_c.add(10);
                if !(i_var3 < DAT_009994fc) {
                    break;
                }
            }
        }
        1
    }
}
