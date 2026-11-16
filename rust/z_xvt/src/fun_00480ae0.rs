// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732948: *mut c_void;
    static mut DAT_0052a334: i8;
    static mut DAT_0052a330: i8;
    static mut DAT_0052a328: i8;
    static mut DAT_0052a324: i8;
    static mut DAT_006db7c0: u8;
    static mut DAT_00703ca0: i8;
    static mut DAT_00703cf0: i8;
    static mut DAT_00703cf1: i8;
    static mut DAT_007091a0: i8;
    static mut DAT_00732a40: *mut u8;
    static mut DAT_006da9e8: i32;
    static mut DAT_005285a0: u8;
    static mut DAT_005288d8: u8;
    static mut DAT_00529370: u8;
    static mut DAT_005285f0: u8;
    static mut DAT_00528928: u8;
    static mut DAT_005293c0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0047ed40(param_1: *mut i8, param_2: *mut i8, param_3: i32, param_4: i32) -> i32;
    fn FUN_00480a00(param_1: *mut i8, param_2: *mut c_void) -> i32;
    fn strncmp(a: *const i8, b: *const i8, n: u32) -> i32;
    fn fclose(f: *mut c_void) -> i32;
    fn strncpy(d: *mut i8, s: *const i8, n: u32) -> *mut i8;
    fn fwrite(p: *const c_void, sz: u32, n: u32, f: *mut c_void) -> u32;
}

unsafe fn str_eq(a: *const u8, b: *const u8) -> bool {
    let mut i = 0usize;
    // SAFETY: both point at nul-terminated strings.
    unsafe {
        loop {
            let x = *a.add(i);
            if x != *b.add(i) {
                return false;
            }
            if x == 0 {
                return true;
            }
            i += 1;
        }
    }
}

unsafe fn str_copy(dst: *mut i8, src: *const i8) {
    let mut i = 0usize;
    // SAFETY: copies through the terminator, as the original does.
    unsafe {
        loop {
            let c = *src.add(i);
            *dst.add(i) = c;
            i += 1;
            if c == 0 {
                break;
            }
        }
    }
}

unsafe fn str_cat(dst: *mut i8, src: *const i8) {
    let mut n = 0usize;
    // SAFETY: dst is nul-terminated.
    unsafe {
        while *dst.add(n) != 0 {
            n += 1;
        }
        str_copy(dst.add(n), src);
    }
}

