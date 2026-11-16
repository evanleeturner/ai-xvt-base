// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052a544: u16;
    static mut DAT_0052a548: i32;
    static mut DAT_0052a53c: u16;
    static mut DAT_0052a540: u32;
    static mut DAT_00602678: i32;
    static mut DAT_006026a0: i32;
    static mut _DAT_006026c0: i32;
    static mut _DAT_006026a8: i32;
    static mut DAT_00602690: i32;
    static mut DAT_006026bc: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f4b0(param_1: u32) -> u32;
    fn FUN_0048f3c0(param_1: u32, param_2: i32) -> u32;
    fn FUN_0047f010(param_1: i32) -> u32;
    fn FUN_0048f4f0(param_1: u32) -> *mut core::ffi::c_void;
    fn FUN_00486000(param_1: *mut i32, param_2: i32);
    fn FUN_0048f510();
    fn FUN_00488bc0(
        param_1: *mut i32,
        param_2: *mut i32,
        param_3: i32,
        param_4: i32,
        param_5: *mut u32,
    ) -> i32;
}

unsafe fn rd(a: u32) -> i32 {
    // SAFETY: the address is a game memory block the original reads here.
    unsafe { (a as usize as *const i32).read_unaligned() }
}

unsafe fn wr(a: u32, v: i32) {
    // SAFETY: the address is a game memory block the original writes here.
    unsafe { (a as usize as *mut i32).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00488830(param_1: u32) -> i32 {
    // SAFETY: the globals and callees are the game's own; the pointers come
    // from the game's allocator and are read and written as the original does.
    unsafe {
        let mut u_var2: u32 = param_1;
        if DAT_0052a548 < param_1 as i32 && DAT_0052a544 != 0 {
            u_var2 = FUN_0048f4b0(DAT_0052a544 as u32);
            DAT_0052a544 = 0;
            DAT_0052a548 = 0;
        }
        if DAT_0052a544 == 0 {
            DAT_0052a544 = FUN_0048f3c0(param_1, 0) as u16;
            if DAT_0052a544 == 0 {
                FUN_0047f010(0);
            }
            DAT_0052a548 = param_1 as i32;
            u_var2 = param_1;
        }
        let p3 = FUN_0048f4f0(((u_var2 >> 16) << 16) | DAT_0052a544 as u32) as usize as u32;
        let mut p4 = FUN_0048f4f0(((p3 >> 16) << 16) | DAT_0052a53c as u32) as usize as u32;
        let mut p7 = p3;
        u_var2 = param_1 >> 2;
        while u_var2 != 0 {
            wr(p7, rd(p4));
            p4 = p4.wrapping_add(4);
            p7 = p7.wrapping_add(4);
            u_var2 = u_var2.wrapping_sub(1);
        }
        u_var2 = param_1 & 3;
        while u_var2 != 0 {
            (p7 as usize as *mut u8).write(*(p4 as usize as *const u8));
            p4 = p4.wrapping_add(1);
            p7 = p7.wrapping_add(1);
            u_var2 = u_var2.wrapping_sub(1);
        }
        p4 = rd(p3) as u32;
        if p3 != p4 {
            let mut i_var12: i32 = 0;
            let i_var11: i32 = p3.wrapping_sub(p4) as i32;
            wr(p3, p4.wrapping_add(i_var11 as u32) as i32);
            let t10 = rd(p3.wrapping_add(10));
            if t10 != 0 {
                wr(p3.wrapping_add(10), t10.wrapping_add(i_var11));
                let mut t6 = rd(p3.wrapping_add(6));
                if 0 < t6 {
                    let mut i_var10: i32 = 0;
                    loop {
                        let t10 = rd(p3.wrapping_add(10));
                        let p9 = (t10 as u32).wrapping_add(i_var10 as u32);
                        let i_var1 = rd(p9);
                        if i_var1 != 0 {
                            wr(p9, i_var1.wrapping_add(i_var11));
                            let t10 = rd(p3.wrapping_add(10));
                            let tv = rd((t10 as u32).wrapping_add(i_var10 as u32));
                            FUN_00486000(tv as u32 as usize as *mut i32, i_var11);
                        }
                        i_var10 = i_var10.wrapping_add(4);
                        i_var12 = i_var12.wrapping_add(1);
                        t6 = rd(p3.wrapping_add(6));
                        if !(i_var12 < t6) {
                            break;
                        }
                    }
                }
            }
        }
        u_var2 = param_1.wrapping_mul(2);
        FUN_0048f510();
        let mut u_var5: u32 = DAT_0052a540;
        if (DAT_0052a540 as i32) < u_var2 as i32 && DAT_0052a53c != 0 {
            u_var5 = FUN_0048f4b0(DAT_0052a53c as u32);
            DAT_0052a53c = 0;
            DAT_0052a540 = 0;
        }
        let mut u_var8: u16 = (u_var5 >> 16) as u16;
        if DAT_0052a53c == 0 {
            let mut u_var6 = FUN_0048f3c0(u_var2, 0);
            DAT_0052a53c = u_var6 as u16;
            if DAT_0052a53c == 0 {
                u_var6 = FUN_0047f010(0);
            }
            u_var8 = (u_var6 >> 16) as u16;
            DAT_0052a540 = u_var2;
        }
        let p7 = FUN_0048f4f0(((u_var8 as u32) << 16) | DAT_0052a53c as u32) as usize as u32;
        wr(p7, rd(p3));
        wr(p7.wrapping_add(4), rd(p3.wrapping_add(4)));
        wr(p7.wrapping_add(8), rd(p3.wrapping_add(8)));
        let ts = (p3.wrapping_add(12) as usize as *const u16).read_unaligned();
        (p7.wrapping_add(12) as usize as *mut u16).write_unaligned(ts);
        wr(p7, p7 as i32);
        wr(p7.wrapping_add(10), p7.wrapping_add(0xe) as i32);
        let mut t6 = rd(p3.wrapping_add(6));
        let mut p9: u32 = p7
            .wrapping_add(0xe)
            .wrapping_add((t6 as u32).wrapping_mul(4));
        t6 = rd(p3.wrapping_add(6));
        let mut local_d4: i32 = (t6 as u32).wrapping_mul(4).wrapping_add(0xe) as i32;
        let mut local_d0: [u32; 52] = [0; 52];
        let mut i_var11: i32 = 0;
        DAT_00602678 = 0;
        DAT_006026a0 = 0;
        _DAT_006026c0 = 0;
        _DAT_006026a8 = 0;
        DAT_00602690 = 0;
        DAT_006026bc = 0;
        wr(p7.wrapping_add(6), 0);
        let mut local_d8: i32 = 0;
        t6 = rd(p3.wrapping_add(6));
        if 0 < t6 {
            loop {
                i_var11 = i_var11.wrapping_add(4);
                let t6b = rd(p7.wrapping_add(6));
                wr(p7.wrapping_add(6), t6b.wrapping_add(1));
                let t10 = rd(p7.wrapping_add(10));
                wr(
                    (t10 as u32).wrapping_sub(4).wrapping_add(i_var11 as u32),
                    p9 as i32,
                );
                let t10 = rd(p3.wrapping_add(10));
                let tv = rd((t10 as u32).wrapping_sub(4).wrapping_add(i_var11 as u32));
                let i_var12 = FUN_00488bc0(
                    p9 as usize as *mut i32,
                    tv as u32 as usize as *mut i32,
                    p3 as i32,
                    p7 as i32,
                    local_d0.as_mut_ptr(),
                );
                local_d4 = local_d4.wrapping_add(i_var12);
                p9 = p9.wrapping_add(i_var12 as u32);
                local_d8 = local_d8.wrapping_add(1);
                t6 = rd(p3.wrapping_add(6));
                if !(local_d8 < t6) {
                    break;
                }
            }
        }
        local_d4
    }
}
