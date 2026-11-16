// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa6138: u32;
    static mut DAT_00aa6114: u32;
    static mut DAT_00b6a5aa: i32;
    static mut DAT_00b6a5ae: i32;
    static mut DAT_00b69d20: [i8; 0x100];
    static mut PTR_s_train_0052c1e8: [*mut i8; 8];
    static mut s__s__s_0052c300: [i8; 8];
    static mut DAT_0051a854: i8;
    static mut DAT_0052dc84: i32;
    static mut PTR_DAT_0052da78: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cc4a0(a: *mut i8, b: *mut i8) -> *mut core::ffi::c_void;
    fn FUN_004cc660(a: *mut core::ffi::c_void, b: *mut i16);
    fn FUN_004cc5c0(a: *mut core::ffi::c_void, b: i32, c: i32);
    fn FUN_004cc6c0(a: *mut core::ffi::c_void, b: *mut u8, c: i32);
    fn FUN_004cc590(a: *mut core::ffi::c_void);
    #[link_name = "sprintf"]
    fn _sprintf(a: *mut i8, b: *const i8, ...) -> i32;
    #[link_name = "fgets"]
    fn _fgets(a: *mut i8, b: i32, c: *mut core::ffi::c_void) -> *mut i8;
    #[link_name = "atoi"]
    fn _atoi(a: *const i8) -> i32;
    #[link_name = "_isctype"]
    fn __isctype(a: i32, b: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004e5590(param_1: *mut u8) {
    if param_1.is_null() {
        return;
    }
    // SAFETY: all globals and callees are the game's own; param_1 points at
    // the caller's 0x1000-byte buffer; misaligned globals are read unaligned.
    unsafe {
        let sel_p = (&raw mut DAT_00b6a5aa).cast::<u8>();
        let key_p = (&raw mut DAT_00b6a5ae).cast::<u8>();
        let buf = (&raw mut DAT_00b69d20).cast::<i8>();

        let mut i = 0usize;
        while i < 0x1000 {
            param_1.add(i).write(0);
            i += 1;
        }

        let mut u_var8: u32 = 0;
        if DAT_00aa6138 != 0 {
            let mut pi = DAT_00aa6114.wrapping_add(0x140) as *const i32;
            loop {
                let i_sel = sel_p.cast::<i32>().read_unaligned();
                let i_key = key_p
                    .byte_add((i_sel as u32).wrapping_mul(4) as usize)
                    .cast::<i32>()
                    .read_unaligned();
                if pi.read_unaligned() == i_key {
                    break;
                }
                pi = pi.wrapping_add(0x52);
                u_var8 = u_var8.wrapping_add(1);
                if !(u_var8 < DAT_00aa6138) {
                    break;
                }
            }
        }
        if !(u_var8 < DAT_00aa6138) {
            return;
        }
        let i_sel = sel_p.cast::<i32>().read_unaligned();
        let name = (&raw mut PTR_s_train_0052c1e8)
            .cast::<*mut i8>()
            .offset(i_sel as isize)
            .read();
        _sprintf(
            buf,
            (&raw const s__s__s_0052c300).cast::<i8>(),
            name,
            u_var8.wrapping_mul(0x148).wrapping_add(DAT_00aa6114) as *mut i8,
        );
        let local_8 = FUN_004cc4a0(buf, &raw mut DAT_0051a854);
        if local_8.is_null() {
            return;
        }
        let i_sel = sel_p.cast::<i32>().read_unaligned();
        if i_sel == 2 || i_sel == 4 || i_sel == 5 {
            _fgets(buf, 0x100, local_8);
            let mut n = _atoi(buf);
            while n != 0 {
                _fgets(buf, 0x100, local_8);
                n = n.wrapping_sub(1);
            }
            u_var8 = 0;
            let mut pb = param_1;
            'outer: while !_fgets(buf, 0x100, local_8).is_null() {
                let mut len: u32 = 0;
                while *buf.add(len as usize) != 0 {
                    len += 1;
                }
                let mut u_var7: u32 = 0;
                if len != 0 {
                    loop {
                        let b = *buf.add(u_var7 as usize) as u8;
                        let c = b as u32;
                        let u_var4: u32 = if DAT_00aa6138 == DAT_00aa6138 && DAT_0052dc84 < 2 {
                            (PTR_DAT_0052da78
                                .byte_add((c * 2) as usize)
                                .cast::<u16>()
                                .read_unaligned() as u32)
                                & 0x157
                        } else {
                            __isctype(c as i32, 0x157) as u32
                        };
                        if u_var4 != 0 || c == 10 {
                            *pb = b;
                            pb = pb.add(1);
                            u_var8 = u_var8.wrapping_add(1);
                            if 0xffe < u_var8 {
                                break 'outer;
                            }
                        }
                        u_var7 += 1;
                        if !(u_var7 < len) {
                            break;
                        }
                    }
                }
                if 0xffe < u_var8 {
                    break;
                }
            }
            param_1.add(u_var8 as usize).write(0);
        } else {
            let mut local_a: i16 = 0;
            FUN_004cc660(local_8, &mut local_a);
            if local_a == 0xe || local_a == 0xd {
                FUN_004cc5c0(local_8, -0x1000, 2);
                FUN_004cc6c0(local_8, param_1, 0x1000);
                param_1.add(0xfff).write(0);
            } else if local_a == 0xc {
                FUN_004cc5c0(local_8, -0x400, 2);
                FUN_004cc6c0(local_8, param_1, 0x400);
                param_1.add(0x3ff).write(0);
            }
        }
        FUN_004cc590(local_8);
    }
}
