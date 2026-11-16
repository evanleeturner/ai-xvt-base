// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0051c4b0: [i8; 1];
    static mut DAT_0051c4bc: i8;
    static mut DAT_00a07bd8: *mut core::ffi::c_void;
    static mut DAT_009d7670: u16;
    static mut DAT_0051bea0: [u8; 1];

    static mut DAT_0099f900: u32;
    static mut DAT_0099f92c: u32;
    static mut DAT_00a60a40: [u32; 1];
    static mut DAT_00a609b0: [u32; 1];
    static mut DAT_00a606f0: u32;
    static mut DAT_00a60704: u32;
    static mut DAT_00a686d4: u32;
    static mut DAT_00a65ce0: u32;
    static mut DAT_00a68600: u32;
    static mut DAT_00a68610: u32;
    static mut DAT_00a68648: u32;
    static mut DAT_00a63380: u32;
    static mut DAT_00a63388: u32;
    static mut DAT_00a63390: u32;
    static mut DAT_00a633b4: u32;
    static mut DAT_00a686b0: u32;
    static mut DAT_00a686d0: u32;
    static mut DAT_00a68730: u32;
    static mut DAT_00a6873c: u32;
    static mut DAT_00a68650: u32;
    static mut DAT_00a6866c: u32;
    static mut DAT_00a68670: u32;
    static mut DAT_00a0a790: u32;
    static mut DAT_00a0a7e0: u32;
    static mut DAT_009a1840: u32;
    static mut DAT_009a1ec4: u32;
    static mut DAT_00a0a0e0: u32;
    static mut DAT_00a0a128: u32;
    static mut DAT_00a08380: u32;
    static mut DAT_00a083b8: u32;
    static mut DAT_00a08bf0: u32;
    static mut DAT_00a08c74: u32;
    static mut DAT_00a0a130: u32;
    static mut DAT_00a0a1d0: u32;
    static mut DAT_00a08350: u32;
    static mut DAT_00a08360: u32;
    static mut DAT_00a607f0: u32;
    static mut DAT_00a60814: u32;
    static mut DAT_00a607ac: u32;
    static mut DAT_00a60820: [u32; 1];
    static mut DAT_00a607b0: u32;
    static mut DAT_0051c56a: i8;
    static mut DAT_00520eb3: i8;
    static mut DAT_00a686e0: [u8; 1];
    static mut DAT_00a60860: u32;
    static mut DAT_00a60984: u32;
    static mut DAT_00a60990: u32;
    static mut DAT_00a609b8: u32;
    static mut DAT_00a60a60: u32;
    static mut DAT_00a633c0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049e720(param_1: *mut i8, param_2: *mut core::ffi::c_void, param_3: i32, param_4: i32);
    fn fseek(file: *mut core::ffi::c_void, offset: i32, origin: i32) -> i32;
    fn ftell(file: *mut core::ffi::c_void) -> i32;
    fn FUN_004ac6e0(param_1: u32);
    fn FUN_004ac5f0(param_1: u32, param_2: i32) -> u32;
    fn FUN_0049eb60(param_1: i32) -> u32;
    fn FUN_004ac720(param_1: u32) -> *mut i8;
    fn fclose(file: *mut core::ffi::c_void) -> i32;
    fn FUN_00425a70(param_1: *mut core::ffi::c_void, param_2: *mut i8) -> i32;
    fn fgets(buf: *mut i8, max: i32, file: *mut core::ffi::c_void) -> *mut i8;
}

unsafe fn copy(dst: *mut i8, src: *const i8, n: u32) {
    let mut i: u32 = 0;
    while i < n {
        // SAFETY: caller supplies valid buffers, as the original's copy loops do.
        unsafe {
            dst.add(i as usize).write(src.add(i as usize).read());
        }
        i = i.wrapping_add(1);
    }
}

