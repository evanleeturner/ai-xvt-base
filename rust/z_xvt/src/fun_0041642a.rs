// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00730894: i16;
    static mut DAT_00732e58: u16;
    static mut DAT_0070acc0: i16;
    static mut DAT_005369d4: u16;
    static mut DAT_005369d8: u16;
    static mut DAT_005369d0: u16;
    static mut DAT_005369dc: u16;
    static mut DAT_006d4a4e: u8;
    static mut DAT_00732e52: u16;
    static mut DAT_006d9980: u16;
    static mut DAT_00732e8c: u16;
    static mut DAT_00732a34: u16;
    static mut DAT_0070a188: i8;
    static mut DAT_006d4998: i8;
    static mut DAT_0070acc2: u8;
    static mut DAT_00709ff4: u32;
    static mut DAT_006ffbf0: i32;
    static mut DAT_0071e524: i8;
    static mut DAT_0052552c: i32;
    static mut DAT_00525534: u32;
    static mut DAT_0052550c: u32;
    static mut DAT_00732e50: u16;
    static mut DAT_0071b012: u16;
    static mut DAT_006da9fa: u16;
    static mut DAT_007325e0: [u8; 0];
    static mut in_AL_input: u8;
    static mut in_ZF_input: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004169b0(param_1: i32);
    fn FUN_00461ff0(param_1: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004651f0();
}

