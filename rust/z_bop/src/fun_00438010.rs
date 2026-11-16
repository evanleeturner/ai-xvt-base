// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009e9683: [u8; 0];
    static mut DAT_009e9a69: [u8; 0];
    static mut DAT_009e9a67: [u8; 0];
    static mut DAT_009e9a6b: [u8; 0];
    static mut DAT_009e9a6d: [u8; 0];
    static mut DAT_009e9a6f: [u8; 0];
    static mut DAT_009e9a73: [u8; 0];
    static mut DAT_009e9a75: [u8; 0];
    static mut DAT_009e9a5b: [u8; 0];
    static mut DAT_009e9a5f: [u8; 0];
    static mut DAT_009e9a63: [u8; 0];
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a7bac: i32;
    static mut DAT_009a8c14: i32;
    static mut DAT_009a8070: i16;
    static mut DAT_00a080f0: i16;
    static mut DAT_009ec47c: i32;
    static mut DAT_009a8e2c: i32;
    static mut DAT_009a8e30: i32;
    static mut DAT_009a8e34: i32;
    static mut DAT_009d8c04: i32;
    static mut DAT_00a07ce8: u32;
    static mut DAT_009d1308: u32;
    static mut DAT_00521f1c: [u32; 0];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, nargs: u32, args: *const u32);
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00427940(a: i16, b: i16, c: i16, d: i16, e: i16, f: i16, g: i32);
    fn FUN_0046a8d0(a: i32, b: i32, c: i32);
    fn FUN_00436470(a: u32, b: u32, c: i32) -> u8;
}

unsafe fn idiv(a: i32, b: i32) -> i32 {
    let q: i32;
    // SAFETY: machine signed divide; faults like the original on bad operands.
    unsafe {
        asm!("cdq", "idiv {d:e}", d = in(reg) b, inout("eax") a => q, out("edx") _);
    }
    q
}

