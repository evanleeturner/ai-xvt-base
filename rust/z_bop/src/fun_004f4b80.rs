// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69e30: i32;
    static mut DAT_00b6a5aa: [u8; 4];
    static mut DAT_00b6a5ae: [u8; 0];
    static mut DAT_00aa6114: i32;
    static mut DAT_00aa6134: i32;
    static mut DAT_00aa6138: i32;
    static mut DAT_00b6a2f6: [u8; 4];
    static mut DAT_00bb00ea: [u8; 4];
    static mut DAT_00bb00fa: [u8; 4];
    static mut DAT_00bb00e6: [u8; 24];
    static mut DAT_00bb73a6: i8;
    static mut DAT_00aa6af0: i32;
    static mut DAT_00aa6af4: i32;
    static mut DAT_00aa6af8: i32;
    static mut DAT_00aa6afc: [u32; 6];
    static mut DAT_00bb260e: [u8; 0];
    static mut DAT_00bb2616: [u8; 0];
    static mut DAT_00bb260a: [u8; 0];
    static mut DAT_00bb242e: [u8; 0];
    static mut DAT_00bb242a: [u8; 0];
    static mut DAT_00bb2436: [u8; 0];
    static mut DAT_00bb2046: [u8; 0];
    static mut DAT_00bb204e: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cfee0() -> i32;
    fn FUN_004d0070(param_1: *mut i32, param_2: *mut u32) -> *mut i32;
    fn FUN_004e5000(param_1: i32);
    fn FUN_004f1700() -> i32;
    fn FUN_004cef70(param_1: i32, param_2: *mut u32, param_3: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
}

unsafe fn rd(p: *const u8) -> i32 {
    // SAFETY: caller supplies a readable address.
    unsafe { p.cast::<i32>().read_unaligned() }
}

unsafe fn wr(p: *mut u8, v: i32) {
    // SAFETY: caller supplies a writable address.
    unsafe { p.cast::<i32>().write_unaligned(v) }
}

unsafe fn rec() -> *mut i32 {
    // SAFETY: reads game globals only.
    unsafe {
        DAT_00aa6114
            .wrapping_add(0x140)
            .wrapping_add(DAT_00aa6134.wrapping_mul(0x148)) as u32 as usize as *mut i32
    }
}

unsafe fn off(base: *mut u8, idx: i32) -> *mut u8 {
    // SAFETY: pointer arithmetic only.
    unsafe { base.byte_add((idx as u32).wrapping_mul(0x28) as usize) }
}

unsafe fn find_sel() {
    // SAFETY: the globals are the game's own memory.
    unsafe {
        if DAT_00aa6114 != 0 {
            DAT_00aa6134 = 0;
            if DAT_00aa6138 != 0 {
                let mut p = (DAT_00aa6114.wrapping_add(0x140)) as u32 as usize as *mut i32;
                loop {
                    let sel = rd((&raw mut DAT_00b6a5aa).cast::<u8>());
                    let cur = rd((&raw mut DAT_00b6a5ae)
                        .cast::<u8>()
                        .byte_add((sel as u32).wrapping_mul(4) as usize));
                    if p.read_unaligned() == cur {
                        break;
                    }
                    p = p.byte_add(0x148);
                    DAT_00aa6134 = DAT_00aa6134.wrapping_add(1);
                    if !(DAT_00aa6134 < DAT_00aa6138) {
                        break;
                    }
                }
            }
        }
    }
}

