// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00757ccc: u32;
    static mut DAT_00757cc0: u32;
    static mut DAT_0081bfba: [u8; 4];
    static mut DAT_0081bfbe: [u8; 0];
    static mut DAT_0081b740: [i8; 0];
    static mut PTR_s_train_005330e0: [*mut i8; 0];
    static mut s__s__s_005331e4: [i8; 0];
    static mut DAT_00523570: [i8; 0];
    static mut DAT_00534aac: u32;
    static mut PTR_DAT_005348a0: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004be160(a: *mut i8, b: *mut i8) -> *mut c_void;
    fn FUN_004be220(f: *mut c_void, a: i32, b: i32) -> i32;
    fn FUN_004be320(f: *mut c_void, p: *mut u8, n: i32) -> i32;
    fn FUN_004be1f0(f: *mut c_void) -> i32;
    fn _fgets(buf: *mut i8, n: i32, f: *mut c_void) -> *mut i8;
    fn _atoi(s: *const i8) -> i32;
    fn __isctype(c: u32, m: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004e4820(param_1: *mut u8) {
    if param_1.is_null() {
        return;
    }
    // SAFETY: the globals and callees are the game's own; param_1 points at
    // a caller buffer of at least 0x400 bytes, handled as the original does.
    unsafe {
        let buf = (&raw mut DAT_0081b740).cast::<i8>();
        let mut i = 0usize;
        while i < 0x400 {
            param_1.add(i).write(0);
            i += 1;
        }
        let mut u_var6: u32 = 0;
        if DAT_00757ccc == 0 {
            return;
        }
        let mut pi_var8 = DAT_00757cc0.wrapping_add(0x140) as *const i32;
        loop {
            let i_train = (&raw const DAT_0081bfba).cast::<u32>().read_unaligned();
            let i_val = (&raw const DAT_0081bfbe)
                .cast::<u8>()
                .byte_add((i_train.wrapping_mul(4)) as usize)
                .cast::<i32>()
                .read_unaligned();
            if pi_var8.read_unaligned() == i_val {
                break;
            }
            pi_var8 = pi_var8.byte_add(0x144);
            u_var6 = u_var6.wrapping_add(1);
            if !(u_var6 < DAT_00757ccc) {
                break;
            }
        }
        if !(u_var6 < DAT_00757ccc) {
            return;
        }
        let i_train = (&raw const DAT_0081bfba).cast::<u32>().read_unaligned() as i32;
        let name = (&raw const PTR_s_train_005330e0)
            .cast::<*mut i8>()
            .offset(i_train as isize)
            .read();
        _sprintf(
            buf,
            (&raw const s__s__s_005331e4).cast::<i8>(),
            name,
            u_var6.wrapping_mul(0x144).wrapping_add(DAT_00757cc0) as *mut i8,
        );
        let file = FUN_004be160(buf, (&raw mut DAT_00523570).cast::<i8>());
        if file.is_null() {
            return;
        }
        let i_train = (&raw const DAT_0081bfba).cast::<u32>().read_unaligned() as i32;
        if i_train == 2 || i_train == 4 {
            _fgets(buf, 0x100, file);
            let mut i_var5 = _atoi(buf);
            while i_var5 != 0 {
                _fgets(buf, 0x100, file);
                i_var5 = i_var5.wrapping_sub(1);
            }
            let mut u_var6: u32 = 0;
            let mut pb = param_1;
            'outer: loop {
                let r = _fgets(buf, 0x100, file);
                if r.is_null() {
                    break;
                }
                let mut len: u32 = 0;
                while buf.add(len as usize).read() != 0 {
                    len = len.wrapping_add(1);
                }
                let mut u_var10: u32 = 0;
                if len != 0 {
                    loop {
                        let b2 = buf.add(u_var10 as usize).cast::<u8>().read();
                        let c = b2 as u32;
                        let u_var4: u32 = if DAT_00534aac < 2 {
                            (PTR_DAT_005348a0
                                .byte_add((c * 2) as usize)
                                .cast::<u16>()
                                .read_unaligned() as u32)
                                & 0x157
                        } else {
                            __isctype(c, 0x157) as u32
                        };
                        if u_var4 != 0 || c == 10 {
                            pb.write(b2);
                            pb = pb.add(1);
                            u_var6 = u_var6.wrapping_add(1);
                            if 0x3fe < u_var6 {
                                break 'outer;
                            }
                        }
                        u_var10 = u_var10.wrapping_add(1);
                        if !(u_var10 < len) {
                            break;
                        }
                    }
                }
                if 0x3fe < u_var6 {
                    break;
                }
            }
            param_1.add(u_var6 as usize).write(0);
        } else {
            FUN_004be220(file, -0x400, 2);
            FUN_004be320(file, param_1, 0x400);
            param_1.add(0x3ff).write(0);
        }
        FUN_004be1f0(file);
    }
}
