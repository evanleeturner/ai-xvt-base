// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732e54: u32;
    static mut DAT_0052b0b4: [u8; 0];
    static mut DAT_0052b0b6: [u8; 0];
    static mut DAT_0052b0bc: [u8; 0];
    static mut DAT_0052b0be: [u8; 0];
    static mut DAT_0052b0c0: [u8; 0];
    static mut DAT_0052b0c3: [u8; 0];
    static mut DAT_00732e94: i32;
    static mut DAT_00732e90: i32;
    static mut DAT_00732e98: i32;
    static mut DAT_006d9d64: i32;
    static mut DAT_006d9d68: i32;
    static mut DAT_006d9d6c: i32;
    static mut DAT_006d9d50: u16;
    static mut DAT_006d4988: i32;
    static mut DAT_006d4980: i32;
    static mut DAT_006d499c: u32;
    static mut DAT_006d9d56: [u8; 4];
    static mut DAT_006db798: i32;
    static mut DAT_0070ab18: i32;
    static mut DAT_0070ab14: i32;
    static mut DAT_0070ac20: i32;
    static mut DAT_0071b704: u16;
    static mut DAT_0071b74a: u16;
    static mut DAT_00732904: i32;
    static mut DAT_00732a3c: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042b1e0(a: u32, b: u32) -> *mut u16;
    fn FUN_0042a230();
    fn FUN_00412810(a: i32, b: i32, c: i32) -> u32;
    fn FUN_0042b150(a: i32) -> i32;
    fn FUN_00444040(a: u32, b: u32) -> u8;
    fn FUN_00429a20(a: u32) -> u8;
    fn FUN_004490f0(a: u32) -> i32;
    fn FUN_00449790(a: u32, b: u32) -> u32;
    fn FUN_00422ab0(a: u32) -> u32;
    fn FUN_004479a0(a: u32, b: u32, c: u32, d: u32) -> u32;
    fn FUN_00478470() -> u8;
}

unsafe fn rb(a: u32) -> u8 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as *const u8).read() }
}
unsafe fn rw(a: u32) -> u16 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as *const u16).read_unaligned() }
}
unsafe fn rd(a: u32) -> u32 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as *const u32).read_unaligned() }
}
unsafe fn wb(a: u32, v: u8) {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as *mut u8).write(v) }
}
unsafe fn ww(a: u32, v: u16) {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as *mut u16).write_unaligned(v) }
}
fn ad<T>(p: *mut T) -> u32 {
    p as usize as u32
}

