// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009e8f54: u32;
    static mut DAT_00527508: u32;
    static mut DAT_005233d8: i32;
    static mut DAT_009994bc: *mut u8;
    static mut DAT_00612ba0: [u8; 1];
    static mut DAT_00523448: [i8; 1];
    static mut DAT_00527720: [i8; 1];
    static mut DAT_005236b0: [i8; 1];
    static mut DAT_00523434: u8;
    static mut DAT_00523438: u8;
    static mut DAT_00523444: [u8; 1];
    static mut DAT_00523445: [u8; 1];
    static mut DAT_00523446: [u8; 1];
    static mut DAT_00a07bd8: *mut core::ffi::c_void;
    static mut DAT_00527ec8: i32;
    static mut DAT_009a7c80: [u8; 1];
    static mut DAT_009a8074: *mut u8;
    static mut DAT_009d77f4: *const core::ffi::c_void;
    static mut DAT_005233c8: i8;
    static mut DAT_00523408: i8;
    static mut DAT_009a7bc0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049e720(
        param_1: *mut i8,
        param_2: *mut core::ffi::c_void,
        param_3: i32,
        param_4: i32,
    ) -> i32;
    fn FUN_0040e9e0(param_1: i32, param_2: i32, param_3: i32);
    fn _fwrite(
        param_1: *mut core::ffi::c_void,
        param_2: i32,
        param_3: i32,
        param_4: *mut core::ffi::c_void,
    ) -> i32;
    fn FUN_0049e9c0(param_1: i32);
    fn FUN_0049c3c0(param_1: *mut i8, param_2: *mut u8);
    fn FUN_004451e0(param_1: i32, param_2: i32);
    fn FUN_0049d010();
    fn FUN_004afa90();
    fn FUN_004451f0(param_1: i32, param_2: i32, param_3: *mut core::ffi::c_void);
    fn FUN_0040e940(param_1: *mut u8, param_2: i32, param_3: i32, param_4: i32) -> u32;
}