unsafe fn tail() -> i32 {
    // SAFETY: the globals and callee are the game's own.
    unsafe {
        wr((&raw mut DAT_00b6a2f6).cast::<u8>(), 1);
        let t = rd((&raw mut DAT_00bb00ea).cast::<u8>());
        wr((&raw mut DAT_00bb00ea).cast::<u8>(), t.wrapping_add(1));
        FUN_004f1700();
        1
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f4b80() -> i32 {
    // SAFETY: all globals, pointers and callees belong to the game; accesses
    // mirror the original's.
    unsafe {
        let b5aa = (&raw mut DAT_00b6a5aa).cast::<u8>();
        if DAT_00b69e30 != 2 {
            let r = FUN_004cfee0();
            if r == 0 {
                let mut local_20: i32 = 0;
                let mut local_1c: u32 = 0;
                let mut local_18: [u32; 6] = [0; 6];
                let d2 = GetTickCount();
                let mut p5;
                loop {
                    p5 = FUN_004d0070(&raw mut local_20, &raw mut local_1c);
                    if p5.is_null() || p5.read_unaligned() != 0x7b {
                        let d3 = GetTickCount();
                        if 30000 < d3.wrapping_sub(d2) {
                            wr(b5aa, 0);
                            return 0;
                        }
                    } else {
                        break;
                    }
                }
                wr(b5aa, 5);
                FUN_004e5000(5);
                find_sel();
                if p5.add(1).read_unaligned() == 0 {
                    wr(b5aa, 0);
                    let i1 = rec().read_unaligned();
                    wr(off((&raw mut DAT_00bb260e).cast::<u8>(), i1), 0);
                    return 0;
                }
                let p6 = rec();
                let idx = p6.read_unaligned();
                core::ptr::copy_nonoverlapping(
                    off((&raw mut DAT_00bb2616).cast::<u8>(), idx) as *const u8,
                    (&raw mut local_18).cast::<u8>(),
                    24,
                );
                let i1 = rd(off((&raw mut DAT_00bb260a).cast::<u8>(), idx));
                core::ptr::copy_nonoverlapping(
                    p5.add(3).cast::<u8>() as *const u8,
                    (&raw mut DAT_00bb00e6).cast::<u8>(),
                    24,
                );
                if p5.add(2).read_unaligned() == i1 {
                    wr((&raw mut DAT_00bb00fa).cast::<u8>(), local_18[5] as i32);
                } else {
                    wr((&raw mut DAT_00bb00fa).cast::<u8>(), 0);
                }
            } else {
                wr(b5aa, 5);
                FUN_004e5000(5);
                find_sel();
                let mut ok = false;
                if DAT_00bb73a6 != 0 {
                    let p5 = rec();
                    let t = rd(off(
                        (&raw mut DAT_00bb242e).cast::<u8>(),
                        p5.read_unaligned(),
                    ));
                    if t != 0 {
                        ok = true;
                    }
                }
                if !ok {
                    wr(b5aa, 0);
                    DAT_00aa6af4 = 0;
                    DAT_00aa6af0 = 0x7b;
                    let i1 = rec().read_unaligned();
                    wr(off((&raw mut DAT_00bb242e).cast::<u8>(), i1), 0);
                    FUN_004cef70(0, &raw mut DAT_00aa6af0 as *mut u32, 8);
                    return 0;
                }
                let p5 = rec();
                DAT_00aa6af0 = 0x7b;
                DAT_00aa6af4 = 1;
                DAT_00aa6af8 = rd(off(
                    (&raw mut DAT_00bb242a).cast::<u8>(),
                    p5.read_unaligned(),
                ));
                core::ptr::copy_nonoverlapping(
                    off((&raw mut DAT_00bb2436).cast::<u8>(), p5.read_unaligned()) as *const u8,
                    (&raw mut DAT_00aa6afc).cast::<u8>(),
                    24,
                );
                FUN_004cef70(0, &raw mut DAT_00aa6af0 as *mut u32, 0x24);
                let idx = rec().read_unaligned();
                core::ptr::copy_nonoverlapping(
                    off((&raw mut DAT_00bb2436).cast::<u8>(), idx) as *const u8,
                    (&raw mut DAT_00bb00e6).cast::<u8>(),
                    24,
                );
            }
            return tail();
        }
        wr(b5aa, 5);
        FUN_004e5000(5);
        find_sel();
        if DAT_00bb73a6 != 0 {
            let i1 = (rec().read_unaligned() as u32).wrapping_mul(0x28) as usize;
            let t = rd((&raw mut DAT_00bb2046).cast::<u8>().byte_add(i1));
            if t != 0 {
                core::ptr::copy_nonoverlapping(
                    (&raw mut DAT_00bb204e).cast::<u8>().byte_add(i1) as *const u8,
                    (&raw mut DAT_00bb00e6).cast::<u8>(),
                    24,
                );
                return tail();
            }
        }
        wr(b5aa, 0);
        let i1 = rec().read_unaligned();
        wr(off((&raw mut DAT_00bb2046).cast::<u8>(), i1), 0);
        0
    }
}
