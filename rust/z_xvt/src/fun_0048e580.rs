// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732948: *mut core::ffi::c_void;
    static mut DAT_005225d0: [i8; 1];
    static mut DAT_0052a558: [i8; 1];
    static mut DAT_00523570: [i8; 1];
    static mut DAT_0052a478: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __strcmpi(a: *const i8, b: *const i8) -> i32;
    fn FUN_0047ed40(a: *mut i8, b: *mut i8, c: i32, d: i32);
    fn FUN_00486080(a: *mut u8, b: *mut u8, c: i32, d: i32);
    fn _fread(a: *mut core::ffi::c_void, b: u32, c: u32, d: *mut core::ffi::c_void) -> u32;
    fn _fclose(a: *mut core::ffi::c_void) -> i32;
    fn FUN_00462920(a: *mut u8, b: i32, c: i32, d: u32) -> u32;
}

#[allow(non_snake_case)]
unsafe fn set_default(param_1: *mut u32) {
    // SAFETY: param_1 is the caller's buffer; the callee and globals are the game's own.
    unsafe {
        param_1.add(4).write_unaligned(8);
        param_1.add(5).write_unaligned(8);
        param_1.add(1).write_unaligned(0x10);
        param_1.write_unaligned(0x100);
        FUN_00486080(
            param_1.add(6) as *mut u8,
            &raw mut DAT_0052a478 as *mut u8,
            8,
            8,
        );
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0048e580(param_1: *mut u32, param_2: *mut i8) -> *mut u8 {
    let mut ac_stack_400: [i8; 256] = [0; 256];
    let mut ab_stack_300: [u8; 768] = [0; 768];
    // SAFETY: param_1 and param_2 are the caller's buffers; the globals and
    // callees are the game's own; pointer arithmetic mirrors the original.
    unsafe {
        let buf = ac_stack_400.as_mut_ptr();
        let abs = ab_stack_300.as_mut_ptr();

        let mut n: u32 = 0;
        let mut p = param_2;
        loop {
            n = n.wrapping_add(1);
            let c = p.read();
            p = p.add(1);
            if c == 0 {
                break;
            }
        }
        let mut i: u32 = 0;
        while i < n {
            buf.wrapping_add(i as usize)
                .write(param_2.wrapping_add(i as usize).read());
            i += 1;
        }

        let mut u_var6: u32 = 0;
        let mut q = buf;
        loop {
            u_var6 = u_var6.wrapping_add(1);
            let c = q.read();
            q = q.add(1);
            if c == 0 {
                break;
            }
        }
        let pc = buf.wrapping_add(u_var6.wrapping_sub(4) as usize);
        let pc1 = buf.wrapping_add(u_var6.wrapping_sub(3) as usize);
        let pc2 = buf.wrapping_add(u_var6.wrapping_sub(2) as usize);

        let r = __strcmpi(pc as *const i8, &raw mut DAT_005225d0 as *const i8);
        if r == 0 {
            pc.write(b't' as i8);
            pc1.write(b'e' as i8);
            pc2.write(b'x' as i8);
            FUN_0047ed40(buf, &raw mut DAT_00523570 as *mut i8, 0, 0);
            pc.write(b'r' as i8);
            pc1.write(b'g' as i8);
            pc2.write(b'b' as i8);
            if DAT_00732948.is_null() {
                FUN_0047ed40(buf, &raw mut DAT_00523570 as *mut i8, 0, 0);
                let p_fvar3 = DAT_00732948;
                if DAT_00732948.is_null() {
                    set_default(param_1);
                    return 0x3058 as *mut u8;
                }
                _fread(param_1 as *mut core::ffi::c_void, 0x200, 1, DAT_00732948);
                let pb = param_1 as *mut u8;
                let i_var5 = (u32::from(pb.add(6).read()).wrapping_mul(0x100))
                    .wrapping_add(u32::from(pb.add(7).read())) as i32;
                let i_var8 = u32::from(pb.add(9).read())
                    .wrapping_add(u32::from(pb.add(8).read()).wrapping_mul(0x100))
                    as i32;
                let u_stack_404 = param_1;
                let mut pb_stack_40c = pb;
                let mut pbvar13 = pb.add(0x18);
                let s_var15 = (i_var8 as u32).wrapping_mul(i_var5 as u32);
                param_1.add(4).write_unaligned(i_var5 as u32);
                param_1.add(5).write_unaligned(i_var8 as u32);
                _fread(pbvar13 as *mut core::ffi::c_void, s_var15, 3, p_fvar3);
                _fclose(p_fvar3);
                let pbvar1 = pbvar13.wrapping_add(s_var15 as usize);
                abs.write(pbvar13.read() >> 3);
                abs.add(1)
                    .write(pbvar13.wrapping_add(s_var15 as usize).read() >> 3);
                let mut pbvar12 = pb.add(0x19);
                abs.add(2).write(
                    pbvar13
                        .wrapping_add(s_var15.wrapping_mul(2) as usize)
                        .read()
                        >> 3,
                );
                pbvar13.write(0);
                let mut bs: [u8; 3] = [0; 3];
                if 1 < (s_var15 as i32) {
                    let mut cnt: u32 = s_var15.wrapping_sub(1);
                    let mut u_var6: u32 = 1;
                    pbvar13 = pbvar12;
                    loop {
                        bs[0] = pbvar13.read() >> 3;
                        bs[1] = pbvar13.wrapping_add(s_var15 as usize).read() >> 3;
                        bs[2] = pbvar13
                            .wrapping_add(s_var15.wrapping_mul(2) as usize)
                            .read()
                            >> 3;
                        let u_var7 = FUN_00462920(bs.as_mut_ptr(), abs as i32, 0, u_var6);
                        let mut b_var4 = u_var7 as u8;
                        let mut i5 = ((u_var7 & 0xff) * 3) as usize;
                        let mut u7 = u_var6;
                        if (ab_stack_300[i5] != bs[0]
                            || ab_stack_300[i5 + 1] != bs[1]
                            || ab_stack_300[i5 + 2] != bs[2])
                            && (u_var6 as i32) < 0x100
                        {
                            b_var4 = u_var6 as u8;
                            u7 = u_var6.wrapping_add(1);
                            i5 = ((u_var6 & 0xff) * 3) as usize;
                            ab_stack_300[i5] = bs[0];
                            ab_stack_300[i5 + 1] = bs[1];
                            ab_stack_300[i5 + 2] = bs[2];
                        }
                        pbvar12.write(b_var4);
                        pbvar12 = pbvar12.add(1);
                        pbvar13 = pbvar13.add(1);
                        cnt = cnt.wrapping_sub(1);
                        u_var6 = u7;
                        if cnt == 0 {
                            break;
                        }
                    }
                }
                let mut pb_stack_408 = pbvar1.wrapping_add(0x1000);
                pbvar13 = abs;
                pb_stack_40c.add(4).cast::<u32>().write_unaligned(0x10);
                pb_stack_40c.write(0);
                pb_stack_40c.add(1).write(1);
                pb_stack_40c.add(2).write(0);
                pb_stack_40c.add(3).write(0);
                pb_stack_40c = pbvar1;
                loop {
                    let mut i5: i32 = 0;
                    let mut p12 = pb_stack_408;
                    let mut p10 = pb_stack_40c;
                    loop {
                        let a0 = u32::from(pbvar13.read());
                        let a1 = u32::from(pbvar13.add(1).read());
                        let a2 = u32::from(pbvar13.add(2).read());
                        let i_var8: i32;
                        let i_var9: i32;
                        if i5 < 8 {
                            let m = i5 as u32;
                            let v = m.wrapping_mul(a0).wrapping_mul(0x100) as i32;
                            let t = (v.wrapping_add(((v >> 31) as u32 & 0xf) as i32)) >> 4;
                            bs[0] = ((t as u32).wrapping_add(a0.wrapping_mul(0x80)) >> 8) as u8;
                            let v = m.wrapping_mul(a1).wrapping_mul(0x100) as i32;
                            let t = (v.wrapping_add(((v >> 31) as u32 & 0xf) as i32)) >> 4;
                            bs[1] = ((t as u32).wrapping_add(a1.wrapping_mul(0x80)) >> 8) as u8;
                            let v = m.wrapping_mul(a2).wrapping_mul(0x100) as i32;
                            i_var9 = (a2 << 7) as i32;
                            i_var8 = (v.wrapping_add(((v >> 31) as u32 & 0xf) as i32)) >> 4;
                        } else {
                            let m = (i5 - 8) as u32;
                            let v =
                                0x1fu32.wrapping_sub(a0).wrapping_mul(m).wrapping_mul(0x100) as i32;
                            let t = (v.wrapping_add(((v >> 31) as u32 & 7) as i32)) >> 3;
                            bs[0] = ((t as u32).wrapping_add(a0.wrapping_mul(0x100)) >> 8) as u8;
                            let v =
                                0x1fu32.wrapping_sub(a1).wrapping_mul(m).wrapping_mul(0x100) as i32;
                            let t = (v.wrapping_add(((v >> 31) as u32 & 7) as i32)) >> 3;
                            bs[1] = ((t as u32).wrapping_add(a1.wrapping_mul(0x100)) >> 8) as u8;
                            let v =
                                0x1fu32.wrapping_sub(a2).wrapping_mul(m).wrapping_mul(0x100) as i32;
                            i_var9 = (a2 << 8) as i32;
                            i_var8 = (v.wrapping_add(((v >> 31) as u32 & 7) as i32)) >> 3;
                        }
                        i5 += 1;
                        bs[2] = ((i_var8.wrapping_add(i_var9)) as u32 >> 8) as u8;
                        let us_tmp: u16 = ((u16::from(bs[0]).wrapping_mul(0x20))
                            .wrapping_add(u16::from(bs[1])))
                        .wrapping_mul(0x40)
                        .wrapping_add(u16::from(bs[2]));
                        p12.cast::<u16>().write_unaligned(us_tmp);
                        bs[0] = bs[0].wrapping_mul(2);
                        bs[1] = bs[1].wrapping_mul(2);
                        bs[2] = bs[2].wrapping_mul(2);
                        let u6 = FUN_00462920(bs.as_mut_ptr(), 0x6da540, 0x40, 0x100);
                        p10.write(u6 as u8);
                        p12 = p12.wrapping_add(0x200);
                        p10 = p10.wrapping_add(0x100);
                        if !(i5 < 0x10) {
                            break;
                        }
                    }
                    pb_stack_408 = pb_stack_408.wrapping_add(2);
                    pb_stack_40c = pb_stack_40c.wrapping_add(1);
                    pbvar13 = pbvar13.add(3);
                    if !(pbvar13 < abs.add(768)) {
                        break;
                    }
                }
                return (pbvar1 as u32).wrapping_add(0x3000u32.wrapping_sub(u_stack_404 as u32))
                    as *mut u8;
            }
        } else {
            let r2 = __strcmpi(pc as *const i8, &raw mut DAT_0052a558 as *const i8);
            if r2 != 0 {
                set_default(param_1);
                return 0x3058 as *mut u8;
            }
            FUN_0047ed40(buf, &raw mut DAT_00523570 as *mut i8, 0, 0);
            if DAT_00732948.is_null() {
                set_default(param_1);
                return 0x3058 as *mut u8;
            }
        }
        let p_fvar3 = DAT_00732948;
        _fread(param_1 as *mut core::ffi::c_void, 0x18, 1, DAT_00732948);
        let mut s_var15 = param_1
            .add(5)
            .read_unaligned()
            .wrapping_mul(param_1.add(4).read_unaligned());
        if param_1.add(2).read_unaligned() == s_var15 {
            s_var15 = param_1.add(3).read_unaligned();
        }
        param_1.write_unaligned(0x100);
        param_1.add(1).write_unaligned(0x10);
        _fread(
            param_1.add(6) as *mut core::ffi::c_void,
            s_var15,
            1,
            p_fvar3,
        );
        _fread(
            (param_1 as u32).wrapping_add(s_var15).wrapping_add(0x1018) as *mut core::ffi::c_void,
            0x2000,
            1,
            p_fvar3,
        );
        _fclose(p_fvar3);
        s_var15.wrapping_add(0x3018) as *mut u8
    }
}
