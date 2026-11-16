// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005375d0: [u32; 8];
    static mut DAT_00536f48: [u32; 8];
    static mut DAT_005397f8: i32;
    static mut DAT_005373b8: i32;
    static mut DAT_005250d4: i32;
    static mut DAT_00536f68: i32;
    static mut _DAT_005371bc: i32;
    static mut DAT_005373c8: i32;
    static mut DAT_005250d8: i32;
    static mut DAT_0052554c: i32;
    static mut DAT_006db5a4: u32;
    static mut DAT_005373c4: i32;
    static mut _DAT_005371b8: i32;
    static mut DAT_005375f0: i32;
    static mut DAT_0072f444: i32;
    static mut DAT_00732a18: i32;
    static mut ram0x0071f2ec: i32;
    static mut DAT_0071b71c: i32;
    static mut DAT_006db5a8: u32;
    static mut _DAT_005373d0: u32;
    static mut DAT_006a89c8: *mut i8;
    static mut DAT_006a89cc: *mut i8;
    static mut DAT_006a89d0: *mut i8;
    static mut DAT_006a89d4: *mut i8;
    static mut DAT_00525568: i32;
    static mut DAT_0085e790: i8;
    static mut DAT_00525558: i32;
    static mut DAT_005250dc: i32;
    static mut DAT_005250e0: i32;
    static mut _DAT_005250e4: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f530() -> i32;
    fn FUN_004330b0() -> i32;
    fn FUN_00432ca0() -> i32;
    fn FUN_00431110(a: i32, b: *mut u32, c: i32);
    fn FUN_00431d30() -> i32;
    fn FUN_00432b70(a: *mut i32, b: *mut u32, c: i32) -> *mut i32;
    fn FUN_004310c0(a: *mut u32, b: i32);
    fn FUN_00438a40();
    fn FUN_00438c30(a: i32, b: *mut core::ffi::c_void, c: i32);
    fn FUN_00432c10(a: i32) -> i32;
    fn FUN_00432cc0(a: i32) -> *mut i32;
    fn FUN_00438b70();
    fn FUN_0042c710();
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn timeGetTime() -> u32;
}