unsafe fn lookup(table: *const u8, line: *const u8) -> i32 {
    let mut idx = 0i32;
    // SAFETY: table is the game's 0x52-stride record table.
    unsafe {
        if *table != 0 {
            let mut p = table;
            loop {
                if str_eq(p, line) {
                    break;
                }
                p = p.add(0x52);
                idx += 1;
                if *p == 0 {
                    break;
                }
            }
        }
    }
    idx
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00480ae0(param_1: *mut i8) -> u32 {
    let mut abs_buf = [0u8; 256];
    let mut acs = [0i8; 256];
    let abp = abs_buf.as_mut_ptr();
    let acp = acs.as_mut_ptr();
    // SAFETY: all globals and callees are the game's own memory; buffers are
    // local and the pointers follow the original's accesses.
    unsafe {
        let tbl = &raw mut DAT_00703ca0;
        let tbl_end = (&raw mut DAT_007091a0) as usize;
        let flags = &raw mut DAT_00703cf0;
        let ptrs = &raw mut DAT_00732a40;

        str_copy(acp, param_1);
        str_cat(acp, &raw mut DAT_0052a334);
        FUN_0047ed40(acp, &raw mut DAT_0052a330, 0, 0);
        let f = DAT_00732948;
        if f.is_null() {
            return 0;
        }
        let mut puvar10: *mut u8 = &raw mut DAT_006db7c0;
        'outer: loop {
            if FUN_00480a00(abp as *mut i8, f) == 0 {
                fclose(f);
                return 0;
            }
            if abs_buf[0] == 0x2a {
                break 'outer;
            }
            let mut s12: u32 = 0;
            let mut p = tbl;
            loop {
                if strncmp(p, abp as *const i8, 0x50) == 0 {
                    break;
                }
                p = p.add(0x55);
                s12 = s12.wrapping_add(1);
                if (p as usize) >= tbl_end {
                    break;
                }
            }
            let slot: u32;
            if s12 == 0x100 {
                let mut s: u32 = 0;
                let mut q = tbl;
                loop {
                    if *q == 0 {
                        break;
                    }
                    q = q.add(0x55);
                    s = s.wrapping_add(1);
                    if (q as usize) >= tbl_end {
                        break;
                    }
                }
                if s == 0x100 {
                    fclose(f);
                    return 0;
                }
                slot = s;
            } else {
                slot = s12;
                if *flags.add(s12.wrapping_mul(0x55) as usize) == 1 {
                    fclose(f);
                    return 0;
                }
            }
            strncpy(
                tbl.add(slot.wrapping_mul(0x55) as usize),
                abp as *const i8,
                0x50,
            );
            *((slot.wrapping_mul(0x55).wrapping_add(0x703cef)) as usize as *mut u8) = 0;
            *flags.add(slot.wrapping_mul(0x55) as usize) = 1;
            let off = (puvar10 as u32).wrapping_sub(0x6db7c0);
            (&raw mut DAT_00703cf1)
                .add(slot.wrapping_mul(0x55) as usize)
                .cast::<u32>()
                .write_unaligned(off);
            ptrs.add(slot as usize).write(puvar10);

            if FUN_00480a00(abp as *mut i8, f) == 0 {
                fclose(f);
                return 0;
            }
            let i9 = lookup(&raw const DAT_005285a0, abp);
            if *(&raw const DAT_005285a0).add((i9 as usize).wrapping_mul(0x52)) == 0 {
                fclose(f);
                return 0;
            }
            *puvar10 = *(&raw const DAT_005285f0).add((i9 as usize).wrapping_mul(0x52));

            if FUN_00480a00(abp as *mut i8, f) == 0 {
                fclose(f);
                return 0;
            }
            let i6 = lookup(&raw const DAT_005288d8, abp);
            if *(&raw const DAT_005288d8).add((i6 as usize).wrapping_mul(0x52)) == 0 {
                fclose(f);
                return 0;
            }
            *puvar10.add(1) = *(&raw const DAT_00528928).add((i6 as usize).wrapping_mul(0x52));
            let mut puvar11 = puvar10.add(1);
            loop {
                if FUN_00480a00(abp as *mut i8, f) == 0 {
                    fclose(f);
                    return 0;
                }
                let i6 = lookup(&raw const DAT_00529370, abp);
                if *(&raw const DAT_00529370).add((i6 as usize).wrapping_mul(0x52)) == 0 {
                    fclose(f);
                    return 0;
                }
                puvar10 = puvar11.add(2);
                *puvar11.add(1) = *(&raw const DAT_005293c0).add((i6 as usize).wrapping_mul(0x52));
                if i6 != 0 {
                    if FUN_00480a00(abp as *mut i8, f) == 0 {
                        fclose(f);
                        return 0;
                    }
                    let mut n: i32 = 0;
                    let mut q = tbl;
                    loop {
                        if strncmp(q, abp as *const i8, 0x50) == 0 {
                            break;
                        }
                        q = q.add(0x55);
                        n = n.wrapping_add(1);
                        if (q as usize) >= tbl_end {
                            break;
                        }
                    }
                    puvar11 = puvar10;
                    if n == 0x100 {
                        n = 0;
                        let mut q = tbl;
                        loop {
                            if *q == 0 {
                                break;
                            }
                            q = q.add(0x55);
                            n = n.wrapping_add(1);
                            if (q as usize) >= tbl_end {
                                break;
                            }
                        }
                        if n == 0x100 {
                            fclose(f);
                            return 0;
                        }
                        strncpy(
                            tbl.add(n.wrapping_mul(0x55) as usize),
                            abp as *const i8,
                            0x50,
                        );
                        *((n.wrapping_mul(0x55).wrapping_add(0x703cef)) as usize as *mut u8) = 0;
                        *flags.add(n.wrapping_mul(0x55) as usize) = 0;
                        *puvar10 = n as u8;
                    } else {
                        *puvar10 = n as u8;
                    }
                    continue;
                }
                DAT_006da9e8 = DAT_006da9e8.wrapping_add(1);
                continue 'outer;
            }
        }
        fclose(f);
        let mut s: u32 = 0;
        loop {
            if *tbl.add(s.wrapping_mul(0x55) as usize) != 0
                && *flags.add(s.wrapping_mul(0x55) as usize) != 1
            {
                return 0;
            }
            s = s.wrapping_add(1);
            if (s as i32) >= 0x100 {
                break;
            }
        }
        str_copy(acp, param_1);
        str_cat(acp, &raw mut DAT_0052a328);
        FUN_0047ed40(acp, &raw mut DAT_0052a324, 0, 1);
        let f = DAT_00732948;
        if !f.is_null() {
            let mut sz: u32 = 0x5500;
            fwrite((&raw const sz).cast::<c_void>(), 4, 1, f);
            fwrite(tbl.cast::<c_void>() as *const c_void, sz, 1, f);
            sz = 0xffff;
            fwrite((&raw const sz).cast::<c_void>(), 4, 1, f);
            fwrite((&raw const DAT_006db7c0).cast::<c_void>(), sz, 1, f);
            fclose(f);
        }
        1
    }
}
