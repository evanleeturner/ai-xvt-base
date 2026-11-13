// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006ccad0: u16;
    static mut DAT_006ccad2: u16;
    static mut DAT_006d4988: *mut core::ffi::c_void;
    static mut DAT_006db71c: *mut core::ffi::c_void;
    static mut DAT_006db6c0: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040f050(param_1: u16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004213b0(param_1: i32) {
    // SAFETY: all pointers are the game's own memory, read and written at the
    // same offsets and in the same order as the original.
    unsafe {
        let p1 = param_1 as usize as *mut u8;
        let i_var3 = p1.byte_add(0x432).cast::<i32>().read_unaligned();
        let p3 = i_var3 as usize as *mut u8;
        let mut u_stack_12: u16 = 0;
        let tmp_ptr = p3.byte_add(0x1f).cast::<u32>().read_unaligned();
        let puvar4 = (tmp_ptr as usize as *mut u8)
            .byte_add(0xab)
            .cast::<u32>()
            .read_unaligned() as usize as *const u32;
        loop {
            let u_var9: u16 = DAT_006ccad0;
            let u_var6: u32 = u_stack_12 as u32;
            let mut u_var8: u16 = DAT_006ccad0;
            if p1.byte_add(0x344 + (u_var6 as usize) * 4).read() == 2 {
                let mut u_var7: u32;
                loop {
                    u_var7 = u_var8 as u32;
                    if u_var8 < DAT_006ccad2
                        && (DAT_006d4988 as *mut u8)
                            .byte_add(u_var7 as usize * 0x23 + 3)
                            .read()
                            != 0
                    {
                        u_var8 = u_var8.wrapping_add(1);
                    } else {
                        break;
                    }
                }
                if DAT_006ccad2 <= u_var8 {
                    return;
                }
                FUN_0040f050(u_var8);
                let i_var5: usize = u_var7.wrapping_mul(0x23) as usize;
                let d = DAT_006d4988 as *mut u8;
                let puvar2 = d.byte_add(i_var5);

                let p_dest =
                    d.byte_add(0x1f + i_var5).cast::<u32>().read_unaligned() as usize as *mut u8;
                p_dest.write(7);

                let b1 = puvar2.byte_add(0x1f).cast::<u32>().read_unaligned() as usize as *mut u8;
                let b2 = p3.byte_add(0x1f).cast::<u32>().read_unaligned() as usize as *mut u8;
                b1.byte_add(0x86).write(b2.byte_add(0x86).read());

                b1.byte_add(0x81).cast::<u16>().write_unaligned(0);
                b1.byte_add(0x7f).cast::<u16>().write_unaligned(0);

                let i_var1 = (DAT_006db71c as usize as u32)
                    .wrapping_add(u_var7.wrapping_sub(u_var9 as u32).wrapping_mul(0x4c));

                b1.byte_add(0xab).cast::<u32>().write_unaligned(i_var1);

                b1.byte_add(0xa3).cast::<u32>().write_unaligned(0);
                b1.byte_add(0xa7).cast::<u32>().write_unaligned(0);

                d.byte_add(2 + i_var5).write(0x10);
                d.byte_add(3 + i_var5).write(200);

                d.byte_add(i_var5)
                    .cast::<i16>()
                    .write_unaligned(DAT_006db6c0);

                DAT_006db6c0 = DAT_006db6c0.wrapping_add(1);

                let p_i1 = i_var1 as usize as *mut u8;
                p_i1.byte_add(0x40)
                    .cast::<u32>()
                    .write_unaligned(puvar4.read_unaligned());
                p_i1.byte_add(0x44).cast::<u16>().write_unaligned(0xffff);
                p_i1.byte_add(0x46)
                    .cast::<i16>()
                    .write_unaligned(((((u_var7 & 7).wrapping_mul(0xec)) as i32) >> 3) as i16);

                p1.byte_add(0x3f2 + (u_var6 as usize) * 4)
                    .cast::<u32>()
                    .write_unaligned(puvar2 as usize as u32);
                p1.byte_add(0x3c6 + (u_var6 as usize) * 2)
                    .cast::<u16>()
                    .write_unaligned(puvar2.cast::<u16>().read_unaligned());
            } else {
                p1.byte_add(0x3f2 + (u_var6 as usize) * 4)
                    .cast::<u32>()
                    .write_unaligned(0);
            }
            u_stack_12 = u_stack_12.wrapping_add(1);
            if 0xf < u_stack_12 {
                return;
            }
        }
    }
}
