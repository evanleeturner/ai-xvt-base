// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00527048: [i8; 19];
    static mut DAT_009994c4: *mut i32;
    static mut DAT_009994c0: [u32; 1];
    static mut DAT_009994cc: u32;
    static mut DAT_009994d0: [u32; 1];
    static mut DAT_009994e0: [u32; 1];
    static mut DAT_009994f0: u32;
    static mut DAT_009994f4: u32;
    static mut DAT_009994f8: u32;
    static mut DAT_009994fc: u32;
    static mut DAT_00999524: u32;
    static mut DAT_00999528: [u32; 1];
    static mut DAT_00999538: [i8; 1];
    static mut DAT_00999548: u32;
    static mut DAT_0099954c: u32;
    static mut DAT_00999550: [i32; 1];
    static mut DAT_009997d0: u32;
    static mut DAT_009997d4: u32;
    static mut DAT_009999d4: u32;
    static mut DAT_009999d8: u32;
    static mut DAT_009999dc: u32;
    static mut DAT_009999e0: u32;
    static mut DAT_00999be0: u32;
    static mut DAT_00999be4: u32;
    static mut DAT_00999be8: u32;
    static mut DAT_00999bec: u32;
    static mut DAT_0099f4ac: u32;
    static mut DAT_0099f6e0: u32;
    static mut DAT_00527020: u32;
    static mut DAT_0052701c: u32;
    static mut DAT_00527024: u32;
    static mut DAT_00527028: u32;
    static mut DAT_0052702c: u32;
    static mut DAT_005599a0: u32;
    static mut DAT_005597a0: u32;
    static mut DAT_005dd9b8: u32;
    static mut DAT_005ee1b8: [u32; 1];
    static mut DAT_009ec608: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046c220();
    fn FUN_004d1940(
        a1: *mut core::ffi::c_void,
        a2: *mut core::ffi::c_void,
        a3: *mut core::ffi::c_void,
        a4: *mut core::ffi::c_void,
        a5: *mut core::ffi::c_void,
        a6: *mut core::ffi::c_void,
        a7: i32,
        a8: *mut core::ffi::c_void,
        a9: *mut core::ffi::c_void,
        a10: *mut core::ffi::c_void,
        a11: *mut core::ffi::c_void,
        a12: *mut core::ffi::c_void,
        a13: *mut core::ffi::c_void,
        a14: *mut core::ffi::c_void,
        a15: *mut core::ffi::c_void,
        a16: *mut core::ffi::c_void,
        a17: *mut core::ffi::c_void,
        a18: *mut core::ffi::c_void,
        a19: *mut core::ffi::c_void,
        a20: *mut core::ffi::c_void,
        a21: *mut core::ffi::c_void,
        a22: *mut core::ffi::c_void,
    ) -> i32;
    fn _strncpy(a: *mut i8, b: *const i8, n: u32) -> *mut i8;
    fn FUN_0046c730();
    fn FUN_0046d350(a: u32, b: *mut u32, c: i32);
    fn FUN_0046e040() -> i32;
    fn FUN_0046f5e0(a: *mut i32, b: *mut u32, c: i32) -> *mut i32;
    fn FUN_0046f840(a: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn timeGetTime() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0046c230(
    param_1: *mut i8,
    param_2: *mut i8,
    param_3: i32,
    param_4: i32,
    param_5: u32,
    param_6: i32,
) -> u32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are used exactly as the original does.
    unsafe {
        let mut local_48: [u32; 10] = [0; 10];
        let mut local_20: [i8; 32] = [0; 32];
        let mut stack_a8: [i32; 15] = [0; 15];
        let mut stack_a4: u32 = 0;

        core::ptr::copy_nonoverlapping(
            (&raw const DAT_00527048) as *const i8,
            local_20.as_mut_ptr(),
            19,
        );
        for i in 19..32 {
            local_20[i] = 0;
        }

        FUN_0046c220();

        let mut p = (&raw mut DAT_009994c0) as *mut u32;
        let mut n: i32 = 0x190e;
        while n != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            n -= 1;
        }
        DAT_009ec608 = 1;
        DAT_005599a0 = 0;
        DAT_009994cc = param_5;
        DAT_009997d0 = 0;
        DAT_009999d8 = 1;
        ((&raw mut DAT_009997d4) as *mut u8).write(0x39);
        DAT_009999d4 = 1;
        DAT_009999dc = 0;
        DAT_00999be4 = 1;
        ((&raw mut DAT_009999e0) as *mut u8).write(0x39);
        DAT_00999be0 = 1;
        DAT_00999524 = 0;

        let mut p1 = &raw mut DAT_00999bec;
        let end = &raw mut DAT_0099f4ac;
        loop {
            p1.write_unaligned(0x7f);
            let p6 = p1.wrapping_add(0x8e);
            p1.add(2).write_unaligned(0x7f);
            p1.add(4).write_unaligned(0x7f);
            p1.add(1).write_unaligned(0x7f);
            p1.add(3).write_unaligned(0x7f);
            p1.add(5).write_unaligned(0x7f);
            p1.add(6).write_unaligned(0);
            p1.offset(-1).write_unaligned(0);
            (p1.add(7) as *mut u8).write(0x39);
            p1.add(0x87).write_unaligned(1);
            p1.add(0x88).write_unaligned(0);
            p1.add(0x89).write_unaligned(0);
            p1.add(0x8a).write_unaligned(0);
            p1 = p6;
            if !(p6 < end) {
                break;
            }
        }

        DAT_00527020 = 0;
        let mut p = (&raw mut DAT_005ee1b8) as *mut u32;
        let mut n: i32 = 0x8400;
        while n != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            n -= 1;
        }
        DAT_0052702c = 0;
        DAT_0099f6e0 = 0;

        macro_rules! call_init {
            () => {
                FUN_004d1940(
                    (&raw mut DAT_009994c4) as *mut core::ffi::c_void,
                    (&raw mut DAT_009994d0) as *mut core::ffi::c_void,
                    (&raw mut DAT_009994e0) as *mut core::ffi::c_void,
                    (&raw mut DAT_009994f0) as *mut core::ffi::c_void,
                    (&raw mut DAT_009994f8) as *mut core::ffi::c_void,
                    (&raw mut DAT_00999528) as *mut core::ffi::c_void,
                    0x5599a8,
                    (&raw mut DAT_00527028) as *mut core::ffi::c_void,
                    (&raw mut DAT_0052702c) as *mut core::ffi::c_void,
                    (&raw mut DAT_00527024) as *mut core::ffi::c_void,
                    (&raw mut DAT_00999be8) as *mut core::ffi::c_void,
                    (&raw mut DAT_0099f6e0) as *mut core::ffi::c_void,
                    (&raw mut DAT_009997d0) as *mut core::ffi::c_void,
                    (&raw mut DAT_009997d4) as *mut core::ffi::c_void,
                    (&raw mut DAT_009999d4) as *mut core::ffi::c_void,
                    (&raw mut DAT_009999d8) as *mut core::ffi::c_void,
                    (&raw mut DAT_009999dc) as *mut core::ffi::c_void,
                    (&raw mut DAT_009999e0) as *mut core::ffi::c_void,
                    (&raw mut DAT_00999be0) as *mut core::ffi::c_void,
                    (&raw mut DAT_00999be4) as *mut core::ffi::c_void,
                    (&raw mut DAT_005dd9b8) as *mut core::ffi::c_void,
                    (&raw mut DAT_0052701c) as *mut core::ffi::c_void,
                )
            };
        }

        macro_rules! call_vt {
            () => {{
                let obj = DAT_009994c4;
                let vt = obj.read_unaligned();
                let fp = ((vt as u32).wrapping_add(0x38) as usize
                    as *const *const core::ffi::c_void)
                    .read_unaligned();
                let f: unsafe extern "stdcall" fn(*mut i32, *mut u32, i32) -> i32 =
                    core::mem::transmute(fp);
                f(obj, local_48.as_mut_ptr(), 0);
            }};
        }

        if param_6 == 1 && param_3 == 1 {
            call_init!();
            if DAT_009994c4.is_null() {
                DAT_00999548 = 1;
                DAT_0099954c = 1;
            } else {
                local_48 = [0; 10];
                local_48[0] = 0x28;
                call_vt!();
            }
            _strncpy((&raw mut DAT_00999528) as *mut i8, param_1, 0x10);
            _strncpy((&raw mut DAT_00999538) as *mut i8, param_2, 0x10);
            let mut s = (&raw mut DAT_00999528) as *mut u32;
            let mut d = (&raw mut DAT_00999550) as *mut u32;
            let mut n: i32 = 10;
            while n != 0 {
                d.write_unaligned(s.read_unaligned());
                s = s.add(1);
                d = d.add(1);
                n -= 1;
            }
            DAT_009994f4 = param_3 as u32;
            DAT_009994fc = 1;
            return 1;
        }

        call_init!();
        local_48 = [0; 10];
        local_48[0] = 0x28;
        call_vt!();
        DAT_009994fc = 0;
        DAT_009994f4 = param_3 as u32;
        FUN_0046c730();
        DAT_009ec608 = 1;
        timeGetTime();
        DAT_005597a0 = 0xd;
        FUN_0046d350(DAT_009994f8, &raw mut DAT_005597a0, 4);
        DAT_005597a0 = 0x39;
        FUN_0046d350(DAT_009994f8, &raw mut DAT_005597a0, 4);

        let i2 = FUN_0046e040();
        let mut i4 = param_6;
        if i2 != 0 {
            while i4 != 0 {
                let p3 = FUN_0046f5e0(stack_a8.as_mut_ptr(), &raw mut stack_a4, 0x3c);
                if p3.is_null() {
                    DAT_009994c4 = core::ptr::null_mut();
                    return 0;
                }
                if p3.read_unaligned() == 0xd {
                    i4 = (i4 as u32).wrapping_sub(1) as i32;
                }
            }
        }

        let r = FUN_0046e040();
        if r == 0 {
            if param_4 == 0 {
                let mut p3;
                loop {
                    p3 = FUN_0046f5e0(stack_a8.as_mut_ptr(), &raw mut stack_a4, 0x3c);
                    if p3.is_null() {
                        DAT_009994c4 = core::ptr::null_mut();
                        return 0;
                    }
                    if p3.read_unaligned() == 0xe {
                        break;
                    }
                }
                DAT_009994fc = p3.add(1).read_unaligned() as u32;
                let mut i4: i32 = 0;
                if 0 < DAT_009994fc as i32 {
                    loop {
                        let mut p3;
                        loop {
                            p3 = FUN_0046f5e0(stack_a8.as_mut_ptr(), &raw mut stack_a4, 0x3c);
                            if p3.is_null() {
                                return 0;
                            }
                            if p3.read_unaligned() == 0xf {
                                break;
                            }
                        }
                        i4 += 1;
                        let mut src = p3.add(2);
                        let idx = (p3.add(1).read_unaligned() as u32).wrapping_mul(10) as i32;
                        let mut dst =
                            ((&raw mut DAT_00999550) as *mut i32).wrapping_offset(idx as isize);
                        let mut n: i32 = 10;
                        while n != 0 {
                            dst.write_unaligned(src.read_unaligned());
                            src = src.add(1);
                            dst = dst.add(1);
                            n -= 1;
                        }
                        if !(i4 < DAT_009994fc as i32) {
                            break;
                        }
                    }
                }
            }
        } else if 1 < param_6 {
            FUN_0046f840(0);
            DAT_005597a0 = 0x39;
            FUN_0046d350(0, &raw mut DAT_005597a0, 4);
        }
        1
    }
}
