// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_009ecc34: i32;
    static mut DAT_009e9670: [u8; 1];
    static mut DAT_009e968a: [u8; 1];
    static mut DAT_009a1ff8: i32;
    static mut _DAT_009ed22e: [u8; 4];
    static mut DAT_00a08368: i16;
    static mut DAT_00a08374: u16;
    static mut DAT_00a08c7c: i16;
    static mut DAT_00a08c78: i16;
    static mut DAT_00a08c7e: u16;
    static mut DAT_00a08a02: u16;
    static mut DAT_00a08340: *mut u8;
    static mut DAT_00a08a04: *mut u8;
    static mut DAT_00a08cac: i16;
    static mut DAT_00a08cae: i16;
    static mut DAT_00a08ca0: i16;
    static mut DAT_00a08ca2: i16;
    static mut DAT_00a0a1d2: i16;
    static mut DAT_00a0a1d4: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00427bd0(a: i16, b: i16, c: i32);
    fn FUN_00427d30(a: i16, b: i32, c: i32);
    fn FUN_00403540(a: u32, b: u32);
    fn FUN_00425e30(a: i32, b: i32, c: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0043a0c0(param_1: u16) {
    // SAFETY: all globals and the pointers derived from them are the game's
    // own memory, accessed at the offsets the original uses.
    unsafe {
        let i_var10: u32 = (param_1 as u32).wrapping_mul(0x23);
        let p_self = DAT_009a1fe8.byte_add(i_var10 as usize);
        let base1 = (&raw mut DAT_009e9670).cast::<u8>();
        let self_idx: u32 = base1
            .byte_add((DAT_009ecc34 as u32).wrapping_mul(0x5bd) as usize)
            .cast::<u32>()
            .read_unaligned();
        let p_ent = DAT_009a1fe8.byte_add(self_idx.wrapping_mul(0x23) as usize);

        let i_var14: i32 = p_self
            .byte_add(4)
            .cast::<u32>()
            .read_unaligned()
            .wrapping_sub(p_ent.byte_add(4).cast::<u32>().read_unaligned())
            as i32;
        let i_var15: i32 = p_self
            .byte_add(8)
            .cast::<u32>()
            .read_unaligned()
            .wrapping_sub(p_ent.byte_add(8).cast::<u32>().read_unaligned())
            as i32;
        let i_var12: i32 = p_self
            .byte_add(0xc)
            .cast::<u32>()
            .read_unaligned()
            .wrapping_sub(p_ent.byte_add(0xc).cast::<u32>().read_unaligned())
            as i32;

        let mut rec: *mut u8 = p_ent.byte_add(0x1f).cast::<*mut u8>().read_unaligned();
        if rec.byte_add(0x92).read() != 0 {
            FUN_00427bd0(
                p_ent.byte_add(0x12).cast::<i16>().read_unaligned(),
                p_ent.byte_add(0x10).cast::<i16>().read_unaligned(),
                p_ent as usize as i32,
            );
            let e = DAT_009a1fe8.byte_add(self_idx.wrapping_mul(0x23) as usize);
            FUN_00427d30(
                e.byte_add(0x14).cast::<i16>().read_unaligned(),
                0,
                e as usize as i32,
            );
        }

        rec = DAT_009a1fe8
            .byte_add(0x1f)
            .byte_add(self_idx.wrapping_mul(0x23) as usize)
            .cast::<*mut u8>()
            .read_unaligned();
        let s93 = rec.byte_add(0x93).cast::<i16>().read_unaligned();
        let s95 = rec.byte_add(0x95).cast::<i16>().read_unaligned();
        let s97 = rec.byte_add(0x97).cast::<i16>().read_unaligned();
        let l3 = (i_var14 as i64).wrapping_mul(s93 as i64);
        let l4 = (i_var15 as i64).wrapping_mul(s95 as i64);
        let l5 = (i_var12 as i64).wrapping_mul(s97 as i64);
        let mut i_var13: u32 = ((l4 >> 15) as u32)
            .wrapping_add((l3 >> 15) as u32)
            .wrapping_add((l5 >> 15) as u32);

        let s99 = rec.byte_add(0x99).cast::<i16>().read_unaligned();
        let s9b = rec.byte_add(0x9b).cast::<i16>().read_unaligned();
        let s9d = rec.byte_add(0x9d).cast::<i16>().read_unaligned();
        let s9f = rec.byte_add(0x9f).cast::<i16>().read_unaligned();
        let sa1 = rec.byte_add(0xa1).cast::<i16>().read_unaligned();
        let sa3 = rec.byte_add(0xa3).cast::<i16>().read_unaligned();
        let l3 = (i_var14 as i64).wrapping_mul(s99 as i64);
        let l4 = (i_var15 as i64).wrapping_mul(s9b as i64);
        let l5 = (i_var12 as i64).wrapping_mul(s9d as i64);
        let l6 = (i_var14 as i64).wrapping_mul(s9f as i64);
        let l7 = (i_var15 as i64).wrapping_mul(sa1 as i64);
        let l8 = (i_var12 as i64).wrapping_mul(sa3 as i64);

        let b_var16 = (i_var13 as i32) < 0;
        if b_var16 {
            i_var13 = 0u32.wrapping_sub(i_var13);
        }

        let p_self = DAT_009a1fe8.byte_add(i_var10 as usize);
        let pc_var2: *mut u8 = p_self.byte_add(0x1f).cast::<*mut u8>().read_unaligned();
        if pc_var2.is_null() {
            DAT_00a08368 = 0x2f;
        } else if p_self.byte_add(2).read() == 9 {
            DAT_00a08368 = 0x2f;
        } else if pc_var2.read() == 1 {
            let v: i32 = (&raw mut _DAT_009ed22e).cast::<i32>().read_unaligned();
            let t = (v as u32).wrapping_add(((v >> 31) as u32) & 3) as i32;
            if ((t >> 2) as u32 & 1) == 0 {
                DAT_00a08368 = 0x37;
            } else {
                DAT_00a08368 = 0x3b;
            }
        } else {
            match pc_var2.byte_add(0x88).read() as i8 {
                0 => DAT_00a08368 = 0x3f,
                1 | 4 => DAT_00a08368 = 0x37,
                2 => DAT_00a08368 = 0x33,
                3 => DAT_00a08368 = 0x3b,
                5 => DAT_00a08368 = 0xd3,
                _ => {}
            }
        }

        let v32: u32 = base1
            .byte_add((DAT_009ecc34 as u32).wrapping_mul(0x5bd) as usize)
            .cast::<u32>()
            .read_unaligned();
        FUN_00403540(v32, param_1 as u32);

        if DAT_009a1ff8 < 0x1dd37 {
            if 0xee9b < DAT_009a1ff8 {
                if DAT_00a08368 == 0x2f {
                    DAT_00a08368 = 0x2e;
                } else if DAT_00a08368 == 0xd3 {
                    DAT_00a08368 = 0xd4;
                } else {
                    DAT_00a08368 = DAT_00a08368.wrapping_sub(1);
                }
            }
        } else if DAT_00a08368 == 0x2f {
            DAT_00a08368 = 0x2d;
        } else if DAT_00a08368 == 0xd3 {
            DAT_00a08368 = 0xd5;
        } else {
            DAT_00a08368 = DAT_00a08368.wrapping_sub(2);
        }

        FUN_00425e30(
            ((l3 >> 15) as u32)
                .wrapping_add((l4 >> 15) as u32)
                .wrapping_add((l5 >> 15) as u32) as i32,
            0u32.wrapping_sub(
                ((l7 >> 15) as u32)
                    .wrapping_add((l6 >> 15) as u32)
                    .wrapping_add((l8 >> 15) as u32),
            ) as i32,
            i_var13 as i32,
        );

        if b_var16 {
            let off = (DAT_00a08374 as u32).wrapping_mul(0xc) as usize;
            DAT_00a08c7c =
                DAT_00a08c7c.wrapping_add((&raw mut DAT_00a08cac).byte_add(off).read_unaligned());
            DAT_00a08c78 =
                DAT_00a08c78.wrapping_add((&raw mut DAT_00a08cae).byte_add(off).read_unaligned());
            if DAT_00a08c78 < 0 {
                DAT_00a08c78 = 0;
            }
            let o = (DAT_00a08c7e as u32).wrapping_mul(6) as usize;
            DAT_00a08340
                .byte_add(o)
                .cast::<i16>()
                .write_unaligned(DAT_00a08c7c);
            DAT_00a08340
                .byte_add(2)
                .byte_add(o)
                .cast::<i16>()
                .write_unaligned(DAT_00a08c78);
            DAT_00a08340
                .byte_add(4)
                .byte_add(o)
                .cast::<i16>()
                .write_unaligned(DAT_00a08368);
            let u_var9 = DAT_00a08c7e;
            DAT_00a08c7e = DAT_00a08c7e.wrapping_add(1);
            if DAT_00a08c7e == 0x30 {
                DAT_00a08c7e = u_var9;
            }
        } else {
            let off = (DAT_00a08374 as u32).wrapping_mul(0xc) as usize;
            DAT_00a08c7c =
                DAT_00a08c7c.wrapping_add((&raw mut DAT_00a08ca0).byte_add(off).read_unaligned());
            DAT_00a08c78 =
                DAT_00a08c78.wrapping_add((&raw mut DAT_00a08ca2).byte_add(off).read_unaligned());
            if DAT_00a08c78 < 0 {
                DAT_00a08c78 = 0;
            }
            let o = (DAT_00a08a02 as u32).wrapping_mul(6) as usize;
            DAT_00a08a04
                .byte_add(o)
                .cast::<i16>()
                .write_unaligned(DAT_00a08c7c);
            DAT_00a08a04
                .byte_add(2)
                .byte_add(o)
                .cast::<i16>()
                .write_unaligned(DAT_00a08c78);
            DAT_00a08a04
                .byte_add(4)
                .byte_add(o)
                .cast::<i16>()
                .write_unaligned(DAT_00a08368);
            let u_var9 = DAT_00a08a02;
            DAT_00a08a02 = DAT_00a08a02.wrapping_add(1);
            if DAT_00a08a02 == 0x30 {
                DAT_00a08a02 = u_var9;
            }
        }

        let us: u16 = (&raw mut DAT_009e968a)
            .cast::<u8>()
            .byte_add((DAT_009ecc34 as u32).wrapping_mul(0x5bd) as usize)
            .cast::<u16>()
            .read_unaligned();
        if us == param_1 {
            DAT_00a0a1d2 = DAT_00a08c7c;
            DAT_00a0a1d4 = DAT_00a08c78;
        }
    }
}
