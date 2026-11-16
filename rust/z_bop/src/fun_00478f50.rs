// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0060f204: i32;
    static mut _DAT_0060f208: i32;
    static mut _DAT_0060f1f0: i32;
    static mut DAT_0060f1d8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0047a430(param_1: *mut i8) -> i32;
}

unsafe fn rd(a: i32) -> i32 {
    // SAFETY: the address is a game structure pointer, read as the original does.
    unsafe { (a as u32 as usize as *const i32).read_unaligned() }
}

unsafe fn slen1(a: i32) -> i32 {
    // SAFETY: a points at a NUL-terminated string in the game's memory.
    unsafe {
        let mut p = a as u32 as usize as *const u8;
        let mut n: i32 = 0;
        loop {
            let c = p.read();
            p = p.add(1);
            n = n.wrapping_add(1);
            if c == 0 {
                break;
            }
        }
        n
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00478f50(param_1: u32, param_2: *mut u32, param_3: *mut u32) -> i32 {
    if param_2.is_null() {
        return 0;
    }
    // SAFETY: all pointers are the game's own structures, accessed exactly as
    // the original does; globals and callee belong to the game.
    unsafe {
        let mut i_var8: i32 = 0x18;
        let name = param_2.read_unaligned();
        if name != 0 {
            i_var8 = slen1(name as i32).wrapping_add(0x18);
        }
        let mut i_var10: i32 = param_2.add(5).read_unaligned() as i32;
        if i_var10 != 0 && param_2.add(4).read_unaligned() != 0 {
            let mut p7: i32 = rd(i_var10.wrapping_add(8));
            if p7 != 0 {
                match param_2.add(1).read_unaligned() {
                    1 => {
                        let mut i_var6: i32 = 0;
                        let mut i_var4: i32 = 0;
                        if 0 < rd(i_var10.wrapping_add(4)) {
                            loop {
                                let mut p3: i32 =
                                    p7.wrapping_add(i_var4.wrapping_add(1).wrapping_mul(4));
                                let mut i_var12: i32 = i_var4.wrapping_add(1);
                                loop {
                                    i_var4 = i_var12;
                                    i_var6 = i_var6.wrapping_add(1);
                                    if rd(p3.wrapping_add(8)) == -1 {
                                        i_var4 = i_var4.wrapping_add(3);
                                        break;
                                    }
                                    let p1 = p3.wrapping_add(12);
                                    p3 = p3.wrapping_add(8);
                                    i_var12 = i_var4.wrapping_add(2);
                                    if rd(p1) == -1 {
                                        i_var4 = i_var4.wrapping_add(4);
                                        break;
                                    }
                                }
                                if !(i_var4 < rd(i_var10.wrapping_add(4))) {
                                    break;
                                }
                            }
                        }
                        i_var8 = i_var8
                            .wrapping_add(4)
                            .wrapping_add(i_var6.wrapping_mul(100))
                            .wrapping_add(DAT_0060f204.wrapping_mul(0xc));
                    }
                    2 => {
                        i_var8 = i_var8.wrapping_add(0x30);
                    }
                    3 => {
                        DAT_0060f204 = rd(i_var10.wrapping_add(4));
                        i_var8 = i_var8.wrapping_add(DAT_0060f204.wrapping_mul(0xc));
                    }
                    4 => {
                        i_var8 = i_var8.wrapping_add(0xc);
                    }
                    5 => {
                        i_var8 = i_var8.wrapping_add(0x24);
                    }
                    6 => {
                        i_var8 = i_var8.wrapping_add(0xc);
                    }
                    7 => {
                        i_var8 = i_var8.wrapping_add(slen1(p7));
                    }
                    9 => {
                        let mut i_var6 = rd(i_var10.wrapping_add(4));
                        if rd(i_var10.wrapping_add(4)) < rd(i_var10.wrapping_add(0x10)) {
                            i_var6 = rd(i_var10.wrapping_add(0x10));
                        }
                        if i_var6 < rd(i_var10.wrapping_add(0x1c)) {
                            i_var6 = rd(i_var10.wrapping_add(0x1c));
                        }
                        if i_var6 < rd(i_var10.wrapping_add(0x28)) {
                            i_var6 = rd(i_var10.wrapping_add(0x28));
                        }
                        if i_var6 < rd(i_var10.wrapping_add(0x34)) {
                            i_var6 = rd(i_var10.wrapping_add(0x34));
                        }
                        if i_var6 < rd(i_var10.wrapping_add(0x40)) {
                            i_var6 = rd(i_var10.wrapping_add(0x40));
                        }
                        i_var8 = i_var8.wrapping_add(i_var6.wrapping_mul(0x38));
                    }
                    0xb => {
                        i_var8 = i_var8.wrapping_add(rd(i_var10.wrapping_add(4)).wrapping_mul(0xc));
                        _DAT_0060f208 = i_var10;
                        param_3.add(0x21).write_unaligned(i_var10 as u32);
                    }
                    0xd => {
                        i_var8 = i_var8.wrapping_add(rd(i_var10.wrapping_add(4)).wrapping_mul(8));
                    }
                    0xf => {
                        let a = rd(i_var10.wrapping_add(0x14));
                        if a != 0 {
                            let b = rd(i_var10.wrapping_add(0x20));
                            if b != 0 {
                                i_var8 = i_var8
                                    .wrapping_add(4)
                                    .wrapping_add(
                                        rd(b)
                                            .wrapping_sub(1)
                                            .wrapping_mul(rd(a).wrapping_sub(1))
                                            .wrapping_mul(100),
                                    )
                                    .wrapping_add(DAT_0060f204.wrapping_mul(0xc));
                            }
                        }
                    }
                    0x10 => {
                        let mut i_var4: i32 = 0;
                        let mut i_var6: i32 = 0;
                        let mut i_stack_d4: i32 = rd(i_var10.wrapping_add(4));
                        if 0 < i_stack_d4 {
                            loop {
                                i_var10 = rd(p7);
                                let mut i_var12: i32 = i_var4;
                                if 2 < i_var10 {
                                    let mut i_var11: i32 = i_var4.wrapping_add(1);
                                    let mut i_var9: i32 = i_var11.wrapping_sub(i_var4);
                                    loop {
                                        i_var12 = i_var11;
                                        if !(i_var9 < i_var10) {
                                            break;
                                        }
                                        i_var6 = i_var6.wrapping_add(1);
                                        if i_var11.wrapping_add(2).wrapping_sub(i_var10) != i_var4 {
                                            i_var12 = i_var11.wrapping_add(3);
                                            if i_var12.wrapping_sub(i_var10) == i_var4 {
                                                break;
                                            }
                                        }
                                        i_var11 = i_var11.wrapping_add(2);
                                        i_var9 = i_var11.wrapping_sub(i_var4);
                                    }
                                }
                                p7 = p7.wrapping_add(4);
                                i_stack_d4 = i_stack_d4.wrapping_sub(1);
                                i_var4 = i_var12;
                                if i_stack_d4 == 0 {
                                    break;
                                }
                            }
                        }
                        i_var8 = i_var8
                            .wrapping_add(4)
                            .wrapping_add(i_var6.wrapping_mul(100))
                            .wrapping_add(DAT_0060f204.wrapping_mul(0xc));
                    }
                    0x11 => {
                        let mut p7b = rd(i_var10.wrapping_add(0x14));
                        if p7b != 0 {
                            let mut i_var6: i32 = 0;
                            let mut cnt = rd(i_var10.wrapping_add(0x10));
                            if 0 < cnt {
                                loop {
                                    let i_var4 = rd(p7b);
                                    p7b = p7b.wrapping_add(4);
                                    i_var6 = i_var6.wrapping_add(i_var4).wrapping_sub(2);
                                    cnt = cnt.wrapping_sub(1);
                                    if cnt == 0 {
                                        break;
                                    }
                                }
                            }
                            i_var8 = i_var8
                                .wrapping_add(4)
                                .wrapping_add(i_var6.wrapping_mul(100))
                                .wrapping_add(DAT_0060f204.wrapping_mul(0xc));
                        }
                    }
                    0x14 => {
                        let r = FUN_0047a430(p7 as u32 as usize as *mut i8);
                        i_var8 = i_var8.wrapping_add(r);
                    }
                    0x15 => {
                        i_var8 = i_var8.wrapping_add(rd(i_var10.wrapping_add(4)).wrapping_mul(4));
                    }
                    0x16 => {
                        i_var8 = i_var8.wrapping_add(0x10);
                    }
                    0x17 => {
                        i_var8 = i_var8.wrapping_add(0x30);
                    }
                    0x19 => {
                        i_var8 = i_var8.wrapping_add(0x48);
                    }
                    _ => {}
                }
            }
        }
        i_var10 = 0;
        let cnt2 = param_2.add(2).read_unaligned();
        if cnt2 != 0 {
            let mut au_stack_d0: [u32; 52] = [0; 52];
            let mut src = param_3;
            for i in 0..52usize {
                au_stack_d0[i] = src.read_unaligned();
                src = src.add(1);
            }
            let mut i_var6: i32 = 0;
            _DAT_0060f208 = 0;
            _DAT_0060f1f0 = 0;
            DAT_0060f1d8 = 0;
            i_var8 = i_var8.wrapping_add(cnt2.wrapping_mul(4) as i32);
            if 0 < cnt2 as i32 {
                loop {
                    i_var6 = i_var6.wrapping_add(1);
                    i_var10 = i_var10.wrapping_add(4);
                    let base = param_2.add(3).read_unaligned();
                    let addr = base.wrapping_sub(4).wrapping_add(i_var10 as u32);
                    let child = (addr as usize as *const u32).read_unaligned();
                    let i_var4 = FUN_00478f50(
                        param_1,
                        child as usize as *mut u32,
                        au_stack_d0.as_mut_ptr(),
                    );
                    i_var8 = i_var8.wrapping_add(i_var4);
                    if !(i_var6 < param_2.add(2).read_unaligned() as i32) {
                        break;
                    }
                }
            }
        }
        i_var8
    }
}
