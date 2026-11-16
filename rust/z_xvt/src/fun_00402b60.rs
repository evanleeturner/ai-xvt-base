// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00536820: [i8; 264];
    static mut DAT_00536720: [u8; 256];
    static mut DAT_005234d8: [*mut u8; 26];
    static mut DAT_00523550: [i8; 8];
    static mut DAT_00523574: [i8; 8];
    static mut DAT_00523578: [i8; 8];
    static mut DAT_005235ec: [i8; 8];
    static mut DAT_005235b0: [i8; 8];
    static mut DAT_005235a4: [i8; 8];
    static mut DAT_0052359c: [i8; 8];
    static mut DAT_005235e8: [i8; 8];
    static mut DAT_005235c8: [i8; 8];
    static mut DAT_005235c4: [i8; 8];
    static mut DAT_005235e0: [i8; 8];
    static mut DAT_005235dc: [i8; 8];
    static mut DAT_005235d4: [i8; 8];
    static mut DAT_005235d0: [i8; 8];
    static mut DAT_005225a0: [i8; 8];
}

unsafe extern "C" {
    fn fscanf(f: *mut c_void, fmt: *const i8, ...) -> i32;
    fn _strnicmp(a: *const i8, b: *const i8, n: u32) -> i32;
    fn printf(fmt: *const i8, ...) -> i32;
    fn ftell(f: *mut c_void) -> i32;
    fn fseek(f: *mut c_void, off: i32, whence: i32) -> i32;
    fn fgetc(f: *mut c_void) -> i32;
    fn _strcmpi(a: *const i8, b: *const i8) -> i32;
    fn atoi(s: *const i8) -> i32;
}

unsafe fn buf() -> *mut i8 {
    // SAFETY: address of the game's scratch buffer.
    unsafe { (&raw mut DAT_00536820).cast::<i8>() }
}

unsafe fn lp1(p: *const i8) -> u32 {
    let mut n: u32 = 0;
    // SAFETY: p points at a NUL-terminated string.
    unsafe {
        while *p.add(n as usize) != 0 {
            n = n.wrapping_add(1);
        }
    }
    n.wrapping_add(1)
}

unsafe fn ri(p: *const u8, off: usize) -> i32 {
    // SAFETY: the caller's pointer addresses readable game memory.
    unsafe { p.byte_add(off).cast::<i32>().read_unaligned() }
}

unsafe fn wi(p: *mut u8, off: usize, v: i32) {
    // SAFETY: the caller's pointer addresses writable output memory.
    unsafe { p.byte_add(off).cast::<i32>().write_unaligned(v) }
}

unsafe fn rp(p: *const u8, off: usize) -> *mut u8 {
    // SAFETY: as ri.
    unsafe { ri(p, off) as u32 as usize as *mut u8 }
}

unsafe fn cp(d: *mut u8, s: *const u8, n: u32) {
    let mut i: u32 = 0;
    // SAFETY: both ranges are valid for n bytes.
    unsafe {
        while i < n {
            *d.add(i as usize) = *s.add(i as usize);
            i = i.wrapping_add(1);
        }
    }
}

unsafe fn peek(f: *mut c_void, ch: i8, out: *mut i8) -> bool {
    // SAFETY: CRT calls on the caller's FILE; out is a writable byte.
    unsafe {
        let pos = ftell(f);
        let r = fscanf(f, (&raw const DAT_00523574).cast::<i8>(), out);
        let ok = r == 1 && *out == ch;
        fseek(f, pos, 0);
        ok
    }
}

unsafe fn skip_to(f: *mut c_void, ch: i8, out: *mut i8) -> bool {
    // SAFETY: CRT calls on the caller's FILE; out is a writable byte.
    unsafe {
        loop {
            if fscanf(f, (&raw const DAT_00523574).cast::<i8>(), out) != 1 {
                return true;
            }
            if *out == ch {
                return false;
            }
        }
    }
}

unsafe fn scan_type() -> u32 {
    let mut idx: u32 = 0;
    // SAFETY: the game's type table and scratch buffer.
    unsafe {
        loop {
            let len = lp1(buf()).wrapping_sub(1);
            let desc = (*(&raw const DAT_005234d8)
                .cast::<*mut u8>()
                .add(idx as usize)) as *const u8;
            let name = rp(desc, 0) as *const i8;
            if _strnicmp(buf(), name, len) == 0 {
                break;
            }
            idx = idx.wrapping_add(1);
            if !(idx < 26) {
                break;
            }
        }
    }
    idx
}

unsafe fn enum_lookup(m: *const u8) -> i32 {
    // SAFETY: m is a member descriptor with an enum table.
    unsafe {
        let tbl = rp(m, 8) as *const u8;
        let names = ri(tbl, 4) as u32;
        let vals = ri(tbl, 8) as u32;
        let mut k: i32 = 0;
        if 0 < ri(tbl, 0) {
            loop {
                let s = (names.wrapping_add((k as u32).wrapping_mul(4)) as usize
                    as *const *const i8)
                    .read_unaligned();
                if _strcmpi(buf(), s) == 0 {
                    break;
                }
                k = k.wrapping_add(1);
                if !(k < ri(tbl, 0)) {
                    break;
                }
            }
        }
        if ri(tbl, 0) == k {
            atoi(buf())
        } else {
            (vals.wrapping_add((k as u32).wrapping_mul(4)) as usize as *const i32).read_unaligned()
        }
    }
}

