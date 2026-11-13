// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00459750(param_1: u32) -> u16;
    fn FUN_00459f30(param_1: u32, param_2: u32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_00425cb0() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004592d0(param_1: u32) -> u16 {
    // SAFETY: the callees and the global table are the game's own; every
    // offset written is inside the entry records the original addresses.
    unsafe {
        let u_var6 = FUN_00459750(0xd);
        if u_var6 == 0xffff {
            return 0xffff;
        }
        FUN_00459f30(u_var6 as u32, param_1 & 0xffff);
        let i_var4: i32 = (u_var6 as u32).wrapping_mul(0x23) as i32;

        let ent =
            |off: i32| -> *mut u8 { DAT_009a1fe8.byte_add(off.wrapping_add(i_var4) as usize) };
        let sub = |off: i32| -> *mut u8 {
            ent(0x1f)
                .cast::<*mut u8>()
                .read_unaligned()
                .byte_add(off as usize)
        };

        sub(0).write(5);
        ent(2).write(0xd);
        sub(1).write(0);
        let b_var5 = FUN_00425cb0() as u8;
        ent(3).write((b_var5 & 1).wrapping_add(0x85));
        let src = DAT_009a1fe8
            .byte_add(((param_1 & 0xffff).wrapping_mul(0x23).wrapping_add(3)) as usize)
            .read();
        sub(0x87).write(src);
        ent(0x1b).cast::<u32>().write_unaligned(0xffff_ffff);

        let u_var7 = FUN_00425cb0() as u16;
        let mut s_var10 = ((u_var7 & 0x7ff).wrapping_add(0x100)) as i16;
        let u_var7 = FUN_00425cb0() as u16;
        let mut s_var9 = ((u_var7 & 0x7ff).wrapping_add(0x100)) as i16;
        let u_var8 = FUN_00425cb0();
        if (u_var8 & 1) != 0 {
            s_var10 = s_var10.wrapping_neg();
        }
        let u_var8 = FUN_00425cb0();
        if (u_var8 & 1) != 0 {
            s_var9 = s_var9.wrapping_neg();
        }
        let p = ent(0x10).cast::<i16>();
        p.write_unaligned(p.read_unaligned().wrapping_add(s_var10));
        let p = ent(0x12).cast::<i16>();
        p.write_unaligned(p.read_unaligned().wrapping_add(s_var9));
        let p2 = ent(0x12).cast::<u16>();
        let u_var7 = p2.read_unaligned();
        if 0x7fff < u_var7 {
            p2.write_unaligned(0u16.wrapping_sub(u_var7));
            let p = ent(0x10).cast::<i16>();
            p.write_unaligned(p.read_unaligned().wrapping_add(-0x8000));
        }
        sub(0x92).write(1);
        let i_var3 = ent(0x1f).cast::<*mut u8>().read_unaligned();
        i_var3.byte_add(0x8b).write(i_var3.byte_add(0x92).read());
        let i_var3 = ent(0x1f).cast::<*mut u8>().read_unaligned();
        let u_var7 = FUN_00425cb0() as u16;
        let p = i_var3.byte_add(0x79).cast::<i16>();
        p.write_unaligned(
            p.read_unaligned()
                .wrapping_add((u_var7 & 0xff) as i16)
                .wrapping_add(0x32),
        );
        sub(0x83).cast::<u16>().write_unaligned(0);
        let u_var7 = FUN_00425cb0() as u16;
        sub(0x81)
            .cast::<u16>()
            .write_unaligned(((u_var7 & 3).wrapping_add(1)).wrapping_mul(0xec));
        ent(0x19).write(0);
        u_var6
    }
}