unsafe fn str_len_nul(p: *const u8) -> usize {
    // SAFETY: p points at a NUL-terminated string owned by the game.
    unsafe {
        let mut n = 0usize;
        while *p.add(n) != 0 {
            n += 1;
        }
        n + 1
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0042c130() -> u32 {
    let mut local_110: u32 = 0;
    let mut b_var3: u8 = 1;
    let mut local_10c: i32 = 0;
    let mut local_104: u32 = 0;
    let mut local_100: [u8; 256] = [0; 256];
    // SAFETY: all globals and callees are the game's own; pointers returned
    // by the game point at valid NUL-terminated strings and buffers fit.
    unsafe {
        let p = &raw mut DAT_005375d0 as *mut u32;
        for i in 0..8 {
            p.add(i).write(0);
        }
        let p = &raw mut DAT_00536f48 as *mut u32;
        for i in 0..8 {
            p.add(i).write(0);
        }
        DAT_005397f8 = 0;
        DAT_005373b8 = 0;
        DAT_005250d4 = 0;
        DAT_00536f68 = 0;
        _DAT_005371bc = 0;
        DAT_005373c8 = 0;
        DAT_005250d8 = 0;
        DAT_0052554c = 0;
        DAT_006db5a4 = 0;
        DAT_005373c4 = 5;
        _DAT_005371b8 = 0x16;
        DAT_005375f0 = 0x17;
        if DAT_0072f444 == 1 {
            DAT_00732a18 = 0;
            ram0x0071f2ec = 0x1e;
            DAT_0071b71c = 0;
            DAT_006db5a8 = 0x1e;
            FUN_0048f530();
            return 1;
        }
        let local_108 = FUN_004330b0();
        let mut i_var7 = FUN_00432ca0();
        _DAT_005373d0 = 0x14;
        FUN_00431110(i_var7, &raw mut _DAT_005373d0, 4);
        i_var7 = FUN_00431d30();
        if i_var7 != 0 {
            i_var7 = 0;
            while i_var7 < local_108 {
                loop {
                    let pi = FUN_00432b70(&mut local_10c, &mut local_104, 0x3c);
                    if pi.is_null() {
                        return 0;
                    }
                    if pi.read() == 0x14 {
                        break;
                    }
                }
                i_var7 = i_var7.wrapping_add(1);
            }
            _DAT_005373d0 = 0x13;
            FUN_004310c0(&raw mut _DAT_005373d0, 4);
        }
        FUN_00438a40();
        FUN_00438c30(1, DAT_006a89c8 as *mut core::ffi::c_void, 0x30);
        loop {
            let pi = FUN_00432b70(&mut local_10c, &mut local_104, 0x3c);
            if pi.is_null() {
                return 0;
            }
            if pi.read() == 0x1d {
                let d_var5 = timeGetTime();
                if (d_var5.wrapping_sub(local_110) as i32) > 200 {
                    let i = FUN_00432c10(local_10c);
                    let p6 = FUN_00432cc0(i);
                    if p6.is_null() {
                        let src = DAT_006a89cc as *const u8;
                        let n = str_len_nul(src);
                        for k in 0..n {
                            local_100[k] = *src.add(k);
                        }
                    } else {
                        let src = p6 as *const u8;
                        let n = str_len_nul(src);
                        for k in 0..n {
                            local_100[k] = *src.add(k);
                        }
                        b_var3 = (b_var3 == 0) as u8;
                        let s2 = if b_var3 != 0 {
                            DAT_006a89d0
                        } else {
                            DAT_006a89d4
                        } as *const u8;
                        let n2 = str_len_nul(s2);
                        let mut end = 0usize;
                        while local_100[end] != 0 {
                            end += 1;
                        }
                        for k in 0..n2 {
                            local_100[end + k] = *s2.add(k);
                        }
                    }
                    FUN_00438c30(3, local_100.as_mut_ptr() as *mut core::ffi::c_void, 0x30);
                    local_110 = d_var5;
                }
            }
            if pi.read() == 0x13 {
                FUN_00438b70();
                let i = FUN_00432ca0();
                _DAT_005373d0 = 0x1b;
                FUN_00431110(i, &raw mut _DAT_005373d0, 4);
                FUN_0048f530();
                DAT_00732a18 = 0;
                DAT_0071b71c = 0;
                DAT_006db5a8 = 0;
                if DAT_00525568 == 0 {
                    ram0x0071f2ec = 0x1e;
                } else {
                    ram0x0071f2ec = 0x82;
                }
                if DAT_0085e790 == 4 {
                    DAT_00525558 = 0x3b;
                } else if DAT_0085e790 == 6 {
                    DAT_00525558 = 0x27;
                } else {
                    DAT_00525558 = 0x1d;
                }
                let mut i_var7 = FUN_00431d30();
                if i_var7 != 0 {
                    DAT_005250d8 = 1;
                    if local_108 != 1 {
                        DAT_005250d8 = 2;
                    }
                    DAT_005250dc = 0;
                    DAT_005250e0 = 0;
                    _DAT_005250e4 = 1;
                    while DAT_005250d8 != 0 && DAT_006db5a8 < 100 {
                        FUN_0042c710();
                        i_var7 = FUN_0048f530();
                        DAT_006db5a8 = DAT_006db5a8.wrapping_add(i_var7 as u32);
                    }
                    DAT_005250d8 = 0;
                    i_var7 = FUN_0048f530();
                    DAT_006db5a8 = DAT_006db5a8.wrapping_add(i_var7 as u32);
                    ram0x0071f2ec = DAT_006db5a8 as i32;
                    if (DAT_006db5a8 as i32) < 0x23 {
                        i_var7 = 0x23u32.wrapping_sub(DAT_006db5a8) as i32;
                        DAT_006db5a8 = DAT_006db5a8.wrapping_add(i_var7 as u32);
                        DAT_0052554c = (DAT_0052554c as u32).wrapping_sub(i_var7 as u32) as i32;
                    }
                }
                return 1;
            }
        }
    }
}
