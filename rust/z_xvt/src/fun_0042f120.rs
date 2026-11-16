// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005251c0: [i8; 1];
    static mut DAT_00733070: [u32; 1];
    static mut DAT_0071e6c8: i8;
    static mut _DAT_00539a00: i32;
    static mut _DAT_0073307c: i32;
    static mut DAT_00733380: i32;
    static mut DAT_00733588: i32;
    static mut DAT_00733384: u32;
    static mut DAT_00733584: i32;
    static mut DAT_0073358c: i32;
    static mut DAT_00733794: i32;
    static mut DAT_00733590: u32;
    static mut DAT_00733790: i32;
    static mut DAT_007330d4: i32;
    static mut DAT_0073379c: [u32; 1];
    static mut DAT_0073905c: [u32; 1];
    static mut DAT_005251a8: i32;
    static mut DAT_005ce218: [u32; 1];
    static mut DAT_005251b4: i32;
    static mut DAT_00739290: i32;
    static mut DAT_00733074: *mut i32;
    static mut DAT_00733080: [u32; 1];
    static mut DAT_00733090: [u32; 1];
    static mut DAT_007330a0: i32;
    static mut DAT_007330a8: i32;
    static mut DAT_007330d8: [u32; 1];
    static mut DAT_005251b0: i32;
    static mut DAT_005251ac: i32;
    static mut DAT_00733798: [u32; 1];
    static mut DAT_005bda18: i32;
    static mut DAT_005251a4: i32;
    static mut DAT_007330f8: i32;
    static mut _DAT_007330fc: i32;
    static mut DAT_007330e8: [i8; 1];
    static mut DAT_00733100: [u32; 1];
    static mut DAT_007330a4: i32;
    static mut DAT_007330ac: i32;
    static mut _DAT_00539800: u32;
    static mut _DAT_00539804: i32;
    static mut DAT_00539808: i32;
    static mut DAT_0053980c: u32;
    static mut DAT_00539810: [u32; 1];
    static mut DAT_00733120: [i32; 1];
    static mut DAT_00733124: [i32; 1];
    static mut DAT_00733110: [i8; 1];
    static mut DAT_0073311c: [u8; 1];
    static mut DAT_0073310c: [u8; 1];
    static mut DAT_007337a0: [u32; 1];
    static mut DAT_007337a4: [u32; 1];
    static mut DAT_007337a8: [u32; 1];
    static mut DAT_007337ac: [u32; 1];
    static mut DAT_007337b0: [u32; 1];
    static mut DAT_007337b4: [u32; 1];
    static mut DAT_007337b8: [u8; 1];
    static mut DAT_007339b8: [u32; 1];
    static mut DAT_007339bc: [u32; 1];
    static mut DAT_007339c0: [u32; 1];
    static mut DAT_007339c4: [u32; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ce3a0(
        a: *mut *mut i32,
        b: *mut u32,
        c: *mut u32,
        d: *mut i32,
        e: *mut i32,
        f: *mut u32,
        g: *mut c_void,
        h: *mut i32,
        i: *mut i32,
        j: *mut i32,
        k: *mut u32,
        l: *mut i32,
        m: *mut i32,
        n: *mut u32,
        o: *mut i32,
        p: *mut i32,
        q: *mut i32,
        r: *mut u32,
        s: *mut i32,
        t: *mut i32,
        u: *mut i32,
        v: *mut i32,
    );
    fn strncpy(dst: *mut i8, src: *const i8, n: u32) -> *mut i8;
    fn FUN_00431110(target: i32, buf: *mut u32, size: i32);
    fn FUN_00431d70(a: *mut u32, b: *mut u32) -> *mut i32;
    fn FUN_00430980(a: i32, b: i32);
    fn FUN_00432df0(a: i32);
    fn FUN_00438a10() -> u32;
    fn FUN_0042fe30();
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn timeGetTime() -> u32;
}

