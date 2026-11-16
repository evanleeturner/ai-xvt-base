// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0066543c: i32;
    static mut DAT_00b6a2a8: i32;
    static mut DAT_0052bf54: i32;
    static mut DAT_0052bfdc: i8;
    static mut DAT_0052bfd4: i8;
    static mut DAT_00b69ccc: u32;
    static mut DAT_00b69cd0: i32;
    static mut DAT_00bb73a7: i8;
    static mut DAT_0052ad74: [i8; 1];
    static mut DAT_00aa6a80: [i8; 100];
    static mut DAT_00aa6af0: u32;
    static mut DAT_00aa6af4: [i8; 1];
    static mut DAT_00b69d20: [i8; 1];
    static mut DAT_00b6a2e0: [i8; 1];
    static mut DAT_0051bf3c: [i8; 1];
    static mut DAT_00aa5df0: [i32; 8];
    static mut DAT_00aa5cb0: [i32; 1];
    static mut DAT_00b6a5a6: [u8; 4];
    static mut DAT_00bb2810: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dcb30();
    fn FUN_004de090(a: *mut i32, b: *mut i32);
    fn FUN_004b5570(a: *mut core::ffi::c_void, b: i32, c: i32);
    fn FUN_004d6460(r: *mut i32, a: i32, b: i32, c: i32, d: i32);
    fn FUN_004ddc10(a: i32) -> *mut i8;
    fn FUN_004db640(a: i32, b: *mut i8, c: *mut i32, d: u32);
    fn FUN_004d6b90(r: *mut i32, a: i32, b: i32) -> i32;
    fn FUN_004dc220() -> u8;
    fn FUN_004dc240() -> u8;
    fn FUN_004de5a0(a: *mut i8, b: i32, c: i32, d: i32);
    fn FUN_004da380(r: *mut i32, b: *mut i8, c: i32, d: i32, e: i32, f: *mut i8) -> i32;
    fn FUN_004d11e0() -> i32;
    fn FUN_004d12b0(a: i32) -> i32;
    fn FUN_004cef70(a: i32, b: *mut u32, c: i32);
    fn FUN_004dbbd0(a: i32, b: i32, c: *mut i32, d: i32, e: i32, f: i32) -> i32;
    fn FUN_004d9d70(r: *mut i32, a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32;
}

unsafe fn c_strlen(p: *const i8) -> u32 {
    let mut n: u32 = 0;
    // SAFETY: p points at a NUL-terminated game buffer.
    unsafe {
        while *p.add(n as usize) != 0 {
            n = n.wrapping_add(1);
        }
    }
    n
}