unsafe fn len_plus_one(mut p: *const i8) -> u32 {
    let mut n: u32 = 0;
    // SAFETY: p points at a NUL-terminated game string.
    unsafe {
        loop {
            n = n.wrapping_add(1);
            let c = p.read();
            p = p.add(1);
            if c == 0 {
                break;
            }
        }
    }
    n
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0049d440() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are used where the original uses them.
    unsafe {
        let demo = (&raw mut DAT_00523448).cast::<i8>();
        let inv = (&raw mut DAT_00527720).cast::<i8>();
        let p0 = (&raw mut DAT_00523444).cast::<u8>();
        let p1 = (&raw mut DAT_00523445).cast::<u8>();
        let p2 = (&raw mut DAT_00523446).cast::<u8>();
        let mut b_stack_104: [u8; 3] = [0; 3];
        let mut local_100: [i8; 12] = [0; 12];

        DAT_009e8f54 &= DAT_00527508;
        if DAT_005233d8 == 1 {
            DAT_009994bc = (&raw mut DAT_00612ba0).cast::<u8>();
            let u6 = len_plus_one(demo) as usize;
            let u2 = p0.add(u6).read();
            b_stack_104[1] = p1.add(u6).read();
            b_stack_104[2] = p2.add(u6).read();
            p0.add(u6).write(0x69);
            p1.add(u6).write(0x6e);
            p2.add(u6).write(0x76);
            let r = FUN_0049e720(demo, (&raw mut DAT_00523434).cast(), 0, 0);
            if r == 0 {
                let mut i = 0;
                while i < 11 {
                    local_100[i] = inv.add(i).read();
                    i += 1;
                }
                let r2 = FUN_0049e720(local_100.as_mut_ptr(), (&raw mut DAT_00523434).cast(), 0, 0);
                if r2 != 0 {
                    FUN_0049e9c0(0);
                    FUN_0049c3c0(local_100.as_mut_ptr(), DAT_009994bc);
                } else {
                    FUN_0040e9e0(DAT_009994bc as i32, 0x40, 0x100);
                    let r3 = FUN_0049e720(demo, (&raw mut DAT_00523438).cast(), 0, 1);
                    if r3 != 0 {
                        _fwrite(DAT_009994bc.cast(), 0x100, 0x100, DAT_00a07bd8);
                        FUN_0049e9c0(1);
                    }
                }
            } else {
                FUN_0049e9c0(0);
                FUN_0049c3c0(demo, DAT_009994bc);
            }
            p0.add(u6).write(u2);
            p1.add(u6).write(b_stack_104[1]);
            p2.add(u6).write(b_stack_104[2]);
        }
        if DAT_009e8f54 != 0 && DAT_005233d8 == 1 {
            FUN_004451e0(0xc0, 8);
        }
        DAT_00527ec8 = 1;
        FUN_0049d010();
        DAT_00527ec8 = 0;
        FUN_004afa90();
        if DAT_009e8f54 != 0 && DAT_005233d8 == 1 {
            FUN_004451f0(0xc0, 8, (&raw mut DAT_009a7c80).cast());
            let u6 = len_plus_one(demo) as usize;
            let u2 = p0.add(u6).read();
            b_stack_104[1] = p1.add(u6).read();
            b_stack_104[2] = p2.add(u6).read();
            p0.add(u6).write(0x70);
            p1.add(u6).write(0x61);
            p2.add(u6).write(0x6c);
            let r = FUN_0049e720(demo, (&raw mut DAT_00523438).cast(), 0, 1);
            if r != 0 {
                _fwrite((&raw mut DAT_009a7c80).cast(), 0x240, 1, DAT_00a07bd8);
                FUN_0049e9c0(1);
            }
            DAT_009994bc = (&raw mut DAT_00612ba0).cast::<u8>();
            FUN_0040e9e0(DAT_009994bc as i32, 0x40, 0x100);
            p0.add(u6).write(0x69);
            p1.add(u6).write(0x6e);
            p2.add(u6).write(0x76);
            let r = FUN_0049e720(demo, (&raw mut DAT_00523438).cast(), 0, 1);
            if r != 0 {
                _fwrite(DAT_009994bc.cast(), 0x100, 0x100, DAT_00a07bd8);
                FUN_0049e9c0(1);
            }
            let pu4 = DAT_009a8074;
            p0.add(u6).write(u2);
            p1.add(u6).write(b_stack_104[1]);
            p2.add(u6).write(b_stack_104[2]);
            FUN_0049c3c0((&raw mut DAT_005236b0).cast(), pu4);
            let mut i5: i32 = 0;
            loop {
                let b = DAT_009a8074.offset(i5 as isize).read();
                let v = DAT_009a8074.offset((0x2fd - i5) as isize).read();
                DAT_009a8074.offset(i5 as isize).write(v >> 2);
                DAT_009a8074.offset((0x2fd - i5) as isize).write(b >> 2);
                let b = DAT_009a8074.offset((i5 + 1) as isize).read();
                let v = DAT_009a8074.offset((0x2fe - i5) as isize).read();
                DAT_009a8074.offset((i5 + 1) as isize).write(v >> 2);
                DAT_009a8074.offset((0x2fe - i5) as isize).write(b >> 2);
                let b = DAT_009a8074.offset((i5 + 2) as isize).read();
                let v = DAT_009a8074.offset((0x2ff - i5) as isize).read();
                DAT_009a8074.offset((i5 + 2) as isize).write(v >> 2);
                let i7 = i5 + 3;
                DAT_009a8074.offset((0x2ff - i5) as isize).write(b >> 2);
                i5 = i7;
                if i7 >= 0x180 {
                    break;
                }
            }
            let f: unsafe extern "C" fn(*mut u8, i32, i32) = core::mem::transmute(DAT_009d77f4);
            f(DAT_009a8074, 0, 0x100);
        }
        b_stack_104[0] = 0;
        b_stack_104[1] = 0;
        b_stack_104[2] = 2;
        let u6 = FUN_0040e940(
            b_stack_104.as_mut_ptr(),
            (&raw mut DAT_009a7bc0) as i32,
            0,
            0x100,
        );
        DAT_005233c8 = u6 as i8;
        DAT_00523408 = u6 as i8;
    }
}
