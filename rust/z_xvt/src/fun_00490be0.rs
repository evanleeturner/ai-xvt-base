// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00658654: *mut i32;
    static mut DAT_0065865c: [i8; 0];
    static mut DAT_006587a0: [u8; 0];
    static mut DAT_006a8814: i32;
    static mut DAT_006a8818: i32;
    static mut DAT_006a881c: i32;
    static mut DAT_006a860c: [i32; 0];
    static mut DAT_006a8610: [i32; 0];
    static mut DAT_006a8614: [i32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn strncmp(a: *const i8, b: *const i8, n: u32) -> i32;
    fn FUN_00417c20(param_1: *mut i32, param_2: *mut i8) -> i32;
}

unsafe fn vt(p: *mut i32, off: u32) -> *const core::ffi::c_void {
    // SAFETY: p is a COM-style object whose first word is a vtable address.
    unsafe {
        let table = p.read() as u32;
        (table.wrapping_add(off) as *const *const core::ffi::c_void).read_unaligned()
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00490be0(
    param_1: *mut i8,
    param_2: i32,
    param_3: i32,
    param_4: *mut i8,
    param_5: i32,
    param_6: i32,
) -> i32 {
    // SAFETY: globals and callees are the game's own; vtable pointers are
    // COM-style objects owned by the game, called as the original does.
    unsafe {
        type F1 = unsafe extern "stdcall" fn(*mut i32) -> i32;
        type F2 = unsafe extern "stdcall" fn(*mut i32, i32) -> i32;
        type F2p = unsafe extern "stdcall" fn(*mut i32, *mut u32) -> i32;
        type F3p = unsafe extern "stdcall" fn(*mut i32, i32, *mut i32) -> i32;
        type F4 = unsafe extern "stdcall" fn(*mut i32, i32, i32, i32) -> i32;

        let a0 = (&raw mut DAT_006a860c).cast::<i32>();
        let a1 = (&raw mut DAT_006a8610).cast::<i32>();
        let a2 = (&raw mut DAT_006a8614).cast::<i32>();
        let names = (&raw mut DAT_0065865c).cast::<i8>();
        let prios = (&raw mut DAT_006587a0).cast::<u8>();

        if DAT_00658654.is_null() {
            return 0;
        }
        if *param_1 == 0 {
            return 0;
        }
        let mut i_var3: i32 = 0;
        let mut p_cur: *mut i8 = names;
        let mut i_var4: i32 = DAT_006a8814.wrapping_sub(1);
        let mut i_var5: i32;
        'found: {
            while -1 < i_var4 {
                i_var5 = i_var4 >> 1;
                let str1 = p_cur.offset((i_var5 as u32).wrapping_mul(0x145) as i32 as isize);
                let r = strncmp(str1, param_1, 0x40);
                if r == 0 {
                    i_var5 = i_var5.wrapping_add(i_var3);
                    break 'found;
                }
                if i_var4 < 1 {
                    break;
                }
                if r < 0 {
                    i_var3 = i_var5.wrapping_add(1).wrapping_add(i_var3);
                    i_var4 = i_var4.wrapping_sub(i_var5.wrapping_add(1));
                    p_cur = str1.offset(0x145);
                } else {
                    i_var4 = i_var5.wrapping_sub(1);
                }
            }
            i_var5 = -1;
        }
        if i_var5 == -1 {
            return 0;
        }
        let entry = (i_var5 as u32).wrapping_mul(0x145) as i32 as isize;
        let mut u_stack_4: u32 = 0;

        if DAT_006a8818 == 8 {
            i_var4 = 0;
            loop {
                let idx = (i_var4 * 3) as isize;
                if a0.offset(idx).read() != -1 {
                    let p = a2.offset(idx).read() as *mut i32;
                    let f: F2p = core::mem::transmute(vt(p, 0x24));
                    let r = f(p, &raw mut u_stack_4);
                    if r != 0 || (u_stack_4 & 5) == 0 {
                        let p = a2.offset(idx).read() as *mut i32;
                        let f: F1 = core::mem::transmute(vt(p, 8));
                        f(p);
                        a0.offset(idx).write(-1);
                        a2.offset(idx).write(0);
                        DAT_006a8818 = DAT_006a8818.wrapping_sub(1);
                        break;
                    }
                }
                i_var4 = i_var4.wrapping_add(1);
                if !(i_var4 < 8) {
                    break;
                }
            }
            if i_var4 == 8 {
                i_var4 = 8;
                i_var3 = 0;
                let mut best: i32 = param_4 as usize as u32 as i32;
                loop {
                    let k = a0.offset((i_var3 * 3) as isize).read();
                    let v = prios
                        .offset((k as u32).wrapping_mul(0x145) as i32 as isize)
                        .read();
                    if (v as i32) < best {
                        i_var4 = i_var3;
                        best = v as i32;
                    }
                    i_var3 = i_var3.wrapping_add(1);
                    if !(i_var3 < 8) {
                        break;
                    }
                }
                if i_var4 == 8 {
                    if param_2 != 0 {
                        i_var4 = 0;
                        loop {
                            let idx = (i_var4 * 3) as isize;
                            if a0.offset(idx).read() == i_var5 {
                                let p = a2.offset(idx).read() as *mut i32;
                                let f: F2 = core::mem::transmute(vt(p, 0x34));
                                f(p, 0);
                                a1.offset(idx).write(DAT_006a881c);
                                DAT_006a881c = DAT_006a881c.wrapping_add(1);
                                return 1;
                            }
                            i_var4 = i_var4.wrapping_add(1);
                            if !(i_var4 < 8) {
                                break;
                            }
                        }
                    }
                    return 0;
                }
                let idx = (i_var4 * 3) as isize;
                let p = a2.offset(idx).read() as *mut i32;
                if a0.offset(idx).read() == i_var5 {
                    let f: F2 = core::mem::transmute(vt(p, 0x34));
                    f(p, 0);
                    a1.offset(idx).write(DAT_006a881c);
                    DAT_006a881c = DAT_006a881c.wrapping_add(1);
                    return 1;
                }
                let f: F1 = core::mem::transmute(vt(p, 0x48));
                f(p);
                let p = a2.offset(idx).read() as *mut i32;
                let f: F1 = core::mem::transmute(vt(p, 8));
                f(p);
                a0.offset(idx).write(-1);
                a2.offset(idx).write(0);
                DAT_006a8818 = DAT_006a8818.wrapping_sub(1);
            }
        } else {
            i_var4 = 0;
            loop {
                if a0.offset((i_var4 * 3) as isize).read() == -1 {
                    break;
                }
                i_var4 = i_var4.wrapping_add(1);
                if !(i_var4 < 8) {
                    break;
                }
            }
        }
        let slot = (i_var4.wrapping_mul(3)) as isize;
        let piv7 = DAT_00658654;
        i_var3 = names
            .offset(entry)
            .offset(0x140)
            .cast::<i32>()
            .read_unaligned();
        let mut i_stack_c: i32 = 0;
        let f: F3p = core::mem::transmute(vt(piv7, 0x14));
        f(piv7, i_var3, &raw mut i_stack_c);
        let piv8 = i_stack_c as usize as *mut i32;
        if piv8.is_null() {
            return 0;
        }
        let f: F2 = core::mem::transmute(vt(piv8, 0x34));
        f(piv8, 0);
        let mut p5 = param_5;
        if p5 < 0x80 {
            if p5 < 0 {
                p5 = 0;
            }
        } else {
            p5 = 0x7f;
        }
        let f: F2 = core::mem::transmute(vt(piv8, 0x3c));
        f(
            piv8,
            ((p5 as u32)
                .wrapping_mul(5)
                .wrapping_sub(0x27b)
                .wrapping_mul(400) as i32)
                / 0x7f,
        );
        u_stack_4 = param_6 as u32;
        if 0x7f < (u_stack_4 as i32) {
            u_stack_4 = 0x7f;
        }
        if (u_stack_4 as i32) < 0 {
            u_stack_4 = 0;
        }
        let f: F2 = core::mem::transmute(vt(piv8, 0x40));
        f(
            piv8,
            (u_stack_4
                .wrapping_mul(5)
                .wrapping_sub(0x13b)
                .wrapping_mul(400) as i32)
                / 0x3f,
        );
        let f: F4 = core::mem::transmute(vt(piv8, 0x30));
        i_var3 = f(piv8, 0, 0, (param_3 == 1) as i32);
        if i_var3 == -0x7787ff6a {
            let i_var1 = names
                .offset(entry)
                .offset(0x140)
                .cast::<i32>()
                .read_unaligned();
            i_var3 = FUN_00417c20(
                i_var1 as usize as *mut i32,
                names.offset(entry).offset(0x40),
            );
            if i_var3 == 1 {
                let f: F2 = core::mem::transmute(vt(piv8, 0x34));
                f(piv8, 0);
                let f: F4 = core::mem::transmute(vt(piv8, 0x30));
                i_var3 = f(piv8, 0, 0, (param_3 == 1) as i32);
                if i_var3 != 0 {
                    return 0;
                }
                a0.offset(slot).write(i_var5);
                a2.offset(slot).write(piv8 as usize as i32);
                a1.offset(slot).write(DAT_006a881c);
                DAT_006a881c = DAT_006a881c.wrapping_add(1);
                DAT_006a8818 = DAT_006a8818.wrapping_add(1);
                return 0;
            }
        } else if i_var3 == 0 {
            a0.offset(slot).write(i_var5);
            a2.offset(slot).write(piv8 as usize as i32);
            a1.offset(slot).write(DAT_006a881c);
            DAT_006a881c = DAT_006a881c.wrapping_add(1);
            DAT_006a8818 = DAT_006a8818.wrapping_add(1);
            i_var3 = 1;
        }
        i_var3
    }
}