#[unsafe(no_mangle)]
#[allow(
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_variables
)]
pub unsafe extern "C" fn FUN_00402b60(
    param_1: *mut c_void,
    param_2: *mut i32,
    param_3: *mut u32,
) -> u32 {
    // SAFETY: the globals, descriptor tables and CRT functions are the game's
    // own; param_2/param_3 are the caller's output buffers, written where the
    // original does.
    unsafe {
        let f = param_1;
        let mut p2: *mut u8 = param_2.cast();
        let f256 = (&raw const DAT_00523550).cast::<i8>();
        let f5e8 = (&raw const DAT_005235e8).cast::<i8>();
        let f5c8 = (&raw const DAT_005235c8).cast::<i8>();
        let f5c4 = (&raw const DAT_005235c4).cast::<i8>();
        let f5a4 = (&raw const DAT_005235a4).cast::<i8>();
        let f59c = (&raw const DAT_0052359c).cast::<i8>();
        let e_err = (&raw const DAT_005235ec).cast::<i8>();
        let e_warn = (&raw const DAT_005235b0).cast::<i8>();
        let e_notsup = (&raw const DAT_00523578).cast::<i8>();
        let nul: *mut u32 = core::ptr::null_mut();

        let mut t: i8 = 0;
        macro_rules! tp {
            () => {
                (&raw mut t)
            };
        }
        let mut local_28: u32 = 0;
        macro_rules! l28 {
            () => {
                (&raw mut local_28).cast::<i8>()
            };
        }

        if !p2.is_null() {
            *param_3 = p2 as usize as u32;
        }
        let mut local_38: u32 = 0;
        if fscanf(f, f256, buf()) != 1 {
            return 0xffffffff;
        }
        let mut local_24: u32 = scan_type();
        local_28 = 0;
        if local_24 == 8 {
            if fscanf(f, f256, buf()) != 1 {
                printf(e_err);
                return 0;
            }
            local_38 = lp1(buf());
            if !p2.is_null() {
                cp(p2, buf().cast(), local_38);
                local_28 = p2 as usize as u32;
                p2 = p2.byte_add(local_38 as usize);
                *param_3 = p2 as usize as u32;
            }
            local_24 = local_38;
            if fscanf(f, f256, buf()) != 1 {
                printf(e_err);
                return local_24;
            }
            local_24 = scan_type();
        }
        if 0x19 < (local_24 as i32) {
            printf(e_notsup);
            if peek(f, b'{' as i8, l28!()) {
                let _ = skip_to(f, b'{' as i8, l28!());
                let _ = skip_to(f, b'}' as i8, l28!());
            }
            return local_38;
        }

        let mut local_8: *mut u8 = core::ptr::null_mut();
        let mut local_34: *mut u8 = core::ptr::null_mut();
        let desc: *const u8 = (*(&raw const DAT_005234d8)
            .cast::<*mut u8>()
            .add(local_24 as usize)) as *const u8;
        let mut p11: *mut u8 = p2;
        if !p2.is_null() {
            wi(p2, 0, local_28 as i32);
            wi(p2, 4, local_24 as i32);
            p11 = p2.byte_add(24);
            wi(p2, 16, ri(desc, 4));
            local_8 = p2;
        }
        let mut p12: *mut u8 = p11;
        if !p11.is_null() {
            wi(local_8, 20, p11 as usize as i32);
            p12 = p11.byte_add((ri(desc, 4) as u32).wrapping_mul(12) as usize);
            local_34 = p11;
        }
        let cnt: i32 = ri(desc, 4);
        local_38 = local_38
            .wrapping_add(0x18)
            .wrapping_add((cnt as u32).wrapping_mul(0xc));

        if local_24 != 7 {
            let _ = skip_to(f, b'{' as i8, l28!());
            let mem = rp(desc, 8) as *const u8;
            let mut iv: i32 = 0;
            if 0 < cnt {
                loop {
                    DAT_00536720[iv as usize] = 0;
                    iv = iv.wrapping_add(1);
                    if !(iv < cnt) {
                        break;
                    }
                }
            }
            local_28 = local_34 as usize as u32;
            let mut local_c: i32 = 0;
            let mut local_18: i32 = 0;
            let mut local_20: u32;
            let mut local_30: i32 = 0;
            let mut local_1c: i32 = 0;
            let mut local_14: i32 = 0;
            let mut local_10: i32 = 0;
            let mut local_4: i32 = 0;
            if 0 < cnt {
                'members: loop {
                    if peek(f, b'}' as i8, tp!()) {
                        break 'members;
                    }
                    local_18 = ftell(f);
                    if fscanf(f, f256, buf()) != 1 {
                        printf(e_err);
                        return local_38;
                    }
                    iv = 0;
                    if 0 < cnt {
                        local_24 = 0;
                        loop {
                            let nm = rp(rp(mem, local_24 as usize), 0) as *const i8;
                            if _strnicmp(buf(), nm, lp1(buf()).wrapping_sub(1)) == 0 {
                                break;
                            }
                            iv = iv.wrapping_add(1);
                            local_24 = local_24.wrapping_add(4);
                            if !(iv < cnt) {
                                break;
                            }
                        }
                    }
                    if cnt == iv {
                        fseek(f, local_18, 0);
                        break 'members;
                    }
                    DAT_00536720[iv as usize] = 1;
                    local_20 = (iv as u32).wrapping_mul(4);
                    local_34 =
                        local_28.wrapping_add((iv as u32).wrapping_mul(12)) as usize as *mut u8;
                    let m = rp(mem, local_20 as usize) as *const u8;
                    let mt = ri(m, 4);
                    'sw: {
                        match mt {
                            1 => {
                                if peek(f, b'"' as i8, tp!()) {
                                    let _ = skip_to(f, b'"' as i8, tp!());
                                    if !p12.is_null() {
                                        wi(local_34, 8, p12 as usize as i32);
                                        wi(local_34, 4, 1);
                                        wi(local_34, 0, 1);
                                    }
                                    loop {
                                        let c = fgetc(f) as i8;
                                        if c == -1 {
                                            printf(e_err);
                                            return local_38;
                                        }
                                        local_38 = local_38.wrapping_add(1);
                                        if c == b'"' as i8 {
                                            break;
                                        }
                                        if !p12.is_null() {
                                            *p12 = c as u8;
                                            p12 = p12.byte_add(1);
                                        }
                                    }
                                    if !p12.is_null() {
                                        *p12 = 0;
                                        p12 = p12.byte_add(1);
                                    }
                                } else if p12.is_null() {
                                    if fscanf(f, f256, buf()) != 1 {
                                        printf(e_err);
                                        return local_38;
                                    }
                                    local_38 = local_38.wrapping_add(lp1(buf()));
                                } else {
                                    wi(local_34, 8, p12 as usize as i32);
                                    wi(local_34, 4, 1);
                                    wi(local_34, 0, 1);
                                    if fscanf(f, f5e8, p12) != 1 {
                                        printf(e_err);
                                        return local_38;
                                    }
                                    let n = lp1(p12 as *const i8);
                                    p12 = p12.byte_add(n as usize);
                                    local_38 = local_38.wrapping_add(n);
                                }
                            }
                            3 => {
                                if p12.is_null() {
                                    let u = FUN_00402b60(f, core::ptr::null_mut(), nul);
                                    local_38 = local_38.wrapping_add(u);
                                } else {
                                    let u = FUN_00402b60(
                                        f,
                                        p12.cast(),
                                        local_34.byte_add(8).cast::<u32>(),
                                    );
                                    local_38 = local_38.wrapping_add(u);
                                    wi(local_34, 4, 1);
                                    p12 = p12.byte_add(u as usize);
                                    wi(local_34, 0, 3);
                                }
                            }
                            4 => {
                                if peek(f, b'[' as i8, tp!()) {
                                    let _ = skip_to(f, b'[' as i8, tp!());
                                    iv = 0;
                                    local_18 = ftell(f);
                                    loop {
                                        if peek(f, b']' as i8, tp!()) {
                                            break;
                                        }
                                        let u = FUN_00402b60(f, core::ptr::null_mut(), nul);
                                        if u != 0 {
                                            iv = iv.wrapping_add(1);
                                            if !peek(f, b']' as i8, tp!()) {
                                                let _ = skip_to(f, b',' as i8, tp!());
                                            }
                                        }
                                    }
                                    fseek(f, local_18, 0);
                                    local_38 = local_38.wrapping_add((iv as u32).wrapping_mul(4));
                                    if !p12.is_null() {
                                        wi(local_34, 8, p12 as usize as i32);
                                        p12 = p12.byte_add((iv as u32).wrapping_mul(4) as usize);
                                        wi(local_34, 4, iv);
                                        wi(local_34, 0, 4);
                                    }
                                    iv = 0;
                                    loop {
                                        if peek(f, b']' as i8, tp!()) {
                                            break;
                                        }
                                        if p12.is_null() {
                                            let u = FUN_00402b60(f, core::ptr::null_mut(), nul);
                                            if u != 0 {
                                                local_38 = local_38.wrapping_add(u);
                                                if !peek(f, b']' as i8, tp!()) {
                                                    let _ = skip_to(f, b',' as i8, tp!());
                                                }
                                            }
                                        } else {
                                            let dst = (ri(local_34, 8) as u32)
                                                .wrapping_add((iv as u32).wrapping_mul(4))
                                                as usize
                                                as *mut u32;
                                            let u = FUN_00402b60(f, p12.cast(), dst);
                                            if u != 0 {
                                                p12 = p12.byte_add(u as usize);
                                                local_38 = local_38.wrapping_add(u);
                                                iv = iv.wrapping_add(1);
                                                if !peek(f, b']' as i8, tp!()) {
                                                    let _ = skip_to(f, b',' as i8, tp!());
                                                }
                                            }
                                        }
                                    }
                                    let _ = skip_to(f, b']' as i8, tp!());
                                } else {
                                    local_38 = local_38.wrapping_add(4);
                                    if !p12.is_null() {
                                        wi(local_34, 8, p12 as usize as i32);
                                        p12 = p12.byte_add(4);
                                        wi(local_34, 4, 1);
                                        wi(local_34, 0, 4);
                                        if !p12.is_null() {
                                            let dst = (ri(local_34, 8) as u32).wrapping_add(4)
                                                as usize
                                                as *mut u32;
                                            let u = FUN_00402b60(f, p12.cast(), dst);
                                            local_38 = local_38.wrapping_add(u);
                                            p12 = p12.byte_add(u as usize);
                                            break 'sw;
                                        }
                                    }
                                    let u = FUN_00402b60(f, core::ptr::null_mut(), nul);
                                    local_38 = local_38.wrapping_add(u);
                                }
                            }
                            5 => {
                                if fscanf(f, f5c8, &raw mut local_30) != 1 {
                                    printf(e_err);
                                    return local_38;
                                }
                                if !p12.is_null() {
                                    wi(local_34, 8, p12 as usize as i32);
                                    wi(local_34, 4, 1);
                                    wi(local_34, 0, 5);
                                    wi(p12, 0, local_30);
                                    p12 = p12.byte_add(4);
                                }
                                local_38 = local_38.wrapping_add(4);
                            }
                            6 => {
                                if peek(f, b'[' as i8, tp!()) {
                                    let _ = skip_to(f, b'[' as i8, tp!());
                                    if !p12.is_null() {
                                        wi(local_34, 8, p12 as usize as i32);
                                    }
                                    iv = 0;
                                    loop {
                                        if peek(f, b']' as i8, tp!()) {
                                            break;
                                        }
                                        if fscanf(f, f5c8, &raw mut local_30) != 1 {
                                            printf(e_err);
                                            return local_38;
                                        }
                                        if !p12.is_null() {
                                            wi(p12, 0, local_30);
                                            p12 = p12.byte_add(4);
                                        }
                                        iv = iv.wrapping_add(1);
                                        local_38 = local_38.wrapping_add(4);
                                        if !peek(f, b']' as i8, tp!()) {
                                            let _ = skip_to(f, b',' as i8, tp!());
                                        }
                                    }
                                    if !p12.is_null() {
                                        wi(local_34, 4, iv);
                                        wi(local_34, 0, 6);
                                    }
                                    let _ = skip_to(f, b']' as i8, tp!());
                                } else {
                                    if !p12.is_null() {
                                        wi(local_34, 8, p12 as usize as i32);
                                    }
                                    if fscanf(f, f5c8, &raw mut local_30) != 1 {
                                        printf(e_err);
                                        return local_38;
                                    }
                                    if !p12.is_null() {
                                        wi(p12, 0, local_30);
                                        p12 = p12.byte_add(4);
                                    }
                                    local_38 = local_38.wrapping_add(4);
                                    if !p12.is_null() {
                                        wi(local_34, 4, 1);
                                        wi(local_34, 0, 6);
                                    }
                                }
                            }
                            7 => {
                                if fscanf(f, f5c4, &raw mut local_1c) != 1 {
                                    printf(e_warn);
                                }
                                if !p12.is_null() {
                                    wi(local_34, 8, p12 as usize as i32);
                                    wi(local_34, 4, 0);
                                    wi(local_34, 0, 5);
                                    wi(p12, 0, local_1c);
                                    p12 = p12.byte_add(4);
                                }
                                local_38 = local_38.wrapping_add(4);
                            }
                            8 => {
                                if peek(f, b'[' as i8, tp!()) {
                                    let _ = skip_to(f, b'[' as i8, tp!());
                                    if !p12.is_null() {
                                        wi(local_34, 8, p12 as usize as i32);
                                    }
                                    iv = 0;
                                    loop {
                                        if peek(f, b']' as i8, tp!()) {
                                            break;
                                        }
                                        if fscanf(f, f5c4, &raw mut local_1c) != 1 {
                                            printf(e_warn);
                                        }
                                        if !p12.is_null() {
                                            wi(p12, 0, local_1c);
                                            p12 = p12.byte_add(4);
                                        }
                                        iv = iv.wrapping_add(1);
                                        local_38 = local_38.wrapping_add(4);
                                        if !peek(f, b']' as i8, tp!()) {
                                            let _ = skip_to(f, b',' as i8, tp!());
                                        }
                                    }
                                    if !p12.is_null() {
                                        wi(local_34, 4, iv);
                                        wi(local_34, 0, 8);
                                    }
                                    let _ = skip_to(f, b']' as i8, tp!());
                                } else {
                                    if !p12.is_null() {
                                        wi(local_34, 8, p12 as usize as i32);
                                    }
                                    if fscanf(f, f5c4, &raw mut local_1c) != 1 {
                                        printf(e_warn);
                                    }
                                    if !p12.is_null() {
                                        wi(p12, 0, local_1c);
                                        p12 = p12.byte_add(4);
                                    }
                                    local_38 = local_38.wrapping_add(4);
                                    if !p12.is_null() {
                                        wi(local_34, 4, 1);
                                        wi(local_34, 0, 8);
                                    }
                                }
                            }
                            9 => {
                                if fscanf(f, f256, buf()) != 1 {
                                    printf(e_err);
                                    return local_38;
                                }
                                let u: u8;
                                if _strcmpi(buf(), (&raw const DAT_005235e0).cast::<i8>()) == 0
                                    || _strcmpi(buf(), (&raw const DAT_005235dc).cast::<i8>()) == 0
                                {
                                    u = 1;
                                } else {
                                    if _strcmpi(buf(), (&raw const DAT_005235d4).cast::<i8>()) != 0
                                        && _strcmpi(buf(), (&raw const DAT_005235d0).cast::<i8>())
                                            != 0
                                    {
                                        printf(e_err);
                                        return local_38;
                                    }
                                    u = 0;
                                }
                                local_38 = local_38.wrapping_add(1);
                                if !p12.is_null() {
                                    wi(local_34, 8, p12 as usize as i32);
                                    wi(local_34, 4, 1);
                                    wi(local_34, 0, 9);
                                    *p12 = u;
                                    p12 = p12.byte_add(1);
                                }
                            }
                            0xb | 0xd => {
                                if fscanf(
                                    f,
                                    f5a4,
                                    &raw mut local_14,
                                    &raw mut local_10,
                                    &raw mut local_4,
                                ) != 3
                                {
                                    printf(e_warn);
                                }
                                p11 = p12;
                                if !p12.is_null() {
                                    wi(local_34, 8, p12 as usize as i32);
                                    wi(local_34, 4, 1);
                                    wi(local_34, 0, mt);
                                    p11 = p12.byte_add(12);
                                    wi(p12, 0, local_14);
                                    wi(p12, 4, local_10);
                                    wi(p12, 8, local_4);
                                }
                                local_38 = local_38.wrapping_add(12);
                                p12 = p11;
                            }
                            0xc | 0xe => {
                                if !peek(f, b'[' as i8, tp!()) {
                                    if !p12.is_null() {
                                        wi(local_34, 8, p12 as usize as i32);
                                    }
                                    if fscanf(
                                        f,
                                        f5a4,
                                        &raw mut local_14,
                                        &raw mut local_10,
                                        &raw mut local_4,
                                    ) != 3
                                    {
                                        printf(e_warn);
                                    }
                                    p11 = p12;
                                    if !p12.is_null() {
                                        wi(p12, 0, local_14);
                                        p11 = p12.byte_add(12);
                                        wi(p12, 4, local_10);
                                        wi(p12, 8, local_4);
                                    }
                                    local_38 = local_38.wrapping_add(12);
                                    p12 = p11;
                                    if !p11.is_null() {
                                        wi(local_34, 4, 1);
                                        wi(local_34, 0, mt);
                                    }
                                } else {
                                    let _ = skip_to(f, b'[' as i8, tp!());
                                    if !p12.is_null() {
                                        wi(local_34, 8, p12 as usize as i32);
                                    }
                                    iv = 0;
                                    loop {
                                        if peek(f, b']' as i8, tp!()) {
                                            break;
                                        }
                                        if fscanf(
                                            f,
                                            f5a4,
                                            &raw mut local_14,
                                            &raw mut local_10,
                                            &raw mut local_4,
                                        ) != 3
                                        {
                                            printf(e_warn);
                                        }
                                        p11 = p12;
                                        if !p12.is_null() {
                                            wi(p12, 0, local_14);
                                            p11 = p12.byte_add(12);
                                            wi(p12, 4, local_10);
                                            wi(p12, 8, local_4);
                                        }
                                        iv = iv.wrapping_add(1);
                                        local_38 = local_38.wrapping_add(12);
                                        let pk = peek(f, b']' as i8, tp!());
                                        p12 = p11;
                                        if !pk {
                                            let _ = skip_to(f, b',' as i8, tp!());
                                        }
                                    }
                                    if !p12.is_null() {
                                        wi(local_34, 4, iv);
                                        wi(local_34, 0, mt);
                                    }
                                    let _ = skip_to(f, b']' as i8, tp!());
                                }
                            }
                            0xf => {
                                if !p12.is_null() {
                                    wi(local_34, 8, p12 as usize as i32);
                                    wi(local_34, 4, 1);
                                    wi(local_34, 0, mt);
                                }
                                iv = 0x10;
                                local_38 = local_38.wrapping_add(0x40);
                                loop {
                                    if fscanf(f, f5c4, &raw mut local_1c) != 1 {
                                        printf(e_warn);
                                    }
                                    if !p12.is_null() {
                                        wi(p12, 0, local_1c);
                                        p12 = p12.byte_add(4);
                                    }
                                    iv = iv.wrapping_sub(1);
                                    if iv == 0 {
                                        break;
                                    }
                                }
                            }
                            0x10 => {
                                if peek(f, b'[' as i8, tp!()) {
                                    let _ = skip_to(f, b'[' as i8, tp!());
                                    if !p12.is_null() {
                                        wi(local_34, 8, p12 as usize as i32);
                                    }
                                    iv = 0;
                                    loop {
                                        if peek(f, b']' as i8, tp!()) {
                                            break;
                                        }
                                        local_30 = 0;
                                        loop {
                                            if fscanf(f, f5c4, &raw mut local_1c) != 1 {
                                                printf(e_warn);
                                            }
                                            if !p12.is_null() {
                                                wi(p12, 0, local_1c);
                                                p12 = p12.byte_add(4);
                                            }
                                            local_38 = local_38.wrapping_add(4);
                                            local_30 = local_30.wrapping_add(1);
                                            if !(local_30 < 0x10) {
                                                break;
                                            }
                                        }
                                        iv = iv.wrapping_add(1);
                                        if !peek(f, b']' as i8, tp!()) {
                                            let _ = skip_to(f, b',' as i8, tp!());
                                        }
                                    }
                                    if !p12.is_null() {
                                        wi(local_34, 4, iv);
                                        wi(local_34, 0, mt);
                                    }
                                    let _ = skip_to(f, b']' as i8, tp!());
                                } else {
                                    if !p12.is_null() {
                                        wi(local_34, 8, p12 as usize as i32);
                                    }
                                    local_30 = 0;
                                    loop {
                                        if fscanf(f, f5c4, &raw mut local_1c) != 1 {
                                            printf(e_warn);
                                        }
                                        if !p12.is_null() {
                                            wi(p12, 0, local_1c);
                                            p12 = p12.byte_add(4);
                                        }
                                        local_38 = local_38.wrapping_add(4);
                                        local_30 = local_30.wrapping_add(1);
                                        if !(local_30 < 0x10) {
                                            break;
                                        }
                                    }
                                    if !p12.is_null() {
                                        wi(local_34, 4, 1);
                                        wi(local_34, 0, mt);
                                    }
                                }
                            }
                            0x11 => {
                                if fscanf(
                                    f,
                                    f59c,
                                    &raw mut local_14,
                                    &raw mut local_10,
                                    &raw mut local_4,
                                ) != 2
                                {
                                    printf(e_warn);
                                }
                                p11 = p12;
                                if !p12.is_null() {
                                    wi(local_34, 8, p12 as usize as i32);
                                    wi(local_34, 4, 1);
                                    p11 = p12.byte_add(8);
                                    wi(local_34, 0, mt);
                                    wi(p12, 0, local_14);
                                    wi(p12, 4, local_10);
                                }
                                local_38 = local_38.wrapping_add(8);
                                p12 = p11;
                            }
                            0x12 => {
                                if !peek(f, b'[' as i8, tp!()) {
                                    if !p12.is_null() {
                                        wi(local_34, 8, p12 as usize as i32);
                                    }
                                    if fscanf(f, f59c, &raw mut local_14, &raw mut local_10) != 2 {
                                        printf(e_warn);
                                    }
                                    p11 = p12;
                                    if !p12.is_null() {
                                        wi(p12, 0, local_14);
                                        p11 = p12.byte_add(8);
                                        wi(p12, 4, local_10);
                                    }
                                    local_38 = local_38.wrapping_add(8);
                                    p12 = p11;
                                    if !p11.is_null() {
                                        wi(local_34, 4, 1);
                                        wi(local_34, 0, mt);
                                    }
                                } else {
                                    let _ = skip_to(f, b'[' as i8, tp!());
                                    if !p12.is_null() {
                                        wi(local_34, 8, p12 as usize as i32);
                                    }
                                    iv = 0;
                                    loop {
                                        if peek(f, b']' as i8, tp!()) {
                                            break;
                                        }
                                        if fscanf(f, f59c, &raw mut local_14, &raw mut local_10)
                                            != 2
                                        {
                                            printf(e_warn);
                                        }
                                        p11 = p12;
                                        if !p12.is_null() {
                                            wi(p12, 0, local_14);
                                            p11 = p12.byte_add(8);
                                            wi(p12, 4, local_10);
                                        }
                                        iv = iv.wrapping_add(1);
                                        local_38 = local_38.wrapping_add(8);
                                        let pk = peek(f, b']' as i8, tp!());
                                        p12 = p11;
                                        if !pk {
                                            let _ = skip_to(f, b',' as i8, tp!());
                                        }
                                    }
                                    if !p12.is_null() {
                                        wi(local_34, 4, iv);
                                        wi(local_34, 0, mt);
                                    }
                                    let _ = skip_to(f, b']' as i8, tp!());
                                }
                            }
                            0x13 => {
                                if !p12.is_null() {
                                    wi(local_34, 8, p12 as usize as i32);
                                    wi(local_34, 4, 1);
                                    wi(local_34, 0, mt);
                                }
                                iv = 4;
                                local_38 = local_38.wrapping_add(0x10);
                                loop {
                                    if fscanf(f, f5c4, &raw mut local_1c) != 1 {
                                        printf(e_warn);
                                    }
                                    if !p12.is_null() {
                                        wi(p12, 0, local_1c);
                                        p12 = p12.byte_add(4);
                                    }
                                    iv = iv.wrapping_sub(1);
                                    if iv == 0 {
                                        break;
                                    }
                                }
                            }
                            0x14 => {
                                if peek(f, b'[' as i8, tp!()) {
                                    let _ = skip_to(f, b'[' as i8, tp!());
                                    if !p12.is_null() {
                                        wi(local_34, 8, p12 as usize as i32);
                                    }
                                    iv = 0;
                                    loop {
                                        if peek(f, b']' as i8, tp!()) {
                                            break;
                                        }
                                        local_30 = 0;
                                        loop {
                                            if fscanf(f, f5c4, &raw mut local_1c) != 1 {
                                                printf(e_warn);
                                            }
                                            if !p12.is_null() {
                                                wi(p12, 0, local_1c);
                                                p12 = p12.byte_add(4);
                                            }
                                            local_38 = local_38.wrapping_add(4);
                                            local_30 = local_30.wrapping_add(1);
                                            if !(local_30 < 4) {
                                                break;
                                            }
                                        }
                                        iv = iv.wrapping_add(1);
                                        if !peek(f, b']' as i8, tp!()) {
                                            let _ = skip_to(f, b',' as i8, tp!());
                                        }
                                    }
                                    if !p12.is_null() {
                                        wi(local_34, 4, iv);
                                        wi(local_34, 0, mt);
                                    }
                                    let _ = skip_to(f, b']' as i8, tp!());
                                } else {
                                    if !p12.is_null() {
                                        wi(local_34, 8, p12 as usize as i32);
                                    }
                                    local_30 = 0;
                                    loop {
                                        if fscanf(f, f5c4, &raw mut local_1c) != 1 {
                                            printf(e_warn);
                                        }
                                        if !p12.is_null() {
                                            wi(p12, 0, local_1c);
                                            p12 = p12.byte_add(4);
                                        }
                                        local_38 = local_38.wrapping_add(4);
                                        local_30 = local_30.wrapping_add(1);
                                        if !(local_30 < 4) {
                                            break;
                                        }
                                    }
                                    if !p12.is_null() {
                                        wi(local_34, 4, 1);
                                        wi(local_34, 0, mt);
                                    }
                                }
                            }
                            0x15 => {
                                if peek(f, b'}' as i8, tp!()) {
                                    let mut k = 0usize;
                                    while k < 8 {
                                        *buf().add(k) = DAT_005225a0[k];
                                        k += 1;
                                    }
                                } else if fscanf(f, f256, buf()) != 1 {
                                    printf(e_err);
                                    return local_38;
                                }
                                local_30 = enum_lookup(m);
                                p11 = p12;
                                if !p12.is_null() {
                                    wi(local_34, 8, p12 as usize as i32);
                                    p11 = p12.byte_add(4);
                                    wi(local_34, 4, 1);
                                    wi(local_34, 0, 0x15);
                                    wi(p12, 0, local_30);
                                }
                                local_38 = local_38.wrapping_add(4);
                                p12 = p11;
                            }
                            0x16 => {
                                if peek(f, b'[' as i8, tp!()) {
                                    let _ = skip_to(f, b'[' as i8, tp!());
                                    if !p12.is_null() {
                                        wi(local_34, 8, p12 as usize as i32);
                                    }
                                    local_18 = 0;
                                    loop {
                                        if peek(f, b']' as i8, tp!()) {
                                            break;
                                        }
                                        if fscanf(f, f256, buf()) != 1 {
                                            printf(e_err);
                                            return local_38;
                                        }
                                        local_30 = enum_lookup(m);
                                        if !p12.is_null() {
                                            wi(p12, 0, local_30);
                                            p12 = p12.byte_add(4);
                                        }
                                        local_38 = local_38.wrapping_add(4);
                                        local_18 = local_18.wrapping_add(1);
                                        if !peek(f, b']' as i8, tp!()) {
                                            let _ = skip_to(f, b',' as i8, tp!());
                                        }
                                    }
                                    if !p12.is_null() {
                                        wi(local_34, 4, local_18);
                                        wi(local_34, 0, mt);
                                    }
                                    let _ = skip_to(f, b']' as i8, tp!());
                                } else {
                                    if !p12.is_null() {
                                        wi(local_34, 8, p12 as usize as i32);
                                    }
                                    if fscanf(f, f256, buf()) != 1 {
                                        printf(e_err);
                                        return local_38;
                                    }
                                    local_30 = enum_lookup(m);
                                    if !p12.is_null() {
                                        wi(p12, 0, local_30);
                                        p12 = p12.byte_add(4);
                                    }
                                    local_38 = local_38.wrapping_add(4);
                                    if !p12.is_null() {
                                        wi(local_34, 4, 1);
                                        wi(local_34, 0, mt);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    local_c = local_c.wrapping_add(1);
                    if !(local_c < cnt) {
                        break;
                    }
                }
            }
            local_c = 0;
            if 0 < cnt {
                iv = 0;
                let mut q11: *mut u8 = local_28 as usize as *mut u8;
                loop {
                    if DAT_00536720[local_c as usize] == 0 {
                        let mm = rp(mem, iv as usize) as *const u8;
                        if !p12.is_null() {
                            let p14 = rp(mm, 0xc) as *const u8;
                            wi(q11, 0, ri(p14, 0));
                            wi(q11, 4, ri(p14, 4));
                            wi(q11, 8, p12 as usize as i32);
                            local_28 = ri(mm, 0x14) as u32;
                            let src = rp(mm, 0x10) as *const u8;
                            cp(p12, src, local_28);
                            p12 = p12.byte_add(ri(mm, 0x14) as u32 as usize);
                        }
                        local_38 = local_38.wrapping_add(ri(mm, 0x14) as u32);
                    }
                    local_c = local_c.wrapping_add(1);
                    q11 = q11.byte_add(12);
                    iv = iv.wrapping_add(4);
                    if !(local_c < cnt) {
                        break;
                    }
                }
            }
            let l_pos = ftell(f);
            local_18 = 0;
            loop {
                let off = ftell(f);
                let r = fscanf(f, (&raw const DAT_00523574).cast::<i8>(), l28!());
                let hit = r == 1 && (local_28 as u8) == b'}';
                fseek(f, off, 0);
                if hit {
                    break;
                }
                let u = FUN_00402b60(f, core::ptr::null_mut(), nul);
                if u != 0 {
                    local_18 = local_18.wrapping_add(1);
                }
            }
            fseek(f, l_pos, 0);
            if local_18 == 0 {
                if !p12.is_null() {
                    wi(local_8, 12, 0);
                    wi(local_8, 8, 0);
                }
            } else {
                local_38 = local_38.wrapping_add((local_18 as u32).wrapping_mul(4));
                if p12.is_null() {
                    p11 = core::ptr::null_mut();
                    p12 = local_28 as usize as *mut u8;
                } else {
                    wi(local_8, 12, p12 as usize as i32);
                    p11 = p12.byte_add((local_18 as u32).wrapping_mul(4) as usize);
                    wi(local_8, 8, local_18);
                }
                loop {
                    if peek(f, b'}' as i8, l28!()) {
                        break;
                    }
                    if p11.is_null() {
                        let u = FUN_00402b60(f, core::ptr::null_mut(), nul);
                        if u != 0 {
                            local_38 = local_38.wrapping_add(u);
                        }
                    } else {
                        let u = FUN_00402b60(f, p11.cast(), p12.cast::<u32>());
                        if u != 0 {
                            p11 = p11.byte_add(u as usize);
                            local_38 = local_38.wrapping_add(u);
                            p12 = p12.byte_add(4);
                        }
                    }
                }
            }
            loop {
                if fscanf(f, (&raw const DAT_00523574).cast::<i8>(), l28!()) != 1 {
                    return local_38;
                }
                if (local_28 as u8) == b'}' {
                    break;
                }
            }
            return local_38;
        }

        if peek(f, b'"' as i8, l28!()) {
            let _ = skip_to(f, b'"' as i8, l28!());
            if !p12.is_null() {
                wi(local_34, 8, p12 as usize as i32);
                wi(local_34, 4, 1);
                wi(local_34, 0, 1);
            }
            loop {
                let c = fgetc(f) as i8;
                if c == -1 {
                    printf(e_err);
                    return local_38;
                }
                local_38 = local_38.wrapping_add(1);
                if c == b'"' as i8 {
                    break;
                }
                if !p12.is_null() {
                    *p12 = c as u8;
                    p12 = p12.byte_add(1);
                }
            }
            if !p12.is_null() {
                *p12 = 0;
                p12 = p12.byte_add(1);
            }
        } else {
            if fscanf(f, f256, buf()) != 1 {
                printf(e_err);
                return local_38;
            }
            let n = lp1(buf());
            if !p12.is_null() {
                cp(p12, buf().cast(), n);
                wi(local_34, 8, p12 as usize as i32);
                p12 = p12.byte_add(n as usize);
                wi(local_34, 4, 1);
                wi(local_34, 0, 1);
            }
            local_38 = local_38.wrapping_add(n);
        }
        if !p12.is_null() {
            wi(local_8, 12, 0);
            wi(local_8, 8, 0);
        }
        local_38
    }
}
