// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b6a2b4: *mut core::ffi::c_void;
    static mut DAT_00aa6114: *mut core::ffi::c_void;
    static mut DAT_00aa6138: u32;
    static mut DAT_00b6a5be: [u8; 4];
    static mut DAT_00b6bba6: [u8; 4];
    static mut DAT_00b6bbda: [u8; 0];
    static mut DAT_00b6a244: i32;
    static mut DAT_00b69d20: [u8; 0];
    static mut DAT_00a91b90: [u8; 0];
    static mut DAT_0051c4bc: i8;
    static mut PTR_s_battle_0052c1f8: *mut i8;
    static mut s__s__s_0052c300: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _free(p: *mut core::ffi::c_void);
    fn FUN_004e5000(a: i32);
    fn _sprintf(a: *mut i8, b: *const i8, ...) -> i32;
    fn FUN_004cc4a0(a: *mut i8, b: *mut i8) -> *mut core::ffi::c_void;
    fn _fgets(a: *mut i8, b: i32, c: *mut core::ffi::c_void) -> *mut i8;
    fn FUN_004cc590(a: *mut core::ffi::c_void);
    fn _atoi(a: *const i8) -> i32;
    fn _malloc(a: u32) -> *mut core::ffi::c_void;
    fn __strcmpi(a: *const i8, b: *const i8) -> i32;
}

unsafe fn slen(p: *const u8) -> u32 {
    let mut n: u32 = 0;
    // SAFETY: p points at a NUL-terminated string in the game's memory.
    unsafe {
        while *p.add(n as usize) != 0 {
            n = n.wrapping_add(1);
        }
    }
    n
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004fcf30() -> u32 {
    let mut local_80 = [0i8; 128];
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are used as the original does.
    unsafe {
        if !DAT_00b6a2b4.is_null() {
            _free(DAT_00b6a2b4);
            DAT_00b6a2b4 = core::ptr::null_mut();
        }
        let u_var7 = DAT_00aa6138;
        let saved = DAT_00aa6114;
        let mut u_var10: u32 = 0;
        DAT_00aa6114 = core::ptr::null_mut();
        FUN_004e5000(4);
        if DAT_00aa6138 != 0 {
            let mut p = (DAT_00aa6114 as *mut u8).byte_add(0x140) as *const i32;
            loop {
                let l = ((&raw mut DAT_00b6a5be) as *const i32).read_unaligned();
                if p.read_unaligned() == l {
                    break;
                }
                p = p.byte_add(0x148);
                u_var10 = u_var10.wrapping_add(1);
                if !(u_var10 < DAT_00aa6138) {
                    break;
                }
            }
        }
        _sprintf(
            local_80.as_mut_ptr(),
            (&raw mut s__s__s_0052c300) as *const i8,
            PTR_s_battle_0052c1f8,
            (DAT_00aa6114 as *mut u8).wrapping_add(u_var10.wrapping_mul(0x148) as usize),
        );
        let src = (DAT_00aa6114 as *mut u8)
            .wrapping_add(u_var10.wrapping_mul(0x148).wrapping_add(0x40) as usize);
        let n = slen(src).wrapping_add(1);
        let dst = (&raw mut DAT_00a91b90) as *mut u8;
        let mut i: u32 = 0;
        while i < n {
            *dst.add(i as usize) = *src.add(i as usize);
            i += 1;
        }
        _free(DAT_00aa6114);
        DAT_00aa6114 = saved;
        DAT_00aa6138 = u_var7;
        let file = FUN_004cc4a0(local_80.as_mut_ptr(), &raw mut DAT_0051c4bc);
        if file.is_null() {
            return 0;
        }
        let buf = (&raw mut DAT_00b69d20) as *mut u8;
        if _fgets(buf as *mut i8, 0xff, file).is_null() {
            FUN_004cc590(file);
            return 0;
        }
        DAT_00b6a244 = _atoi(buf as *const i8);
        let mut i_var9: i32 = 0;
        DAT_00b6a2b4 = _malloc((DAT_00b6a244 as u32).wrapping_mul(0x148));
        if DAT_00b6a244 < 1 {
            return 1;
        }
        let mut i_var8: u32 = 0;
        loop {
            if _fgets(buf as *mut i8, 0xff, file).is_null() {
                DAT_00b6a244 = i_var9;
                FUN_004cc590(file);
                return 0;
            }
            let idx = slen(buf).wrapping_add(1).wrapping_sub(2);
            if *buf.wrapping_add(idx as usize) == b'\n' {
                let idx = slen(buf).wrapping_add(1).wrapping_sub(2);
                *buf.wrapping_add(idx as usize) = 0;
            }
            let mut u7: u32 = 0;
            let mut found = false;
            let mut b_var2 = false;
            if DAT_00aa6138 != 0 {
                let mut i_var11: u32 = 0;
                loop {
                    let r = __strcmpi(
                        buf as *const i8,
                        (DAT_00aa6114 as *mut u8).wrapping_add(i_var11 as usize) as *const i8,
                    );
                    if r == 0 {
                        found = true;
                        let l = ((&raw mut DAT_00b6bba6) as *const u32).read_unaligned();
                        if (l.wrapping_sub(1) as i32) >= 0 {
                            let mut pb = ((&raw mut DAT_00b6bbda) as *mut u8)
                                .wrapping_add(l.wrapping_sub(1).wrapping_mul(4) as usize);
                            loop {
                                let v = (pb as *const i32).read_unaligned();
                                if v == i_var9 {
                                    b_var2 = true;
                                    break;
                                }
                                pb = pb.wrapping_sub(4);
                                if !((pb as u32) > 0xb6bbd9) {
                                    break;
                                }
                            }
                        }
                        break;
                    }
                    i_var11 = i_var11.wrapping_add(0x148);
                    u7 = u7.wrapping_add(1);
                    if !(u7 < DAT_00aa6138) {
                        break;
                    }
                }
            }
            if found {
                let s = (DAT_00aa6114 as *mut u8).wrapping_add(u7.wrapping_mul(0x148) as usize);
                let d = (DAT_00b6a2b4 as *mut u8).wrapping_add(i_var8 as usize);
                core::ptr::copy_nonoverlapping(s, d, 0x148);
                core::ptr::write_bytes(d.wrapping_add(0xc0), 0, 0x80);
                if b_var2 {
                    (d.wrapping_add(0x144) as *mut u32).write_unaligned(1);
                }
            }
            i_var8 = i_var8.wrapping_add(0x148);
            i_var9 = i_var9.wrapping_add(1);
            if DAT_00b6a244 <= i_var9 {
                return 1;
            }
        }
    }
}
