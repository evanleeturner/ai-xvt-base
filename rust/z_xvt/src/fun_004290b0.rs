// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525750: [i32; 0];
    static mut DAT_00525740: [i32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004272f0() -> *mut core::ffi::c_void;
    fn FUN_00427480(param_1: i32);
    fn FUN_004282d0(param_1: i32);
    fn FUN_00427660(param_1: *mut core::ffi::c_void);
    fn malloc(size: u32) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004290b0(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
) {
    // SAFETY: the callees and globals are the game's own; all pointers are
    // the caller's buffers and the context, read and written as the original does.
    unsafe {
        let ctx = FUN_004272f0();
        if ctx.is_null() {
            return;
        }
        let c = ctx.cast::<u8>();
        c.byte_add(0x1028).cast::<i32>().write_unaligned(0);
        c.byte_add(0x1030).cast::<i32>().write_unaligned(param_3);
        c.byte_add(0x1034).cast::<i32>().write_unaligned(param_4);
        c.byte_add(0x108e)
            .cast::<i32>()
            .write_unaligned(param_3.wrapping_mul(param_4));

        let mut out_ptr = malloc(
            (param_3 as u32)
                .wrapping_mul(param_4 as u32)
                .wrapping_mul(6),
        )
        .cast::<u8>();
        c.byte_add(0x1086)
            .cast::<u32>()
            .write_unaligned(out_ptr as usize as u32);

        let mut pb = param_1.wrapping_add(0x10) as *const u8;
        let mut i_var9: i32 = 0;
        let mut b6: u8 = pb.read();

        while b6 != 0xff {
            while b6 != 0xfe {
                let mut local_4: u8 = pb.read();
                if local_4 == 0xfb {
                    pb = pb.add(3);
                    i_var9 = (pb.sub(1).read() as i32)
                        .wrapping_mul(0x100)
                        .wrapping_add(pb.sub(2).read() as i32);
                } else if local_4 == 0xfc {
                    let mut count: u8 = pb.add(1).read();
                    pb = pb.add(2);
                    loop {
                        out_ptr.write(0x80);
                        out_ptr = out_ptr.add(1);
                        out_ptr.write(0x80);
                        out_ptr = out_ptr.add(1);
                        out_ptr.write(0x80);
                        out_ptr = out_ptr.add(1);
                        out_ptr.write(0);
                        out_ptr = out_ptr.add(1);
                        out_ptr.write(0);
                        out_ptr = out_ptr.add(1);
                        out_ptr.write(0);
                        out_ptr = out_ptr.add(1);
                        count = count.wrapping_sub(1);
                        if count == 0 {
                            break;
                        }
                    }
                } else {
                    let u_var7: u32;
                    if local_4 == 0xfd {
                        u_var7 = pb.add(2).read() as u32;
                        local_4 = pb.add(1).read();
                        pb = pb.add(3);
                    } else {
                        pb = pb.add(1);
                        let sh = (*(&raw const DAT_00525750)
                            .cast::<i32>()
                            .offset(param_5 as isize))
                            & 0x1f;
                        u_var7 = ((local_4 as u32) >> sh).wrapping_add(i_var9 as u32);
                        let mask = *(&raw const DAT_00525740)
                            .cast::<i32>()
                            .offset(param_5 as isize);
                        local_4 = (mask & (local_4 as i32)) as u8;
                    }
                    let i_var8 = (u_var7 as i32).wrapping_mul(4);
                    let src = i_var8.wrapping_add(param_2) as *const u8;
                    loop {
                        out_ptr.write(src.read());
                        out_ptr = out_ptr.add(1);
                        out_ptr.write(src.add(1).read());
                        out_ptr = out_ptr.add(1);
                        out_ptr.write(src.add(2).read());
                        out_ptr = out_ptr.add(1);
                        out_ptr.write(0);
                        out_ptr = out_ptr.add(1);
                        out_ptr.write(0);
                        out_ptr = out_ptr.add(1);
                        out_ptr.write(0);
                        out_ptr = out_ptr.add(1);
                        local_4 = local_4.wrapping_sub(1);
                        if local_4 == 0 {
                            break;
                        }
                    }
                }
                b6 = pb.read();
            }
            pb = pb.add(1);
        }

        FUN_00427480(ctx as usize as i32);
        FUN_004282d0(ctx as usize as i32);
        FUN_00427660(ctx);
    }
}
