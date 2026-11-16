// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758830: i32;
    static mut DAT_0075964b: [u8; 4];
    static mut DAT_00759657: [u8; 4];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dd570(param_1: *mut i32, param_2: i32, param_3: i32, param_4: u16);
    fn FUN_004dd000(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004dd030(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004dd0b0(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dd3e0(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: u16,
    param_5: i32,
) {
    // SAFETY: the callees and globals are the game's own; param_1 points at
    // the caller's rectangle and the destination surface memory is written
    // exactly where the original does.
    unsafe {
        if param_5 == 0 {
            FUN_004dd570(param_1, param_2, param_3, param_4);
            return;
        }
        if !(param_1.read() < param_1.add(2).read()
            && param_1.add(1).read() < param_1.add(3).read())
        {
            return;
        }
        let mut l: [i32; 4] = [0; 4];
        let lp = l.as_mut_ptr();
        FUN_004dd000(lp, param_1);
        FUN_004dd030(lp, param_2, param_3);
        let l0 = core::ptr::read_volatile(lp);
        let l1 = core::ptr::read_volatile(lp.add(1));
        let l2 = core::ptr::read_volatile(lp.add(2));
        let l3 = core::ptr::read_volatile(lp.add(3));
        if !(l0 < 0x281 && (-1 < l2 && l1 < 0x1e1) && -1 < l3) {
            return;
        }
        FUN_004dd0b0(lp);
        let i_var1: u32 = u32::from_ne_bytes(core::ptr::read(&raw const DAT_0075964b));
        let mut c1 = core::ptr::read_volatile(lp.add(3)).wrapping_add(1);
        let c0 = core::ptr::read_volatile(lp);
        let c1y = core::ptr::read_volatile(lp.add(1));
        let c2 = core::ptr::read_volatile(lp.add(2));
        let u_var4: u32 = (c2 as u32).wrapping_sub(c0 as u32).wrapping_add(1);
        let i_var6: i32 = i32::from_ne_bytes(core::ptr::read(&raw const DAT_00759657));
        let base = DAT_00758830 as u32;
        if i_var6 == 8 {
            let mut p8: u32 = i_var1
                .wrapping_mul(c1y as u32)
                .wrapping_add(c0 as u32)
                .wrapping_add(base);
            if c1y < c1 {
                c1 = (c1 as u32).wrapping_sub(c1y as u32) as i32;
                loop {
                    let u_var3 = param_4 as u8;
                    let mut p = p8;
                    let mut n = u_var4 >> 2;
                    while n != 0 {
                        (p as *mut u32).write_unaligned((u_var3 as u32).wrapping_mul(0x01010101));
                        p = p.wrapping_add(4);
                        n -= 1;
                    }
                    let mut n = u_var4 & 3;
                    while n != 0 {
                        (p as *mut u8).write(u_var3);
                        p = p.wrapping_add(1);
                        n -= 1;
                    }
                    p8 = p8.wrapping_add(i_var1);
                    c1 = c1.wrapping_sub(1);
                    if c1 == 0 {
                        break;
                    }
                }
            }
        } else {
            if i_var6 != 0x10 {
                return;
            }
            let mut p8: u32 = i_var1
                .wrapping_mul(c1y as u32)
                .wrapping_add((c0 as u32).wrapping_mul(2))
                .wrapping_add(base);
            if c1y < c1 {
                c1 = (c1 as u32).wrapping_sub(c1y as u32) as i32;
                loop {
                    let mut n = u_var4;
                    let mut p = p8;
                    if 0 < (u_var4 as i32) {
                        while n != 0 {
                            (p as *mut u16).write_unaligned(param_4);
                            p = p.wrapping_add(2);
                            n -= 1;
                        }
                    }
                    p8 = p8.wrapping_add(i_var1);
                    c1 = c1.wrapping_sub(1);
                    if c1 == 0 {
                        break;
                    }
                }
            }
        }
    }
}