unsafe fn flush_block() {
    // SAFETY: the globals and the callee are the game's own memory and code.
    unsafe {
        if DAT_00730894 != 0 && (DAT_00732e58 as i16) < DAT_0070acc0 {
            DAT_005369d4 = DAT_0070acc0 as u16;
            DAT_005369d8 = DAT_00732e58;
            DAT_005369d0 = (DAT_006d4a4e as u16).wrapping_add(DAT_00732e52);
            DAT_005369dc = DAT_00732e52;
            if (DAT_00732e58 as i32) < (DAT_006d9980 as i16 as i32) {
                DAT_005369d8 = DAT_006d9980;
            }
            if (DAT_00732e52 as i32) < (DAT_00732e8c as i16 as i32) {
                DAT_005369dc = DAT_00732e8c;
            }
            if (DAT_00732a34 as i16 as i32) < (DAT_005369d0 as i32) {
                DAT_005369d0 = DAT_00732a34;
            }
            if DAT_005369dc < DAT_005369d0 {
                FUN_004169b0(0);
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0041642a() {
    // SAFETY: globals and callees are the game's own; the glyph pointers and
    // the video pointer are read and written exactly where the original does.
    unsafe {
        let mut in_al: u8 = in_AL_input;
        let in_zf: u8 = in_ZF_input;

        if in_zf != 0 {
            flush_block();
            DAT_00732e58 = DAT_006d9980;
            DAT_00732e52 = DAT_00732e52
                .wrapping_add(DAT_006d4a4e as u16)
                .wrapping_add(1);
            return;
        }
        if in_al > 0x1f {
            if DAT_0070a188 != 0 && DAT_006d4998 == 0 && in_al > 0x60 && in_al < 0x7b {
                in_al = in_al.wrapping_sub(0x20);
            }
            let pbvar4 = ((in_al.wrapping_sub(0x20) as u32)
                .wrapping_mul(DAT_0070acc2 as u32)
                .wrapping_add(DAT_00709ff4)) as *const u8;
            let bvar1: u8 = pbvar4.read();
            let bvar2: u8 = pbvar4.add(1).read();
            let ustack_c: u32 = bvar1 as u32;
            if (DAT_0070acc0 as i32) <= (DAT_00732e58 as i16 as i32).wrapping_add(ustack_c as i32)
                && DAT_006ffbf0 != 0
            {
                flush_block();
                DAT_00732e58 = DAT_006d9980;
                DAT_00732e52 = DAT_00732e52.wrapping_add(bvar2 as u16).wrapping_add(2);
            }
            let mut uvar8: u32 = 0;
            let mut istack8: i32 = DAT_00732e52 as i16 as i32;
            let ustack10: u32 = bvar2 as u32;
            let mut puvar10 = pbvar4.add(2) as *const u32;
            if istack8 < (ustack10 as i32).wrapping_add(istack8) {
                loop {
                    let mut uvar9: u32 = puvar10.read_unaligned();
                    let mut uvar7: u32 = ustack_c;
                    if DAT_0071e524 != 0 {
                        uvar7 = ustack_c.wrapping_add(1);
                    }
                    let mut c4: i32 = DAT_00732e58 as i16 as i32;
                    let ivar5: i32 = DAT_006d9980 as i16 as i32;
                    if c4 < ivar5 {
                        if (uvar7 as i32) <= ivar5.wrapping_sub(c4) {
                            break;
                        }
                        uvar7 = uvar7.wrapping_sub(ivar5 as u32).wrapping_add(c4 as u32);
                        let cs: i8 = DAT_00732e58 as i8;
                        let sh =
                            ((DAT_006d9980 as i8 as i32).wrapping_sub(cs as i32) & 0x1f) as u32;
                        uvar9 = uvar9.wrapping_shl(sh);
                        c4 = ivar5;
                    }
                    if (DAT_00732a34 as i16 as i32) <= istack8 {
                        break;
                    }
                    if istack8 < (DAT_00732e8c as i16 as i32) {
                        uvar7 = 0;
                    }
                    if (DAT_0070acc0 as i32) < c4.wrapping_add(uvar7 as i32) {
                        uvar7 = (DAT_0070acc0 as i32 as u32).wrapping_sub(c4 as u32);
                        if (uvar7 as i32) < 1 {
                            break;
                        }
                    }
                    let ivar5b = FUN_00461ff0(istack8);
                    let mut uvar6: u32 = (ivar5b as u32).wrapping_add((c4 as u32).wrapping_mul(2));
                    if DAT_0052552c != 0x13 && DAT_00525534 == 0xa0000 {
                        let divisor = DAT_0052550c;
                        let q: u32;
                        let r: u32;
                        core::arch::asm!(
                            "div {d}",
                            d = in(reg) divisor,
                            inout("eax") uvar6 => q,
                            inout("edx") 0u32 => r,
                        );
                        let _ = q;
                        uvar6 = r;
                        FUN_004651f0();
                    }
                    let mut puvar3 = uvar6.wrapping_add(DAT_00525534 & 0xffff_fffe) as *mut u16;
                    while uvar7 != 0 {
                        uvar7 = uvar7.wrapping_sub(1);
                        let idx: u32;
                        if uvar9 & 0x8000_0000 == 0 {
                            if DAT_0071e524 == 0 || (uvar8 & 0x8000_0000) == 0 {
                                idx = DAT_00732e50 as u32;
                            } else {
                                idx = DAT_0071b012 as u32;
                            }
                        } else {
                            idx = DAT_006da9fa as u32;
                        }
                        let src = (&raw mut DAT_007325e0)
                            .cast::<u8>()
                            .byte_add(idx.wrapping_mul(2) as usize)
                            .cast::<u16>()
                            .read_unaligned();
                        puvar3.write_unaligned(src);
                        uvar9 = uvar9.wrapping_mul(2);
                        uvar8 = uvar8.wrapping_mul(2);
                        puvar3 = puvar3.add(1);
                    }
                    istack8 = istack8.wrapping_add(1);
                    uvar8 = puvar10.read_unaligned() >> 1;
                    puvar10 = puvar10.add(2);
                    if !(istack8 < (DAT_00732e52 as i16 as i32).wrapping_add(ustack10 as i32)) {
                        break;
                    }
                }
            }
            DAT_00732e58 = DAT_00732e58.wrapping_add(bvar1 as u16);
            if DAT_0070acc0 <= (DAT_00732e58 as i16) && DAT_006ffbf0 != 0 {
                flush_block();
                DAT_00732e58 = DAT_006d9980;
                DAT_00732e52 = DAT_00732e52.wrapping_add(bvar2 as u16).wrapping_add(2);
            }
        }
    }
}
