// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4a02: u8;
    static mut DAT_00709310: [i32; 0];
    static mut DAT_007092e8: [i32; 0];
    static mut _DAT_00732a3c: u32;
    static mut DAT_006d4980: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071b730: [u8; 0];
    static mut DAT_0071b73c: [u8; 0];
    static mut DAT_0071b74a: [u8; 0];
    static mut DAT_0070ad48: [u8; 0];
    static mut DAT_0072f508: [u8; 0];
    static mut DAT_007092d9: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00442720(param_1: u16, param_2: u8);
}

unsafe fn rb(a: u32) -> u8 {
    // SAFETY: address into the game's own memory.
    unsafe { (a as *const u8).read() }
}
unsafe fn rw(a: u32) -> u16 {
    // SAFETY: address into the game's own memory.
    unsafe { (a as *const u16).read_unaligned() }
}
unsafe fn rd(a: u32) -> u32 {
    // SAFETY: address into the game's own memory.
    unsafe { (a as *const u32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00442410() {
    // SAFETY: all globals and addresses are the game's own memory, accessed
    // where the original does.
    unsafe {
        let mut local_4: i32 = 10;
        let mut local_c: i32 = 0;
        if DAT_006d4a02 == 2 {
            let mut i_var5: i32 = 0;
            local_c = 0;
            let a = (&raw mut DAT_007092e8).cast::<i32>();
            let b = (&raw mut DAT_00709310).cast::<i32>();
            for i in 0..8usize {
                let s = a.add(i).read().wrapping_add(b.add(i).read());
                if local_c < s {
                    local_c = s;
                    local_4 = i_var5;
                }
                i_var5 = i_var5.wrapping_add(1);
            }
        }
        if (_DAT_00732a3c as i32) < DAT_006d4980 {
            let mut local_8: u32 = _DAT_00732a3c.wrapping_mul(0x23);
            let mut u_var6: u32 = _DAT_00732a3c;
            loop {
                'cont: {
                    let mut i_var5: u32 = (DAT_006d4988 as u32).wrapping_add(local_8);
                    if rb(i_var5.wrapping_add(3)) == 0 {
                        break 'cont;
                    }
                    let i_var1: i32 = DAT_0071ecf4.wrapping_mul(0x5bd);
                    let u32v = (&raw mut DAT_0071b730)
                        .cast::<u8>()
                        .byte_offset(i_var1 as isize)
                        .cast::<u32>()
                        .read_unaligned();
                    if u32v == u_var6 {
                        break 'cont;
                    }
                    let mut b_var3: u8 = 0;
                    let p1 = rd(i_var5.wrapping_add(0x1f));
                    let mut u_var2: u32 = rb(p1.wrapping_add(0x87)) as u32;
                    let us: u16 = (&raw mut DAT_0071b73c)
                        .cast::<u8>()
                        .byte_offset(i_var1 as isize)
                        .cast::<u16>()
                        .read_unaligned();
                    let slot_owner = |u6: u32| -> u8 {
                        let idx = rb((u6 & 0xffff)
                            .wrapping_mul(0x23)
                            .wrapping_add(0x16)
                            .wrapping_add(DAT_006d4988 as u32))
                            as usize
                            * 0x566;
                        (&raw mut DAT_0070ad48).cast::<u8>().add(idx).read()
                    };
                    if DAT_006d4a02 == 2 && us as u32 == u_var2 && rb(i_var5.wrapping_add(2)) == 0 {
                        b_var3 = 0x2f;
                    } else if local_4 == 10
                        || (*(&raw mut DAT_00709310).cast::<i32>().add(u_var2 as usize))
                            .wrapping_add(
                                *(&raw mut DAT_007092e8).cast::<i32>().add(u_var2 as usize),
                            )
                            != local_c
                    {
                        let tmp = rd(i_var5.wrapping_add(0x1b)) as i32;
                        if tmp != -1 && DAT_0071ecf4 != tmp {
                            if DAT_006d4a02 == 2 {
                                let b = slot_owner(u_var6) as u32;
                                let b_var7: i32 = if b == us as u32 {
                                    0
                                } else {
                                    ((&raw mut DAT_0072f508)
                                        .cast::<u8>()
                                        .add((b as usize) + (us as usize) * 0x1e5)
                                        .read()
                                        == 0) as i32
                                };
                                let mask: u32 = if b_var7 == 0 { 0xa0 } else { 0 };
                                b_var3 = (mask.wrapping_add(0x33)) as u8;
                            } else {
                                b_var3 = match rb(p1.wrapping_add(0x86)) {
                                    0 => 0x3f,
                                    1 | 4 => 0x37,
                                    2 => 0x33,
                                    _ => 0x3b,
                                };
                            }
                        }
                    } else {
                        b_var3 = 0xd4;
                    }
                    if b_var3 != 0 {
                        let tmp = rd(p1.wrapping_add(0xa7));
                        u_var2 = us as u32;
                        if DAT_007092d9 == 0
                            && rb(tmp.wrapping_add(0x1f0).wrapping_add(u_var2)) == 0
                        {
                            let b = slot_owner(u_var6) as u32;
                            let b_var7: bool = if b == u_var2 {
                                false
                            } else {
                                (&raw mut DAT_0072f508)
                                    .cast::<u8>()
                                    .add((b as usize) + (u_var2 as usize) * 0x1e5)
                                    .read()
                                    == 0
                            };
                            if b_var7 {
                                break 'cont;
                            }
                        }
                        let mut b_var7 = false;
                        if !((u_var6 as u16 == 0xffff) || DAT_006d4980 <= (u_var6 & 0xffff) as i32)
                        {
                            i_var5 = (u_var6 & 0xffff)
                                .wrapping_mul(0x23)
                                .wrapping_add(DAT_006d4988 as u32);
                            if rd(i_var5.wrapping_add(0x1b)) as i32 != -1 {
                                let p2 = rd(i_var5.wrapping_add(0x1f));
                                let t = rd(p2.wrapping_add(0xa7));
                                if t != 0
                                    && (rb(t.wrapping_add(0x1e0)) & 1) != 0
                                    && rb(t.wrapping_add(0x237)) != 0
                                    && rb(t.wrapping_add(0x233)) == 3
                                    && rw(t.wrapping_add(0x238)) != 0
                                {
                                    b_var7 = true;
                                }
                            }
                        }
                        let us2: u16 = (&raw mut DAT_0071b74a)
                            .cast::<u8>()
                            .byte_offset(i_var1 as isize)
                            .cast::<u16>()
                            .read_unaligned();
                        if !b_var7 && us2 as u32 != u_var6 {
                            FUN_00442720(u_var6 as u16, b_var3);
                        }
                    }
                }
                u_var6 = u_var6.wrapping_add(1);
                local_8 = local_8.wrapping_add(0x23);
                if !((u_var6 as i32) < DAT_006d4980) {
                    break;
                }
            }
        }
    }
}