unsafe fn next_line(f: *mut core::ffi::c_void, buf: *mut i8) -> Option<i32> {
    loop {
        // SAFETY: buf is the caller's 0x400-byte buffer with one pad byte before it.
        unsafe {
            if fgets(buf, 0x400, f).is_null() {
                return None;
            }
            buf.add(0x3ff).write(0);
            if buf.read() != b'/' as i8 || buf.add(1).read() != b'/' as i8 {
                let mut n: i32 = 0;
                while buf.offset(n as isize).read() != 0 {
                    n += 1;
                }
                let mut len = n;
                if buf.offset(n as isize - 1).read() == b'\n' as i8 {
                    buf.offset(n as isize - 1).write(0);
                    len = n - 1;
                }
                return Some(len);
            }
        }
    }
}

unsafe fn put(dst: &mut *mut i8, buf: *const i8, len: i32, slot: *mut u32) {
    // SAFETY: dst and slot are the game's own string pool and pointer table.
    unsafe {
        copy(*dst, buf, (len as u32).wrapping_add(1));
        slot.write_unaligned(*dst as usize as u32);
        *dst = (*dst).offset(len as isize + 1);
    }
}

unsafe fn fill(
    f: *mut core::ffi::c_void,
    buf: *mut i8,
    dst: &mut *mut i8,
    start: *mut u32,
    end: *mut u32,
) -> bool {
    let mut p = start;
    loop {
        // SAFETY: the pointer tables and pool belong to the game.
        unsafe {
            let Some(len) = next_line(f, buf) else {
                return false;
            };
            put(dst, buf, len, p);
        }
        p = p.wrapping_add(1);
        if !(p < end) {
            break;
        }
    }
    true
}

