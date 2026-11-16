// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006da530: u32;
    static mut DAT_006ccad0: u16;
    static mut DAT_006d4988: i32;
    static mut DAT_006db71c: *mut u32;
    static mut DAT_00732a38: u32;
    static mut DAT_0070a000: u32;
    static mut DAT_007091a0: *mut u32;
    static mut DAT_00703c88: u32;
    static mut DAT_0071eae8: u32;
    static mut DAT_006cca90: u16;
    static mut DAT_00732940: *mut u32;
    static mut DAT_0072f4e0: u32;
    static mut DAT_006ccaa8: u16;
    static mut DAT_006db798: *mut u32;
    static mut DAT_0071f2d8: [u32; 2];
    static mut DAT_0070ac30: [u32; 2];
    static mut DAT_006d49a0: i32;
    static mut DAT_006d4a60: [u32; 0];
    static mut DAT_007092c0: [u32; 0];
    static mut DAT_006db6c0: u16;
    static mut DAT_0071e6a0: [u32; 0];
    static mut DAT_00732fe4: i32;
    static mut DAT_0070acf0: [u32; 0];
    static mut DAT_007308b0: [u32; 0];
    static mut DAT_006daa00: [u32; 0];
    static mut DAT_006db57c: u32;
    static mut DAT_0072f444: u32;
    static mut DAT_0070ace0: u16;
    static mut DAT_00732e9c: u32;
    static mut DAT_006da4f0: u32;
    static mut DAT_00732a3c: u32;
    static mut DAT_006d4980: u32;
    static mut DAT_00703acc: u32;
    static mut DAT_0073088c: u32;
    static mut DAT_0071b704: u32;
    static mut DAT_00732904: u32;
    static mut DAT_006da4f4: u32;
    static mut DAT_00703ac8: u32;
    static mut DAT_00703c90: u32;
    static mut DAT_0071e53c: u32;
    static mut DAT_006db7a4: u32;
    static mut DAT_00703ac0: u32;
    static mut DAT_00703ca0: [u32; 0];
    static mut DAT_006da9e8: u32;
    static mut DAT_006db7c0: [u32; 0];
    static mut DAT_006d9d44: u32;
    static mut DAT_00703abc: u8;
    static mut DAT_00732e88: u32;
    static mut DAT_0071b740: [i8; 0];
    static mut DAT_0071e528: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042bbf0(param_1: *mut u32, param_2: u32) -> u32;
}

#[inline(always)]
fn rol1(x: u32) -> u32 {
    x.rotate_left(1)
}

