// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00730894: u16;
    static mut DAT_00732e58: u16;
    static mut DAT_0070acc0: i16;
    static mut DAT_005f10ec: u16;
    static mut DAT_005f10f4: u16;
    static mut DAT_005f10e8: u16;
    static mut DAT_005f10f8: u16;
    static mut DAT_006d4a4e: u8;
    static mut DAT_00732e52: u16;
    static mut DAT_006d9980: u16;
    static mut DAT_00732e8c: u16;
    static mut DAT_00732a34: u16;
    static mut DAT_0070a188: i8;
    static mut DAT_006d4998: i8;
    static mut DAT_0070acc2: u8;
    static mut DAT_00709ff4: *mut u8;
    static mut DAT_006ffbf0: i32;
    static mut DAT_005f1d20: *mut u8;
    static mut DAT_0052552c: i32;
    static mut DAT_00525534: u32;
    static mut DAT_0052550c: u32;
    static mut DAT_0070acc8: i32;
    static mut DAT_0071e524: i8;
    static mut DAT_006da9fa: u8;
    static mut DAT_00732e50: u8;
    static mut DAT_0071b012: u8;
    static mut DAT_005f1d24: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00463e10(param_1: i16);
}

unsafe fn machine_umod(n: u32, d: u32) -> u32 {
    let q: u32;
    let r: u32;
    // SAFETY: the machine's own unsigned divide; faults on a zero divisor as the original does.
    unsafe {
        core::arch::asm!(
            "div {d:e}",
            d = in(reg) d,
            inout("eax") n => q,
            inout("edx") 0u32 => r,
        );
    }
    let _ = q;
    r
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0046307a() {
    // SAFETY: globals and callee belong to the game; pointers are the game's own
    // buffers, read and written exactly where the original does. The register
    // inputs are deliberately left uninitialized as in the decompile.
    unsafe {
        macro_rules! flush_block {
            () => {
                if DAT_00730894 != 0 && (DAT_00732e58 as i16) < DAT_0070acc0 {
                    DAT_005f10ec = DAT_0070acc0 as u16;
                    DAT_005f10f4 = DAT_00732e58;
                    DAT_005f10e8 = (DAT_006d4a4e as u16).wrapping_add(DAT_00732e52);
                    DAT_005f10f8 = DAT_00732e52;
                    if (DAT_00732e58 as i32) < (DAT_006d9980 as i16 as i32) {
                        DAT_005f10f4 = DAT_006d9980;
                    }
                    if (DAT_00732e52 as i32) < (DAT_00732e8c as i16 as i32) {
                        DAT_005f10f8 = DAT_00732e8c;
                    }
                    if (DAT_00732a34 as i16 as i32) < (DAT_005f10e8 as i32) {
                        DAT_005f10e8 = DAT_00732a34;
                    }
                    if DAT_005f10f8 < DAT_005f10e8 {
                        FUN_00463e10(0);
                    }
                }
            };
        }

        let mut in_al: u8 = core::mem::MaybeUninit::<u8>::uninit().assume_init();
        let in_zf: i32 = core::mem::MaybeUninit::<i32>::uninit().assume_init();

        if in_zf != 0 {
            flush_block!();
            DAT_00732e58 = DAT_006d9980;
            DAT_00732e52 = DAT_00732e52.wrapping_add(DAT_006d4a4e as u16);
            return;
        }
        if in_al < 0x20 {
            return;
        }
        if DAT_0070a188 != 0 && DAT_006d4998 == 0 && 0x60 < in_al && in_al < 0x7b {
            in_al = in_al.wrapping_sub(0x20);
        }
        let off = (in_al.wrapping_sub(0x20) as u32).wrapping_mul(DAT_0070acc2 as u32);
        let mut pb: *mut u8 = DAT_00709ff4.byte_add(off as usize);
        let b1: u8 = pb.read();
        let b2: u8 = pb.byte_add(1).read();
        pb = pb.byte_add(2);
        let u0c: u32 = b1 as u32;

        if (DAT_0070acc0 as i32) <= ((DAT_00732e58 as i16 as i32 as u32).wrapping_add(u0c) as i32)
            && DAT_006ffbf0 != 0
        {
            flush_block!();
            DAT_00732e58 = DAT_006d9980;
            DAT_00732e52 = DAT_00732e52.wrapping_add(b2 as u16).wrapping_add(2);
        }

        let mut b10: u8 = 0;
        let mut flag: i32 = 0;
        let mut i11: i32 = DAT_00732e52 as i16 as i32;
        let mut i4: i32 = i11;
        if i11 < (DAT_00732e8c as i16 as i32) {
            i4 = DAT_00732e8c as i16 as i32;
        }
        let mut u8v: u32 = DAT_005f1d20
            .byte_add((i4 as u32).wrapping_mul(4) as usize)
            .cast::<u32>()
            .read_unaligned();

        if DAT_0052552c == 0x13 {
            if DAT_00525534 != 0xa0000 {
                flag = 1;
            }
        } else if DAT_00525534 == 0xa0000 {
            u8v = machine_umod(u8v, DAT_0052550c);
            let mut i12: i32 = (i11 as u32).wrapping_add(1).wrapping_add(b2 as u32) as i32;
            if (DAT_00732a34 as i16 as i32) < i12 {
                i12 = DAT_00732a34 as i16 as i32;
            }
            let t = (i12 as u32)
                .wrapping_sub(i4 as u32)
                .wrapping_mul(DAT_0070acc8 as u32)
                .wrapping_add(u8v) as i32;
            if 0xffff < t {
                flag = 1;
            }
        } else {
            flag = 1;
        }

        if flag != 0 {
            if i11 < ((i11 as u32).wrapping_add(b2 as u32) as i32) {
                loop {
                    let mut b7: u8 = pb.read();
                    u8v = u0c;
                    if DAT_0071e524 != 0 {
                        u8v = u0c.wrapping_add(1);
                    }
                    i4 = DAT_00732e58 as i16 as i32;
                    let i12: i32 = DAT_006d9980 as i16 as i32;
                    if i4 < i12 {
                        if (u8v as i32) <= ((i12 as u32).wrapping_sub(i4 as u32) as i32) {
                            break;
                        }
                        u8v = u8v.wrapping_sub(i12 as u32).wrapping_add(i4 as u32);
                        let n =
                            ((DAT_006d9980 as u8).wrapping_sub(DAT_00732e58 as u8) & 0x1f) as u32;
                        b7 = ((b7 as u32).wrapping_shl(n)) as u8;
                        i4 = i12;
                    }
                    if (DAT_00732a34 as i16 as i32) <= i11 {
                        break;
                    }
                    if (DAT_00732e8c as i16 as i32) <= i11 {
                        if (DAT_0070acc0 as i32) < (u8v.wrapping_add(i4 as u32) as i32) {
                            u8v = (DAT_0070acc0 as i32 as u32).wrapping_sub(i4 as u32);
                            if (u8v as i32) < 1 {
                                break;
                            }
                        }
                        let mut u9: u32 = DAT_005f1d20
                            .byte_add((i11 as u32).wrapping_mul(4) as usize)
                            .cast::<u32>()
                            .read_unaligned()
                            .wrapping_add(i4 as u32);
                        if DAT_0052552c != 0x13 && DAT_00525534 == 0xa0000 {
                            u9 = machine_umod(u9, DAT_0052550c);
                        }
                        let mut p13: *mut u8 = u9.wrapping_add(DAT_00525534) as *mut u8;
                        while u8v != 0 {
                            u8v = u8v.wrapping_sub(1);
                            let mut u5 = DAT_006da9fa;
                            if (b7 & 0x80) == 0 {
                                u5 = DAT_00732e50;
                                if DAT_0071e524 != 0 && (b10 & 0x80) != 0 {
                                    u5 = DAT_0071b012;
                                }
                            }
                            p13.write(u5);
                            p13 = p13.byte_add(1);
                            b10 = b10.wrapping_mul(2);
                            b7 = b7.wrapping_mul(2);
                        }
                    }
                    i11 = i11.wrapping_add(1);
                    b10 = pb.read();
                    pb = pb.byte_add(2);
                    b10 >>= 1;
                    if !(i11 < ((DAT_00732e52 as i16 as i32 as u32).wrapping_add(b2 as u32) as i32))
                    {
                        break;
                    }
                }
            }
        } else {
            i4 = u8v.wrapping_add(DAT_00525534) as i32;
            if i11 < ((b2 as u32).wrapping_add(i11 as u32) as i32) {
                loop {
                    let mut b7: u8 = pb.read();
                    u8v = u0c;
                    if DAT_0071e524 != 0 {
                        u8v = u0c.wrapping_add(1);
                    }
                    let mut c8: i32 = DAT_00732e58 as i16 as i32;
                    let i12: i32 = DAT_006d9980 as i16 as i32;
                    if c8 < i12 {
                        if (u8v as i32) <= ((i12 as u32).wrapping_sub(c8 as u32) as i32) {
                            break;
                        }
                        u8v = u8v.wrapping_sub(i12 as u32).wrapping_add(c8 as u32);
                        let n = ((DAT_006d9980 as u8).wrapping_sub(c8 as u8) & 0x1f) as u32;
                        b7 = ((b7 as u32).wrapping_shl(n)) as u8;
                        c8 = i12;
                    }
                    if (DAT_00732a34 as i16 as i32) <= i11 {
                        break;
                    }
                    if (DAT_00732e8c as i16 as i32) <= i11 {
                        if (DAT_0070acc0 as i32) < ((c8 as u32).wrapping_add(u8v) as i32) {
                            u8v = (DAT_0070acc0 as i32 as u32).wrapping_sub(c8 as u32);
                            if (u8v as i32) < 1 {
                                break;
                            }
                        }
                        let mut p13: *mut u8 = (c8 as u32).wrapping_add(i4 as u32) as *mut u8;
                        while u8v != 0 {
                            u8v = u8v.wrapping_sub(1);
                            let mut u5 = DAT_006da9fa;
                            if (b7 & 0x80) == 0 {
                                u5 = DAT_00732e50;
                                if DAT_0071e524 != 0 && (b10 & 0x80) != 0 {
                                    u5 = DAT_0071b012;
                                }
                            }
                            p13.write(u5);
                            p13 = p13.byte_add(1);
                            b7 = b7.wrapping_mul(2);
                            b10 = b10.wrapping_mul(2);
                        }
                        i4 = (i4 as u32).wrapping_add(DAT_005f1d24.read() as u32) as i32;
                    }
                    i11 = i11.wrapping_add(1);
                    b10 = pb.read() >> 1;
                    pb = pb.byte_add(2);
                    if !(i11 < ((DAT_00732e52 as i16 as i32 as u32).wrapping_add(b2 as u32) as i32))
                    {
                        break;
                    }
                }
            }
        }

        DAT_00732e58 = DAT_00732e58.wrapping_add(b1 as u16);
        if DAT_0070acc0 <= (DAT_00732e58 as i16) && DAT_006ffbf0 != 0 {
            flush_block!();
            DAT_00732e58 = DAT_006d9980;
            DAT_00732e52 = DAT_00732e52.wrapping_add(b2 as u16).wrapping_add(2);
        }
    }
}
