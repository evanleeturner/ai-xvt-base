// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00521f19: [i8; 1];
    static mut DAT_009a7ed0: [i16; 1];
    static mut DAT_00662ce8: f32;
    static mut DAT_00662cec: f32;
    static mut DAT_00663980: f32;
    static mut DAT_00662cf0: f32;
    static mut DAT_00663984: f32;
    static mut DAT_00663658: i32;
    static mut DAT_00663988: f32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac720(param_1: i32) -> *mut core::ffi::c_void;
    fn FUN_00471ff0(param_1: *mut i32);
    fn FUN_004adcc0(param_1: i32) -> i32;
    fn FUN_004ac740();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004add60(param_1: i32) {
    // SAFETY: all globals and callees are the game's own memory; the pointers
    // read below are the game's structures, accessed at the original offsets.
    unsafe {
        let mut local_10: f32 = 1.0737418e+09;
        let mut local_14: f32 = 1.0737418e+09;
        let mut local_18: f32 = 1.0737418e+09;
        let mut local_4: f32 = -1.0737418e+09;
        let mut local_8: f32 = -1.0737418e+09;
        let mut local_c: f32 = -1.0737418e+09;

        let flag = (&raw mut DAT_00521f19)
            .cast::<i8>()
            .byte_offset(param_1.wrapping_mul(0x1a) as isize)
            .read();
        if (flag as i32 & 1) != 0 {
            let hi = ((param_1.wrapping_mul(0x19) as u32) >> 16) as i16 as i32;
            let lo = (&raw mut DAT_009a7ed0)
                .cast::<i16>()
                .offset(param_1 as isize)
                .read() as u16 as i32;
            let arg = hi.wrapping_shl(16) | lo;
            let pi_var1 = FUN_004ac720(arg).cast::<i32>();
            if (pi_var1.read_unaligned() as u32) != (pi_var1 as usize as u32) {
                FUN_00471ff0(pi_var1);
            }
            let count_val: i32 = pi_var1.byte_add(6).read_unaligned();
            let array_base_val: i32 = pi_var1.byte_add(10).read_unaligned();
            let mut i_var4: i32 = 0;
            let mut i_var5: i32 = 0;
            if 0 < count_val {
                loop {
                    let i_var2 = ((array_base_val.wrapping_add(i_var4)) as usize as *const i32)
                        .read_unaligned();
                    if i_var2 != 0
                        && ((i_var2.wrapping_add(4)) as usize as *const i32).read_unaligned()
                            != 0x14
                    {
                        let i_var2 = FUN_004adcc0(i_var2);
                        if i_var2 != 0 {
                            let n = ((i_var2.wrapping_add(0x10)) as usize as *const i32)
                                .read_unaligned();
                            if 1 < n {
                                let base = ((i_var2.wrapping_add(0x14)) as usize as *const i32)
                                    .read_unaligned();
                                let addr =
                                    base.wrapping_add(n.wrapping_mul(0xc)).wrapping_sub(0x18);
                                let p = addr as usize as *const f32;
                                let v0 = p.read_unaligned();
                                if v0 < local_18 {
                                    local_18 = v0;
                                }
                                let v1 = p.add(1).read_unaligned();
                                if v1 < local_14 {
                                    local_14 = v1;
                                }
                                let v2 = p.add(2).read_unaligned();
                                if v2 < local_10 {
                                    local_10 = v2;
                                }
                                let v3 = p.add(3).read_unaligned();
                                if local_c < v3 {
                                    local_c = v3;
                                }
                                let v4 = p.add(4).read_unaligned();
                                if local_8 < v4 {
                                    local_8 = v4;
                                }
                                let v5 = p.add(5).read_unaligned();
                                if local_4 < v5 {
                                    local_4 = v5;
                                }
                            }
                        }
                    }
                    i_var4 = i_var4.wrapping_add(4);
                    i_var5 = i_var5.wrapping_add(1);
                    if !(i_var5 < count_val) {
                        break;
                    }
                }
            }
            let off12 = param_1.wrapping_mul(0xc) as isize;
            (&raw mut DAT_00662ce8)
                .cast::<u8>()
                .byte_offset(off12)
                .cast::<f32>()
                .write_unaligned(local_18);
            (&raw mut DAT_00662cec)
                .cast::<u8>()
                .byte_offset(off12)
                .cast::<f32>()
                .write_unaligned(local_14);
            (&raw mut DAT_00663980)
                .cast::<u8>()
                .byte_offset(off12)
                .cast::<f32>()
                .write_unaligned(local_c);
            (&raw mut DAT_00662cf0)
                .cast::<u8>()
                .byte_offset(off12)
                .cast::<f32>()
                .write_unaligned(local_10);
            (&raw mut DAT_00663984)
                .cast::<u8>()
                .byte_offset(off12)
                .cast::<f32>()
                .write_unaligned(local_8);
            (&raw mut DAT_00663658)
                .cast::<u8>()
                .byte_offset(param_1.wrapping_mul(4) as isize)
                .cast::<i32>()
                .write_unaligned(1);
            (&raw mut DAT_00663988)
                .cast::<u8>()
                .byte_offset(off12)
                .cast::<f32>()
                .write_unaligned(local_4);
            FUN_004ac740();
        }
    }
}
