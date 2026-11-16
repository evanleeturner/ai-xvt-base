// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_009a20b0: u32;
    static mut DAT_009cd26c: u32;
    static mut DAT_009fd438: u32;
    static mut DAT_009d8112: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00459750(param_1: u32) -> u16;
    fn FUN_00459f30(param_1: u32, param_2: u32);
    fn FUN_00425cb0() -> u32;
    fn FUN_00425d80(param_1: i16, param_2: u32) -> u32;
    fn FUN_00427bd0(param_1: i16, param_2: i16, param_3: i32);
    fn FUN_004464c0(param_1: i32);
}

#[inline(always)]
unsafe fn rd_ptr(e: *mut u8) -> *mut u8 {
    // SAFETY: e points at a game entry whose +0x1f field holds a pointer.
    unsafe { e.byte_add(0x1f).cast::<*mut u8>().read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00459480(param_1: u32) -> u16 {
    // SAFETY: all globals, callees and entry memory belong to the game; the
    // accesses mirror the original in order and width.
    unsafe {
        let u_var8 = FUN_00459750(0xd);
        if u_var8 == 0xffff {
            return 0xffff;
        }
        FUN_00459f30(u_var8 as u32, param_1 & 0xffff);
        let i_var12: u32 = (u_var8 as u32).wrapping_mul(0x23);

        let base = DAT_009a1fe8;
        let mut p = rd_ptr(base.byte_add(i_var12 as usize));
        *p = 5;
        DAT_009a1fe8.byte_add(2 + i_var12 as usize).write(0xd);
        p = rd_ptr(DAT_009a1fe8.byte_add(i_var12 as usize));
        *p.byte_add(1) = 2;
        DAT_009a1fe8.byte_add(3 + i_var12 as usize).write(0x9d);
        p = rd_ptr(DAT_009a1fe8.byte_add(i_var12 as usize));
        let src = DAT_009a1fe8
            .byte_add(((param_1 & 0xffff).wrapping_mul(0x23)).wrapping_add(3) as usize)
            .read();
        *p.byte_add(0x87) = src;
        DAT_009a1fe8
            .byte_add(0x1b + i_var12 as usize)
            .cast::<u32>()
            .write_unaligned(0xffff_ffff);

        let mut u_var9 = FUN_00425cb0() as u16;
        let mut s_var13 = ((u_var9 & 0x1fff) as u32).wrapping_add(0x100) as i16;
        u_var9 = FUN_00425cb0() as u16;
        let mut s_var11 = ((u_var9 & 0x1fff) as u32).wrapping_add(0x100) as i16;
        let mut u_var10 = FUN_00425cb0();
        if (u_var10 & 1) != 0 {
            s_var13 = s_var13.wrapping_neg();
        }
        u_var10 = FUN_00425cb0();
        if (u_var10 & 1) != 0 {
            s_var11 = s_var11.wrapping_neg();
        }

        let mut e = DAT_009a1fe8.byte_add(i_var12 as usize);

        let mut s = e.byte_add(0x10).cast::<i16>().read_unaligned();
        s = (s as i32).wrapping_add(-0x8000) as i16;
        e.byte_add(0x10).cast::<i16>().write_unaligned(s);
        s = e.byte_add(0x10).cast::<i16>().read_unaligned();
        s = (s as i32).wrapping_add(s_var13 as i32) as i16;
        e.byte_add(0x10).cast::<i16>().write_unaligned(s);

        s = e.byte_add(0x12).cast::<i16>().read_unaligned();
        s = (-0x8000i32).wrapping_sub(s as i32) as i16;
        e.byte_add(0x12).cast::<i16>().write_unaligned(s);
        s = e.byte_add(0x12).cast::<i16>().read_unaligned();
        s = (s as i32).wrapping_add(s_var11 as i32) as i16;
        e.byte_add(0x12).cast::<i16>().write_unaligned(s);

        u_var9 = e.byte_add(0x12).cast::<u16>().read_unaligned();
        if 0x7fff < u_var9 {
            let us = 0u32.wrapping_sub(u_var9 as u32) as u16;
            e.byte_add(0x12).cast::<u16>().write_unaligned(us);
            s = e.byte_add(0x10).cast::<i16>().read_unaligned();
            s = (s as i32).wrapping_add(-0x8000) as i16;
            e.byte_add(0x10).cast::<i16>().write_unaligned(s);
        }

        p = rd_ptr(e);
        *p.byte_add(0x92) = 1;
        p = rd_ptr(e);
        *p.byte_add(0x8b) = *p.byte_add(0x92);

        u_var9 = FUN_00425cb0() as u16;
        p = rd_ptr(e);
        let us = ((u_var9 & 0xf) as u32).wrapping_add(0x23) as u16;
        p.byte_add(0x79).cast::<u16>().write_unaligned(us);

        p = rd_ptr(e);
        p.byte_add(0x83).cast::<u16>().write_unaligned(0);

        u_var9 = FUN_00425cb0() as u16;
        p = rd_ptr(e);
        let us = ((u_var9 & 3) as u32).wrapping_add(0x27) as u16;
        p.byte_add(0x81).cast::<u16>().write_unaligned(us);

        *e.byte_add(0x19) = 2;

        e = DAT_009a1fe8.byte_add(i_var12 as usize);

        p = rd_ptr(e);
        let mut u32v = e.byte_add(4).cast::<u32>().read_unaligned();
        p.byte_add(6).cast::<u32>().write_unaligned(u32v);

        let u_var4 = e.byte_add(8).cast::<u32>().read_unaligned();
        p = rd_ptr(e);
        p.byte_add(10).cast::<u32>().write_unaligned(u_var4);

        p = rd_ptr(e);
        u32v = e.byte_add(0xc).cast::<u32>().read_unaligned();
        p.byte_add(0xe).cast::<u32>().write_unaligned(u32v);

        p = rd_ptr(e);
        s = p.byte_add(0x79).cast::<i16>().read_unaligned();
        u_var10 = FUN_00425d80(s, ((u_var4 >> 0x10) << 0x10) | (DAT_009d8112 as u32));
        let u_var14 = u_var10 & 0xffff;
        if (u_var10 as i16) != 0 {
            p = rd_ptr(e);
            if *p.byte_add(0x8b) != 0 {
                let a = e.byte_add(0x12).cast::<i16>().read_unaligned();
                let b = e.byte_add(0x10).cast::<i16>().read_unaligned();
                FUN_00427bd0(a, b, e as i32);
            }
            p = rd_ptr(e);
            let a = p.byte_add(0x8c).cast::<i16>().read_unaligned();
            let l5 = (a as i32 as i64).wrapping_mul(u_var14 as i32 as i64);
            p = rd_ptr(e);
            let b = p.byte_add(0x8e).cast::<i16>().read_unaligned();
            let l6 = (b as i32 as i64).wrapping_mul(u_var14 as i32 as i64);
            p = rd_ptr(e);
            let c = p.byte_add(0x90).cast::<i16>().read_unaligned();
            let l7 = (c as i32 as i64).wrapping_mul(u_var14 as i32 as i64);

            DAT_009a20b0 = (((l5 as u32) >> 0xf) | ((((l5 as u64) >> 0x20) as u32) << 0x11)) << 2;
            DAT_009cd26c = (((l6 as u32) >> 0xf) | ((((l6 as u64) >> 0x20) as u32) << 0x11)) << 2;
            DAT_009fd438 = (((l7 as u32) >> 0xf) | ((((l7 as u64) >> 0x20) as u32) << 0x11)) << 2;
            FUN_004464c0(e as i32);
        }
        u_var8
    }
}
