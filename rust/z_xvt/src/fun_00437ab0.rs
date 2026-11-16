// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005ef238: i32;
    static mut DAT_0073290c: i32;
    static mut DAT_00525554: i32;
    static mut DAT_005ef23c: i32;
    static mut DAT_005ef230: i32;
    static mut DAT_005ef234: i32;
    static mut DAT_00525790: [i32; 20];
    static mut DAT_005257e0: i32;
    static mut DAT_006db5a8: u32;
    static mut DAT_0071b71c: u32;
    static mut DAT_00732a18: u32;
    static mut DAT_006db5a4: i32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071b740: i8;
    static mut DAT_0071e528: i8;
    static mut DAT_0072f444: i32;
    static mut DAT_007092c0: i8;
    static mut DAT_00732e48: i32;
    static mut DAT_00525558: i32;
    static mut DAT_00525550: i32;
    static mut DAT_0052554c: i32;
    static mut ram0x0071f2ec: i32;
    static mut DAT_0070aaf0: [i32; 0];
    static mut DAT_006a89a8: *mut i8;
    static mut DAT_006a89c4: *mut i8;
    static mut DAT_006a89b0: *mut u8;
    static mut DAT_006a89c0: *mut u8;
    static mut DAT_006d4a4e: u16;
    static mut DAT_00525520: i32;
    static mut DAT_0052551c: i32;
    static mut DAT_00525524: i32;
    static mut DAT_00525528: i32;
    static mut DAT_0052ae54: i32;
    static mut DAT_00732a30: *const core::ffi::c_void;
    static mut DAT_0071e51c: i32;
    static mut DAT_006db57c: i32;
    static mut DAT_00525720: i8;
    static mut DAT_00732800: [i8; 0];
    static mut DAT_0071b704: i32;
    static mut DAT_00732904: i32;
    static mut DAT_006d4988: i32;
    static mut s_Fell_Behind__tickcounter___7d_se_00525a0c: [i8; 0];
    static mut s_Too_far_Ahead__tickcounter___7d_s_005259d0: [i8; 0];
    static mut s_R___2d_U___2d_N___2d_O___2d_T____00525980: [i8; 0];
    static mut s_Raw_0__2d_1__2d_2__2d_3__2d_4__2_00525934: [i8; 0];
    static mut s_Raw_10__2d_11__2d_12__2d_13__2d_1_005258e8: [i8; 0];
    static mut s_Pct_0__2d_1__2d_2__2d_3__2d_4__2_0052589c: [i8; 0];
    static mut s_Pct_10__2d_11__2d_12__2d_13__2d_1_00525850: [i8; 0];
    static mut s________Long_Update___d_______War_00525804: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    #[link_name = "sprintf"]
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_0048f530() -> i32;
    fn FUN_0042c710();
    fn FUN_00431d30() -> i32;
    fn FUN_0042d530();
    fn FUN_0049a600(param_1: u32);
    fn FUN_00490a00();
    fn FUN_0042c5c0(param_1: i32);
    fn FUN_00438a40();
    fn FUN_0042b340() -> *mut i8;
    fn FUN_00438c30(param_1: i32, param_2: *mut u8, param_3: i32);
    fn FUN_004a53f0() -> i32;
    fn FUN_004a5470() -> u32;
    fn FUN_0042c5a0();
    fn FUN_0048ee20(param_1: i8);
    fn FUN_004a7040();
    fn FUN_004a6a80();
    fn FUN_004a6f90();
    fn FUN_0049adc0(param_1: i32);
    fn FUN_00432ca0() -> i32;
    fn FUN_00433680(param_1: i32) -> i32;
    fn FUN_0049b4e0();
    fn FUN_0049b2e0();
    fn FUN_0042c670(param_1: i32);
    fn FUN_0042c600(param_1: i32);
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_0045bb40();
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, nargs: u32, args: *const u32);
}

unsafe fn slot(i: i32) -> *mut i8 {
    // SAFETY: the game's player table, indexed as the original does.
    unsafe { (&raw mut DAT_0071b740).offset(i.wrapping_mul(0x5bd) as isize) }
}

