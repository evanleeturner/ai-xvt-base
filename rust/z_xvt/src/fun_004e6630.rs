// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00757ccc: u32;
    static mut DAT_00757cc0: i32;
    static mut DAT_00757cc8: u32;
    static mut DAT_0081b740: [i8; 256];
    static mut DAT_0052a330: [i8; 4];
    static mut DAT_005331e4: [i8; 4];
    static mut DAT_005330e0: [*mut i8; 1];
    static mut DAT_0085e7b7: i8;
    static mut DAT_0085e7b8: i32;
    static mut DAT_0081bfba: [u8; 4];
    static mut DAT_0081bfbe: [u8; 4];
    static mut DAT_0081c012: [u8; 4];
    static mut DAT_0081c016: [u8; 4];
    static mut DAT_0081d32a: [u8; 4];
    static mut DAT_0081d406: [u8; 4];
    static mut DAT_0081d40a: [u8; 4];
    static mut DAT_0081d80e: [u8; 4];
    static mut DAT_0081dc0e: [u8; 4];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004be160(name: *mut i8, mode: *mut i8) -> *mut c_void;
    fn _fgets(buf: *mut i8, n: i32, f: *mut c_void) -> *mut i8;
    fn FUN_004be1f0(f: *mut c_void);
    fn _atoi(s: *const i8) -> i32;
    fn _srand(seed: u32);
    fn _rand() -> i32;
    fn _tolower(c: i32) -> i32;
    fn FUN_004e43d0(a: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
    fn RaiseException(code: u32, flags: u32, n: u32, args: *const u32);
}

unsafe fn rd32(p: *mut u8) -> i32 {
    // SAFETY: p points at 4 readable bytes of the game's memory.
    unsafe { p.cast::<i32>().read_unaligned() }
}

unsafe fn wr32(p: *mut u8, v: i32) {
    // SAFETY: p points at 4 writable bytes of the game's memory.
    unsafe { p.cast::<i32>().write_unaligned(v) }
}

unsafe fn ent(idx: i32) -> *mut u8 {
    // SAFETY: address of the game's table, offset as the original does.
    unsafe {
        (&raw mut DAT_0081bfbe)
            .cast::<u8>()
            .wrapping_add((idx as u32).wrapping_mul(4) as usize)
    }
}