unsafe fn c_strcat(dst: *mut i8, src: *const i8) {
    // SAFETY: dst and src are the game's NUL-terminated buffers.
    unsafe {
        let mut d = dst.add(c_strlen(dst) as usize);
        let mut s = src;
        loop {
            let c = *s;
            *d = c;
            if c == 0 {
                break;
            }
            s = s.add(1);
            d = d.add(1);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d8690(param_1: i32) {
    let mut local_18: [i32; 4] = [0; 4];
    let mut local_8: i32 = 0;
    let mut local_4: i32 = 0;
    let mut i_var4: i32;
    let mut i_var5: i32;
    // SAFETY: all globals and callees are the game's own; buffers are the
    // game's NUL-terminated strings used as the original does.
    unsafe {
        if param_1 == 0 {
            FUN_004dcb30();
            DAT_0066543c = 0;
        }
        if DAT_00b6a2a8 == 0 {
            DAT_0052bf54 = 0;
        } else {
            FUN_004de090(&raw mut local_4, &raw mut local_8);
            if DAT_0052bf54 == 0 {
                FUN_004b5570((&raw mut DAT_0052bfdc).cast(), 0, 0);
                FUN_004d6460(local_18.as_mut_ptr(), 0x22c, 0x5f, 0x24c, 0x69);
                let p = FUN_004ddc10(0xd9);
                FUN_004db640(10, p, local_18.as_mut_ptr(), DAT_00b69ccc);
                i_var4 = FUN_004d6b90(local_18.as_mut_ptr(), local_4, local_8);
                if i_var4 != 0 && (FUN_004dc220() != 0 || FUN_004dc240() != 0) {
                    if DAT_00bb73a7 != 0 {
                        FUN_004de5a0((&raw mut DAT_0052ad74).cast(), 1, 0, 0xff);
                    }
                    DAT_0052bf54 = 1;
                }
                FUN_004b5570((&raw mut DAT_0052bfd4).cast(), 0, 0);
                FUN_004d6460(local_18.as_mut_ptr(), 0x208, 0x5f, 0x228, 0x69);
                let p = FUN_004ddc10(0x192);
                FUN_004db640(10, p, local_18.as_mut_ptr(), 0xffff);
            } else {
                FUN_004b5570((&raw mut DAT_0052bfd4).cast(), 0, 0);
                FUN_004d6460(local_18.as_mut_ptr(), 0x208, 0x5f, 0x228, 0x69);
                let p = FUN_004ddc10(0x192);
                FUN_004db640(10, p, local_18.as_mut_ptr(), DAT_00b69ccc);
                i_var4 = FUN_004d6b90(local_18.as_mut_ptr(), local_4, local_8);
                if i_var4 != 0 && (FUN_004dc220() != 0 || FUN_004dc240() != 0) {
                    if DAT_00bb73a7 != 0 {
                        FUN_004de5a0((&raw mut DAT_0052ad74).cast(), 1, 0, 0xff);
                    }
                    DAT_0052bf54 = 0;
                }
                FUN_004b5570((&raw mut DAT_0052bfdc).cast(), 0, 0);
                FUN_004d6460(local_18.as_mut_ptr(), 0x22c, 0x5f, 0x24c, 0x69);
                let p = FUN_004ddc10(0xd9);
                FUN_004db640(10, p, local_18.as_mut_ptr(), 0xffff);
            }
        }
        FUN_004d6460(local_18.as_mut_ptr(), 0x1cd, 0x196, 0x253, 0x1a8);
        let a80 = (&raw mut DAT_00aa6a80).cast::<i8>();
        let b20 = (&raw mut DAT_00b69d20).cast::<i8>();
        i_var4 = FUN_004da380(
            local_18.as_mut_ptr(),
            a80,
            100,
            0,
            0xc,
            core::ptr::null_mut(),
        );
        'done: {
            if i_var4 == 0 || *a80 == 0 {
                break 'done;
            }
            DAT_00aa6af0 = 0x4d;
            i_var4 = FUN_004d11e0();
            i_var5 = FUN_004d12b0(i_var4);
            i_var4 = DAT_0052bf54;
            if i_var5 == 0 {
                *b20 = 4;
            } else if DAT_0052bf54 == 1 {
                *b20 = 2;
            } else {
                *b20 = 3;
            }
            *b20.add(1) = 0;
            c_strcat(b20, (&raw mut DAT_00b6a2e0).cast());
            c_strcat(b20, (&raw mut DAT_0051bf3c).cast());
            let len = c_strlen(b20);
            *b20.add(len as usize) = 1;
            *b20.add(len.wrapping_add(1) as usize) = 0;
            c_strcat(b20, a80);
            let len = c_strlen(b20);
            let dst = (&raw mut DAT_00aa6af4).cast::<i8>();
            let mut k: u32 = 0;
            while k < len {
                *dst.add(k as usize) = *b20.add(k as usize);
                k = k.wrapping_add(1);
            }
            let total = len.wrapping_add(4) as i32;
            let mut simple = i_var4 == 0;
            if !simple {
                i_var4 = 0;
                let base = (&raw mut DAT_00aa5df0).cast::<i32>();
                let mut p = base;
                loop {
                    i_var5 = FUN_004d11e0();
                    if i_var5 == *p {
                        break;
                    }
                    p = p.add(1);
                    i_var4 = i_var4.wrapping_add(1);
                    if !(p < base.add(8)) {
                        break;
                    }
                }
                if i_var4 == 8 {
                    simple = true;
                }
            }
            if simple {
                FUN_004cef70(0, &raw mut DAT_00aa6af0, total);
            } else {
                i_var4 = 0;
                loop {
                    let i13 = (&raw const DAT_00b6a5a6).cast::<i32>().read_unaligned();
                    let idx = (i_var4 as u32).wrapping_add((i13 as u32).wrapping_mul(8));
                    let v = (&raw mut DAT_00aa5cb0)
                        .cast::<i32>()
                        .wrapping_add(idx as usize)
                        .read();
                    if v != 0 {
                        let len = c_strlen(b20);
                        let i13 = (&raw const DAT_00b6a5a6).cast::<i32>().read_unaligned();
                        let idx = (i_var4 as u32).wrapping_add((i13 as u32).wrapping_mul(8));
                        let v = (&raw mut DAT_00aa5cb0)
                            .cast::<i32>()
                            .wrapping_add(idx as usize)
                            .read();
                        FUN_004cef70(v, &raw mut DAT_00aa6af0, len.wrapping_add(4) as i32);
                    }
                    i_var4 = i_var4.wrapping_add(1);
                    if i_var4 >= 8 {
                        break;
                    }
                }
            }
            let mut z = a80.cast::<u32>();
            let mut n = 0x19;
            while n != 0 {
                z.write_unaligned(0);
                z = z.add(1);
                n -= 1;
            }
        }
        FUN_004d6460(local_18.as_mut_ptr(), 0x1cd, 0x75, 0x253, 0x197);
        i_var5 = FUN_004dbbd0(0xc, DAT_00b69cd0, local_18.as_mut_ptr(), 0xffff, 2, 0x400);
        i_var4 = i_var5.wrapping_sub(0x14);
        if i_var5 < 0x15 {
            i_var4 = 0;
        }
        if 0 < i_var4 {
            i_var4 = i_var4.wrapping_add(1);
            if DAT_0066543c == i_var4 {
                DAT_0066543c = DAT_0066543c.wrapping_sub(1);
            }
            FUN_004d6460(local_18.as_mut_ptr(), 0x254, 0x75, 0x25d, 0x197);
            i_var5 = FUN_004d9d70(
                local_18.as_mut_ptr(),
                i_var4.wrapping_sub(DAT_0066543c).wrapping_sub(1),
                i_var4,
                0,
                5,
                DAT_00bb2810,
                7,
            );
            DAT_0066543c = i_var4.wrapping_sub(i_var5).wrapping_sub(1);
        }
        FUN_004d6460(local_18.as_mut_ptr(), 0x1cd, 0x75, 0x253, 0x18e);
        FUN_004dbbd0(
            0xc,
            DAT_00b69cd0,
            local_18.as_mut_ptr(),
            0xffff,
            2,
            i_var4.wrapping_sub(DAT_0066543c).wrapping_sub(1),
        );
    }
}
