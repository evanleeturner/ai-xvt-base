// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00650904: [i16; 0];
    static mut DAT_00650914: [i16; 0];
    static mut DAT_00650924: [i16; 0];
    static mut DAT_00743d02: [i16; 0];
    static mut DAT_00743d2e: [i16; 0];
    static mut DAT_00743d86: [i16; 0];
    static mut DAT_00650936: [i16; 0];
    static mut DAT_00650946: [i16; 0];
    static mut DAT_00650956: [i16; 0];
    static mut DAT_00650966: [i16; 0];
    static mut DAT_00650976: [i16; 0];
    static mut DAT_00650986: [i16; 0];
    static mut DAT_006507a2: [u8; 0];
    static mut DAT_00650434: i32;
    static mut DAT_0075766c: i16;
    static mut DAT_0081bfb6: [u8; 4];
    static mut DAT_007438d7: [u8; 0];
    static mut DAT_007438d2: [u8; 0];
    static mut DAT_007438d8: [u8; 0];
    static mut DAT_007438e4: [u8; 0];
    static mut DAT_00533800: [i32; 0];
    static mut DAT_005339a8: [i8; 0];
    static mut DAT_005339ac: [i8; 0];
    static mut DAT_005339b0: [i8; 0];
    static mut DAT_005339b4: [i8; 0];
    static mut DAT_0081b740: [i8; 0];
    static mut DAT_00533e94: [i8; 0];
    static mut DAT_00533e90: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dd000(param_1: *mut i32, param_2: *mut u32);
    fn FUN_004f33c0(
        param_1: *mut i32,
        param_2: i16,
        param_3: i16,
        param_4: *mut i16,
        param_5: *mut i16,
    );
    fn FUN_004f4410(param_1: *mut u32, param_2: u32, param_3: i16, param_4: i16);
    fn FUN_004f4200(
        param_1: *mut i8,
        param_2: i32,
        param_3: i16,
        param_4: i16,
        param_5: i16,
        param_6: i16,
    );
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004d0920(param_1: *mut i8, param_2: *mut i32, param_3: i32, param_4: i32);
    fn FUN_004da7c0(param_1: i32, param_2: *mut i8, param_3: i32, param_4: i32, param_5: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f3b60(param_1: *mut u32, param_2: u32) {
    let mut u_stack_46: u32 = 0;
    let mut local_42: i16 = 0;
    let mut local_40: i16 = 0;
    let mut local_38: [i32; 4] = [0; 4];
    let mut ac_stack_28: [i8; 40] = [0; 40];
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are used at the offsets the original uses.
    unsafe {
        let mut s_var11: i16 = 0;
        FUN_004dd000(local_38.as_mut_ptr(), param_1);
        loop {
            if (&raw mut DAT_00650904)
                .cast::<i16>()
                .offset(s_var11 as isize)
                .read()
                != 0
            {
                let s_var3 = (&raw mut DAT_00650914)
                    .cast::<i16>()
                    .offset(s_var11 as isize)
                    .read();
                let idx = (s_var3 as i32).wrapping_mul(0x2b1) as isize;
                FUN_004f33c0(
                    local_38.as_mut_ptr(),
                    (&raw mut DAT_00743d02).cast::<i16>().offset(idx).read(),
                    (&raw mut DAT_00743d2e).cast::<i16>().offset(idx).read(),
                    &raw mut local_40,
                    &raw mut local_42,
                );
                FUN_004f4410(
                    param_1,
                    param_2,
                    s_var3,
                    (&raw mut DAT_00650924)
                        .cast::<i16>()
                        .offset(s_var11 as isize)
                        .read(),
                );
            }
            s_var11 = s_var11.wrapping_add(1);
            if s_var11 >= 8 {
                break;
            }
        }

        let mut s_stack_3e: i16 = 0;
        loop {
            let i_var5 = s_stack_3e as i32;
            let i5 = i_var5 as isize;
            if (&raw mut DAT_00650936).cast::<i16>().offset(i5).read() != 0 {
                let s11 = (&raw mut DAT_00650946).cast::<i16>().offset(i5).read();
                FUN_004f33c0(
                    local_38.as_mut_ptr(),
                    (&raw mut DAT_00650956).cast::<i16>().offset(i5).read(),
                    (&raw mut DAT_00650966).cast::<i16>().offset(i5).read(),
                    &raw mut local_40,
                    &raw mut local_42,
                );
                let src: *const i8 = (&raw mut DAT_006507a2)
                    .cast::<u8>()
                    .byte_offset((s11 as i32).wrapping_mul(4) as isize)
                    .cast::<*const i8>()
                    .read_unaligned();
                let mut n: u32 = 0;
                while src.add(n as usize).read() != 0 {
                    n = n.wrapping_add(1);
                }
                n = n.wrapping_add(1);
                let dst = ac_stack_28.as_mut_ptr();
                let mut k: u32 = 0;
                while k < n {
                    dst.add(k as usize).write(src.add(k as usize).read());
                    k = k.wrapping_add(1);
                }

                let mut sv: i16 = 0;
                let mut c = ac_stack_28[0];
                while c != 0 {
                    let p = dst.offset(sv as isize);
                    if p.read() == 0x5b {
                        p.write(2);
                    }
                    if p.read() == 0x5d {
                        p.write(1);
                    }
                    sv = sv.wrapping_add(1);
                    c = dst.offset(sv as isize).read();
                }
                FUN_004f4200(
                    dst,
                    1,
                    local_40,
                    local_42,
                    (&raw mut DAT_00650976).cast::<i16>().offset(i5).read(),
                    (&raw mut DAT_00650986).cast::<i16>().offset(i5).read(),
                );
            }
            s_stack_3e = s_stack_3e.wrapping_add(1);
            if 7 < s_stack_3e {
                let mut i_stack_3c: i32 = 1;
                s_stack_3e = 0;
                if 0 < DAT_0075766c {
                    loop {
                        let i_var8 = s_stack_3e as i32;
                        let i_var1 = i_var8.wrapping_mul(0x562);
                        let i1 = i_var1 as isize;
                        let i_var5b = i_var8.wrapping_mul(0x2b1).wrapping_add(DAT_00650434);
                        let ib = i_var5b as isize;
                        let u_var4 = (&raw mut DAT_00743d2e).cast::<i16>().offset(ib).read() as u16;
                        u_stack_46 = (u_stack_46 & 0xffff0000) | (u_var4 as u32);
                        if (&raw mut DAT_00743d86).cast::<i16>().offset(ib).read() != 0 {
                            match (&raw mut DAT_007438d7).cast::<u8>().offset(i1).read() {
                                0 => u_stack_46 = u_var4 as u32,
                                1 => u_stack_46 = (1u32 << 16) | (u_var4 as u32),
                                2 => u_stack_46 = (2u32 << 16) | (u_var4 as u32),
                                3 => u_stack_46 = (3u32 << 16) | (u_var4 as u32),
                                4 => u_stack_46 = (1u32 << 16) | (u_var4 as u32),
                                5 => u_stack_46 = (4u32 << 16) | (u_var4 as u32),
                                _ => {}
                            }
                            let tbl = (&raw mut DAT_007438d2).cast::<u8>().offset(i1).read() as i16;
                            let i_var9 = ((&raw mut DAT_00533800)
                                .cast::<i32>()
                                .offset(tbl as isize)
                                .read() as u32)
                                .wrapping_mul(0x10) as i32;
                            let o9 = i_var9 as isize;
                            let a8 = (&raw mut DAT_005339a8).cast::<u8>().byte_offset(o9);
                            let ac = (&raw mut DAT_005339ac).cast::<u8>().byte_offset(o9);
                            let b0 = (&raw mut DAT_005339b0).cast::<u8>().byte_offset(o9);
                            let b4 = (&raw mut DAT_005339b4).cast::<u8>().byte_offset(o9);
                            let i_var13 = b0
                                .cast::<i32>()
                                .read_unaligned()
                                .wrapping_sub(a8.cast::<i32>().read_unaligned())
                                .wrapping_add(1);
                            let i_var8b = b4
                                .cast::<i32>()
                                .read_unaligned()
                                .wrapping_sub(ac.cast::<i32>().read_unaligned())
                                .wrapping_add(1);
                            FUN_004f33c0(
                                local_38.as_mut_ptr(),
                                (&raw mut DAT_00743d02).cast::<i16>().offset(ib).read(),
                                u_stack_46 as i16,
                                &raw mut local_40,
                                &raw mut local_42,
                            );
                            local_40 = local_40.wrapping_sub((i_var13 >> 1) as i16);
                            local_42 = local_42.wrapping_sub((i_var8b >> 1) as i16);
                            let buf = (&raw mut DAT_0081b740).cast::<i8>();
                            sprintf(
                                buf,
                                (&raw mut DAT_00533e94).cast::<i8>(),
                                (u_stack_46 >> 16) as i16 as i32,
                            );
                            FUN_004d0920(buf, a8.cast::<i32>(), local_40 as i32, local_42 as i32);
                            let local_81bfb6: i32 =
                                (&raw mut DAT_0081bfb6).cast::<i32>().read_unaligned();
                            if (&raw mut DAT_007438e4).cast::<u8>().offset(i1).read() != 0
                                && ((&raw mut DAT_007438d8).cast::<u8>().offset(i1).read() as i32)
                                    == local_81bfb6
                            {
                                local_40 = local_40.wrapping_add(i_var13 as i16);
                                local_42 = local_42.wrapping_add(i_var8b as i16);
                                sprintf(buf, (&raw mut DAT_00533e90).cast::<i8>(), i_stack_3c);
                                FUN_004da7c0(10, buf, local_40 as i32, local_42 as i32, 0xffff);
                                i_stack_3c = i_stack_3c.wrapping_add(1);
                            }
                        }
                        s_stack_3e = s_stack_3e.wrapping_add(1);
                        if s_stack_3e >= DAT_0075766c {
                            break;
                        }
                    }
                }
                return;
            }
        }
    }
}
