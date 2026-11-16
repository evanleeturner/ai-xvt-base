// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_005201dc: f32;
    static mut DAT_00602698: *mut f32;
    static mut DAT_00602688: *mut f32;
    static mut DAT_0060267c: *mut f32;
    static mut DAT_006a8858: i32;
    static mut DAT_006d4984: u16;
    static mut DAT_007091ae: u16;
    static mut DAT_00709ffc: i32;
    static mut DAT_00732f3c: i32;
    static mut DAT_006a885e: [u8; 4];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004859c0(
    param_1: i32,
    param_2: *mut u32,
    param_3: *mut f32,
    param_4: *mut f32,
) -> u32 {
    let mut param_3 = param_3;
    let mut pf10 = param_4;
    // SAFETY: all pointers are the caller's vertex/edge buffers and the
    // globals are the game's own memory, accessed where the original does.
    unsafe {
        if 0x8000_0000u32 < (*param_4.add(2)).to_bits() {
            let pf11 = param_3.add(2);
            pf10 = param_3;
            param_3 = param_4;
            if 0x8000_0000u32 < (*pf11).to_bits() {
                return 0xffff_ffff;
            }
        }
        if 0x8000_0000u32 < (*param_3.add(2)).to_bits() {
            DAT_00602698 = DAT_00602688;
            let a = (param_1 as u32).wrapping_add(0xbc) as *const i32;
            let b = (param_1 as u32).wrapping_add(200) as *mut i32;
            let sum = (a.read_unaligned() as u32).wrapping_add(b.read_unaligned() as u32);
            DAT_0060267c = (DAT_006a8858 as u32).wrapping_add(sum.wrapping_mul(0x18)) as *mut f32;
            DAT_00602688 = DAT_0060267c;
            let b = (param_1 as u32).wrapping_add(200) as *mut i32;
            b.write_unaligned((b.read_unaligned() as u32).wrapping_add(1) as i32);
            let p = &raw mut DAT_006a885e as *mut u32;
            p.write_unaligned(p.read_unaligned().wrapping_add(1));
            let f4 = _DAT_005201dc / *pf10.add(2);
            let i6 = ((DAT_006d4984 >> 1) as i32).wrapping_add(DAT_00709ffc);
            let f1 = *pf10.add(1);
            let f2 = *param_3.add(1);
            let f5 =
                *param_3.add(2) / ((*param_3.add(2) - (DAT_00732f3c as f32) * f4) + _DAT_005201dc);
            let f3 = *param_3.add(1);
            *DAT_00602688 = (DAT_00732f3c as f32)
                * (((*pf10 - ((DAT_007091ae >> 1) as f32)) * f4 - *param_3) * f5 + *param_3);
            *DAT_00602688.add(1) =
                (DAT_00732f3c as f32) * (((f1 - (i6 as f32)) * f4 - f2) * f5 + f3);
            *DAT_00602688 = ((DAT_007091ae >> 1) as f32) + *DAT_00602688;
            *DAT_00602688.add(1) = (((DAT_006d4984 >> 1) as i32).wrapping_add(DAT_00709ffc) as f32)
                + *DAT_00602688.add(1);
            *DAT_00602688.add(2) = DAT_00732f3c as f32;
            *DAT_00602688.add(3) = (*pf10.add(3) - *param_3.add(3)) * f5 + *param_3.add(3);
            param_3 = DAT_00602688;
        }
        let mut u9: u32;
        if (*param_3.add(1)).to_bits() < 0x8000_0001 {
            u9 = (*param_3.add(1) as i64) as u32;
            if (u9 as i32 as f32) != *param_3.add(1) {
                u9 = u9.wrapping_add(1);
            }
        } else {
            u9 = 0;
        }
        let mut u7: u32;
        if (*pf10.add(1)).to_bits() < 0x8000_0001 {
            u7 = (*pf10.add(1) as i64) as u32;
            if (u7 as i32 as f32) != *pf10.add(1) {
                u7 = u7.wrapping_add(1);
            }
        } else {
            u7 = 0;
        }
        if u7 != u9 {
            let mut u8 = u7;
            let mut pf11 = pf10;
            if (u7 as i32) < (u9 as i32) {
                u8 = u9;
                u9 = u7;
                pf11 = param_3;
                param_3 = pf10;
            }
            if 0 < (u8 as i32) {
                let u7 = DAT_006d4984 as u32;
                if (u9 as i32) < (u7 as i32) {
                    if (u7 as i32) < (u8 as i32) {
                        u8 = u7;
                    }
                    *param_2 = u8;
                    let f3 = _DAT_005201dc / (*pf11.add(1) - *param_3.add(1));
                    *param_2.add(4) = ((*pf11 - *param_3) * f3) as i64 as u32;
                    let f1 = *pf11.add(3);
                    let f2 = *param_3.add(3);
                    *param_2.add(6) = 0;
                    *param_2.add(5) = ((f1 - f2) * f3) as i64 as u32;
                    let f1 = *param_3.add(1);
                    *param_2.add(2) = ((*param_2.add(4) as f32) * ((u9 as i32 as f32) - f1)
                        + *param_3) as i64 as u32;
                    let f2 = *param_3.add(3);
                    *param_2.add(1) = u9;
                    *param_2.add(3) =
                        (((u9 as i32 as f32) - f1) * (*param_2.add(5) as f32) + f2) as i64 as u32;
                    return u9;
                }
                return 0xffff_ffff;
            }
            return 0xffff_ffff;
        }
        0xffff_ffff
    }
}
