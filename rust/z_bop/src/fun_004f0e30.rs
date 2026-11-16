// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b9e: [u8; 1024];
    static mut DAT_00aa7f9a: u8;
    static mut DAT_00aa7f9b: u8;
    static mut DAT_00aa7f9c: u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FindResourceA(hModule: *mut c_void, lpName: *const i8, lpType: *const i8) -> *mut c_void;
    fn LoadResource(hModule: *mut c_void, hResInfo: *mut c_void) -> *mut c_void;
    fn LockResource(hResData: *mut c_void) -> *mut c_void;
    fn _lopen(lpPathName: *const i8, iReadWrite: i32) -> i32;
    fn _lread(hFile: i32, lpBuffer: *mut c_void, uBytes: u32) -> u32;
    fn _lclose(hFile: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f0e30(param_1: *mut i32, param_2: *const i8) -> *mut u8 {
    let mut frame_buf = [0u32; 0x43c / 4];
    let frame = frame_buf.as_mut_ptr() as *mut u8;
    // SAFETY: all pointers are offsets into the local frame buffer, resources
    // and the game's own memory, accessed where the original does.
    unsafe {
        let local_43c = frame;
        let local_438 = frame.add(4);
        let local_42a = frame.add(0x12);
        let local_418 = frame.add(0x24);
        let ustack_414 = frame.add(0x28);
        let local_410 = frame.add(0x2c);
        let local_400 = frame.add(0x3c);
        let frame_end = frame.add(0x43c);

        let mut p4 = local_400;
        let mut u3: u32 = 0;
        loop {
            let p6 = p4.add(4);
            *p4 = ((((u3 & 0xe0) >> 5) * 0xff) / 7) as u8;
            *p4.add(1) = ((((u3 & 0x1c) >> 2) * 0xff) / 7) as u8;
            *p4.add(2) = (((u3 & 3) * 0xff) / 3) as u8;
            *p4.add(3) = 0;
            p4 = p6;
            u3 = u3.wrapping_add(1);
            if !(p6 < frame_end) {
                break;
            }
        }

        if !param_2.is_null() {
            let h_res_info = FindResourceA(core::ptr::null_mut(), param_2, 2usize as *const i8);
            if h_res_info.is_null() {
                let h_file = _lopen(param_2, 0);
                if h_file != -1 {
                    _lread(h_file, local_410 as *mut c_void, 0xe);
                    _lread(h_file, local_438 as *mut c_void, 0x28);
                    _lread(h_file, local_400 as *mut c_void, 0x400);
                    _lclose(h_file);
                    let cnt = local_418 as *mut i32;
                    if (local_438 as *const i32).read_unaligned() == 0x28 {
                        let w = (local_42a as *const u16).read_unaligned();
                        if w < 9 {
                            if cnt.read_unaligned() == 0 {
                                cnt.write_unaligned(1u32.wrapping_shl((w as u32) & 0x1f) as i32);
                            }
                        } else {
                            cnt.write_unaligned(0);
                        }
                    } else {
                        cnt.write_unaligned(0);
                    }
                    if 0 < cnt.read_unaligned() {
                        let mut p = local_400;
                        loop {
                            let u1 = *p;
                            *p = *p.add(2);
                            cnt.write_unaligned(cnt.read_unaligned().wrapping_sub(1));
                            *p.add(2) = u1;
                            p = p.add(4);
                            if cnt.read_unaligned() == 0 {
                                break;
                            }
                        }
                    }
                }
            } else {
                let h_res_data = LoadResource(core::ptr::null_mut(), h_res_info);
                let pu2 = LockResource(h_res_data) as *const u8;
                let mut u3: u32;
                let mut b9: bool;
                if pu2.is_null()
                    || (pu2 as *const u32).read_unaligned() < 0x28
                    || 8 < (pu2.add(0xe) as *const u16).read_unaligned()
                {
                    u3 = 0;
                    b9 = u3 == 0;
                } else {
                    u3 = (pu2.add(0x20) as *const u32).read_unaligned();
                    b9 = false;
                    if u3 == 0 {
                        u3 = 1u32.wrapping_shl(
                            ((pu2.add(0xe) as *const u16).read_unaligned() as u32) & 0x1f,
                        );
                        b9 = u3 == 0;
                    }
                }
                if !b9 && -1 < (u3 as i32) {
                    let hdr = (pu2 as *const u32).read_unaligned();
                    let mut p4 = pu2.wrapping_add(hdr as usize).wrapping_add(2);
                    let mut p6 = local_400;
                    loop {
                        u3 = u3.wrapping_sub(1);
                        *p6 = *p4;
                        *p6.add(1) = *p4.wrapping_sub(1);
                        *p6.add(2) = *p4.wrapping_sub(2);
                        *p6.add(3) = 0;
                        p4 = p4.wrapping_add(4);
                        p6 = p6.add(4);
                        if u3 == 0 {
                            break;
                        }
                    }
                }
            }
        }

        let p4 = local_43c;
        let vtable = param_1.read() as u32;
        let fp = ((vtable.wrapping_add(0x14)) as *const i32).read_unaligned() as u32 as usize
            as *const c_void;
        let f: unsafe extern "stdcall" fn(*mut i32, i32, *mut u8, *mut u8, i32) -> i32 =
            core::mem::transmute(fp);
        f(param_1, 0x44, local_400, p4, 0);
        if !p4.is_null() {
            core::ptr::copy_nonoverlapping(
                ustack_414 as *const u8,
                &raw mut DAT_00aa7b9e as *mut u8,
                0x400,
            );
            let t = &raw mut DAT_00aa7b9e as *mut u8;
            *t = 0;
            *t.add(1) = 0;
            *t.add(2) = 0;
            DAT_00aa7f9a = 0xff;
            DAT_00aa7f9b = 0xff;
            DAT_00aa7f9c = 0xff;
        }
        p4
    }
}