unsafe fn scaled_div(v: i32) -> i32 {
    // SAFETY: reads the game's globals; the divide check mirrors the original.
    unsafe {
        let tmp = (v as u32).wrapping_shl((DAT_009d8c04 & 0x1f) as u32) as i32;
        if DAT_009a8e34 == 0 || (tmp == i32::MIN && DAT_009a8e34 == -1) {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        idiv(tmp, DAT_009a8e34)
    }
}

unsafe fn calc(u_var8: u32, ivar2: isize) -> u32 {
    // SAFETY: the globals are the game's own memory.
    unsafe {
        let i5 = scaled_div(DAT_009a8e2c);
        let i6 = scaled_div(DAT_009a8e30);
        let mut u7 = (i6 as u32)
            .wrapping_mul(i6 as u32)
            .wrapping_add((i5 as u32).wrapping_mul(i5 as u32));
        let us0 = (&raw const DAT_009e9a69)
            .cast::<u8>()
            .byte_add(ivar2 as usize)
            .cast::<u16>()
            .read_unaligned();
        let us1 = (&raw const DAT_009e9a67)
            .cast::<u8>()
            .byte_add(ivar2 as usize)
            .cast::<u16>()
            .read_unaligned();
        if us0 as u32 == u_var8 || us1 as u32 == u_var8 {
            let a = (DAT_009a8c14 >> 4) as u32;
            let b = (DAT_009a7bac >> 4) as u32;
            u7 = u7
                .wrapping_add(a.wrapping_mul(a))
                .wrapping_add(b.wrapping_mul(b));
        }
        u7
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00438010(param_1: i32) -> u32 {
    // SAFETY: all globals and callees are the game's own; pointers are
    // computed from game tables at the offsets the original uses.
    unsafe {
        let ivar2 = (param_1 as u32).wrapping_mul(0x5bd) as i32 as isize;
        let rd16 = |p: *const u8| p.byte_add(ivar2 as usize).cast::<i16>().read_unaligned();
        let rd32 = |p: *const u8| p.byte_add(ivar2 as usize).cast::<i32>().read_unaligned();
        'done: {
            let s4: i16;
            let s11: i16;
            let s10: i16;
            let b = (&raw const DAT_009e9683)
                .cast::<u8>()
                .byte_add(ivar2 as usize)
                .read();
            if b < 2 {
                let s0 = rd16((&raw const DAT_009e9a6f).cast());
                let s1 = rd16((&raw const DAT_009e9a6b).cast());
                let s2 = rd16((&raw const DAT_009e9a6d).cast());
                let s3 = rd16((&raw const DAT_009e9a73).cast());
                let s4b = rd16((&raw const DAT_009e9a75).cast());
                FUN_00427940(s0, s1, s2, 0, s3, s4b, 0);
                let us0 = (&raw const DAT_009e9a69)
                    .cast::<u8>()
                    .byte_add(ivar2 as usize)
                    .cast::<u16>()
                    .read_unaligned();
                if us0 == 0xffff {
                    break 'done;
                }
                let iv9 = (us0 as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(DAT_009a1fe8 as u32);
                let e1 = ((iv9.wrapping_add(4)) as usize as *const i32).read_unaligned();
                let e2 = ((iv9.wrapping_add(8)) as usize as *const i32).read_unaligned();
                let e3 = ((iv9.wrapping_add(0xc)) as usize as *const i32).read_unaligned();
                let i0 = rd32((&raw const DAT_009e9a5b).cast());
                let i1 = rd32((&raw const DAT_009e9a5f).cast());
                let i2 = rd32((&raw const DAT_009e9a63).cast());
                FUN_0046a8d0(
                    e1.wrapping_sub(i0),
                    e2.wrapping_sub(i1),
                    e3.wrapping_sub(i2),
                );
                s4 = 0;
                s11 = DAT_009a8070;
                s10 = DAT_00a080f0;
            } else {
                let v = (((b & 0x7f) as u32) << 0xe) as i32;
                s4 = (v / -0x7f) as i16;
                s11 = 0;
                s10 = 0x4000;
            }
            FUN_00427940(0, s10, s11, 0, s4, 0, 0);
        }

        let mut local_4: u32 = 0xffff;
        let mut iv9: i32 = 0;
        let mut u_var8: u32 = 0;
        let sq = (DAT_009a7bac as u32)
            .wrapping_mul(DAT_009a7bac as u32)
            .wrapping_add((DAT_009a8c14 as u32).wrapping_mul(DAT_009a8c14 as u32));
        let mut local_8: u32 = ((sq as i32) >> 3) as u32;
        let tbl = (&raw const DAT_00521f1c).cast::<u32>();
        if 0 < DAT_009ec47c {
            loop {
                let base = DAT_009a1fe8.wrapping_add(iv9) as u32;
                let u7 = ((base.wrapping_add(3)) as usize as *const u8).read() as u32;
                if u7 != 0 {
                    let k = ((base.wrapping_add(2)) as usize as *const u8).read();
                    if k <= 5 {
                        let r = FUN_00436470(
                            u_var8,
                            tbl.add((u7.wrapping_mul(0x1a)) as usize).read(),
                            param_1,
                        );
                        if r != 0 {
                            let u7 = calc(u_var8, ivar2);
                            if (u7 as i32) < (local_8 as i32) {
                                local_8 = u7;
                                local_4 = u_var8;
                            }
                        }
                    }
                }
                iv9 = iv9.wrapping_add(0x23);
                u_var8 = u_var8.wrapping_add(1);
                if !((u_var8 as i32) < DAT_009ec47c) {
                    break;
                }
            }
        }
        if (DAT_00a07ce8 as i32) < (DAT_009d1308.wrapping_add(DAT_00a07ce8) as i32) {
            let mut iv9 = DAT_00a07ce8.wrapping_mul(0x23) as i32;
            let mut u_var8 = DAT_00a07ce8;
            loop {
                let base = DAT_009a1fe8.wrapping_add(iv9) as u32;
                let u7 = ((base.wrapping_add(3)) as usize as *const u8).read() as u32;
                if u7 != 0 {
                    let b1 = ((base.wrapping_add(2)) as usize as *const u8).read();
                    if 7 < b1 && b1 < 10 {
                        let r = FUN_00436470(
                            u_var8,
                            tbl.add((u7.wrapping_mul(0x1a)) as usize).read(),
                            param_1,
                        );
                        if r != 0 {
                            let u7 = calc(u_var8, ivar2);
                            if (u7 as i32) < (local_8 as i32) {
                                local_8 = u7;
                                local_4 = u_var8;
                            }
                        }
                    }
                }
                iv9 = iv9.wrapping_add(0x23);
                u_var8 = u_var8.wrapping_add(1);
                if !((u_var8 as i32) < (DAT_009d1308.wrapping_add(DAT_00a07ce8) as i32)) {
                    break;
                }
            }
        }
        local_4
    }
}
