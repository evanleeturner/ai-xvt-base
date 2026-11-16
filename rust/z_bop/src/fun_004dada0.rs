// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aabd11: [u8; 0];
    static mut DAT_00aa80a3: [u8; 0];
    static mut DAT_00aa86ab: i8;
    static mut DAT_00aac319: i8;
    static mut DAT_00aa7b0d: u8;
    static mut DAT_00aa7b13: [u8; 0];
    static mut DAT_00aa7b1f: [u8; 0];
    static mut DAT_00aa7b32: [u8; 0];
    static mut DAT_00aa7b3a: [u8; 0];
    static mut DAT_00aa6cfc: *mut u32;
    static mut DAT_00aa6d00: [u8; 0];
    static mut DAT_0051a854: i8;
    static mut DAT_0052c068: [i8; 0];
    static mut DAT_0052bb00: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004cc4a0(a: *mut i8, b: *mut i8) -> *mut c_void;
    fn FUN_004cc590(a: *mut c_void);
    fn FUN_004dc0a0(a: *mut i8, b: i32) -> i32;
    fn FUN_004d4970();
    fn malloc(n: u32) -> *mut c_void;
    fn free(p: *mut c_void);
    fn realloc(p: *mut c_void, n: u32) -> *mut c_void;
    fn FUN_004d48e0() -> *mut u32;
    fn FUN_004d37e0() -> i32;
    fn FUN_004b7840(a: *mut u8, b: *mut u8, c: u32) -> u32;
    fn FUN_004dc020(a: *mut i8, b: *mut c_void, c: u32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn CreateFontA(
        h: i32,
        w: i32,
        esc: i32,
        ori: i32,
        weight: i32,
        italic: i32,
        underline: i32,
        strike: i32,
        charset: i32,
        outprec: i32,
        clipprec: i32,
        quality: i32,
        pitch: i32,
        face: *const i8,
    ) -> *mut c_void;
    fn GetDC(hwnd: *mut c_void) -> *mut c_void;
    fn SelectObject(hdc: *mut c_void, obj: *mut c_void) -> *mut c_void;
    fn SetMapMode(hdc: *mut c_void, mode: i32) -> i32;
    fn SetTextCharacterExtra(hdc: *mut c_void, extra: i32) -> i32;
    fn GetTextExtentPoint32A(hdc: *mut c_void, s: *const i8, n: i32, size: *mut i32) -> i32;
    fn ReleaseDC(hwnd: *mut c_void, hdc: *mut c_void) -> i32;
    fn DeleteObject(obj: *mut c_void) -> i32;
    fn SetTextColor(hdc: *mut c_void, c: u32) -> u32;
    fn SetBkColor(hdc: *mut c_void, c: u32) -> u32;
    fn SetBkMode(hdc: *mut c_void, m: i32) -> i32;
    fn SetRect(r: *mut i32, a: i32, b: i32, c: i32, d: i32) -> i32;
    fn ExtTextOutA(
        hdc: *mut c_void,
        x: i32,
        y: i32,
        opts: u32,
        r: *const i32,
        s: *const i8,
        n: u32,
        dx: *const i32,
    ) -> i32;
}

#[inline(always)]
unsafe fn vslot(obj: *mut c_void, idx: usize) -> *const c_void {
    // SAFETY: obj is a COM-style object whose first word is a vtable pointer.
    unsafe { (obj as *const *const *const c_void).read().add(idx).read() }
}