unsafe fn call_init() {
    // SAFETY: the callee and globals are the game's own memory.
    unsafe {
        FUN_004ce3a0(
            &raw mut DAT_00733074,
            (&raw mut DAT_00733080).cast::<u32>(),
            (&raw mut DAT_00733090).cast::<u32>(),
            &raw mut DAT_007330a0,
            &raw mut DAT_007330a8,
            (&raw mut DAT_007330d8).cast::<u32>(),
            0x539a08usize as *mut c_void,
            &raw mut DAT_005251b0,
            &raw mut DAT_005251b4,
            &raw mut DAT_005251ac,
            (&raw mut DAT_00733798).cast::<u32>(),
            &raw mut DAT_00739290,
            &raw mut DAT_00733380,
            &raw mut DAT_00733384,
            &raw mut DAT_00733584,
            &raw mut DAT_00733588,
            &raw mut DAT_0073358c,
            &raw mut DAT_00733590,
            &raw mut DAT_00733790,
            &raw mut DAT_00733794,
            &raw mut DAT_005bda18,
            &raw mut DAT_005251a4,
        );
    }
}

unsafe fn vt_slot(obj: *mut i32, off: u32) -> *const c_void {
    // SAFETY: obj is the game's object; its first word is the vtable address.
    unsafe {
        let vt = obj.read() as u32;
        (vt.wrapping_add(off) as usize as *const u32).read_unaligned() as usize as *const c_void
    }
}

unsafe fn vt38(o: *mut i32, a: *mut u32) {
    // SAFETY: slot 0x38 of the game's object is a stdcall method.
    unsafe {
        let f: unsafe extern "stdcall" fn(*mut i32, *mut u32) =
            core::mem::transmute(vt_slot(o, 0x38));
        f(o, a);
    }
}

unsafe fn vt30_4(o: *mut i32, a: i32, b: *mut c_void, c: i32) {
    // SAFETY: slot 0x30 of the game's object is a stdcall method.
    unsafe {
        let f: unsafe extern "stdcall" fn(*mut i32, i32, *mut c_void, i32) =
            core::mem::transmute(vt_slot(o, 0x30));
        f(o, a, b, c);
    }
}

unsafe fn vt30_5(o: *mut i32, a: i32, b: *mut c_void, c: i32, d: i32) {
    // SAFETY: slot 0x30 of the game's object is a stdcall method.
    unsafe {
        let f: unsafe extern "stdcall" fn(*mut i32, i32, *mut c_void, i32, i32) =
            core::mem::transmute(vt_slot(o, 0x30));
        f(o, a, b, c, d);
    }
}

unsafe fn vt0c(o: *mut i32, a: i32, b: i32) {
    // SAFETY: slot 0x0c of the game's object is a stdcall method.
    unsafe {
        let f: unsafe extern "stdcall" fn(*mut i32, i32, i32) =
            core::mem::transmute(vt_slot(o, 0x0c));
        f(o, a, b);
    }
}

unsafe fn vt1c(o: *mut i32, a: i32, b: i32) {
    // SAFETY: slot 0x1c of the game's object is a stdcall method.
    unsafe {
        let f: unsafe extern "stdcall" fn(*mut i32, i32, i32) =
            core::mem::transmute(vt_slot(o, 0x1c));
        f(o, a, b);
    }
}

unsafe fn send_players(msg: *mut i32) {
    // SAFETY: the globals are the game's own tables; msg is the received message.
    unsafe {
        _DAT_00539804 = 0;
        _DAT_00539800 = 0x3b;
        DAT_00539808 = DAT_00739290;
        DAT_0053980c = timeGetTime();
        let mut iv14: i32 = 0;
        if 0 < DAT_00739290 {
            let mut p3 = (&raw mut DAT_00539810).cast::<u32>();
            let mut p12 = (&raw mut DAT_00733798).cast::<u32>();
            loop {
                iv14 = iv14.wrapping_add(1);
                p3.write(p12.read());
                p3.add(1).cast::<u8>().write(p12.add(3).cast::<u8>().read());
                p3.cast::<u8>().add(5).write(p12.add(5).cast::<u8>().read());
                p3.cast::<u8>().add(6).write(p12.add(4).cast::<u8>().read());
                p3.cast::<u8>().add(7).write(p12.add(6).cast::<u8>().read());
                p3 = p3.wrapping_add(2);
                p12 = p12.wrapping_add(0x8e);
                if !(iv14 < DAT_00739290) {
                    break;
                }
            }
        }
        FUN_00431110(
            msg.add(2).read(),
            &raw mut _DAT_00539800,
            (DAT_00739290 as u32).wrapping_mul(8).wrapping_add(0x10) as i32,
        );
    }
}

