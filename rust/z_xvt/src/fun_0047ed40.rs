// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0070ab20: [i8; 0];
    static mut DAT_005281c4: u32;
    static mut DAT_00732948: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _fopen(param_1: *const i8, param_2: *const i8) -> *mut core::ffi::c_void;
    fn FUN_004be6c0() -> i8;
    fn FUN_0047e680() -> i8;
    fn FUN_0047f010(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0047ed40(
    param_1: *mut i8,
    param_2: *mut i8,
    param_3: i32,
    param_4: i32,
) -> u32 {
    // SAFETY: the globals and callees are the game's own; param_1 and param_2
    // are the caller's NUL-terminated strings, and the game's buffer is
    // large enough for the copies, as in the original.
    unsafe {
        let buf = (&raw mut DAT_0070ab20) as *mut i8;

        // strlen(param_1) + 1
        let mut len: u32 = 0;
        loop {
            let c = param_1.add(len as usize).read();
            len = len.wrapping_add(1);
            if c == 0 {
                break;
            }
        }
        let mut i: u32 = 0;
        while i < len {
            buf.add(i as usize).write(param_1.add(i as usize).read());
            i += 1;
        }

        loop {
            if param_4 != 2 {
                let mut n = 4;
                while n != 0 {
                    DAT_00732948 = _fopen(param_1, param_2);
                    n -= 1;
                    if !DAT_00732948.is_null() {
                        return 1;
                    }
                }
            }
            if param_4 != 1 {
                buf.cast::<u32>()
                    .write_unaligned((&raw mut DAT_005281c4).read_unaligned());
                let c1 = FUN_004be6c0();
                buf.write(c1);

                let mut len2: u32 = 0;
                loop {
                    let c = param_1.add(len2 as usize).read();
                    len2 = len2.wrapping_add(1);
                    if c == 0 {
                        break;
                    }
                }
                let mut end: usize = 0;
                while buf.add(end).read() != 0 {
                    end += 1;
                }
                let mut j: u32 = 0;
                while j < len2 {
                    buf.add(end + j as usize)
                        .write(param_1.add(j as usize).read());
                    j += 1;
                }
                let mut n = 4;
                while n != 0 {
                    DAT_00732948 = _fopen(buf, param_2);
                    n -= 1;
                    if !DAT_00732948.is_null() {
                        return 1;
                    }
                }
            }
            if param_3 == 0 {
                DAT_00732948 = core::ptr::null_mut();
                return 0;
            }
            loop {
                let c = FUN_0047e680();
                if c == b'R' as i8 || c == b'r' as i8 {
                    break;
                }
                if c == b'F' as i8 || c == b'f' as i8 {
                    FUN_0047f010(1);
                    DAT_00732948 = core::ptr::null_mut();
                    return 0;
                }
            }
        }
    }
}
