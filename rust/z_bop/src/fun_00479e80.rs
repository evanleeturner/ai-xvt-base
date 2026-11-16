// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a07bd8: *mut c_void;
    static mut DAT_0051ae00: [i8; 0];
    static mut DAT_0051a854: [i8; 0];
    static mut DAT_005272cc: [i8; 0];
    static mut DAT_005271e0: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __strcmpi(a: *const i8, b: *const i8) -> i32;
    fn FUN_0049e720(p1: *mut i8, p2: *mut i8, p3: i32, p4: i32);
    fn FUN_004720d0(p1: *mut u8, p2: *mut u8, p3: i32, p4: i32);
    fn _fread(buf: *mut c_void, size: u32, count: u32, f: *mut c_void) -> u32;
    fn _fclose(f: *mut c_void) -> i32;
    fn FUN_0040e940(p1: *mut u8, p2: i32, p3: i32, p4: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00479e80(param_1: *mut u32, param_2: *mut i8) -> *mut u8 {
    // SAFETY: the callees and globals are the game's own; param_1 points at the
    // caller's image buffer and param_2 at a NUL-terminated path, accessed
    // exactly where the original does.
    unsafe {
        let mut ac_stack_400: [i8; 256] = [0; 256];
        let mut ab_stack_300: [u8; 768] = [0; 768];
        let buf = ac_stack_400.as_mut_ptr();

        // copy the string including its terminator
        let mut src = param_2 as *const i8;
        let mut dst = buf;
        let mut u_var6: u32 = 0;
        loop {
            let c = src.read();
            dst.write(c);
            src = src.wrapping_add(1);
            dst = dst.wrapping_add(1);
            u_var6 = u_var6.wrapping_add(1);
            if c == 0 {
                break;
            }
        }
        let pc_var11 = buf.wrapping_offset(u_var6.wrapping_sub(4) as i32 as isize);
        let i_var5 = __strcmpi(pc_var11, (&raw mut DAT_0051ae00).cast::<i8>());
        if i_var5 == 0 {
            pc_var11.write(b't' as i8);
            pc_var11.wrapping_add(1).write(b'e' as i8);
            pc_var11.wrapping_add(2).write(b'x' as i8);
            FUN_0049e720(buf, (&raw mut DAT_0051a854).cast::<i8>(), 0, 0);
            pc_var11.write(b'r' as i8);
            pc_var11.wrapping_add(1).write(b'g' as i8);
            pc_var11.wrapping_add(2).write(b'b' as i8);
            if DAT_00a07bd8.is_null() {
                FUN_0049e720(buf, (&raw mut DAT_0051a854).cast::<i8>(), 0, 0);
                let p_f = DAT_00a07bd8;
                if p_f.is_null() {
                    param_1.add(4).write(8);
                    param_1.add(5).write(8);
                    param_1.add(1).write(0x10);
                    param_1.write(0x100);
                    FUN_004720d0(
                        param_1.add(6).cast::<u8>(),
                        (&raw mut DAT_005271e0).cast::<u8>(),
                        8,
                        8,
                    );
                    return 0x3058usize as *mut u8;
                }
                _fread(param_1.cast::<c_void>(), 0x200, 1, DAT_00a07bd8);
                let pb = param_1.cast::<u8>();
                let i_var5: i32 = (pb.add(6).read() as i32)
                    .wrapping_mul(0x100)
                    .wrapping_add(pb.add(7).read() as i32);
                let i_var8: i32 = (pb.add(9).read() as i32)
                    .wrapping_add((pb.add(8).read() as i32).wrapping_mul(0x100));
                let u_stack_404: u32 = param_1 as usize as u32;
                let mut pb_stack_40c: *mut u8 = pb;
                let mut pb_var13: *mut u8 = param_1.add(6).cast::<u8>();
                let s_var15: u32 = (i_var8 as u32).wrapping_mul(i_var5 as u32);
                param_1.add(4).write(i_var5 as u32);
                param_1.add(5).write(i_var8 as u32);
                _fread(pb_var13.cast::<c_void>(), s_var15, 3, p_f);
                _fclose(p_f);
                let pb_var1 = pb_var13.byte_add(s_var15 as usize);
                ab_stack_300[0] = pb_var13.read() >> 3;
                ab_stack_300[1] = pb_var13.byte_add(s_var15 as usize).read() >> 3;
                let mut pb_var12: *mut u8 = pb.byte_add(0x19);
                ab_stack_300[2] = pb_var13.byte_add(s_var15.wrapping_mul(2) as usize).read() >> 3;
                pb_var13.write(0);
                let mut col: [u8; 3] = [0; 3];
                if 1 < (s_var15 as i32) {
                    let mut counter: u32 = s_var15.wrapping_sub(1);
                    let mut u_var6: u32 = 1;
                    pb_var13 = pb_var12;
                    loop {
                        col[0] = pb_var13.read() >> 3;
                        col[1] = pb_var13.byte_add(s_var15 as usize).read() >> 3;
                        col[2] = pb_var13.byte_add(s_var15.wrapping_mul(2) as usize).read() >> 3;
                        let u_var7 = FUN_0040e940(
                            col.as_mut_ptr(),
                            ab_stack_300.as_mut_ptr() as usize as i32,
                            0,
                            u_var6,
                        );
                        let mut b_var4 = u_var7 as u8;
                        let mut i5 = ((u_var7 & 0xff).wrapping_mul(3)) as usize;
                        let mut u_var7 = u_var6;
                        if (ab_stack_300[i5] != col[0]
                            || ab_stack_300[i5 + 1] != col[1]
                            || ab_stack_300[i5 + 2] != col[2])
                            && (u_var6 as i32) < 0x100
                        {
                            b_var4 = u_var6 as u8;
                            u_var7 = u_var6.wrapping_add(1);
                            i5 = ((u_var6 & 0xff).wrapping_mul(3)) as usize;
                            ab_stack_300[i5] = col[0];
                            ab_stack_300[i5 + 1] = col[1];
                            ab_stack_300[i5 + 2] = col[2];
                        }
                        pb_var12.write(b_var4);
                        pb_var12 = pb_var12.add(1);
                        pb_var13 = pb_var13.add(1);
                        counter = counter.wrapping_sub(1);
                        u_var6 = u_var7;
                        if counter == 0 {
                            break;
                        }
                    }
                }
                let mut pb_stack_408: *mut u8 = pb_var1.byte_add(0x1000);
                pb_var13 = ab_stack_300.as_mut_ptr();
                param_1.add(1).write(0x10);
                pb_stack_40c.write(0);
                pb_stack_40c.add(1).write(1);
                pb_stack_40c.add(2).write(0);
                pb_stack_40c.add(3).write(0);
                pb_stack_40c = pb_var1;
                let end = ab_stack_300.as_mut_ptr().wrapping_add(0x300);
                loop {
                    let mut i5: i32 = 0;
                    let mut pb_var12: *mut u8 = pb_stack_408;
                    let mut pb_var10: *mut u8 = pb_stack_40c;
                    loop {
                        let v0 = pb_var13.read() as u32;
                        let v1 = pb_var13.add(1).read() as u32;
                        let v2 = pb_var13.add(2).read() as u32;
                        let i_var8: i32;
                        let i_var9: i32;
                        if i5 < 8 {
                            let a = (i5 as u32).wrapping_mul(v0).wrapping_mul(0x100) as i32;
                            col[0] = (((((a as u32).wrapping_add(((a >> 31) as u32) & 0xf) as i32)
                                >> 4) as u32)
                                .wrapping_add(v0.wrapping_mul(0x80))
                                >> 8) as u8;
                            let a = (i5 as u32).wrapping_mul(v1).wrapping_mul(0x100) as i32;
                            col[1] = (((((a as u32).wrapping_add(((a >> 31) as u32) & 0xf) as i32)
                                >> 4) as u32)
                                .wrapping_add(v1.wrapping_mul(0x80))
                                >> 8) as u8;
                            let a = (i5 as u32).wrapping_mul(v2).wrapping_mul(0x100) as i32;
                            i_var9 = (v2 << 7) as i32;
                            i_var8 =
                                ((a as u32).wrapping_add(((a >> 31) as u32) & 0xf) as i32) >> 4;
                        } else {
                            let m = (i5.wrapping_sub(8)) as u32;
                            let a = (0x1fu32.wrapping_sub(v0))
                                .wrapping_mul(m)
                                .wrapping_mul(0x100) as i32;
                            col[0] = (((((a as u32).wrapping_add(((a >> 31) as u32) & 7) as i32)
                                >> 3) as u32)
                                .wrapping_add(v0.wrapping_mul(0x100))
                                >> 8) as u8;
                            let a = (0x1fu32.wrapping_sub(v1))
                                .wrapping_mul(m)
                                .wrapping_mul(0x100) as i32;
                            col[1] = (((((a as u32).wrapping_add(((a >> 31) as u32) & 7) as i32)
                                >> 3) as u32)
                                .wrapping_add(v1.wrapping_mul(0x100))
                                >> 8) as u8;
                            let a = (0x1fu32.wrapping_sub(v2))
                                .wrapping_mul(m)
                                .wrapping_mul(0x100) as i32;
                            i_var9 = (v2 << 8) as i32;
                            i_var8 = ((a as u32).wrapping_add(((a >> 31) as u32) & 7) as i32) >> 3;
                        }
                        i5 = i5.wrapping_add(1);
                        col[2] = ((i_var8 as u32).wrapping_add(i_var9 as u32) >> 8) as u8;
                        let packed: u16 = ((col[0] as u16)
                            .wrapping_mul(0x20)
                            .wrapping_add(col[1] as u16))
                        .wrapping_mul(0x40)
                        .wrapping_add(col[2] as u16);
                        pb_var12.cast::<u16>().write_unaligned(packed);
                        col[0] = col[0].wrapping_mul(2);
                        col[1] = col[1].wrapping_mul(2);
                        col[2] = col[2].wrapping_mul(2);
                        let r = FUN_0040e940(col.as_mut_ptr(), 0x9a7bc0, 0x40, 0x100);
                        pb_var10.write(r as u8);
                        pb_var12 = pb_var12.byte_add(0x200);
                        pb_var10 = pb_var10.byte_add(0x100);
                        if i5 >= 0x10 {
                            break;
                        }
                    }
                    pb_stack_408 = pb_stack_408.add(2);
                    pb_stack_40c = pb_stack_40c.add(1);
                    pb_var13 = pb_var13.add(3);
                    if pb_var13 >= end {
                        break;
                    }
                }
                return ((pb_var1 as usize as u32).wrapping_add(0x3000u32.wrapping_sub(u_stack_404)))
                    as usize as *mut u8;
            }
        } else {
            let i_var5 = __strcmpi(pc_var11, (&raw mut DAT_005272cc).cast::<i8>());
            if i_var5 != 0 {
                param_1.add(4).write(8);
                param_1.add(5).write(8);
                param_1.add(1).write(0x10);
                param_1.write(0x100);
                FUN_004720d0(
                    param_1.add(6).cast::<u8>(),
                    (&raw mut DAT_005271e0).cast::<u8>(),
                    8,
                    8,
                );
                return 0x3058usize as *mut u8;
            }
            FUN_0049e720(buf, (&raw mut DAT_0051a854).cast::<i8>(), 0, 0);
            if DAT_00a07bd8.is_null() {
                param_1.add(4).write(8);
                param_1.add(5).write(8);
                param_1.add(1).write(0x10);
                param_1.write(0x100);
                FUN_004720d0(
                    param_1.add(6).cast::<u8>(),
                    (&raw mut DAT_005271e0).cast::<u8>(),
                    8,
                    8,
                );
                return 0x3058usize as *mut u8;
            }
        }
        let p_f = DAT_00a07bd8;
        _fread(param_1.cast::<c_void>(), 0x18, 1, DAT_00a07bd8);
        let mut s_var15: u32 = param_1.add(4).read().wrapping_mul(param_1.add(5).read());
        if param_1.add(2).read() == s_var15 {
            s_var15 = param_1.add(3).read();
        }
        param_1.write(0x100);
        param_1.add(1).write(0x10);
        _fread(param_1.add(6).cast::<c_void>(), s_var15, 1, p_f);
        _fread(
            param_1
                .cast::<u8>()
                .byte_add(s_var15 as usize)
                .byte_add(0x1018)
                .cast::<c_void>(),
            0x2000,
            1,
            p_f,
        );
        _fclose(p_f);
        s_var15.wrapping_add(0x3018) as usize as *mut u8
    }
}
