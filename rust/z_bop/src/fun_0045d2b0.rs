// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a2000: i16;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a1ff8: u32;
    static mut DAT_009a73b0: u16;
    static mut DAT_009a73b6: [u8; 4];
    static mut DAT_009a73da: u8;
    static mut DAT_00a07cec: u16;
    static mut DAT_00a08104: i32;
    static mut DAT_009d1320: [u8; 0];
    static mut DAT_0051a1e4: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00432fa0(a: u16, b: u16, c: u16) -> u16;
    fn FUN_00403540(a: u32, b: u32);
    fn FUN_00403070(a: u32) -> u16;
    fn FUN_0045dba0(a: u16, b: u32) -> u16;
}

unsafe fn r8(a: i32) -> u8 {
    // SAFETY: the address is the game's own memory, as in the original.
    unsafe { (a as usize as *const u8).read() }
}

unsafe fn r16(a: i32) -> u16 {
    // SAFETY: the address is the game's own memory, as in the original.
    unsafe { (a as usize as *const u16).read_unaligned() }
}

unsafe fn r32(a: i32) -> i32 {
    // SAFETY: the address is the game's own memory, as in the original.
    unsafe { (a as usize as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045d2b0(
    param_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: u16,
    param_5: u16,
) -> u16 {
    let mut unaff_ebp: u32 = 0;
    let mut u_stack_6: u16 = 0xffff;
    let mut local_4: u16 = 0xffff;
    let mut u_stack_2: u16 = 0xffff;
    let mut local_8: u16 = 0;
    // SAFETY: globals and callees are the game's own; all pointers are the
    // game's records, accessed at the offsets the original uses.
    unsafe {
        if 0 < DAT_009a2000 {
            loop {
                let mut u_var2 = FUN_00432fa0(local_8, param_1, param_2);
                let u_var3 = FUN_00432fa0(local_8, param_4, param_5);
                if param_3 == 1 {
                    u_var2 |= u_var3;
                } else {
                    u_var2 &= u_var3;
                }
                if u_var2 != 0 {
                    unaff_ebp = (unaff_ebp & 0xffff_0000) | (DAT_00a07cec as u32);
                    if (DAT_00a07cec as i32) < DAT_009a1fe0 {
                        loop {
                            let u_var6 = unaff_ebp & 0xffff;
                            let idx = unaff_ebp as u16;
                            'body: {
                                let rec =
                                    u_var6.wrapping_mul(0x23).wrapping_add(DAT_009a1fe8 as u32)
                                        as i32;
                                if r8(rec.wrapping_add(3)) == 0 {
                                    break 'body;
                                }
                                let i4 = r32(rec.wrapping_add(0x1f));
                                let i4 = r32(i4.wrapping_add(0xa9));
                                let mut i5: i32 = 0xe;
                                let mut p7 = ((&raw mut DAT_009d1320) as *mut u8 as usize as u32)
                                    .wrapping_add(
                                        (r8(i4.wrapping_add(0x35)) as u32).wrapping_mul(0x55),
                                    ) as usize
                                    as *const i8;
                                let mut p8 = (&raw mut DAT_0051a1e4) as *mut i8 as *const i8;
                                let mut b9: bool;
                                loop {
                                    // i5 is nonzero on first entry
                                    i5 -= 1;
                                    b9 = p7.read() == p8.read();
                                    p7 = p7.add(1);
                                    p8 = p8.add(1);
                                    if !b9 || i5 == 0 {
                                        break;
                                    }
                                }
                                if !b9 {
                                    break 'body;
                                }
                                if (r8(i4.wrapping_add(0x57)) as u16) != local_8 {
                                    break 'body;
                                }
                                if r16(DAT_00a08104.wrapping_add(0x36e)) == idx {
                                    break 'body;
                                }
                                let r: i32 = 'calc: {
                                    if idx == 0xffff {
                                        break 'calc 0;
                                    }
                                    let rec = (DAT_009a1fe8 as u32)
                                        .wrapping_add(u_var6.wrapping_mul(0x23))
                                        as i32;
                                    let i5 = r32(rec.wrapping_add(0x1f));
                                    if i5 == 0 {
                                        let b = r8(rec.wrapping_add(3)) == 0;
                                        break 'calc 1 - (b as i32);
                                    }
                                    if r8(rec.wrapping_add(3)) == 0 {
                                        break 'calc 0;
                                    }
                                    let i4 = r32(i5.wrapping_add(0xa9));
                                    if i4 == 0 {
                                        break 'calc 1;
                                    }
                                    if r8(i4.wrapping_add(0x5e)) == 0x15
                                        && r8(i4.wrapping_add(0x5f)) != 0
                                    {
                                        break 'calc 0;
                                    }
                                    let c = r8(i4.wrapping_add(7)) as i8;
                                    if c == 1 || c == 3 || c == 4 {
                                        break 'calc 0;
                                    }
                                    if r8(i4.wrapping_add(0x23d)) == 3
                                        && r8(i4.wrapping_add(0x241)) != 0
                                    {
                                        FUN_00403540(u_var6, DAT_009a73b0 as u32);
                                        let c = r8((DAT_009a73b0 as u32)
                                            .wrapping_mul(0x23)
                                            .wrapping_add(2)
                                            .wrapping_add(DAT_009a1fe8 as u32)
                                            as i32)
                                            as i8;
                                        if c == 0 || c == 1 || c == 2 {
                                            if (DAT_009a1ff8 as i32) >= 0x8001 {
                                                break 'calc 0;
                                            }
                                        } else if (DAT_009a1ff8 as i32) >= 0x10001 {
                                            break 'calc 0;
                                        }
                                    }
                                    let i5 = (DAT_009a1fe8 as u32)
                                        .wrapping_add(u_var6.wrapping_mul(0x23))
                                        as i32;
                                    let i10 = r32(i5.wrapping_add(0x1b));
                                    if i10 == -1 {
                                        let i10 = r32(i5.wrapping_add(0x1f));
                                        let i10 = r32(i10.wrapping_add(0xa9));
                                        let u11 = r32(i10.wrapping_add(0xcb)) as u32;
                                        let u12 = r32(i4.wrapping_add(0xc3)) as u32;
                                        let b = u11 < u12;
                                        break 'calc 1 - (b as i32);
                                    }
                                    1
                                };
                                if r == 0 {
                                    break 'body;
                                }
                                if (DAT_009a73da & 4) != 0 {
                                    if FUN_00403070(unaff_ebp) == 0 {
                                        break 'body;
                                    }
                                }
                                if (DAT_009a73da & 1) != 0 {
                                    if FUN_0045dba0(idx, 0xff) == 0 {
                                        break 'body;
                                    }
                                }
                                FUN_00403540(DAT_009a73b0 as u32, u_var6);
                                if DAT_009a1ff8 < (((u_stack_2 as u32) << 16) | (local_4 as u32)) {
                                    local_4 = DAT_009a1ff8 as u16;
                                    u_stack_2 = (DAT_009a1ff8 >> 16) as u16;
                                    u_stack_6 = idx;
                                }
                            }
                            let next = idx.wrapping_add(1);
                            unaff_ebp = (unaff_ebp & 0xffff_0000) | (next as u32);
                            if !((next as i32) < DAT_009a1fe0) {
                                break;
                            }
                        }
                    }
                }
                local_8 = local_8.wrapping_add(1);
                if !((local_8 as i32) < (DAT_009a2000 as i32)) {
                    break;
                }
            }
        }
        if u_stack_6 != 0xffff {
            let i15 = i32::from_ne_bytes(DAT_009a73b6);
            ((i15.wrapping_add(0x18)) as usize as *mut u16).write_unaligned(u_stack_6);
            let u13 = r16((u_stack_6 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(DAT_009a1fe8 as u32) as i32);
            ((i15.wrapping_add(0x1a)) as usize as *mut u16).write_unaligned(u13);
            ((i15.wrapping_add(0x1e)) as usize as *mut u8).write(1);
        }
    }
    u_stack_6
}