/// Fold `n` dwords starting at `p`; returns accumulator and the advanced pointer.
#[inline(always)]
unsafe fn run(mut p: *const u8, n: u32, mut acc: u32) -> (u32, *const u8) {
    let mut i = 0;
    while i < n {
        // SAFETY: caller supplies a pointer into the game's own data.
        let v = unsafe { p.cast::<u32>().read_unaligned() };
        acc = rol1(acc ^ v);
        p = p.wrapping_add(4);
        i += 1;
    }
    (acc, p)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0042b3c0() -> u32 {
    // SAFETY: all globals and pointers are the game's own memory, read where
    // the original reads them.
    unsafe {
        let mut local_38: u32 = 0;
        if 0 < (DAT_006da530 as i32) {
            let mut pc = ((DAT_006ccad0 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(3)
                .wrapping_add(DAT_006d4988 as u32)) as *const i8;
            let mut p3 = DAT_006db71c as *const u8;
            let mut cnt = DAT_006da530;
            loop {
                if *pc != 0 {
                    let (l4, _) = run(p3, 0x13, 0);
                    local_38 = rol1(local_38 ^ l4);
                }
                p3 = p3.wrapping_add(0x13 * 4);
                pc = pc.wrapping_add(0x23);
                cnt = cnt.wrapping_sub(1);
                if cnt == 0 {
                    break;
                }
            }
        }
        let mut i1 = DAT_00732a38.wrapping_sub(DAT_0070a000) as i32;
        if 0 < i1 {
            let mut pc = (DAT_006d4988 as u32).wrapping_add(3) as *const i8;
            let mut pb = DAT_007091a0 as *const u8;
            let mut i7 = i1;
            loop {
                if *pc != 0 {
                    let (mut l18, p) = run(pb, 0x22, 0);
                    l18 ^= p.read() as u32;
                    local_38 ^= rol1(l18);
                    local_38 = rol1(local_38);
                }
                pb = pb.wrapping_add(0xaf);
                pc = pc.wrapping_add(0x23);
                i7 = i7.wrapping_sub(1);
                if i7 == 0 {
                    break;
                }
            }
            if 0 < i1 {
                let mut pc = (DAT_006d4988 as u32).wrapping_add(3) as *const i8;
                loop {
                    if *pc != 0 {
                        let (mut l30, p) = run(pc.wrapping_sub(3) as *const u8, 7, 0);
                        l30 ^= p.cast::<u16>().read_unaligned() as u32;
                        let u8v = rol1(l30) ^ (p.wrapping_add(2).read() as u32);
                        local_38 ^= rol1(u8v);
                        local_38 = rol1(local_38);
                    }
                    pc = pc.wrapping_add(0x23);
                    i1 = i1.wrapping_sub(1);
                    if i1 == 0 {
                        break;
                    }
                }
            }
        }
        if (DAT_00732a38 as i32) < (DAT_00703c88.wrapping_add(DAT_00732a38) as i32) {
            let mut pc = DAT_00732a38
                .wrapping_mul(0x23)
                .wrapping_add(3)
                .wrapping_add(DAT_006d4988 as u32) as *const i8;
            let mut cnt = DAT_00703c88;
            loop {
                if *pc != 0 {
                    let (mut l34, p) = run(pc.wrapping_sub(3) as *const u8, 7, 0);
                    l34 ^= p.cast::<u16>().read_unaligned() as u32;
                    let u4 = rol1(l34) ^ (p.wrapping_add(2).read() as u32);
                    local_38 ^= rol1(u4);
                    local_38 = rol1(local_38);
                }
                pc = pc.wrapping_add(0x23);
                cnt = cnt.wrapping_sub(1);
                if cnt == 0 {
                    break;
                }
            }
        }
        if 0 < (DAT_0071eae8 as i32) {
            let mut pc = (DAT_006cca90 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(3)
                .wrapping_add(DAT_006d4988 as u32) as *const i8;
            let mut pb = DAT_00732940 as *const u8;
            let mut cnt = DAT_0071eae8;
            loop {
                if *pc != 0 {
                    let (mut l1c, p) = run(pb, 0xf9, 0);
                    l1c ^= p.cast::<u16>().read_unaligned() as u32;
                    local_38 ^= rol1(l1c);
                    local_38 = rol1(local_38);
                }
                pb = pb.wrapping_add(0x436);
                pc = pc.wrapping_add(0x23);
                cnt = cnt.wrapping_sub(1);
                if cnt == 0 {
                    break;
                }
            }
        }
        if 0 < (DAT_0072f4e0 as i32) {
            let mut pc = (DAT_006ccaa8 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(3)
                .wrapping_add(DAT_006d4988 as u32) as *const i8;
            let mut p3 = DAT_006db798 as *const u8;
            let mut cnt = DAT_0072f4e0;
            loop {
                if *pc != 0 {
                    let (l8, _) = run(p3, 2, 0);
                    local_38 = rol1(local_38 ^ l8);
                }
                p3 = p3.wrapping_add(8);
                pc = pc.wrapping_add(0x23);
                cnt = cnt.wrapping_sub(1);
                if cnt == 0 {
                    break;
                }
            }
        }
        let (local_c, _) = run((&raw mut DAT_0071f2d8) as *const u8, 2, 0);
        let (mut local_10, _) = run((&raw mut DAT_0070ac30) as *const u8, 2, 0);
        local_10 = rol1(local_38 ^ local_c) ^ local_10;
        local_38 = rol1(local_10);
        if 0 < DAT_006d49a0 {
            let mut cnt = DAT_006d49a0;
            let mut pq = (&raw mut DAT_006d4a60) as *const u8;
            loop {
                let (mut l20, p) = run(pq, 0x49, 0);
                l20 ^= p.cast::<u16>().read_unaligned() as u32;
                local_38 ^= rol1(l20);
                local_38 = rol1(local_38);
                pq = pq.wrapping_add(0x126);
                cnt = cnt.wrapping_sub(1);
                if cnt == 0 {
                    break;
                }
            }
        }
        let (local_14, _) = run((&raw mut DAT_007092c0) as *const u8, 0x34c, 0);
        let mut u8_ = rol1(local_38 ^ local_14) ^ (DAT_006db6c0 as u32);
        let (mut l24, p) = run((&raw mut DAT_0071e6a0) as *const u8, 5, 0);
        l24 ^= p.cast::<u16>().read_unaligned() as u32;
        u8_ = rol1(u8_) ^ rol1(l24);
        u8_ = rol1(u8_) ^ (DAT_00732fe4 as u32);
        let (mut l28, p) = run((&raw mut DAT_006d49a0) as *const u8, 0x28, 0);
        l28 ^= p.cast::<u16>().read_unaligned() as u32;
        u8_ = rol1(u8_) ^ rol1(l28);
        local_38 = rol1(u8_);
        if 0 < DAT_006d49a0 {
            let mut cnt = DAT_006d49a0;
            let mut pq = (&raw mut DAT_0070acf0) as *const u8;
            loop {
                let (mut l2c, p) = run(pq, 0x158, 0);
                l2c ^= p.cast::<u16>().read_unaligned() as u32;
                local_38 ^= rol1(l2c);
                local_38 = rol1(local_38);
                pq = pq.wrapping_add(0x566);
                cnt = cnt.wrapping_sub(1);
                if cnt == 0 {
                    break;
                }
            }
        }
        let mut v = FUN_0042bbf0((&raw mut DAT_007308b0) as *mut u32, 0x1c80);
        local_38 = rol1(local_38 ^ v);
        let mut p3 = (&raw mut DAT_006daa00) as *mut u32;
        loop {
            v = FUN_0042bbf0(p3, 0x126);
            local_38 = rol1(local_38 ^ v);
            p3 = (p3 as usize).wrapping_add(0x126) as *mut u32;
            if !((p3 as usize) < ((&raw mut DAT_006db57c) as usize)) {
                break;
            }
        }
        let mut u = rol1(local_38 ^ DAT_0072f444) ^ (DAT_0070ace0 as u32);
        u = rol1(u) ^ DAT_0071eae8;
        u = rol1(u) ^ DAT_006da530;
        u = rol1(u) ^ DAT_0072f4e0;
        u = rol1(u) ^ DAT_00732e9c;
        u = rol1(u) ^ DAT_006da4f0;
        u = rol1(u) ^ DAT_0070a000;
        u = rol1(u) ^ DAT_00732a3c;
        u = rol1(u) ^ DAT_006d4980;
        u = rol1(u) ^ DAT_00703acc;
        u = rol1(u) ^ DAT_0073088c;
        u = rol1(u) ^ DAT_0071b704;
        u = rol1(u) ^ DAT_00732904;
        u = rol1(u) ^ DAT_006da4f4;
        u = rol1(u) ^ DAT_00703ac8;
        u = rol1(u) ^ DAT_00703c90;
        u = rol1(u) ^ DAT_0071e53c;
        u = rol1(u) ^ DAT_006db7a4;
        u = rol1(u) ^ DAT_00703ac0;
        u = rol1(u) ^ DAT_00732a38;
        let mut u4 = rol1(u) ^ DAT_00703c88;
        v = FUN_0042bbf0((&raw mut DAT_00703ca0) as *mut u32, 0x5500);
        u = rol1(u4) ^ v;
        u4 = rol1(u) ^ DAT_006da9e8;
        v = FUN_0042bbf0((&raw mut DAT_006db7c0) as *mut u32, 0x1ffff);
        u = rol1(u4) ^ v;
        u = rol1(u) ^ DAT_006d9d44;
        u = rol1(u) ^ (DAT_00703abc as u32);
        u = rol1(u) ^ DAT_00732e88;
        local_38 = rol1(u);
        let mut pc = (&raw mut DAT_0071b740) as *mut i8;
        loop {
            if *pc != 0 {
                v = FUN_0042bbf0(pc.wrapping_sub(0x10) as *mut u32, 0x5bd);
                local_38 = rol1(local_38 ^ v);
            }
            pc = pc.wrapping_add(0x5bd);
            if !((pc as usize) < ((&raw mut DAT_0071e528) as usize)) {
                break;
            }
        }
        local_38
    }
}