type BltFn = unsafe extern "stdcall" fn(*mut c_void, u32, u32, *mut c_void, *mut i32, u32) -> i32;
type GetDcFn = unsafe extern "stdcall" fn(*mut c_void, *mut *mut c_void) -> i32;
type RelDcFn = unsafe extern "stdcall" fn(*mut c_void, *mut c_void) -> i32;

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dada0(mut param_1: i32) -> u32 {
    // SAFETY: all globals, callees and the object tables are the game's own;
    // pointers are used at the offsets and widths the original uses.
    unsafe {
        if param_1 < 0x100 {
            if param_1 < 1 {
                param_1 = 1;
            }
        } else {
            param_1 = 0xff;
        }
        let mut pi_var11: *mut u8 = core::ptr::null_mut();
        let mut local_444: u32 = 0;
        let local_424: *mut u8 = (&raw mut DAT_00aabd11)
            .cast::<u8>()
            .byte_add((param_1 as u32).wrapping_mul(4) as usize);
        let mut i_tmp: i32 = local_424.cast::<i32>().read_unaligned();
        if i_tmp != 0 {
            return 1;
        }
        let mut i_var12: i32 = 0;
        let mut pc_var3: *const i8 = &raw mut DAT_00aa86ab as *const i8;
        loop {
            if *pc_var3 == 0 {
                pi_var11 = (&raw mut DAT_00aa80a3)
                    .cast::<u8>()
                    .byte_add((i_var12 as u32).wrapping_mul(0x60b) as usize);
                break;
            }
            pc_var3 = pc_var3.byte_add(0x60b);
            i_var12 = i_var12.wrapping_add(1);
            if !(pc_var3 < &raw mut DAT_00aac319 as *const i8) {
                break;
            }
        }
        if pi_var11.is_null() {
            return 0;
        }
        let mut local_400: [u32; 256] = [0; 256];
        let buf: *mut i8 = local_400.as_mut_ptr().cast::<i8>();
        sprintf(buf, (&raw mut DAT_0052c068).cast::<i8>(), param_1);
        let p_fvar4 = FUN_004cc4a0(buf, &raw mut DAT_0051a854);
        if !p_fvar4.is_null() {
            FUN_004cc590(p_fvar4);
            i_var12 = FUN_004dc0a0(buf, i_var12);
            if i_var12 != 0 {
                return 1;
            }
        }
        let local_43c = CreateFontA(
            param_1.wrapping_neg(),
            0,
            0,
            0,
            400,
            0,
            0,
            0,
            0,
            4,
            0,
            3,
            0x12,
            (&raw mut DAT_0052bb00).cast::<i8>(),
        );
        if local_43c.is_null() {
            return 0;
        }
        let local_434: u32 = DAT_00aa7b0d as u32;
        FUN_004d4970();
        let mut local_450: *mut c_void = GetDC(core::ptr::null_mut());
        let pv_var5 = SelectObject(local_450, local_43c);
        SetMapMode(local_450, 1);
        SetTextCharacterExtra(local_450, 0);
        pi_var11.byte_add(0x504).write(0);
        i_var12 = 1;
        let mut local_452: [i8; 2] = [0; 2];
        let mut local_430: [i32; 2] = [0; 2];
        loop {
            local_452[0] = i_var12 as i8;
            local_452[1] = 0;
            GetTextExtentPoint32A(local_450, local_452.as_ptr(), 1, local_430.as_mut_ptr());
            if local_430[0] < 0 {
                local_430[0] = 0;
            }
            if local_430[1] < 0 {
                local_430[1] = 0;
            }
            let i_var10 = i_var12.wrapping_add(1);
            pi_var11
                .byte_add((i_var12 as usize).wrapping_add(0x504))
                .write(local_430[0] as u8);
            pi_var11
                .byte_add((i_var12 as usize).wrapping_add(0x404))
                .write(local_430[1] as u8);
            local_444 =
                local_444.wrapping_add((local_430[1] as u32).wrapping_mul(local_430[0] as u32));
            i_var12 = i_var10;
            if !(i_var10 < 0x100) {
                break;
            }
        }
        pi_var11
            .byte_add(0x404)
            .write(pi_var11.byte_add(0x405).read());
        SelectObject(local_450, pv_var5);
        ReleaseDC(core::ptr::null_mut(), local_450);
        let mut local_440: *mut u32 = malloc(local_444).cast::<u32>();
        if local_440.is_null() {
            DeleteObject(local_43c);
            if local_434 != 0 {
                DAT_00aa6cfc = FUN_004d48e0();
            }
            return 0;
        }
        let pv_tmp: *mut c_void = local_440.cast::<c_void>();
        pi_var11.cast::<*mut c_void>().write_unaligned(pv_tmp);
        i_tmp = 0;
        pi_var11.byte_add(4).cast::<i32>().write_unaligned(i_tmp);
        let mut local_420: [i32; 4] = [0, 0, 0x280, 0x1e0];
        loop {
            let pv_obj_a: *mut c_void = (&raw mut DAT_00aa7b3a)
                .cast::<*mut c_void>()
                .read_unaligned();
            let pv_obj_b: *mut c_void = (&raw mut DAT_00aa7b32)
                .cast::<*mut c_void>()
                .read_unaligned();
            let f: BltFn = core::mem::transmute(vslot(pv_obj_a, 7));
            i_var12 = f(pv_obj_a, 0, 0, pv_obj_b, local_420.as_mut_ptr(), 0);
            if i_var12 == 0 {
                break;
            }
            if i_var12 == -0x7789fe3e {
                i_var12 = FUN_004d37e0();
                if i_var12 != 0 {
                    DeleteObject(local_43c);
                    if local_434 != 0 {
                        DAT_00aa6cfc = FUN_004d48e0();
                    }
                    free(local_440.cast::<c_void>());
                    return 0;
                }
            } else if i_var12 != -0x7789fde4 {
                free(local_440.cast::<c_void>());
                DeleteObject(local_43c);
                if local_434 != 0 {
                    DAT_00aa6cfc = FUN_004d48e0();
                }
                return 0;
            }
        }
        let mut s_stack_448: u32 = 0;
        let mut i_stack_44c: i32 = 1;
        let mut u_var6: u32 = local_400[0];
        let mut t_stack_410: [i32; 4] = [0; 4];
        'outer: loop {
            let mut pv_obj_b: *mut c_void = (&raw mut DAT_00aa7b32)
                .cast::<*mut c_void>()
                .read_unaligned();
            let f: GetDcFn = core::mem::transmute(vslot(pv_obj_b, 0x11));
            i_var12 = f(pv_obj_b, &raw mut local_450);
            if i_var12 != 0 {
                if i_var12 == -0x7789fe3e {
                    let i_var10 = FUN_004d37e0();
                    i_var12 = 0;
                    if i_var10 != 0 {
                        break 'outer;
                    }
                }
                if i_var12 == -0x7789fde4 {
                    continue 'outer;
                }
                if i_var12 != 0 {
                    break 'outer;
                }
            }
            SetMapMode(local_450, 1);
            let pv_var5 = SelectObject(local_450, local_43c);
            SetTextColor(local_450, 0xffffff);
            SetBkColor(local_450, 0);
            SetBkMode(local_450, 2);
            SetRect(t_stack_410.as_mut_ptr(), 0, 0, 0x280, 0x1e0);
            local_452[1] = 0;
            local_452[0] = i_stack_44c as i8;
            ExtTextOutA(
                local_450,
                0,
                0,
                2,
                t_stack_410.as_ptr(),
                local_452.as_ptr(),
                1,
                core::ptr::null(),
            );
            SelectObject(local_450, pv_var5);
            pv_obj_b = (&raw mut DAT_00aa7b32)
                .cast::<*mut c_void>()
                .read_unaligned();
            let g: RelDcFn = core::mem::transmute(vslot(pv_obj_b, 0x1a));
            g(pv_obj_b, local_450);
            DAT_00aa6cfc = FUN_004d48e0();
            let mut u_stack_428: u32 = 0;
            pi_var11
                .byte_add((i_stack_44c as usize).wrapping_add(1).wrapping_mul(4))
                .cast::<u32>()
                .write_unaligned(s_stack_448);
            let i_stack_438: *mut u8 = pi_var11.byte_add(i_stack_44c as usize);
            if i_stack_438.byte_add(0x404).read() != 0 {
                loop {
                    i_tmp = (&raw mut DAT_00aa7b1f).cast::<i32>().read_unaligned();
                    let mut call: Option<(*mut u32, u8)> = None;
                    if i_tmp == 8 {
                        call = Some((DAT_00aa6cfc, i_stack_438.byte_add(0x504).read()));
                    } else if i_tmp == 0x10 {
                        i_var12 = 0;
                        if i_stack_438.byte_add(0x504).read() != 0 {
                            loop {
                                let s_var2: i16 = DAT_00aa6cfc
                                    .byte_add((i_var12 as usize).wrapping_mul(2))
                                    .cast::<i16>()
                                    .read_unaligned();
                                local_400
                                    .as_mut_ptr()
                                    .cast::<u8>()
                                    .byte_add(i_var12 as usize)
                                    .write(0xff);
                                if s_var2 == 0 {
                                    local_400
                                        .as_mut_ptr()
                                        .cast::<u8>()
                                        .byte_add(i_var12 as usize)
                                        .write(0);
                                }
                                i_var12 = i_var12.wrapping_add(1);
                                if !(i_var12 < i_stack_438.byte_add(0x504).read() as i32) {
                                    break;
                                }
                            }
                        }
                        call = Some((local_400.as_mut_ptr(), i_stack_438.byte_add(0x504).read()));
                    }
                    if let Some((src, b)) = call {
                        u_var6 = FUN_004b7840(
                            (&raw mut DAT_00aa6d00).cast::<u8>(),
                            src.cast::<u8>(),
                            b as u32,
                        );
                    }
                    if (local_444 as i32) <= (s_stack_448.wrapping_add(u_var6) as i32) {
                        local_444 = ((local_444.wrapping_mul(3)) as i32 / 2) as u32;
                        let pv_tmp = pi_var11.cast::<*mut c_void>().read_unaligned();
                        let pv_var7 = realloc(pv_tmp, local_444);
                        pi_var11.cast::<*mut c_void>().write_unaligned(pv_var7);
                        if pv_var7.is_null() {
                            break;
                        }
                        local_440 = pv_var7
                            .cast::<u8>()
                            .byte_add(s_stack_448 as usize)
                            .cast::<u32>();
                    }
                    i_tmp = pi_var11.cast::<i32>().read_unaligned();
                    if i_tmp == 0 {
                        break;
                    }
                    core::ptr::copy_nonoverlapping(
                        (&raw mut DAT_00aa6d00).cast::<u8>() as *const u8,
                        local_440.cast::<u8>(),
                        u_var6 as usize,
                    );
                    s_stack_448 = s_stack_448.wrapping_add(u_var6);
                    local_440 = local_440.byte_add(u_var6 as usize);
                    i_tmp = (&raw mut DAT_00aa7b13).cast::<i32>().read_unaligned();
                    DAT_00aa6cfc = DAT_00aa6cfc.byte_add(i_tmp as usize);
                    u_stack_428 = u_stack_428.wrapping_add(1);
                    if !(u_stack_428 < i_stack_438.byte_add(0x404).read() as u32) {
                        break;
                    }
                }
            }
            FUN_004d4970();
            i_tmp = pi_var11.cast::<i32>().read_unaligned();
            if i_tmp == 0 {
                break 'outer;
            }
            i_stack_44c = i_stack_44c.wrapping_add(1);
            if 0xff < i_stack_44c {
                break 'outer;
            }
        }
        'lab358: {
            while i_var12 == -0x7789fde4 {
                let pv_obj_b: *mut c_void = (&raw mut DAT_00aa7b32)
                    .cast::<*mut c_void>()
                    .read_unaligned();
                let pv_obj_a: *mut c_void = (&raw mut DAT_00aa7b3a)
                    .cast::<*mut c_void>()
                    .read_unaligned();
                let f: BltFn = core::mem::transmute(vslot(pv_obj_b, 7));
                i_var12 = f(pv_obj_b, 0, 0, pv_obj_a, local_420.as_mut_ptr(), 0);
                if i_var12 == 0 {
                    break 'lab358;
                }
                if i_var12 == -0x7789fe3e {
                    let i_var10 = FUN_004d37e0();
                    i_var12 = 0;
                    if i_var10 != 0 {
                        break;
                    }
                }
            }
            i_stack_44c = 0;
        }
        if local_434 != 0 {
            DAT_00aa6cfc = FUN_004d48e0();
        }
        DeleteObject(local_43c);
        if (s_stack_448 as i32) < (local_444 as i32) {
            let pv_tmp = pi_var11.cast::<*mut c_void>().read_unaligned();
            let pv_var7 = realloc(pv_tmp, s_stack_448);
            pi_var11.cast::<*mut c_void>().write_unaligned(pv_var7);
            if pv_var7.is_null() {
                i_stack_44c = 0;
            }
        }
        if i_stack_44c != 0x100 {
            let pv_tmp = pi_var11.cast::<*mut c_void>().read_unaligned();
            if !pv_tmp.is_null() {
                free(pv_tmp);
            }
            return 0;
        }
        pi_var11
            .byte_add(0x604)
            .cast::<i32>()
            .write_unaligned(param_1);
        pi_var11.byte_add(0x608).write(1);
        pi_var11.byte_add(0x609).write(0);
        pi_var11.byte_add(0x60a).write(1);
        let u_tmp: u32 = pi_var11 as u32;
        local_424.cast::<u32>().write_unaligned(u_tmp);
        sprintf(buf, (&raw mut DAT_0052c068).cast::<i8>(), param_1);
        FUN_004dc020(buf, pi_var11.cast::<c_void>(), s_stack_448);
        1
    }
}
