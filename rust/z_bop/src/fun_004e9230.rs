// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69d20: [i8; 256];
    static mut DAT_00b69d1e: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    #[link_name = "_fgets"]
    fn fgets(buf: *mut i8, n: i32, f: *mut core::ffi::c_void) -> *mut i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004e9230(param_1: *mut core::ffi::c_void) -> i32 {
    let mut local_84: i32 = 0;
    let mut local_80: [i8; 128] = [0; 128];
    // SAFETY: the buffer and functions are the game's own memory; the copy
    // writes where the original does, into the local buffer.
    unsafe {
        let buf = &raw mut DAT_00b69d20 as *mut i8;
        loop {
            loop {
                loop {
                    let r = fgets(buf, 0x100, param_1);
                    if r.is_null() {
                        return local_84;
                    }
                    if !(buf.read() == b'/' as i8 && buf.add(1).read() == b'/' as i8) {
                        break;
                    }
                }
                if buf.read() == b'[' as i8 {
                    break;
                }
                let r = fgets(buf, 0x100, param_1);
                if r.is_null() {
                    return local_84;
                }
                let r = fgets(buf, 0x100, param_1);
                if r.is_null() {
                    return local_84;
                }
                local_84 = local_84.wrapping_add(1);
            }
            let mut len: usize = 0;
            while buf.add(len).read() != 0 {
                len += 1;
            }
            (&raw mut DAT_00b69d1e).add(len.wrapping_add(1)).write(0);
            let src = buf.add(1);
            let mut n: usize = 0;
            while src.add(n).read() != 0 {
                n += 1;
            }
            n += 1;
            let dst = local_80.as_mut_ptr();
            let mut i = 0;
            while i < n {
                dst.add(i).write(src.add(i).read());
                i += 1;
            }
        }
    }
}