unsafe fn slen(mut p: *const u8) -> u32 {
    let mut n: u32 = 0;
    // SAFETY: p points at a NUL-terminated string.
    unsafe {
        while p.read() != 0 {
            n = n.wrapping_add(1);
            p = p.add(1);
        }
    }
    n
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004e6630() -> u32 {
    let mut local_80: [u8; 128] = [0; 128];
    // SAFETY: all globals and callees are the game's own; pointers address
    // the game's buffers and tables exactly as the original does.
    unsafe {
        let idxp = (&raw mut DAT_0081bfba).cast::<u8>();
        let buf = (&raw mut DAT_0081b740).cast::<i8>();
        let l80 = local_80.as_mut_ptr();
        let mut u11: u32 = 0;
        if DAT_00757ccc != 0 {
            let mut p = (DAT_00757cc0 as u32).wrapping_add(0x140) as usize as *const i32;
            loop {
                let g_idx = rd32(idxp);
                let g_val = rd32(ent(g_idx));
                if p.read_unaligned() == g_val {
                    break;
                }
                p = p.wrapping_byte_add(0x144);
                u11 = u11.wrapping_add(1);
                if !(u11 < DAT_00757ccc) {
                    break;
                }
            }
        }
        let g_idx = rd32(idxp);
        let name = (&raw const DAT_005330e0)
            .cast::<*const i8>()
            .wrapping_add(g_idx as u32 as usize)
            .read();
        _sprintf(
            l80.cast::<i8>(),
            (&raw const DAT_005331e4).cast::<i8>(),
            name,
            u11.wrapping_mul(0x144).wrapping_add(DAT_00757cc0 as u32),
        );
        let file = FUN_004be160(l80.cast::<i8>(), (&raw mut DAT_0052a330).cast::<i8>());
        if file.is_null() {
            return 0;
        }
        if _fgets(buf, 0xff, file).is_null() {
            FUN_004be1f0(file);
            return 0;
        }
        let i_var6 = _atoi(buf);
        let g_idx = rd32(idxp);
        let mut i_var12: i32;
        let i_var4: i32;
        if g_idx == 2 {
            i_var12 = 0;
            wr32((&raw mut DAT_0081d32a).cast::<u8>(), i_var6);
            i_var4 = rd32((&raw mut DAT_0081d80e).cast::<u8>());
        } else {
            let seed = GetTickCount();
            _srand(seed);
            let g_val = DAT_0085e7b8.wrapping_add(2);
            wr32((&raw mut DAT_0081d40a).cast::<u8>(), g_val);
            i_var12 = 0;
            let mut v4 = 0;
            if DAT_0085e7b7 != 0 {
                let tmp = _rand();
                if i_var6 == 0 || (tmp == i32::MIN && i_var6 == -1) {
                    RaiseException(0xC0000094, 0, 0, core::ptr::null());
                }
                let rem: i32;
                core::arch::asm!(
                    "cdq",
                    "idiv {d:e}",
                    d = in(reg) i_var6,
                    inout("eax") tmp => _,
                    out("edx") rem,
                );
                i_var12 = rem;
                v4 = rem;
            }
            i_var4 = v4;
        }
        wr32((&raw mut DAT_0081d80e).cast::<u8>(), i_var4);
        i_var12 = i_var12.wrapping_add(1);
        loop {
            _fgets(buf, 0xff, file);
            let len = slen(buf.cast::<u8>());
            let last = buf.wrapping_add(len.wrapping_sub(1) as usize);
            if last.read() == b'\n' as i8 {
                last.write(0);
            }
            i_var12 = i_var12.wrapping_sub(1);
            if i_var12 == 0 {
                break;
            }
        }
        FUN_004be1f0(file);
        if DAT_0081b740[0] == 0 {
            return 0;
        }
        wr32((&raw mut DAT_0081c012).cast::<u8>(), 1);
        let g_idx = rd32(idxp).wrapping_sub(1);
        wr32(idxp, g_idx);
        let g_idx = rd32(idxp);
        let g_val = rd32(ent(g_idx));
        wr32((&raw mut DAT_0081c016).cast::<u8>(), g_val);

        let n = slen(buf.cast::<u8>()).wrapping_add(1);
        let mut i: u32 = 0;
        while i < n {
            l80.wrapping_add(i as usize)
                .write(buf.cast::<u8>().wrapping_add(i as usize).read());
            i = i.wrapping_add(1);
        }
        let len = slen(l80);
        let mut u11: u32 = 0;
        if len != 0 {
            loop {
                let u13 = u11.wrapping_add(1);
                let c = _tolower(l80.wrapping_add(u11 as usize).read() as i32);
                l80.wrapping_add(u11 as usize).write(c as u8);
                let l = slen(l80);
                u11 = u13;
                if !(u13 < l) {
                    break;
                }
            }
        }
        let mut i_var6: i32 = 0;
        let g_idx = rd32(idxp);
        FUN_004e43d0(g_idx);
        u11 = 0;
        if DAT_00757ccc != 0 {
            loop {
                let mut a = l80 as *const u8;
                let mut b = (DAT_00757cc0 as u32).wrapping_add(i_var6 as u32) as usize as *const u8;
                let equal;
                loop {
                    let ca = a.read();
                    let cb = b.read();
                    if ca != cb {
                        equal = false;
                        break;
                    }
                    if ca == 0 {
                        equal = true;
                        break;
                    }
                    a = a.add(1);
                    b = b.add(1);
                }
                if equal {
                    let p1 = u11
                        .wrapping_mul(0x144)
                        .wrapping_add(0x140)
                        .wrapping_add(DAT_00757cc0 as u32) as usize
                        as *const i32;
                    let g_val = p1.read_unaligned();
                    let g_idx = rd32(idxp);
                    wr32(ent(g_idx), g_val);
                    let g_idx = rd32(idxp);
                    if g_idx == 3 {
                        let g_val = p1.read_unaligned();
                        wr32((&raw mut DAT_0081d406).cast::<u8>(), g_val);
                    }
                    break;
                }
                i_var6 = i_var6.wrapping_add(0x144);
                u11 = u11.wrapping_add(1);
                if !(u11 < DAT_00757ccc) {
                    break;
                }
            }
        }
        if DAT_00757cc0 != 0 {
            DAT_00757cc8 = 0;
            if DAT_00757ccc != 0 {
                let mut p = (DAT_00757cc0 as u32).wrapping_add(0x140) as usize as *const i32;
                loop {
                    let g_idx = rd32(idxp);
                    let g_val2 = rd32(ent(g_idx));
                    if p.read_unaligned() == g_val2 {
                        let g_val = DAT_00757cc8 as i32;
                        wr32((&raw mut DAT_0081dc0e).cast::<u8>(), g_val);
                        return 1;
                    }
                    p = p.wrapping_byte_add(0x144);
                    DAT_00757cc8 = DAT_00757cc8.wrapping_add(1);
                    if !(DAT_00757cc8 < DAT_00757ccc) {
                        break;
                    }
                }
            }
        }
        1
    }
}