unsafe fn remove_player(msg: *mut i32) {
    // SAFETY: the globals are the game's own tables; msg is the received message.
    unsafe {
        let mut iv14: i32 = 0;
        if 0 < DAT_00739290 {
            let base = (&raw mut DAT_00733798).cast::<u32>();
            let mut p9 = base.cast::<i32>();
            while p9.read() != msg.add(2).read() {
                p9 = p9.wrapping_add(0x8e);
                iv14 = iv14.wrapping_add(1);
                if DAT_00739290 <= iv14 {
                    return;
                }
            }
            DAT_00739290 = (DAT_00739290 as u32).wrapping_sub(1) as i32;
            let mut src = base.wrapping_add((DAT_00739290 as u32).wrapping_mul(0x8e) as usize);
            let mut dst = base.wrapping_add((iv14 as u32).wrapping_mul(0x8e) as usize);
            let mut n = 0x8e;
            while n != 0 {
                dst.write(src.read());
                src = src.add(1);
                dst = dst.add(1);
                n -= 1;
            }
            let i = (DAT_00739290 as u32).wrapping_mul(0x8e) as usize;
            base.wrapping_add(i).write(0);
            (&raw mut DAT_0073379c)
                .cast::<u32>()
                .wrapping_add(i)
                .write(0x7f);
            (&raw mut DAT_007337a4)
                .cast::<u32>()
                .wrapping_add(i)
                .write(0x7f);
            (&raw mut DAT_007337ac)
                .cast::<u32>()
                .wrapping_add(i)
                .write(0x7f);
            (&raw mut DAT_007337a0)
                .cast::<u32>()
                .wrapping_add(i)
                .write(0x7f);
            (&raw mut DAT_007337a8)
                .cast::<u32>()
                .wrapping_add(i)
                .write(0x7f);
            (&raw mut DAT_007337b0)
                .cast::<u32>()
                .wrapping_add(i)
                .write(0x7f);
            (&raw mut DAT_007337b4)
                .cast::<u32>()
                .wrapping_add(i)
                .write(0);
            let j = (DAT_00739290 as u32).wrapping_mul(0x238) as usize;
            (&raw mut DAT_007337b8)
                .cast::<u8>()
                .wrapping_add(j)
                .write(0x39);
            (&raw mut DAT_007339b8)
                .cast::<u32>()
                .wrapping_add(i)
                .write(1);
            (&raw mut DAT_007339bc)
                .cast::<u32>()
                .wrapping_add(i)
                .write(0);
            (&raw mut DAT_007339c0)
                .cast::<u32>()
                .wrapping_add(i)
                .write(0);
            (&raw mut DAT_007339c4)
                .cast::<u32>()
                .wrapping_add(i)
                .write(0);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_variables, unused_assignments, unused_mut)]
pub unsafe extern "C" fn FUN_0042f120(
    param_1: *mut i8,
    param_2: *mut i8,
    param_3: i32,
    param_4: u32,
    param_5: u32,
    param_6: i32,
) -> u32 {
    let mut local_78: [u32; 4] = [0; 4];
    let mut local_48: [u32; 10] = [0; 10];
    let mut local_20: [u8; 32] = [0; 32];
    let l78 = local_78.as_mut_ptr();
    let mut piVar6: *mut i32 = core::ptr::null_mut();
    let mut iVar7: i32;
    let mut iVar14: i32;
    let iVar17: i32;
    let mut DVar4: u32;
    let mut DVar5: u32;

    // SAFETY: the globals and callees are the game's own memory; all pointers
    // are read and written where the original does.
    unsafe {
        let s = (&raw mut DAT_005251c0).cast::<u8>();
        let l = local_20.as_mut_ptr();
        l.cast::<u32>()
            .write_unaligned(s.cast::<u32>().read_unaligned());
        l.add(4)
            .cast::<u32>()
            .write_unaligned(s.add(4).cast::<u32>().read_unaligned());
        l.add(8)
            .cast::<u32>()
            .write_unaligned(s.add(8).cast::<u32>().read_unaligned());
        l.add(12)
            .cast::<u32>()
            .write_unaligned(s.add(12).cast::<u32>().read_unaligned());
        l.add(16)
            .cast::<u16>()
            .write_unaligned(s.add(16).cast::<u16>().read_unaligned());
        l.add(0x12).write(s.add(0x12).read());
        let local_zero: u32 = 0;
        l.add(0x13).cast::<u32>().write_unaligned(local_zero);
        l.add(0x17).cast::<u32>().write_unaligned(local_zero);
        l.add(0x1b).cast::<u32>().write_unaligned(local_zero);
        l.add(0x1f).write(0);

        let mut p3 = (&raw mut DAT_00733070).cast::<u32>();
        let mut n = 0x190e;
        while n != 0 {
            p3.write(0);
            p3 = p3.add(1);
            n -= 1;
        }
        DAT_0071e6c8 = 1;
        _DAT_00539a00 = 0;
        _DAT_0073307c = param_5 as i32;
        DAT_00733380 = 0;
        DAT_00733588 = 1;
        (&raw mut DAT_00733384).cast::<u8>().write(0x39);
        DAT_00733584 = 1;
        DAT_0073358c = 0;
        DAT_00733794 = 1;
        (&raw mut DAT_00733590).cast::<u8>().write(0x39);
        DAT_00733790 = 1;
        DAT_007330d4 = 0;
        let mut p3 = (&raw mut DAT_0073379c).cast::<u32>();
        let end = (&raw mut DAT_0073905c).cast::<u32>();
        loop {
            p3.write(0x7f);
            let p12 = p3.wrapping_add(0x8e);
            p3.add(2).write(0x7f);
            p3.add(4).write(0x7f);
            p3.add(1).write(0x7f);
            p3.add(3).write(0x7f);
            p3.add(5).write(0x7f);
            p3.add(6).write(0);
            p3.wrapping_sub(1).write(0);
            p3.add(7).cast::<u8>().write(0x39);
            p3.add(0x87).write(1);
            p3.add(0x88).write(0);
            p3.add(0x89).write(0);
            p3.add(0x8a).write(0);
            p3 = p12;
            if !(p12 < end) {
                break;
            }
        }
        DAT_005251a8 = 0;
        let mut p3 = (&raw mut DAT_005ce218).cast::<u32>();
        let mut n = 0x8400;
        while n != 0 {
            p3.write(0);
            p3 = p3.add(1);
            n -= 1;
        }
        DAT_005251b4 = 0;
        DAT_00739290 = 0;

        if param_6 == 1 && param_3 == 1 {
            call_init();
            if DAT_00733074.is_null() {
                DAT_007330f8 = 1;
                _DAT_007330fc = 1;
            } else {
                let mut k = 0;
                while k < 10 {
                    local_48[k] = 0;
                    k += 1;
                }
                local_48[0] = 0x28;
                vt38(DAT_00733074, local_48.as_mut_ptr());
            }
            strncpy((&raw mut DAT_007330d8).cast::<i8>(), param_1, 0x10);
            strncpy((&raw mut DAT_007330e8).cast::<i8>(), param_2, 0x10);
            let mut p3 = (&raw mut DAT_007330d8).cast::<u32>();
            let mut p12 = (&raw mut DAT_00733100).cast::<u32>();
            let mut n = 10;
            while n != 0 {
                p12.write(p3.read());
                p3 = p3.add(1);
                p12 = p12.add(1);
                n -= 1;
            }
            DAT_007330a4 = param_3;
            DAT_007330ac = 1;
            return 1;
        }

        call_init();
        let mut k = 0;
        while k < 10 {
            local_48[k] = 0;
            k += 1;
        }
        iVar17 = 0;
        local_48[0] = 0x28;
        vt38(DAT_00733074, local_48.as_mut_ptr());
        iVar7 = 0;
        DAT_007330a4 = param_3;
        DAT_007330ac = 0;
        vt30_4(
            DAT_00733074,
            0,
            FUN_0042fe30 as *const c_void as *mut c_void,
            0,
        );
        DAT_0071e6c8 = 1;
        timeGetTime();
        _DAT_00539800 = 0xd;
        FUN_00431110(DAT_007330a8, &raw mut _DAT_00539800, 4);
        _DAT_00539800 = 0x39;
        FUN_00431110(DAT_007330a8, &raw mut _DAT_00539800, 4);

        if DAT_007330a4 != 0 {
            iVar14 = param_6;
            if param_6 != 0 {
                loop {
                    DVar4 = timeGetTime();
                    'wait: {
                        loop {
                            DVar5 = timeGetTime();
                            if DVar5.wrapping_sub(DVar4) < 0xea61 {
                                loop {
                                    piVar6 = FUN_00431d70(l78, l78.add(2));
                                    if piVar6.is_null() {
                                        break;
                                    }
                                    if iVar7 != 0 {
                                        break 'wait;
                                    }
                                    FUN_00430980(piVar6.read(), piVar6 as usize as i32);
                                }
                                continue;
                            }
                            piVar6 = core::ptr::null_mut();
                            break 'wait;
                        }
                    }
                    if piVar6.is_null() {
                        DAT_00733074 = core::ptr::null_mut();
                        return 0;
                    }
                    if piVar6.read() == 0xd {
                        iVar14 = (iVar14 as u32).wrapping_sub(1) as i32;
                    }
                    if iVar14 == 0 {
                        break;
                    }
                }
            }
            if DAT_007330a4 != 0 {
                if param_6 < 2 {
                    return 1;
                }
                FUN_00432df0(0);
                _DAT_00539800 = 0x39;
                FUN_00431110(0, &raw mut _DAT_00539800, 4);
                return 1;
            }
        }

        let local_tail: u32 = l.add(0x1c).cast::<u32>().read_unaligned();
        if local_tail != 0 {
            return 1;
        }

        loop {
            DVar4 = timeGetTime();
            'got: {
                loop {
                    DVar5 = timeGetTime();
                    if DVar5.wrapping_sub(DVar4) < 0xea61 {
                        loop {
                            piVar6 = FUN_00431d70(l78, l78.add(2));
                            if piVar6.is_null() {
                                break;
                            }
                            if iVar7 != 0 {
                                break 'got;
                            }
                            FUN_00430980(piVar6.read(), piVar6 as usize as i32);
                        }
                        continue;
                    }
                    piVar6 = core::ptr::null_mut();
                    break 'got;
                }
            }
            if piVar6.is_null() {
                DAT_00733074 = core::ptr::null_mut();
                return 0;
            }
            if piVar6.read() == 0xe {
                iVar7 = 0;
                DAT_007330ac = piVar6.add(1).read();
                if DAT_007330ac < 1 {
                    return 1;
                }
                loop {
                    DVar4 = timeGetTime();
                    'wait2: {
                        'l5c0: loop {
                            DVar5 = timeGetTime();
                            if DVar5.wrapping_sub(DVar4) < 0xea61 {
                                'l5d6: loop {
                                    piVar6 = FUN_00431d70(l78.add(3), l78.add(2));
                                    if piVar6.is_null() {
                                        continue 'l5c0;
                                    }
                                    if iVar17 != 0 {
                                        break 'wait2;
                                    }
                                    iVar14 = piVar6.read();
                                    if iVar14 == 3 {
                                        if DAT_007330a4 != 0 {
                                            if DAT_0071e6c8 == 0 {
                                                if piVar6.add(1).read() == 1 {
                                                    if piVar6.add(2).read() != DAT_007330f8 {
                                                        send_players(piVar6);
                                                        _DAT_00539804 = 0;
                                                        _DAT_00539800 = 0x3a;
                                                        FUN_00431110(
                                                            piVar6.add(2).read(),
                                                            &raw mut _DAT_00539800,
                                                            8,
                                                        );
                                                    }
                                                    DAT_007330ac = 0;
                                                    iVar14 = 0;
                                                    vt30_5(
                                                        DAT_00733074,
                                                        0,
                                                        FUN_0042fe30 as *const c_void
                                                            as *mut c_void,
                                                        0,
                                                        0,
                                                    );
                                                    if 0 < DAT_007330ac {
                                                        piVar6 =
                                                            (&raw mut DAT_00733120).cast::<i32>();
                                                        loop {
                                                            if DAT_007330f8 != piVar6.read() {
                                                                vt0c(
                                                                    DAT_00733074,
                                                                    DAT_007330a0,
                                                                    piVar6.read(),
                                                                );
                                                            }
                                                            piVar6 = piVar6.wrapping_add(10);
                                                            iVar14 = iVar14.wrapping_add(1);
                                                            if !(iVar14 < DAT_007330ac) {
                                                                break;
                                                            }
                                                        }
                                                    }
                                                }
                                            } else if piVar6.add(1).read() == 1
                                                && piVar6.add(2).read() != DAT_007330f8
                                            {
                                                send_players(piVar6);
                                                _DAT_00539800 = 0x3a;
                                                _DAT_00539804 = FUN_00438a10() as i32;
                                                DAT_0053980c = 0;
                                                DAT_00539808 = 2;
                                                FUN_00431110(
                                                    piVar6.add(2).read(),
                                                    &raw mut _DAT_00539800,
                                                    0x10,
                                                );
                                            }
                                        }
                                    } else if iVar14 == 5 {
                                        if DAT_007330a4 != 0 {
                                            if DAT_0071e6c8 == 0 {
                                                if piVar6.add(1).read() == 1 {
                                                    DAT_007330ac = 0;
                                                    vt30_5(
                                                        DAT_00733074,
                                                        0,
                                                        FUN_0042fe30 as *const c_void
                                                            as *mut c_void,
                                                        0,
                                                        0,
                                                    );
                                                    vt1c(
                                                        DAT_00733074,
                                                        DAT_007330a0,
                                                        piVar6.add(2).read(),
                                                    );
                                                    remove_player(piVar6);
                                                }
                                            } else if piVar6.add(1).read() == 1 {
                                                iVar14 = 0;
                                                if 0 < DAT_007330ac {
                                                    let mut piVar9 =
                                                        (&raw mut DAT_00733120).cast::<i32>();
                                                    loop {
                                                        if piVar9.read() == piVar6.add(2).read() {
                                                            let idx = (iVar14 as u32)
                                                                .wrapping_mul(10)
                                                                as usize;
                                                            let q = (&raw mut DAT_00733124)
                                                                .cast::<i32>()
                                                                .wrapping_add(idx);
                                                            if q.read() != 0 {
                                                                _DAT_00539800 = 0xd;
                                                                FUN_00431110(
                                                                    DAT_007330f8,
                                                                    &raw mut _DAT_00539800,
                                                                    4,
                                                                );
                                                            }
                                                            vt1c(
                                                                DAT_00733074,
                                                                DAT_007330a0,
                                                                piVar6.add(2).read(),
                                                            );
                                                            q.write(0);
                                                            break;
                                                        }
                                                        piVar9 = piVar9.wrapping_add(10);
                                                        iVar14 = iVar14.wrapping_add(1);
                                                        if !(iVar14 < DAT_007330ac) {
                                                            break;
                                                        }
                                                    }
                                                }
                                                remove_player(piVar6);
                                            }
                                        }
                                    } else if iVar14 == 0x103 && piVar6.add(1).read() == 1 {
                                        iVar14 = 0;
                                        if 0 < DAT_007330ac {
                                            let mut piVar9 = (&raw mut DAT_00733120).cast::<i32>();
                                            loop {
                                                if piVar9.read() == piVar6.add(2).read() {
                                                    let mut uVar10: u32 = 0xffffffff;
                                                    let iVar8 =
                                                        (iVar14 as u32).wrapping_mul(0x28) as i32;
                                                    let mut b9 =
                                                        piVar6.add(7).cast::<i8>() as *const i8;
                                                    let mut b15: *const i8;
                                                    loop {
                                                        b15 = b9;
                                                        if uVar10 == 0 {
                                                            break;
                                                        }
                                                        uVar10 = uVar10.wrapping_sub(1);
                                                        b15 = b9.wrapping_add(1);
                                                        let c = b9.read();
                                                        b9 = b15;
                                                        if c == 0 {
                                                            break;
                                                        }
                                                    }
                                                    uVar10 = !uVar10;
                                                    let mut s3 = b15.wrapping_sub(uVar10 as usize)
                                                        as *const u32;
                                                    let mut d12 = (&raw mut DAT_00733110)
                                                        .cast::<i8>()
                                                        .wrapping_offset(iVar8 as isize)
                                                        .cast::<u32>();
                                                    let mut uVar11 = uVar10 >> 2;
                                                    while uVar11 != 0 {
                                                        d12.write_unaligned(s3.read_unaligned());
                                                        s3 = s3.wrapping_add(1);
                                                        d12 = d12.wrapping_add(1);
                                                        uVar11 = uVar11.wrapping_sub(1);
                                                    }
                                                    let mut r = uVar10 & 3;
                                                    let mut sb = s3.cast::<u8>();
                                                    let mut db = d12.cast::<u8>();
                                                    while r != 0 {
                                                        db.write(sb.read());
                                                        sb = sb.wrapping_add(1);
                                                        db = db.wrapping_add(1);
                                                        r = r.wrapping_sub(1);
                                                    }
                                                    uVar10 = 0xffffffff;
                                                    let mut b13 = (&raw mut DAT_00733110)
                                                        .cast::<i8>()
                                                        .wrapping_offset(iVar8 as isize)
                                                        as *const i8;
                                                    loop {
                                                        if uVar10 == 0 {
                                                            break;
                                                        }
                                                        uVar10 = uVar10.wrapping_sub(1);
                                                        let c = b13.read();
                                                        b13 = b13.wrapping_add(1);
                                                        if c == 0 {
                                                            break;
                                                        }
                                                    }
                                                    let mut uVar11: u32 = 0xffffffff;
                                                    let mut b13 = (!uVar10)
                                                        .wrapping_add(0x1c)
                                                        .wrapping_add(piVar6 as usize as u32)
                                                        as usize
                                                        as *const i8;
                                                    let mut b16: *const i8;
                                                    loop {
                                                        b16 = b13;
                                                        if uVar11 == 0 {
                                                            break;
                                                        }
                                                        uVar11 = uVar11.wrapping_sub(1);
                                                        b16 = b13.wrapping_add(1);
                                                        let c = b13.read();
                                                        b13 = b16;
                                                        if c == 0 {
                                                            break;
                                                        }
                                                    }
                                                    uVar11 = !uVar11;
                                                    let mut sc = b16.wrapping_sub(uVar11 as usize);
                                                    let mut dc = (&raw mut DAT_00733100)
                                                        .cast::<u32>()
                                                        .wrapping_add(
                                                            (iVar14 as u32).wrapping_mul(10)
                                                                as usize,
                                                        )
                                                        .cast::<i8>();
                                                    let mut c4 = uVar11 >> 2;
                                                    while c4 != 0 {
                                                        dc.cast::<u32>().write_unaligned(
                                                            sc.cast::<u32>().read_unaligned(),
                                                        );
                                                        sc = sc.wrapping_add(4);
                                                        dc = dc.wrapping_add(4);
                                                        c4 = c4.wrapping_sub(1);
                                                    }
                                                    let mut c1 = uVar11 & 3;
                                                    while c1 != 0 {
                                                        dc.write(sc.read());
                                                        sc = sc.wrapping_add(1);
                                                        dc = dc.wrapping_add(1);
                                                        c1 = c1.wrapping_sub(1);
                                                    }
                                                    (&raw mut DAT_0073311c)
                                                        .cast::<u8>()
                                                        .wrapping_offset(iVar8 as isize)
                                                        .write(0);
                                                    (&raw mut DAT_0073310c)
                                                        .cast::<u8>()
                                                        .wrapping_offset(iVar8 as isize)
                                                        .write(0);
                                                    break;
                                                }
                                                piVar9 = piVar9.wrapping_add(10);
                                                iVar14 = iVar14.wrapping_add(1);
                                                if !(iVar14 < DAT_007330ac) {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            piVar6 = core::ptr::null_mut();
                            break 'wait2;
                        }
                    }
                    if piVar6.is_null() {
                        return 0;
                    }
                    if piVar6.read() == 0xf {
                        let mut piVar9 = piVar6.add(2);
                        let mut dst = (&raw mut DAT_00733100)
                            .cast::<u32>()
                            .wrapping_add((piVar6.add(1).read() as u32).wrapping_mul(10) as usize)
                            .cast::<i32>();
                        let mut n = 10;
                        while n != 0 {
                            dst.write(piVar9.read());
                            piVar9 = piVar9.add(1);
                            dst = dst.add(1);
                            n -= 1;
                        }
                        iVar7 = (iVar7 as u32).wrapping_add(1) as i32;
                        if DAT_007330ac <= iVar7 {
                            return 1;
                        }
                    }
                }
            }
        }
    }
}