unsafe fn call_2a230(a: u32, b: u32, c: u32, d: i16, e: u16, f: i16) {
    // SAFETY: register/stack convention of the original; caller cleans 16 bytes.
    unsafe {
        asm!(
            "push {f}",
            "push {e}",
            "push {d}",
            "push {c}",
            "call {fun}",
            "add esp, 16",
            f = in(reg) f as i32 as u32,
            e = in(reg) e as u32,
            d = in(reg) d as i32 as u32,
            c = in(reg) c,
            fun = sym FUN_0042a230,
            in("ecx") a,
            in("edx") b,
            clobber_abi("C"),
        );
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0044d7a0() -> u32 {
    // SAFETY: all globals, callees and pointers are the game's own memory.
    unsafe {
        let mut local_28 = [0u8; 40];
        if rb(DAT_00732e54.wrapping_add(7)) == 3 {
            return DAT_00732e54 & 0xffff0000;
        }
        if rw(DAT_00732e54.wrapping_add(0x1df)) == 0 {
            return DAT_00732e54 & 0xffff0000;
        }
        if rw(DAT_00732e54.wrapping_add(0x1e1)) != 0 {
            return DAT_00732e54 & 0xffff0000;
        }
        let mut local_42: u16 = 0;
        let mut u_var10: u32 = DAT_00732e54;
        if rb(DAT_00732e54.wrapping_add(0x228)) != 0 {
            loop {
                let mut i_var5 = (local_42 as u32)
                    .wrapping_add((rb(DAT_00732e54.wrapping_add(4)) as u32).wrapping_mul(0xef));
                let mut u_stack_40: u16 = rb(ad(&raw mut DAT_0052b0b4).wrapping_add(i_var5)) as u16;
                if u_stack_40
                    < (rb(ad(&raw mut DAT_0052b0b6).wrapping_add(i_var5)) as u16).wrapping_add(1)
                {
                    loop {
                        u_var10 = u_stack_40 as u32;
                        let idx = rb(ad(&raw mut DAT_0052b0c3)
                            .wrapping_add(u_var10.wrapping_mul(8))
                            .wrapping_add(
                                (rb(DAT_00732e54.wrapping_add(4)) as u32).wrapping_mul(0xef),
                            )) as u32;
                        if rb(idx.wrapping_add(0x2ca).wrapping_add(DAT_00732e54)) == 0 {
                            i_var5 = DAT_00732e54.wrapping_add(u_var10.wrapping_mul(4));
                            let t = rb(i_var5.wrapping_add(0x344));
                            if (t == 0x90 || t == 0x95) && rb(i_var5.wrapping_add(0x346)) != 0 {
                                if rb(i_var5.wrapping_add(0x347)) == 0 {
                                    let puvar6 = FUN_0042b1e0(DAT_00732e54, u_var10);
                                    *puvar6 = 0xffff;
                                    DAT_00732e94 = DAT_006d9d64;
                                    DAT_00732e90 = DAT_006d9d68;
                                    DAT_00732e98 = DAT_006d9d6c;
                                    let i_var7 = (rb(DAT_00732e54.wrapping_add(4)) as u32)
                                        .wrapping_mul(0xef);
                                    i_var5 = i_var7.wrapping_add(u_var10.wrapping_mul(8));
                                    let tmp_us = rw(ad(&raw mut DAT_0052b0be).wrapping_add(i_var5));
                                    let tmp_s1 =
                                        rw(ad(&raw mut DAT_0052b0bc).wrapping_add(i_var5)) as i16;
                                    let tmp_s2 = rw(ad(&raw mut DAT_0052b0c0)
                                        .wrapping_add(u_var10.wrapping_mul(8))
                                        .wrapping_add(i_var7))
                                        as i16;
                                    call_2a230(
                                        DAT_006d9d50 as u32,
                                        tmp_us as u32,
                                        (DAT_006d9d50 as u32)
                                            .wrapping_mul(0x23)
                                            .wrapping_add(DAT_006d4988 as u32),
                                        tmp_s1,
                                        tmp_us,
                                        tmp_s2,
                                    );
                                    if rb((DAT_006d9d50 as u32)
                                        .wrapping_mul(0x23)
                                        .wrapping_add(3)
                                        .wrapping_add(DAT_006d4988 as u32))
                                        == b'5'
                                    {
                                        DAT_0070ab18 = DAT_0070ab18.wrapping_mul(2);
                                        DAT_0070ab14 = DAT_0070ab14.wrapping_mul(2);
                                        DAT_0070ac20 = DAT_0070ac20.wrapping_mul(2);
                                    }
                                    DAT_00732e94 = DAT_00732e94.wrapping_add(DAT_0070ab18);
                                    DAT_00732e90 = DAT_00732e90.wrapping_add(DAT_0070ab14);
                                    DAT_00732e98 = DAT_00732e98.wrapping_add(DAT_0070ac20);
                                    let mut local_38: u32 = 0x40000;
                                    let mut local_3e: u16 = 0xffff;
                                    let mut u_var14: u16 = DAT_0071b704;
                                    if (u_var14 as i32) < DAT_00732904 {
                                        loop {
                                            let i_var11 = (u_var14 as u32)
                                                .wrapping_mul(0x23)
                                                .wrapping_add(DAT_006d4988 as u32);
                                            if rb(i_var11.wrapping_add(3)) != 0 {
                                                let i_var1 = (DAT_006db798 as u32).wrapping_add(
                                                    (u_var14.wrapping_sub(DAT_0071b704) as u32)
                                                        .wrapping_mul(8),
                                                );
                                                if rb(i_var1.wrapping_add(1)) != 0
                                                    && rw(i_var1.wrapping_add(4)) == DAT_006d9d50
                                                {
                                                    let mut s_var15: i16 = 0;
                                                    let mut u_var12: u16 = DAT_0071b704;
                                                    while (u_var12 as i32) < DAT_00732904 {
                                                        if rb((u_var12 as u32)
                                                            .wrapping_mul(0x23)
                                                            .wrapping_add(3)
                                                            .wrapping_add(DAT_006d4988 as u32))
                                                            != 0
                                                            && u_var14 != u_var12
                                                        {
                                                            let i5 = (DAT_006db798 as u32)
                                                                .wrapping_add(
                                                                    (u_var12
                                                                        .wrapping_sub(DAT_0071b704)
                                                                        as u32)
                                                                        .wrapping_mul(8),
                                                                );
                                                            if 4 < rb(i5.wrapping_add(1))
                                                                && rw(i5.wrapping_add(4)) == u_var14
                                                            {
                                                                s_var15 = s_var15.wrapping_add(1);
                                                            }
                                                        }
                                                        u_var12 = u_var12.wrapping_add(1);
                                                    }
                                                    if s_var15 == 0 {
                                                        let ta = rd(i_var11.wrapping_add(4));
                                                        let tb = rd(i_var11.wrapping_add(8));
                                                        let tc = rd(i_var11.wrapping_add(0xc));
                                                        DAT_006d499c = FUN_00412810(
                                                            ta.wrapping_sub(DAT_00732e94 as u32)
                                                                as i32,
                                                            tb.wrapping_sub(DAT_00732e90 as u32)
                                                                as i32,
                                                            tc.wrapping_sub(DAT_00732e98 as u32)
                                                                as i32,
                                                        );
                                                        if 0x4000 < (DAT_006d499c as i32)
                                                            && DAT_006d499c < local_38
                                                        {
                                                            local_3e = u_var14;
                                                            local_38 = DAT_006d499c;
                                                        }
                                                    }
                                                }
                                            }
                                            u_var14 = u_var14.wrapping_add(1);
                                            if !((u_var14 as i32) < DAT_00732904) {
                                                break;
                                            }
                                        }
                                    }
                                    if local_3e == 0xffff {
                                        let mut u_var9: u32 = DAT_00732a3c as u32;
                                        local_38 = 0x40000;
                                        local_3e = 0xffff;
                                        if (u_var9 as i32) < DAT_006d4980 {
                                            loop {
                                                let mut i5 = (u_var9 & 0xffff).wrapping_mul(0x23);
                                                let u14 = u_var9 as u16;
                                                if rb((DAT_006d4988 as u32)
                                                    .wrapping_add(i5)
                                                    .wrapping_add(3))
                                                    != 0
                                                {
                                                    let ta = rd((DAT_006d4988 as u32)
                                                        .wrapping_add(i5)
                                                        .wrapping_add(0x1f));
                                                    let i7 = rd(ta.wrapping_add(0xa7));
                                                    let i11 = FUN_0042b150(i7 as i32) as u32;
                                                    let mut cond = 0;
                                                    if rw(i11.wrapping_add(0x18)) == DAT_006d9d50
                                                        && (rb(i11.wrapping_add(0x34)) == 0x0c
                                                            || rb(i11.wrapping_add(0x34)) == 0x17)
                                                    {
                                                        cond = 1;
                                                    } else if rw(i7.wrapping_add(0x6a)) == u14 {
                                                        cond = 1;
                                                    } else if rw(i7.wrapping_add(0x6e)) == u14 {
                                                        cond = 1;
                                                    } else {
                                                        let tb = rd((DAT_006d4988 as u32)
                                                            .wrapping_add(0x1b)
                                                            .wrapping_add(i5));
                                                        if tb as i32 != -1 {
                                                            let tc = rd((DAT_006d9d50 as u32)
                                                                .wrapping_mul(0x23)
                                                                .wrapping_add(0x1f)
                                                                .wrapping_add(DAT_006d4988 as u32));
                                                            if FUN_00444040(
                                                                u_var9,
                                                                rb(tc.wrapping_add(0x87)) as u32,
                                                            ) == 1
                                                            {
                                                                let tb2 = rd((DAT_006d4988 as u32)
                                                                    .wrapping_add(0x1b)
                                                                    .wrapping_add(i5));
                                                                let tu =
                                                                    rw(ad(&raw mut DAT_0071b74a)
                                                                        .wrapping_add(
                                                                            tb2.wrapping_mul(0x5bd),
                                                                        ));
                                                                if tu == DAT_006d9d50 {
                                                                    cond = 1;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    if cond != 0
                                                        && FUN_00429a20(u_var9 & 0xffff) != 0
                                                    {
                                                        let mut u_var13: u16 = 0;
                                                        let mut u_var12: u16 = DAT_0071b704;
                                                        while (u_var12 as i32) < DAT_00732904 {
                                                            if rb((u_var12 as u32)
                                                                .wrapping_mul(0x23)
                                                                .wrapping_add(3)
                                                                .wrapping_add(DAT_006d4988 as u32))
                                                                != 0
                                                                && u14 != u_var12
                                                            {
                                                                let i7b = (DAT_006db798 as u32)
                                                                    .wrapping_add(
                                                                        (u_var12.wrapping_sub(
                                                                            DAT_0071b704,
                                                                        )
                                                                            as u32)
                                                                            .wrapping_mul(8),
                                                                    );
                                                                if rb(i7b.wrapping_add(1)) != 0
                                                                    && rw(i7b.wrapping_add(4))
                                                                        == u14
                                                                {
                                                                    u_var13 =
                                                                        u_var13.wrapping_add(1);
                                                                }
                                                            }
                                                            u_var12 = u_var12.wrapping_add(1);
                                                        }
                                                        if u_var13 < 2 {
                                                            i5 = (DAT_006d4988 as u32)
                                                                .wrapping_add(i5);
                                                            let ta = rd(i5.wrapping_add(4));
                                                            let tb = rd(i5.wrapping_add(8));
                                                            let tc = rd(i5.wrapping_add(0xc));
                                                            DAT_006d499c = FUN_00412810(
                                                                ta.wrapping_sub(DAT_00732e94 as u32)
                                                                    as i32,
                                                                tb.wrapping_sub(DAT_00732e90 as u32)
                                                                    as i32,
                                                                tc.wrapping_sub(DAT_00732e98 as u32)
                                                                    as i32,
                                                            );
                                                            if DAT_006d499c < local_38 {
                                                                local_3e = u14;
                                                                local_38 = DAT_006d499c;
                                                            }
                                                        }
                                                    }
                                                }
                                                u_var9 = u14.wrapping_add(1) as u32;
                                                if !((u_var9 as i32) < DAT_006d4980) {
                                                    break;
                                                }
                                            }
                                        }
                                    } else {
                                        *puvar6 = local_3e;
                                    }
                                    if local_3e != 0xffff {
                                        *puvar6 = local_3e;
                                    }
                                    if *puvar6 != 0xffff {
                                        let mut u_var14: u16 = 0;
                                        let mut local_4c = rd(ad(&raw mut DAT_006d9d56));
                                        let u_var2 = rw(local_4c.wrapping_add(0x18));
                                        local_28[0] = 0;
                                        local_4c = rd(ad(&raw mut DAT_006d9d56));
                                        ww(local_4c.wrapping_add(0x18), *puvar6);
                                        local_4c = rd(ad(&raw mut DAT_006d9d56));
                                        if (rw(local_4c.wrapping_add(0x18)) as i32) < DAT_006d4980 {
                                            let mut u_var12: u16 = 0;
                                            local_4c = rd(ad(&raw mut DAT_006d9d56));
                                            let i5 = (rw(local_4c.wrapping_add(0x18)) as u32)
                                                .wrapping_mul(0x23);
                                            let i7 = FUN_004490f0(rb((DAT_006d4988 as u32)
                                                .wrapping_add(3)
                                                .wrapping_add(i5))
                                                as u32);
                                            if 0 < i7 {
                                                loop {
                                                    let u_var8 = FUN_00449790(
                                                        rb((DAT_006d4988 as u32)
                                                            .wrapping_add(3)
                                                            .wrapping_add(i5))
                                                            as u32,
                                                        u_var12 as u32,
                                                    );
                                                    if (u_var8 as i16) == 1 || (u_var8 as i16) == 3
                                                    {
                                                        *local_28
                                                            .as_mut_ptr()
                                                            .add(u_var14 as usize) = u_var12 as u8;
                                                        u_var14 = u_var14.wrapping_add(1);
                                                    }
                                                    u_var12 = u_var12.wrapping_add(1);
                                                    if !((u_var12 as i32) < i7) {
                                                        break;
                                                    }
                                                }
                                            }
                                            u_var14 = FUN_00422ab0(u_var14 as u32) as u16;
                                            local_4c = rd(ad(&raw mut DAT_006d9d56));
                                            ww(
                                                local_4c.wrapping_add(0x1c),
                                                local_28.as_ptr().add(u_var14 as usize).read()
                                                    as u16,
                                            );
                                        } else {
                                            local_4c = rd(ad(&raw mut DAT_006d9d56));
                                            ww(local_4c.wrapping_add(0x1c), 0);
                                        }
                                        let u_var9 = FUN_004479a0(
                                            DAT_006d9d50 as u32,
                                            u_var10,
                                            rb(DAT_00732e54
                                                .wrapping_add(0x344)
                                                .wrapping_add(u_var10.wrapping_mul(4)))
                                                as u32,
                                            0xffff,
                                        );
                                        if u_var9 != 0xffffffff {
                                            let b_var4 = FUN_00478470();
                                            wb(
                                                (DAT_006db798 as u32)
                                                    .wrapping_add(1)
                                                    .wrapping_add(u_var9.wrapping_mul(8)),
                                                (b_var4 & 3).wrapping_add(3),
                                            );
                                            wb(
                                                DAT_00732e54
                                                    .wrapping_add(0x347)
                                                    .wrapping_add(u_var10.wrapping_mul(4)),
                                                0x14,
                                            );
                                        }
                                        local_4c = rd(ad(&raw mut DAT_006d9d56));
                                        ww(local_4c.wrapping_add(0x18), u_var2);
                                    }
                                } else {
                                    wb(
                                        i_var5.wrapping_add(0x347),
                                        rb(i_var5.wrapping_add(0x347)).wrapping_sub(1),
                                    );
                                }
                            }
                        }
                        u_stack_40 = u_stack_40.wrapping_add(1);
                        let idx2 = (local_42 as u32).wrapping_add(
                            (rb(DAT_00732e54.wrapping_add(4)) as u32).wrapping_mul(0xef),
                        );
                        if !(u_stack_40
                            < (rb(ad(&raw mut DAT_0052b0b6).wrapping_add(idx2)) as u16)
                                .wrapping_add(1))
                        {
                            break;
                        }
                    }
                }
                local_42 = local_42.wrapping_add(1);
                u_var10 = local_42 as u32;
                if !(u_var10 < rb(DAT_00732e54.wrapping_add(0x228)) as u32) {
                    break;
                }
            }
        }
        u_var10 & 0xffff0000
    }
}
