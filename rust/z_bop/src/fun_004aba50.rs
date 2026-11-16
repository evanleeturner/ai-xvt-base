// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00527ea0: i32;
    static mut DAT_005233f4: *mut u32;
    static mut DAT_00527ecc: *mut u32;
    static mut DAT_009d8c08: i32;
    static mut DAT_00527f78: i32;
    static mut DAT_00527eac: i32;
    static mut DAT_00527ed0: i32;
    static mut DAT_0066e710: u32;
    static mut DAT_00803f90: u32;
    static mut _DAT_009ed23b: [u8; 2];
    static mut DAT_0066e700: *mut i32;
    static mut DAT_00803f80: *mut i32;
    static mut DAT_0066ddcc: *mut i32;
    static mut DAT_00803b70: i32;
    static mut DAT_0066ddc4: i32;
    static mut DAT_005233e0: i32;
    static mut DAT_005233d8: i32;
    static mut DAT_0066dde0: i32;
    static mut DAT_005233e8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d5410() -> *mut u32;
    fn FUN_004d5070() -> i32;
    fn FUN_004ac790(param_1: *mut u32);
    fn FUN_004aba10() -> i32;
    fn FUN_004ac780(param_1: i32);
    fn FUN_0040e0c0(param_1: i32, param_2: i32, param_3: i32, param_4: i32);
}

unsafe fn clear(l: &mut [i32; 27]) {
    let mut i = 0;
    while i < 27 {
        l[i] = 0;
        i += 1;
    }
    l[0] = 0x6c;
}

unsafe fn slot(obj: *mut i32, off: u32) -> *const core::ffi::c_void {
    // SAFETY: obj is a game COM-style object; its first word is the vtable.
    unsafe {
        let vt = obj.read_unaligned();
        let f = ((vt as u32).wrapping_add(off) as usize as *const i32).read_unaligned();
        f as u32 as usize as *const core::ffi::c_void
    }
}

unsafe fn create(obj: *mut i32, l: *mut i32) -> i32 {
    // SAFETY: calls the game's vtable entry at offset 100.
    unsafe {
        let f: unsafe extern "stdcall" fn(*mut i32, i32, *mut i32, i32, i32) -> i32 =
            core::mem::transmute(slot(obj, 100));
        f(obj, 0, l, 0, 0)
    }
}

unsafe fn desc(obj: *mut i32, l: *mut i32) {
    // SAFETY: calls the game's vtable entry at offset 0x58.
    unsafe {
        let f: unsafe extern "stdcall" fn(*mut i32, *mut i32) -> i32 =
            core::mem::transmute(slot(obj, 0x58));
        f(obj, l);
    }
}

unsafe fn adjust(l: &mut [i32; 27]) {
    // SAFETY: the globals are the game's own.
    unsafe {
        let iv = ((DAT_0066ddc4 as u32).wrapping_sub(DAT_005233e0 as u32) >> 1)
            .wrapping_mul(DAT_005233d8 as u32);
        l[2] = (((DAT_0066dde0 as u32).wrapping_sub(DAT_005233e8 as u32) >> 1)
            .wrapping_mul(l[2] as u32)) as i32;
        DAT_005233f4 = (DAT_005233f4 as usize as u32)
            .wrapping_add(l[2] as u32)
            .wrapping_add(iv) as usize as *mut u32;
        DAT_00527ecc = (DAT_00527ecc as usize as u32)
            .wrapping_add(l[2] as u32)
            .wrapping_add(iv) as usize as *mut u32;
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004aba50() {
    let mut local_6c = [0i32; 27];
    // SAFETY: all globals and callees are the game's own memory and code.
    unsafe {
        if DAT_00527ea0 == 1 {
            DAT_005233f4 = FUN_004d5410();
            DAT_00527ecc = DAT_005233f4;
            DAT_009d8c08 = FUN_004d5070();
            return;
        }
        if DAT_00527f78 != 0 {
            DAT_00527f78 = DAT_00527f78.wrapping_add(1);
            return;
        }
        DAT_00527f78 = 1;
        if DAT_00527eac == 0 {
            if DAT_00527ed0 != 0 {
                FUN_004ac790(&raw mut DAT_0066e710);
                let i = FUN_004aba10();
                FUN_004ac780((i as u32).wrapping_mul(0x1e0) as i32);
                DAT_005233f4 = &raw mut DAT_0066e710;
                DAT_00527ecc = &raw mut DAT_0066e710;
                return;
            }
            FUN_004ac790(&raw mut DAT_00803f90);
            let i = FUN_004aba10();
            FUN_004ac780((i as u32).wrapping_mul(0x1e0) as i32);
            DAT_005233f4 = &raw mut DAT_00803f90;
            DAT_00527ecc = &raw mut DAT_00803f90;
        } else {
            let v = u16::from_ne_bytes(_DAT_009ed23b);
            if v == 0 {
                clear(&mut local_6c);
                loop {
                    let r = create(DAT_0066e700, local_6c.as_mut_ptr());
                    if r == 0 {
                        let p = local_6c[9] as u32 as usize as *mut u32;
                        FUN_004ac790(p);
                        DAT_005233f4 = p;
                        DAT_00527ecc = p;
                        clear(&mut local_6c);
                        desc(DAT_0066e700, local_6c.as_mut_ptr());
                        DAT_00803b70 = local_6c[2];
                        adjust(&mut local_6c);
                        let i = FUN_004aba10();
                        FUN_004ac780((i as u32).wrapping_mul(0x1e0) as i32);
                        return;
                    }
                    if r != -0x7789fde4 {
                        break;
                    }
                }
                return;
            }
            if DAT_00527ed0 == 0 {
                clear(&mut local_6c);
                loop {
                    let r = create(DAT_00803f80, local_6c.as_mut_ptr());
                    if r == 0 {
                        break;
                    }
                    if r != -0x7789fde4 {
                        return;
                    }
                }
                let p = local_6c[9] as u32 as usize as *mut u32;
                FUN_004ac790(p);
                DAT_005233f4 = p;
                DAT_00527ecc = p;
                clear(&mut local_6c);
                desc(DAT_00803f80, local_6c.as_mut_ptr());
                DAT_00803b70 = local_6c[2];
                adjust(&mut local_6c);
                let i = FUN_004aba10();
                FUN_004ac780((i as u32).wrapping_mul(0x1e0) as i32);
                if DAT_009d8c08 != DAT_00803b70 {
                    DAT_009d8c08 = DAT_00803b70;
                    FUN_0040e0c0(DAT_005233f4 as usize as i32, DAT_00803b70, 0x1e0, -1);
                    return;
                }
            } else {
                clear(&mut local_6c);
                loop {
                    let r = create(DAT_0066ddcc, local_6c.as_mut_ptr());
                    if r == 0 {
                        break;
                    }
                    if r != -0x7789fde4 {
                        return;
                    }
                }
                let p = local_6c[9] as u32 as usize as *mut u32;
                FUN_004ac790(p);
                DAT_005233f4 = p;
                DAT_00527ecc = p;
                clear(&mut local_6c);
                desc(DAT_0066ddcc, local_6c.as_mut_ptr());
                DAT_00803b70 = local_6c[2];
                let i = FUN_004aba10();
                FUN_004ac780((i as u32).wrapping_mul(0x1e0) as i32);
                if DAT_009d8c08 != DAT_00803b70 {
                    DAT_009d8c08 = DAT_00803b70;
                    FUN_0040e0c0(DAT_005233f4 as usize as i32, DAT_00803b70, 0x1e0, -1);
                    return;
                }
            }
        }
    }
}
