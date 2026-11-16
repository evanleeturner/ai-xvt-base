// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006da600: [u8; 576];
    static mut DAT_006da840: u8;
    static mut DAT_0071b014: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004290b0(a: i32, b: i32, c: u32, d: i32, e: i32);
}

unsafe fn best_index(p: *const u8) -> u8 {
    // SAFETY: p points at a 4-byte pixel; the palette is the game's own memory.
    unsafe {
        let b3 = p.read();
        let b1 = p.add(1).read();
        let b2 = p.add(2).read();
        let mut c12: u8 = 0x40;
        let mut c4: u8 = 0x40;
        let mut best: i32 = 0x7fffffff;
        let mut q = (&raw mut DAT_006da600) as *const u8;
        let end = (&raw mut DAT_006da840) as *const u8;
        loop {
            let a = ((b3 >> 2) as i32).wrapping_sub(q.read() as i32);
            let b = ((b1 >> 2) as i32).wrapping_sub(q.add(1).read() as i32);
            let c = ((b2 >> 2) as i32).wrapping_sub(q.add(2).read() as i32);
            let d = a
                .wrapping_mul(a)
                .wrapping_add(c.wrapping_mul(c))
                .wrapping_add(b.wrapping_mul(b));
            if d < best {
                best = d;
                c4 = c12;
            }
            q = q.add(3);
            c12 = c12.wrapping_add(1);
            if !(q < end) {
                break;
            }
        }
        c4
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004626c0(param_1: *mut i32) {
    // SAFETY: param_1 points at the game's image block; all offsets are
    // read and written where the original does.
    unsafe {
        let base = param_1 as usize as i32;
        let mut local_20: *mut u8 = (param_1.read().wrapping_add(base)) as usize as *mut u8;
        if param_1.add(0xb).read() == 0x18 {
            let mut pix = (param_1.add(3).read().wrapping_add(base)) as usize as *const u8;
            param_1
                .add(5)
                .write((local_20 as usize as i32).wrapping_sub(base));
            let mut i: u32 = 0;
            if param_1.add(0xc).read() != 0 {
                loop {
                    let c = best_index(pix);
                    pix = pix.add(4);
                    i = i.wrapping_add(1);
                    local_20.write(c);
                    local_20 = local_20.add(1);
                    if !(i < param_1.add(0xc).read() as u32) {
                        break;
                    }
                }
            }
        }
        let mut local_10: u32 = 0;
        if param_1.add(6).read() != 0 {
            loop {
                let off = param_1
                    .add(4)
                    .read()
                    .wrapping_add((local_10.wrapping_mul(4)) as i32)
                    .wrapping_add(base);
                let i6 = ((off as usize) as *const i32)
                    .read_unaligned()
                    .wrapping_add(base);
                ((i6.wrapping_add(0xc)) as usize as *mut i32)
                    .write_unaligned((local_20 as usize as i32).wrapping_sub(i6));
                if ((i6.wrapping_add(0x24)) as usize as *const i32).read_unaligned() == 0x18 {
                    let mut pix = (((i6.wrapping_add(4)) as usize as *const i32)
                        .read_unaligned()
                        .wrapping_add(i6)) as usize as *const u8;
                    if DAT_0071b014 != 0 {
                        let a = ((i6.wrapping_add(8)) as usize as *const i32)
                            .read_unaligned()
                            .wrapping_add(i6);
                        let c = ((i6.wrapping_add(0x10)) as usize as *const u32).read_unaligned();
                        let d = ((i6.wrapping_add(0x14)) as usize as *const i32).read_unaligned();
                        let e = ((i6.wrapping_add(0x20)) as usize as *const i32).read_unaligned();
                        FUN_004290b0(a, pix as usize as i32, c, d, e);
                    }
                    let mut i: u32 = 0;
                    if ((i6.wrapping_add(0x28)) as usize as *const i32).read_unaligned() != 0 {
                        loop {
                            let c = best_index(pix);
                            pix = pix.add(4);
                            i = i.wrapping_add(1);
                            local_20.write(c);
                            local_20 = local_20.add(1);
                            if !(i
                                < ((i6.wrapping_add(0x28)) as usize as *const u32).read_unaligned())
                            {
                                break;
                            }
                        }
                    }
                }
                local_10 = local_10.wrapping_add(1);
                if !(local_10 < param_1.add(6).read() as u32) {
                    break;
                }
            }
        }
    }
}