unsafe fn count_players() {
    // SAFETY: walks the game's own player table between its two addresses.
    unsafe {
        let mut n: i32 = 0;
        let mut p = &raw mut DAT_0071b740 as *const i8;
        let end = &raw mut DAT_0071e528 as *const i8;
        loop {
            let c = *p;
            if c == 1 || c == 2 {
                DAT_0072f444 = DAT_0072f444.wrapping_add(1);
            }
            if c == 1 {
                n = n.wrapping_add(1);
            }
            p = p.wrapping_offset(0x5bd);
            if !(p < end) {
                break;
            }
        }
        if n == 0 {
            DAT_007092c0 = 1;
        }
        if *slot(DAT_0071ecf4) == 0 {
            DAT_007092c0 = 1;
        }
    }
}

unsafe fn smooth(t: u32) -> u32 {
    // SAFETY: reads the game's globals only.
    unsafe {
        let mut r = t;
        if DAT_005ef238 != 0 {
            r = (DAT_005ef234 as u32).wrapping_add(DAT_005ef238 as u32);
            let q = DAT_005ef234 >> 3;
            if r < t {
                let d = t.wrapping_sub(r) as i32;
                let mut n = d;
                if q < d {
                    n = q;
                }
                if n == 0 {
                    n = 1;
                }
                if 4 < n {
                    n = d;
                }
                r = r.wrapping_add(n as u32);
            } else if t < r {
                let d = r.wrapping_sub(t) as i32;
                let mut n = d;
                if q < d {
                    n = q;
                }
                if n == 0 {
                    n = 1;
                }
                if 4 < n {
                    n = d;
                }
                r = r.wrapping_sub(n as u32);
            }
        }
        r
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00437ab0() {
    let mut abstack_36c = [0u8; 80];
    let mut local_31c = [0u8; 256];
    let mut acstack_21c = [0i8; 180];
    let mut local_168 = [0i8; 180];
    let mut local_b4 = [0i8; 180];
    let mut i3: i32 = 0;
    let mut i5: i32 = 0;
    let mut i7: i32 = 0;
    let mut i8: i32;
    let mut i12: i32 = 0;
    let mut i17: i32 = 0;
    let mut local_378: i32 = 0;
    let mut u374: u32 = 0;
    // SAFETY: every global, callee and pointer is the game's own memory, used
    // exactly as the original does; local buffers are sized as in the original.
    unsafe {
        DAT_005ef238 = 0;
        DAT_0073290c = 0;
        DAT_00525554 = 0;
        DAT_005ef23c = 0;
        DAT_005ef230 = 0;
        let arr = (&raw mut DAT_00525790).cast::<i32>();
        let mut k: i32 = 0x14;
        let mut pa = arr;
        while k != 0 {
            *pa = 0;
            pa = pa.add(1);
            k -= 1;
        }
        loop {
            loop {
                local_378 = 0;
                i7 = FUN_0048f530();
                DAT_006db5a8 = DAT_006db5a8.wrapping_add(i7 as u32);
                i17 = DAT_006db5a8 as i32;
                i7 = FUN_0048f530();
                i7 = DAT_006db5a8.wrapping_add(i7 as u32) as i32;
                i3 = (i7 as u32).wrapping_sub(DAT_0071b71c) as i32;
                loop {
                    DAT_006db5a8 = i7 as u32;
                    if !(i3 < 8) {
                        break;
                    }
                    i7 = FUN_0048f530();
                    i7 = DAT_006db5a8.wrapping_add(i7 as u32) as i32;
                    i3 = (i7 as u32).wrapping_sub(DAT_0071b71c) as i32;
                }
                FUN_0042c710();
                i3 = FUN_00431d30();
                if i3 == 0 && 0x1ba8 < DAT_006db5a4 {
                    *slot(DAT_0071ecf4) = 0;
                    FUN_0042c670(DAT_0071ecf4);
                    return;
                }
                i3 = FUN_0048f530();
                DAT_0072f444 = 0;
                i3 = DAT_006db5a8.wrapping_add(i3 as u32) as i32;
                count_players();
                DAT_006db5a8 = i3 as u32;
                if DAT_007092c0 == 0 {
                    break;
                }
                if DAT_00732a18 == DAT_0071b71c {
                    return;
                }
                if 0x49c < (i3 as u32).wrapping_sub(DAT_0071b71c) as i32 {
                    return;
                }
                i7 = FUN_0048f530();
                DAT_006db5a8 = DAT_006db5a8.wrapping_add(i7 as u32);
                i7 = DAT_006db5a8.wrapping_sub(DAT_0071b71c) as i32;
                while i7 < 8 {
                    i7 = FUN_0048f530();
                    DAT_006db5a8 = DAT_006db5a8.wrapping_add(i7 as u32);
                    i7 = DAT_006db5a8.wrapping_sub(DAT_0071b71c) as i32;
                }
            }
            if DAT_00732e48 == 1 {
                i8 = FUN_0048f530();
                let u9 = DAT_006db5a8.wrapping_add(i8 as u32);
                let mut u18 = smooth(u9);
                if (u18.wrapping_sub(DAT_0071b71c) as i32) < 4 {
                    u18 = DAT_0071b71c.wrapping_add(4);
                }
                DAT_005ef234 = u18.wrapping_sub(DAT_005ef238 as u32) as i32;
                DAT_006db5a8 = u18;
                FUN_0042d530();
                DAT_00525558 = DAT_006db5a8.wrapping_sub(DAT_0071b71c) as i32;
                DAT_00525550 = 0;
                FUN_0049a600(DAT_006db5a8);
                DAT_0071b71c = DAT_006db5a8;
                DAT_00732a18 = DAT_006db5a8;
                DAT_005ef238 = DAT_006db5a8 as i32;
                DAT_006db5a8 = DAT_006db5a8.wrapping_sub(u18).wrapping_add(u9);
                FUN_00490a00();
            } else {
                i8 = FUN_0048f530();
                DAT_006db5a8 = DAT_006db5a8.wrapping_add(i8 as u32);
                i8 = DAT_005ef234 >> 3;
                if DAT_00732a18.wrapping_add(ram0x0071f2ec as u32) < DAT_006db5a8 {
                    i12 = (DAT_006db5a8
                        .wrapping_sub(ram0x0071f2ec as u32)
                        .wrapping_sub(DAT_00732a18) as i32)
                        >> 4;
                    if i12 == 0 {
                        i12 = 1;
                    }
                    if i8 < i12 {
                        i12 = i8;
                    }
                    i5 = 0i32.wrapping_sub(i12);
                    DAT_006db5a8 = DAT_006db5a8.wrapping_add(i5 as u32);
                    DAT_0052554c = DAT_0052554c.wrapping_add(i12);
                } else if DAT_006db5a8 < DAT_00732a18.wrapping_add(ram0x0071f2ec as u32) {
                    i5 = (DAT_00732a18
                        .wrapping_sub(DAT_006db5a8)
                        .wrapping_add(ram0x0071f2ec as u32) as i32)
                        >> 4;
                    if i5 == 0 {
                        i5 = 1;
                    }
                    if i8 < i5 {
                        i5 = i8;
                    }
                    i12 = 0i32.wrapping_sub(i5);
                    DAT_006db5a8 = DAT_006db5a8.wrapping_add(i5 as u32);
                    DAT_0052554c = DAT_0052554c.wrapping_add(i12);
                }
                if (DAT_006db5a8 as i32) < (DAT_00732a18 as i32) {
                    _sprintf(
                        local_b4.as_mut_ptr(),
                        (&raw const s_Fell_Behind__tickcounter___7d_se_00525a0c).cast::<i8>(),
                        DAT_006db5a8,
                        DAT_00732a18,
                        DAT_00732a18
                            .wrapping_sub(DAT_006db5a8)
                            .wrapping_add(ram0x0071f2ec as u32),
                    );
                    i8 = DAT_00732a18
                        .wrapping_sub(DAT_006db5a8)
                        .wrapping_add(ram0x0071f2ec as u32) as i32;
                    DAT_006db5a8 = DAT_006db5a8.wrapping_add(i8 as u32);
                    DAT_0052554c = DAT_0052554c.wrapping_sub(i8);
                }
                let mut far = false;
                if ((ram0x0071f2ec as u32).wrapping_add(0x674) as i32)
                    < (DAT_006db5a8.wrapping_sub(DAT_00732a18) as i32)
                {
                    i8 = FUN_00431d30();
                    if i8 == 0 {
                        far = true;
                    }
                }
                if far {
                    FUN_0042c5c0(DAT_0071ecf4);
                    _sprintf(
                        local_168.as_mut_ptr(),
                        (&raw const s_Too_far_Ahead__tickcounter___7d_s_005259d0).cast::<i8>(),
                        DAT_006db5a8,
                        DAT_00732a18,
                        DAT_006db5a8.wrapping_sub(DAT_00732a18),
                    );
                    i5 = 0;
                    i8 = FUN_0048f530();
                    DAT_006db5a4 = 0;
                    DAT_006db5a8 = DAT_006db5a8.wrapping_add(i8 as u32);
                    FUN_00438a40();
                    let mut u18: u32 = 0xffffffff;
                    let mut pc4 = DAT_006a89a8;
                    let mut pc14;
                    loop {
                        pc14 = pc4;
                        if u18 == 0 {
                            break;
                        }
                        u18 = u18.wrapping_sub(1);
                        pc14 = pc4.add(1);
                        let c2 = *pc4;
                        pc4 = pc14;
                        if c2 == 0 {
                            break;
                        }
                    }
                    u18 = !u18;
                    let mut pb10 = pc14.wrapping_sub(u18 as usize) as *const u8;
                    let mut pb15 = local_31c.as_mut_ptr();
                    let mut u9 = u18 >> 2;
                    while u9 != 0 {
                        pb15.cast::<u32>()
                            .write_unaligned(pb10.cast::<u32>().read_unaligned());
                        pb10 = pb10.add(4);
                        pb15 = pb15.add(4);
                        u9 = u9.wrapping_sub(1);
                    }
                    u18 &= 3;
                    while u18 != 0 {
                        *pb15 = *pb10;
                        pb10 = pb10.add(1);
                        pb15 = pb15.add(1);
                        u18 -= 1;
                    }
                    pc4 = FUN_0042b340();
                    if !pc4.is_null() {
                        u18 = 0xffffffff;
                        loop {
                            pc14 = pc4;
                            if u18 == 0 {
                                break;
                            }
                            u18 = u18.wrapping_sub(1);
                            pc14 = pc4.add(1);
                            let c2 = *pc4;
                            pc4 = pc14;
                            if c2 == 0 {
                                break;
                            }
                        }
                        u18 = !u18;
                        i8 = -1;
                        let mut pb10b = local_31c.as_mut_ptr() as *const u8;
                        loop {
                            pb15 = pb10b as *mut u8;
                            if i8 == 0 {
                                break;
                            }
                            i8 = i8.wrapping_sub(1);
                            pb15 = pb10b.add(1) as *mut u8;
                            let b1 = *pb10b;
                            pb10b = pb15;
                            if b1 == 0 {
                                break;
                            }
                        }
                        pb10 = pc14.wrapping_sub(u18 as usize) as *const u8;
                        pb15 = pb15.sub(1);
                        u9 = u18 >> 2;
                        while u9 != 0 {
                            pb15.cast::<u32>()
                                .write_unaligned(pb10.cast::<u32>().read_unaligned());
                            pb10 = pb10.add(4);
                            pb15 = pb15.add(4);
                            u9 = u9.wrapping_sub(1);
                        }
                        u18 &= 3;
                        while u18 != 0 {
                            *pb15 = *pb10;
                            pb10 = pb10.add(1);
                            pb15 = pb15.add(1);
                            u18 -= 1;
                        }
                    }
                    FUN_00438c30(1, local_31c.as_mut_ptr(), 0x34);
                    loop {
                        if (DAT_006db5a8.wrapping_sub(DAT_00732a18) as i32) <= ram0x0071f2ec {
                            break;
                        }
                        i8 = FUN_004a53f0();
                        if i8 != 0 {
                            u18 = FUN_004a5470();
                            if u18 == 0x1b {
                                DAT_007092c0 = 1;
                                FUN_0042c600(DAT_0071ecf4);
                                i7 = DAT_0071ecf4;
                                *(&raw mut DAT_0070aaf0)
                                    .cast::<i32>()
                                    .offset(DAT_0071ecf4 as isize) = 1;
                                *slot(i7) = 0;
                                FUN_0042c670(i7);
                                return;
                            }
                        }
                        i8 = DAT_006db5a8 as i32;
                        FUN_0042c710();
                        i12 = FUN_0048f530();
                        DAT_006db5a4 = DAT_006db5a4.wrapping_add(
                            (DAT_006db5a8.wrapping_add(i12 as u32)).wrapping_sub(i8 as u32) as i32,
                        );
                        i5 = (i5 as u32)
                            .wrapping_sub(i8 as u32)
                            .wrapping_add(DAT_006db5a8)
                            .wrapping_add(i12 as u32) as i32;
                        DAT_006db5a8 = i8 as u32;
                        if 0x1ba8 < DAT_006db5a4 {
                            DAT_007092c0 = 1;
                            FUN_0042c600(DAT_0071ecf4);
                            i7 = DAT_0071ecf4;
                            *(&raw mut DAT_0070aaf0)
                                .cast::<i32>()
                                .offset(DAT_0071ecf4 as isize) = 1;
                            *slot(i7) = 0;
                            FUN_0042c670(DAT_0071ecf4);
                            return;
                        }
                        if 0xec < i5 {
                            i5 = 0;
                            FUN_0042c5a0();
                        }
                        u18 = (0x1ba8u32.wrapping_sub(DAT_006db5a4 as u32) as i32 / 0x76) as u32;
                        if u18 != u374 {
                            let pb: *mut u8;
                            if (u18 as i32) < 0x32 {
                                _sprintf(
                                    abstack_36c.as_mut_ptr().cast::<i8>(),
                                    DAT_006a89c4,
                                    (u18 as i32) / 2,
                                    (u18 & 1).wrapping_mul(5),
                                );
                                pb = abstack_36c.as_mut_ptr();
                            } else {
                                let mut t = DAT_006a89b0;
                                if (u18 & 1) != 0 {
                                    t = DAT_006a89c0;
                                }
                                pb = t;
                            }
                            FUN_00438c30(3, pb, 0x34);
                            u374 = u18;
                        }
                        DAT_0072f444 = 0;
                        count_players();
                        if !(DAT_007092c0 == 0 || DAT_00732a18 != DAT_0071b71c) {
                            break;
                        }
                    }
                    FUN_0048ee20(0);
                    let u11 = (DAT_00525520 as u32).wrapping_sub(DAT_0052551c as u32) >> 1;
                    u18 = DAT_006d4a4e as u32;
                    let u16 = (DAT_00525520 as u32).wrapping_add(DAT_0052551c as u32);
                    u9 = (DAT_00525528 as u32).wrapping_add(DAT_00525524 as u32);
                    if DAT_005257e0 != 0 {
                        FUN_004a7040();
                        DAT_0052ae54 = 0;
                        FUN_004a6a80();
                        let f: unsafe extern "C" fn(i32, u32, u32, u32, u32) =
                            core::mem::transmute(DAT_00732a30);
                        f(
                            DAT_005257e0,
                            ((u16 >> 1).wrapping_sub(u11 / 2).wrapping_sub(1)) & 0xffff,
                            ((u9 >> 1)
                                .wrapping_sub(u18.wrapping_mul(5) / 2)
                                .wrapping_sub(1))
                                & 0xffff,
                            u11.wrapping_add(2) & 0xffff,
                            u18.wrapping_mul(5).wrapping_add(2),
                        );
                        FUN_004a6f90();
                        DAT_0052ae54 = 1;
                        FUN_004a7040();
                    }
                    FUN_0048f530();
                    DAT_0072f444 = 0;
                    count_players();
                    if DAT_007092c0 != 0 && DAT_00732a18 == DAT_0071b71c {
                        return;
                    }
                    DAT_006db5a8 = DAT_00732a18.wrapping_add(ram0x0071f2ec as u32);
                }
                let u18 = DAT_006db5a8;
                if DAT_0071b71c < DAT_006db5a8 {
                    let mut u9 = smooth(DAT_006db5a8);
                    if (u9.wrapping_sub(DAT_0071b71c) as i32) < 4 {
                        u9 = DAT_0071b71c.wrapping_add(4);
                    }
                    DAT_005ef234 = u9.wrapping_sub(DAT_005ef238 as u32) as i32;
                    DAT_006db5a8 = u9;
                    FUN_0042d530();
                    FUN_0049adc0(DAT_005ef234);
                    DAT_00525550 = 1;
                    local_378 = DAT_006db5a8.wrapping_sub(DAT_0071b71c) as i32;
                    FUN_0049a600(DAT_006db5a8);
                    DAT_0071b71c = DAT_006db5a8;
                    DAT_005ef238 = DAT_006db5a8 as i32;
                    DAT_006db5a8 = DAT_006db5a8.wrapping_sub(u9).wrapping_add(u18);
                }
            }
            i8 = FUN_0048f530();
            DAT_006db5a8 = DAT_006db5a8.wrapping_add(i8 as u32);
            i12 = DAT_006db5a8.wrapping_sub(i3 as u32) as i32;
            i8 = FUN_0048f530();
            i8 = DAT_006db5a8.wrapping_add(i8 as u32) as i32;
            i5 = (i8 as u32)
                .wrapping_sub(ram0x0071f2ec as u32)
                .wrapping_sub(DAT_00732a18) as i32;
            if i5 < 0x1d8 {
                DAT_0071e51c = 0;
            } else if i5 < 0x3b0 {
                DAT_0071e51c = 1;
            } else {
                DAT_0071e51c = 2;
                if 0x587 < i5 {
                    DAT_0071e51c = 3;
                }
            }
            DAT_006db5a8 = i8 as u32;
            if DAT_005ef23c == 0 {
                DAT_006db57c = 0;
            } else {
                i5 = FUN_00432ca0();
                i5 = FUN_00433680(i5);
                DAT_005ef230 =
                    DAT_005ef230.wrapping_add(i5.wrapping_sub(DAT_005ef23c).wrapping_mul(10));
                if DAT_005ef230 == 0 {
                    DAT_006db57c = 0;
                } else if DAT_005ef230 < 10 {
                    DAT_006db57c = 1;
                } else {
                    DAT_006db57c = 2;
                    if 0x13 < DAT_005ef230 {
                        DAT_006db57c = 3;
                    }
                }
                DAT_005ef23c = i5;
                if DAT_005ef230 != 0 {
                    DAT_005ef230 = DAT_005ef230.wrapping_sub(1);
                }
            }
            FUN_0049b4e0();
            if 1 < DAT_00732e48 {
                DAT_00525554 = 1;
            }
            FUN_0045bb40();
            DAT_00525554 = 0;
            FUN_00490a00();
            FUN_0049b2e0();
            i5 = FUN_0048f530();
            DAT_006db5a8 = DAT_006db5a8.wrapping_add(i5 as u32);
            i17 = DAT_006db5a8.wrapping_sub(i17 as u32) as i32;
            if i17 == 0 {
                i17 = 1;
            }
            if DAT_00525720 != 0 {
                if i17 == 0 {
                    RaiseException(0xC0000094, 0, 0, core::ptr::null());
                }
                let i_quot = (0xeci64 / (i17 as i64)) as i32;
                _sprintf(
                    acstack_21c.as_mut_ptr(),
                    (&raw const s_R___2d_U___2d_N___2d_O___2d_T____00525980).cast::<i8>(),
                    DAT_006db5a8.wrapping_sub(i8 as u32),
                    i12,
                    i3.wrapping_sub(i7),
                    i17.wrapping_sub(i3.wrapping_sub(i7))
                        .wrapping_sub(i12)
                        .wrapping_sub(DAT_006db5a8.wrapping_sub(i8 as u32) as i32),
                    i17,
                    i_quot,
                    DAT_006db5a8,
                    DAT_0071b71c,
                    DAT_00732a18,
                    DAT_006db5a8.wrapping_sub(DAT_00732a18),
                    ram0x0071f2ec,
                    DAT_0052554c,
                );
                if i12 < 0 {
                    i12 = 0;
                }
                if i12 < 0x14 {
                    let p = arr.offset(i12 as isize);
                    *p = (*p).wrapping_add(1);
                } else {
                    let p = arr.add(19);
                    *p = (*p).wrapping_add(1);
                }
                let buf = (&raw mut DAT_00732800).cast::<i8>();
                _sprintf(
                    buf,
                    (&raw const s_Raw_0__2d_1__2d_2__2d_3__2d_4__2_00525934).cast::<i8>(),
                    *arr.add(0),
                    *arr.add(1),
                    *arr.add(2),
                    *arr.add(3),
                    *arr.add(4),
                    *arr.add(5),
                    *arr.add(6),
                    *arr.add(7),
                    *arr.add(8),
                    *arr.add(9),
                );
                let mut u18: u32 = 0;
                _sprintf(
                    buf,
                    (&raw const s_Raw_10__2d_11__2d_12__2d_13__2d_1_005258e8).cast::<i8>(),
                    *arr.add(10),
                    *arr.add(11),
                    *arr.add(12),
                    *arr.add(13),
                    *arr.add(14),
                    *arr.add(15),
                    *arr.add(16),
                    *arr.add(17),
                    *arr.add(18),
                    *arr.add(19),
                );
                let mut p6 = arr as *const i32;
                let end = &raw mut DAT_005257e0 as *const i32;
                loop {
                    i7 = *p6;
                    p6 = p6.add(1);
                    u18 = u18.wrapping_add(i7 as u32);
                    if !(p6 < end) {
                        break;
                    }
                }
                if u18 != 0 {
                    let pc = |n: usize| -> u32 { ((*arr.add(n)) as u32).wrapping_mul(100) / u18 };
                    _sprintf(
                        buf,
                        (&raw const s_Pct_0__2d_1__2d_2__2d_3__2d_4__2_0052589c).cast::<i8>(),
                        pc(0),
                        pc(1),
                        pc(2),
                        pc(3),
                        pc(4),
                        pc(5),
                        pc(6),
                        pc(7),
                        pc(8),
                        pc(9),
                    );
                    _sprintf(
                        buf,
                        (&raw const s_Pct_10__2d_11__2d_12__2d_13__2d_1_00525850).cast::<i8>(),
                        pc(10),
                        pc(11),
                        pc(12),
                        pc(13),
                        pc(14),
                        pc(15),
                        pc(16),
                        pc(17),
                        pc(18),
                        pc(19),
                    );
                }
                i17 = 0;
                i7 = 0;
                if DAT_0071b704 < DAT_00732904 {
                    let mut pc4 = (DAT_0071b704 as u32)
                        .wrapping_mul(0x23)
                        .wrapping_add(3)
                        .wrapping_add(DAT_006d4988 as u32)
                        as *mut i8;
                    i3 = DAT_00732904.wrapping_sub(DAT_0071b704);
                    loop {
                        if *pc4 != 0 {
                            if *pc4.offset(-1) == 6 {
                                i7 = i7.wrapping_add(1);
                            } else {
                                i17 = i17.wrapping_add(1);
                            }
                        }
                        pc4 = pc4.wrapping_offset(0x23);
                        i3 = i3.wrapping_sub(1);
                        if i3 == 0 {
                            break;
                        }
                    }
                }
                if 7 < i12 {
                    _sprintf(
                        buf,
                        (&raw const s________Long_Update___d_______War_00525804).cast::<i8>(),
                        i12,
                        local_378,
                        i7,
                        i17,
                    );
                }
            }
        }
    }
}
