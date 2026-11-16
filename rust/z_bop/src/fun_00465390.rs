// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a8c2c: i32;
    static mut DAT_00a60a04: *mut i8;
    static mut DAT_00a60a10: *mut u8;
    static mut DAT_00a60a1c: *mut u8;
    static mut DAT_00557360: u32;
    static mut DAT_00557364: i32;
    static mut DAT_00523420: i32;
    static mut DAT_00557368: [i8; 0];
    static mut DAT_005242e0: i32;
    static mut DAT_0055978c: i32;
    static mut DAT_00556f48: [u8; 0];
    static mut DAT_00557588: [u8; 0];
    static mut DAT_00556f08: [u32; 0];
    static mut DAT_00557788: [i32; 0];
    static mut DAT_0055778c: i32;
    static mut DAT_00557790: i32;
    static mut DAT_00557794: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac760() -> i32;
    fn FUN_00448da0();
    fn FUN_0046f660(param_1: i32) -> i32;
    fn FUN_0046f720(param_1: *mut u8) -> *mut i8;
    fn FUN_00448f90(param_1: i32, param_2: *mut u8, param_3: i32);
    fn FUN_0046d300(param_1: *mut u32, param_2: i32);
    fn FUN_00417960(param_1: *mut i8, param_2: *mut i8) -> i32;
    fn FUN_0046d350(param_1: i32, param_2: *mut u32, param_3: i32);
    fn FUN_004aa7f0() -> i32;
    fn FUN_004aa870() -> u32;
    fn FUN_00463d00();
    fn FUN_00463c50(param_1: i32);
    fn FUN_00448ed0();
    fn FUN_00417900(param_1: *mut u32, param_2: *mut u8, param_3: i32) -> i32;
    fn FUN_004178f0(param_1: i32) -> i32;
    fn FUN_004658c0(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00465390(param_1: i32, param_2: *mut u8, param_3: i32) -> u32 {
    // SAFETY: all globals and callees are the game's own; pointers are the
    // caller's buffers and the game's tables, accessed as the original does.
    unsafe {
        let mut local_100: [u32; 64] = [0; 64];
        let buf = local_100.as_mut_ptr() as *mut u8;
        let mut local_104: u32 = 1;
        let mut i_var2: i32 = FUN_004ac760();
        let mut local_118: i32 = 0;
        DAT_009a8c2c = DAT_009a8c2c.wrapping_add(i_var2);
        FUN_00448da0();

        // strlen + 1 of DAT_00a60a04, copy to buffer
        let src = DAT_00a60a04 as *const u8;
        let mut n: usize = 0;
        loop {
            let c = src.add(n).read();
            n += 1;
            if c == 0 {
                break;
            }
        }
        let mut k = 0usize;
        while k < n {
            buf.add(k).write(src.add(k).read());
            k += 1;
        }

        let pc = FUN_0046f720(FUN_0046f660(param_1) as u32 as *mut u8);
        if !pc.is_null() {
            let s = pc as *const u8;
            let mut m: usize = 0;
            loop {
                let c = s.add(m).read();
                m += 1;
                if c == 0 {
                    break;
                }
            }
            let mut e: usize = 0;
            while buf.add(e).read() != 0 {
                e += 1;
            }
            let mut k = 0usize;
            while k < m {
                buf.add(e + k).write(s.add(k).read());
                k += 1;
            }
        }
        FUN_00448f90(1, buf, 0x30);
        DAT_00557360 = 0x17;
        DAT_00557364 = param_1;
        FUN_0046d300(&raw mut DAT_00557360, 8);
        DAT_00557360 = 0x3d;
        DAT_00557364 = DAT_00523420;
        i_var2 = FUN_00417960((&raw mut DAT_00557368).cast::<i8>(), param_2 as *mut i8);
        FUN_0046d350(param_1, &raw mut DAT_00557360, i_var2.wrapping_add(8));
        let mut local_124: i32 = 10;
        DAT_005242e0 = 1;
        let mut u_var9: u32 = local_100[0];
        let tbl = (&raw mut DAT_00557788).cast::<i32>();
        let a48 = (&raw mut DAT_00556f48).cast::<u8>();
        let a88 = (&raw mut DAT_00557588).cast::<u8>();
        let a08 = (&raw mut DAT_00556f08).cast::<u32>();
        loop {
            i_var2 = 0;
            DAT_0055978c = 0;
            loop {
                if 0xeb < i_var2 {
                    break;
                }
                let i_var5 = FUN_004aa7f0();
                if i_var5 != 0 && FUN_004aa870() == 0x1b {
                    local_124 = 1;
                    i_var2 = 0xec;
                    break;
                }
                let i_var5 = DAT_009a8c2c;
                FUN_00463d00();
                let i_var6 = FUN_004ac760();
                i_var2 = i_var2
                    .wrapping_sub(i_var5)
                    .wrapping_add(DAT_009a8c2c)
                    .wrapping_add(i_var6);
                DAT_009a8c2c = i_var5;
                if DAT_0055978c != 0 {
                    break;
                }
            }
            local_118 = local_118.wrapping_add(i_var2);
            if 0xeb < local_118 {
                local_118 = 0;
                DAT_00557360 = 0x1d;
                FUN_0046d350(param_1, &raw mut DAT_00557360, 4);
                FUN_0046d300(&raw mut DAT_00557360, 4);
                u_var9 = (u_var9 == 0) as u32;
                let mut pb = DAT_00a60a10;
                if u_var9 != 0 {
                    pb = DAT_00a60a1c;
                }
                FUN_00448f90(3, pb, 0x30);
            }
            let cond = if DAT_0055978c != 0 {
                true
            } else {
                local_124 = local_124.wrapping_sub(1);
                local_124 == 0
            };
            if !cond {
                continue;
            }
            if DAT_0055978c == 0 {
                let r = FUN_0046f660(param_1);
                FUN_00463c50(r);
                FUN_00448ed0();
                FUN_004ac760();
                DAT_005242e0 = 0;
                return 0;
            }
            i_var2 = 0;
            let i_var5 = FUN_00417900(a48 as *mut u32, param_2, param_3);
            for k in 0..16usize {
                a08.add(k).write(0);
            }
            let mut i_stack_120: i32 = 0;
            let i_var6 = FUN_004178f0(param_3);
            tbl.write(0x3f);
            let mut u_var10: u32 = 0x1ec;
            DAT_0055778c = DAT_00523420;
            let mut pi_var7: *mut i32 = &raw mut DAT_00557794;
            DAT_00557790 = 0;
            let mut i_stack_108: i32 = 0;
            if 0 < i_var5 {
                let mut i_stack_110: i32 = 0;
                loop {
                    let x = a48
                        .byte_add(i_stack_110 as isize as usize)
                        .cast::<i32>()
                        .read_unaligned();
                    let y = a88
                        .byte_add(i_stack_110 as isize as usize)
                        .cast::<i32>()
                        .read_unaligned();
                    if x == y {
                        i_stack_120 = i_stack_120.wrapping_add(i_var6);
                    } else {
                        let mut i_stack_11c = i_var6;
                        if param_3 < i_var6.wrapping_add(i_stack_120) {
                            i_stack_11c = param_3.wrapping_sub(i_stack_120);
                            if i_stack_11c < 0 {
                                i_stack_11c = 0;
                            }
                        }
                        while i_stack_11c != 0 {
                            let mut u_stack_114 = (i_stack_11c as u32).wrapping_add(8);
                            if (u_var10 as i32) < (u_stack_114 as i32) {
                                u_stack_114 = u_var10;
                            }
                            pi_var7.write_unaligned(i_stack_120);
                            let u_var11 = u_stack_114.wrapping_sub(8);
                            pi_var7.byte_add(4).write_unaligned(u_var11 as i32);
                            let sp = param_2.offset(i_stack_120 as isize);
                            let dp = pi_var7.byte_add(8).cast::<u8>();
                            let mut k = 0u32;
                            while k != u_var11 {
                                dp.add(k as usize).write(sp.add(k as usize).read());
                                k = k.wrapping_add(1);
                            }
                            let len = pi_var7.byte_add(4).read_unaligned();
                            i_stack_11c = i_stack_11c.wrapping_sub(len);
                            i_stack_120 = i_stack_120.wrapping_add(len);
                            u_var10 = u_var10.wrapping_sub(u_stack_114);
                            pi_var7 = pi_var7.byte_add(u_stack_114 as usize);
                            if u_var10 < 0x20 {
                                pi_var7.write_unaligned(-1);
                                let mut i_var13 = i_var2.wrapping_add(1);
                                FUN_0046d350(
                                    param_1,
                                    tbl.wrapping_offset(
                                        (i_var2 as u32).wrapping_mul(0x80) as i32 as isize
                                    ) as *mut u32,
                                    0x1fcu32.wrapping_sub(u_var10) as i32,
                                );
                                if i_var13 == 0x10 {
                                    let r = FUN_004658c0(param_1, 0x10);
                                    if r == 0 {
                                        FUN_00448ed0();
                                        FUN_004ac760();
                                        DAT_005242e0 = 0;
                                        return 0;
                                    }
                                    DAT_00557360 = 0x1d;
                                    FUN_0046d300(&raw mut DAT_00557360, 4);
                                    u_var9 = (u_var9 == 0) as u32;
                                    let mut pb = DAT_00a60a10;
                                    if u_var9 != 0 {
                                        pb = DAT_00a60a1c;
                                    }
                                    i_var13 = 0;
                                    FUN_00448f90(3, pb, 0x30);
                                    for k in 0..16usize {
                                        a08.add(k).write(0);
                                    }
                                }
                                u_var10 = 0x1ec;
                                let off = (i_var13 as u32).wrapping_mul(0x80) as i32 as isize;
                                tbl.wrapping_offset(off).write(0x3f);
                                pi_var7 = (&raw mut DAT_00557794).wrapping_offset(off);
                                (&raw mut DAT_0055778c)
                                    .wrapping_offset(off)
                                    .write(DAT_00523420);
                                (&raw mut DAT_00557790).wrapping_offset(off).write(i_var13);
                                i_var2 = i_var13;
                            }
                        }
                    }
                    i_stack_110 = i_stack_110.wrapping_add(4);
                    i_stack_108 = i_stack_108.wrapping_add(1);
                    if !(i_stack_108 < i_var5) {
                        break;
                    }
                }
            }
            if u_var10 < 0x1f0 {
                pi_var7.write_unaligned(-1);
                FUN_0046d350(
                    param_1,
                    tbl.wrapping_offset((i_var2 as u32).wrapping_mul(0x80) as i32 as isize)
                        as *mut u32,
                    0x1fcu32.wrapping_sub(u_var10) as i32,
                );
                let r = FUN_004658c0(param_1, i_var2.wrapping_add(1));
                if r == 0 {
                    local_104 = 0;
                }
            }
            FUN_00448ed0();
            FUN_004ac760();
            DAT_005242e0 = 0;
            return local_104;
        }
    }
}
