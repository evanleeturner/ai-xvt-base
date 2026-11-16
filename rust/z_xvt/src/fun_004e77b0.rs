// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081b740: [i8; 256];
    static mut DAT_0081b73e: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _fgets(param_1: *mut i8, param_2: u32, param_3: i32) -> *mut i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004e77b0(param_1: i32) -> i32 {
    let mut local_84: i32 = 0;
    let mut local_80: [i8; 128] = [0; 128];
    // SAFETY: the globals and fgets are the game's own; the buffer is the
    // game's line buffer, read and written where the original does.
    unsafe {
        let buf = &raw mut DAT_0081b740 as *mut i8;
        loop {
            loop {
                loop {
                    let r = _fgets(buf, 0x100, param_1);
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
                let r = _fgets(buf, 0x100, param_1);
                if r.is_null() {
                    return local_84;
                }
                let r = _fgets(buf, 0x100, param_1);
                if r.is_null() {
                    return local_84;
                }
                local_84 = local_84.wrapping_add(1);
            }
            // strlen(buf) + 1
            let mut n: u32 = 0;
            while buf.add(n as usize).read() != 0 {
                n = n.wrapping_add(1);
            }
            n = n.wrapping_add(1);
            (&raw mut DAT_0081b73e).offset(n as isize).write(0);

            // copy string from buf+1, including terminator
            let src0 = buf.add(1);
            let mut len: u32 = 0;
            while src0.add(len as usize).read() != 0 {
                len = len.wrapping_add(1);
            }
            len = len.wrapping_add(1);
            let mut src = src0;
            let mut dst = local_80.as_mut_ptr();
            let mut c = len >> 2;
            while c != 0 {
                (dst as *mut u32).write_unaligned((src as *const u32).read_unaligned());
                src = src.add(4);
                dst = dst.add(4);
                c -= 1;
            }
            let mut c = len & 3;
            while c != 0 {
                dst.write(src.read());
                src = src.add(1);
                dst = dst.add(1);
                c -= 1;
            }
        }
    }
}
