// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn FID_conflict___expand(
        param_1: *mut core::ffi::c_void,
        param_2: i32,
    ) -> *mut core::ffi::c_void;
}

#[inline(always)]
unsafe fn rd(base: *mut u8, off: usize) -> u32 {
    // SAFETY: caller guarantees base points at the game's object.
    unsafe { base.byte_add(off).cast::<u32>().read_unaligned() }
}

#[inline(always)]
unsafe fn wr(base: *mut u8, off: usize, v: u32) {
    // SAFETY: caller guarantees base points at the game's object.
    unsafe { base.byte_add(off).cast::<u32>().write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00427480(param_1: i32) {
    let base = param_1 as usize as *mut u8;
    let mut u_var7: u32 = 0;
    if param_1 == 0 {
        return;
    }
    // SAFETY: base points at the game's own object, and the buffer it
    // holds is the game's run array; reads and writes follow the original.
    unsafe {
        let mut pcvar5 = rd(base, 0x1086) as usize as *mut u8;
        let b_var1 = *pcvar5.add(3);
        wr(base, 0x108e, 0);
        wr(base, 0x1092, (b_var1 as u32).wrapping_add(1));
        *pcvar5.add(3) = 0xff;
        if rd(base, 0x1028) == 0 {
            let mut pcvar4 = pcvar5;
            if rd(base, 0x1034).wrapping_mul(rd(base, 0x1030)) != 0 {
                loop {
                    let t = rd(base, 0x1092);
                    let pcvar2: *mut u8;
                    let u_var6: u32;
                    if t == 0 {
                        pcvar2 = pcvar4.add(6);
                        u_var6 = *pcvar4.add(9) as u32;
                    } else {
                        u_var6 = t.wrapping_sub(1);
                        pcvar2 = pcvar4;
                    }
                    wr(base, 0x1092, u_var6);
                    if *pcvar5 == *pcvar2
                        && *pcvar5.add(1) == *pcvar2.add(1)
                        && *pcvar5.add(2) == *pcvar2.add(2)
                        && *pcvar5.add(3) != 0xff
                    {
                        *pcvar5.add(3) = (*pcvar5.add(3)).wrapping_add(1);
                    } else {
                        if rd(base, 0x108e) != 0 {
                            pcvar5 = pcvar5.add(6);
                        }
                        let t = rd(base, 0x108e).wrapping_add(1);
                        wr(base, 0x108e, t);
                        pcvar5
                            .cast::<u32>()
                            .write_unaligned(pcvar2.cast::<u32>().read_unaligned());
                        pcvar5
                            .add(4)
                            .cast::<u16>()
                            .write_unaligned(pcvar2.add(4).cast::<u16>().read_unaligned());
                        *pcvar5.add(3) = 0;
                    }
                    u_var7 = u_var7.wrapping_add(1);
                    let u_var6 = rd(base, 0x1034).wrapping_mul(rd(base, 0x1030));
                    pcvar4 = pcvar2;
                    if !(u_var7 <= u_var6 && u_var6.wrapping_sub(u_var7) != 0) {
                        break;
                    }
                }
            }
        } else {
            let mut pcvar4 = pcvar5;
            if rd(base, 0x1034).wrapping_mul(rd(base, 0x1030)) != 0 {
                loop {
                    let t = rd(base, 0x1092);
                    let pcvar2: *mut u8;
                    let u_var6: u32;
                    if t == 0 {
                        pcvar2 = pcvar5.add(6);
                        u_var6 = *pcvar5.add(9) as u32;
                    } else {
                        u_var6 = t.wrapping_sub(1);
                        pcvar2 = pcvar5;
                    }
                    wr(base, 0x1092, u_var6);
                    if *pcvar4 == *pcvar2
                        && *pcvar4.add(1) == *pcvar2.add(1)
                        && *pcvar4.add(2) == *pcvar2.add(2)
                        && pcvar4.add(4).cast::<u16>().read_unaligned()
                            == pcvar2.add(4).cast::<u16>().read_unaligned()
                        && *pcvar4.add(3) != 0xff
                    {
                        *pcvar4.add(3) = (*pcvar4.add(3)).wrapping_add(1);
                    } else {
                        if rd(base, 0x108e) != 0 {
                            pcvar4 = pcvar4.add(6);
                        }
                        let t = rd(base, 0x108e).wrapping_add(1);
                        wr(base, 0x108e, t);
                        pcvar4
                            .cast::<u32>()
                            .write_unaligned(pcvar2.cast::<u32>().read_unaligned());
                        pcvar4
                            .add(4)
                            .cast::<u16>()
                            .write_unaligned(pcvar2.add(4).cast::<u16>().read_unaligned());
                        *pcvar4.add(3) = 0;
                    }
                    u_var7 = u_var7.wrapping_add(1);
                    let u_var6 = rd(base, 0x1034).wrapping_mul(rd(base, 0x1030));
                    pcvar5 = pcvar2;
                    if !(u_var7 <= u_var6 && u_var6.wrapping_sub(u_var7) != 0) {
                        break;
                    }
                }
            }
        }
        let buf = rd(base, 0x1086) as usize as *mut core::ffi::c_void;
        let cnt = rd(base, 0x108e);
        let pv_var3 = FID_conflict___expand(buf, cnt.wrapping_mul(6) as i32);
        wr(base, 0x1086, pv_var3 as usize as u32);
        let t = rd(base, 0x108e);
        if rd(base, 0x1024) == 1 {
            if (rd(base, 0x1034)
                .wrapping_mul(rd(base, 0x1030))
                .wrapping_mul(3)
                >> 2)
                <= t
            {
                wr(base, 0x102c, 1);
                return;
            }
        } else if (rd(base, 0x1034).wrapping_mul(rd(base, 0x1030)) >> 1) <= t {
            wr(base, 0x102c, 1);
        }
    }
}