unsafe fn nested(
    f: *mut core::ffi::c_void,
    buf: *mut i8,
    dst: &mut *mut i8,
    start: *mut u32,
    end: *mut u32,
) -> bool {
    let mut i: i32 = 0;
    let mut base = start;
    loop {
        let idx = (i as u32) % 0x2f;
        // SAFETY: tables and pool belong to the game.
        let cnt = unsafe {
            (&raw const DAT_0051bea0)
                .cast::<u8>()
                .add(idx as usize)
                .read()
        } as i32;
        let mut p = base;
        if cnt != 0 {
            let mut k: i32 = 0;
            loop {
                // SAFETY: as above.
                unsafe {
                    let Some(len) = next_line(f, buf) else {
                        return false;
                    };
                    put(dst, buf, len, p);
                }
                p = p.wrapping_add(1);
                k += 1;
                if !(k < cnt) {
                    break;
                }
            }
        }
        base = base.wrapping_add(0xe);
        i += 1;
        if !(base < end) {
            break;
        }
    }
    true
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004248b0(param_1: i32) {
    if param_1 == 0 {
        return;
    }
    let mut pad = [0i8; 0x401];
    // SAFETY: the game's globals/functions are its own; buf has a pad byte before it.
    unsafe {
        let buf = pad.as_mut_ptr().add(1);
        let name = (&raw mut DAT_0051c4b0).cast::<i8>();
        FUN_0049e720(name, (&raw mut DAT_0051c4bc).cast(), 1, 0);
        let file = DAT_00a07bd8;
        let mut pf = file;
        let mut dst: *mut i8;
        'fail: {
            if file.is_null() {
                break 'fail;
            }
            fseek(file, 0, 2);
            let mut u3 = ftell(file) as u32;
            if 32000 < u3 {
                FUN_004ac6e0(DAT_009d7670 as u32);
                u3 = FUN_004ac5f0(u3, 0);
                DAT_009d7670 = u3 as u16;
                if DAT_009d7670 == 0 {
                    u3 = FUN_0049eb60(0);
                }
            }
            dst = FUN_004ac720(((u3 >> 0x10) << 0x10) | DAT_009d7670 as u32);
            fseek(file, 0, 0);
            let l5 = ftell(file);
            if l5 != 0 {
                fclose(file);
                FUN_0049e720(name, (&raw mut DAT_0051c4bc).cast(), 1, 0);
                pf = DAT_00a07bd8;
                if pf.is_null() {
                    break 'fail;
                }
            }

            let mut p = &raw mut DAT_0099f900;
            loop {
                let n = FUN_00425a70(pf, buf);
                if n == -1 {
                    break 'fail;
                }
                copy(dst, buf, (n as u32).wrapping_add(1));
                p.write_unaligned(dst as usize as u32);
                dst = dst.offset(n as isize + 1);
                p = p.add(1);
                if !(p < &raw mut DAT_0099f92c) {
                    break;
                }
            }

            let mut i: i32 = 0;
            loop {
                let Some(len) = next_line(pf, buf) else {
                    break 'fail;
                };
                let slot = if i < 4 {
                    (&raw mut DAT_00a60a40).cast::<u32>().offset(i as isize)
                } else {
                    (&raw mut DAT_00a609b0).cast::<u32>().offset(i as isize)
                };
                put(&mut dst, buf, len, slot);
                i += 1;
                if !(i < 0x24) {
                    break;
                }
            }

            if !fill(
                pf,
                buf,
                &mut dst,
                &raw mut DAT_00a606f0,
                &raw mut DAT_00a60704,
            ) {
                break 'fail;
            }

            let mut i: i32 = 0;
            loop {
                let Some(len) = next_line(pf, buf) else {
                    break 'fail;
                };
                i += 1;
                put(&mut dst, buf, len, &raw mut DAT_00a686d4);
                if !(i < 1) {
                    break;
                }
            }

            if !nested(
                pf,
                buf,
                &mut dst,
                &raw mut DAT_00a65ce0,
                &raw mut DAT_00a68600,
            ) {
                break 'fail;
            }

            if !fill(
                pf,
                buf,
                &mut dst,
                &raw mut DAT_00a68610,
                &raw mut DAT_00a68648,
            ) {
                break 'fail;
            }
            if !fill(
                pf,
                buf,
                &mut dst,
                &raw mut DAT_00a63380,
                &raw mut DAT_00a63388,
            ) {
                break 'fail;
            }
            if !fill(
                pf,
                buf,
                &mut dst,
                &raw mut DAT_00a63390,
                &raw mut DAT_00a633b4,
            ) {
                break 'fail;
            }
            if !fill(
                pf,
                buf,
                &mut dst,
                &raw mut DAT_00a686b0,
                &raw mut DAT_00a686d0,
            ) {
                break 'fail;
            }
            if !fill(
                pf,
                buf,
                &mut dst,
                &raw mut DAT_00a68730,
                &raw mut DAT_00a6873c,
            ) {
                break 'fail;
            }
            if !fill(
                pf,
                buf,
                &mut dst,
                &raw mut DAT_00a68650,
                &raw mut DAT_00a6866c,
            ) {
                break 'fail;
            }
            if !fill(
                pf,
                buf,
                &mut dst,
                &raw mut DAT_00a68670,
                &raw mut DAT_00a686b0,
            ) {
                break 'fail;
            }
            if !fill(
                pf,
                buf,
                &mut dst,
                &raw mut DAT_00a0a790,
                &raw mut DAT_00a0a7e0,
            ) {
                break 'fail;
            }

            let mut p = &raw mut DAT_009a1840;
            loop {
                let Some(len) = next_line(pf, buf) else {
                    break 'fail;
                };
                let mut src = buf as *const i8;
                let mut n: i32 = 0;
                p.write_unaligned(dst as usize as u32);
                if 0 < len {
                    loop {
                        if src.read() == b'\\' as i8 {
                            let c = src.add(2).read();
                            if src.add(1).read() == b'0' as i8 {
                                dst.write(c.wrapping_sub(0x30));
                            } else {
                                dst.write(c.wrapping_sub(0x28));
                            }
                            src = src.add(3);
                            n += 3;
                        } else {
                            dst.write(src.read());
                            src = src.add(1);
                            n += 1;
                        }
                        dst = dst.add(1);
                        if !(n < len) {
                            break;
                        }
                    }
                }
                dst.write(0);
                dst = dst.add(1);
                p = p.add(1);
                if !(p < &raw mut DAT_009a1ec4) {
                    break;
                }
            }

            if !fill(
                pf,
                buf,
                &mut dst,
                &raw mut DAT_00a0a0e0,
                &raw mut DAT_00a0a128,
            ) {
                break 'fail;
            }
            if !fill(
                pf,
                buf,
                &mut dst,
                &raw mut DAT_00a08380,
                &raw mut DAT_00a083b8,
            ) {
                break 'fail;
            }
            if !fill(
                pf,
                buf,
                &mut dst,
                &raw mut DAT_00a08bf0,
                &raw mut DAT_00a08c74,
            ) {
                break 'fail;
            }
            if !fill(
                pf,
                buf,
                &mut dst,
                &raw mut DAT_00a0a130,
                &raw mut DAT_00a0a1d0,
            ) {
                break 'fail;
            }
            if !fill(
                pf,
                buf,
                &mut dst,
                &raw mut DAT_00a08350,
                &raw mut DAT_00a08360,
            ) {
                break 'fail;
            }
            if !fill(
                pf,
                buf,
                &mut dst,
                &raw mut DAT_00a607f0,
                &raw mut DAT_00a60814,
            ) {
                break 'fail;
            }

            let mut i: i32 = 0;
            loop {
                let Some(len) = next_line(pf, buf) else {
                    break 'fail;
                };
                let slot = if i == 0xd {
                    &raw mut DAT_00a607ac
                } else {
                    (&raw mut DAT_00a60820).cast::<u32>().offset(i as isize)
                };
                put(&mut dst, buf, len, slot);
                i += 1;
                if !(i < 0xe) {
                    break;
                }
            }

            if !fill(
                pf,
                buf,
                &mut dst,
                &raw mut DAT_00a607b0,
                &raw mut DAT_00a607f0,
            ) {
                break 'fail;
            }

            let mut idx: i32 = 0;
            let mut sp: *mut i8 = &raw mut DAT_0051c56a;
            loop {
                let Some(len) = next_line(pf, buf) else {
                    break 'fail;
                };
                sp.cast::<u32>().write_unaligned(dst as usize as u32);
                let t = (&raw mut DAT_00a686e0).cast::<u8>().offset(idx as isize);
                let c = buf.read();
                if c == b'm' as i8 {
                    t.write(0);
                } else if c == b'f' as i8 {
                    t.write(1);
                } else if c == b'n' as i8 {
                    t.write(2);
                } else {
                    FUN_0049eb60(3);
                }
                let cnt = (len as u32).wrapping_sub(1);
                copy(dst, buf.add(2), cnt);
                dst = dst.offset(len as isize - 1);
                sp = sp.add(0x101);
                idx += 1;
                if !(sp < &raw mut DAT_00520eb3) {
                    break;
                }
            }

            if !fill(
                pf,
                buf,
                &mut dst,
                &raw mut DAT_00a60860,
                &raw mut DAT_00a60984,
            ) {
                break 'fail;
            }
            if !fill(
                pf,
                buf,
                &mut dst,
                &raw mut DAT_00a60990,
                &raw mut DAT_00a609b8,
            ) {
                break 'fail;
            }

            let mut found = false;
            let mut i: i32 = 0;
            loop {
                if (&raw mut DAT_00a686e0)
                    .cast::<u8>()
                    .offset(i as isize)
                    .read()
                    == 1
                {
                    found = true;
                    break;
                }
                i += 1;
                if !(i < 0x49) {
                    break;
                }
            }
            if found {
                if !nested(
                    pf,
                    buf,
                    &mut dst,
                    &raw mut DAT_00a60a60,
                    &raw mut DAT_00a63380,
                ) {
                    break 'fail;
                }
            }

            let mut found = false;
            let mut i: i32 = 0;
            loop {
                if (&raw mut DAT_00a686e0)
                    .cast::<u8>()
                    .offset(i as isize)
                    .read()
                    == 2
                {
                    found = true;
                    break;
                }
                i += 1;
                if !(i < 0x49) {
                    break;
                }
            }
            if found {
                if !nested(
                    pf,
                    buf,
                    &mut dst,
                    &raw mut DAT_00a633c0,
                    &raw mut DAT_00a65ce0,
                ) {
                    break 'fail;
                }
            }
            fclose(pf);
            return;
        }
        fclose(pf);
        FUN_0049eb60(2);
    }
}
