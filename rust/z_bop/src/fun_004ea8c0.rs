// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052c53c: [i8; 1];
    static mut DAT_0051c4bc: u8;
    static mut DAT_00aa6110: *mut core::ffi::c_void;
    static mut DAT_00b69d20: [i8; 0x400];
    static mut DAT_00b69e1f: i8;
    static mut DAT_00b69d1e: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cc4a0(param_1: *mut i8, param_2: *mut i8) -> i32;
    fn FUN_004cc590(param_1: i32) -> u32;
    fn _malloc(size: u32) -> *mut core::ffi::c_void;
    fn _free(ptr: *mut core::ffi::c_void);
    fn _fgets(ptr: *mut i8, n: i32, stream: i32) -> *mut i8;
    fn _strncpy(dst: *mut i8, src: *const i8, n: u32) -> *mut i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ea8c0() -> u32 {
    // SAFETY: all globals and callees are the game's own; the buffer is the
    // one the original allocates and fills at the same offsets and order.
    unsafe {
        let file = FUN_004cc4a0(
            (&raw mut DAT_0052c53c).cast::<i8>(),
            (&raw mut DAT_0051c4bc).cast::<i8>(),
        );
        if file == 0 {
            return 0;
        }
        if !DAT_00aa6110.is_null() {
            _free(DAT_00aa6110);
            DAT_00aa6110 = core::ptr::null_mut();
        }
        let mut p = _malloc(0xba00).cast::<u32>();
        DAT_00aa6110 = p.cast();
        let mut n: i32 = 0x2e80;
        while n != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            n -= 1;
        }
        if DAT_00aa6110.is_null() {
            FUN_004cc590(file);
            return 0;
        }
        let buf = (&raw mut DAT_00b69d20).cast::<i8>();
        let mut off: i32 = 0;
        loop {
            let mut idx: i32 = 0;
            while idx < 5 {
                loop {
                    let r = _fgets(buf, 0x400, file);
                    if r.is_null() {
                        FUN_004cc590(file);
                        return 1;
                    }
                    DAT_00b69e1f = 0;
                    if !(buf.read() == b'/' as i8 && buf.add(1).read() == b'/' as i8) {
                        break;
                    }
                }
                let mut len: u32 = 0;
                while buf.add(len as usize).read() != 0 {
                    len = len.wrapping_add(1);
                }
                let last = (&raw mut DAT_00b69d1e).add(len.wrapping_add(1) as usize);
                if last.read() == b'\n' as i8 {
                    last.write(0);
                }
                let base = DAT_00aa6110
                    .cast::<u8>()
                    .byte_add(off as usize)
                    .cast::<i8>();
                match idx {
                    0 => {
                        _strncpy(base, buf, 0x40);
                    }
                    1 => {
                        _strncpy(base.byte_add(0x40), buf, 0x40);
                    }
                    2 => {
                        _strncpy(base.byte_add(0x80), buf, 0x40);
                    }
                    3 => {
                        _strncpy(base.byte_add(0xc0), buf, 0x100);
                    }
                    4 => {
                        _strncpy(base.byte_add(0x1c0), buf, 0x40);
                    }
                    _ => {}
                }
                idx += 1;
            }
            off = off.wrapping_add(0x200);
            if 0xb9ff < off {
                return FUN_004cc590(file);
            }
        }
    }
}
