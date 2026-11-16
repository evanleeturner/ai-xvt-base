// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d8112: i16;
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_009ec604: i32;
    static mut DAT_00a081e8: i32;
    static mut DAT_00a08298: i32;
    static mut DAT_009ec5fe: u16;
    static mut DAT_009a1fd4: i32;
    static mut DAT_009a1fd8: i32;
    static mut DAT_009a1fd0: i32;
    static mut DAT_00a07cec: u16;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009d80c0: i32;
    static mut DAT_009d8c18: i32;
    static mut DAT_009d1150: i32;
    static mut DAT_00a07bdc: i32;
    static mut DAT_00a07c54: i32;
    static mut DAT_00a07c58: i32;
    static mut DAT_00a07ce8: i32;
    static mut DAT_009d1308: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00427bd0(a: i16, b: i16, c: i32);
    fn FUN_00425d80(a: i16, b: u32) -> u32;
    fn FUN_0041c1d0(a: u16, b: u16) -> u32;
    fn FUN_00446700(a: u16, b: u16) -> u32;
}

#[inline(always)]
fn fx_mul(s: i16, u: u32) -> u32 {
    let l = (s as i32 as i64).wrapping_mul(u as i32 as i64);
    ((l as u32) >> 15) | ((((l as u64) >> 32) as u32) << 17)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0041cfd0(param_1: u16, param_2: i16) -> u16 {
    // SAFETY: all globals and the pointed-to entity records are the game's own
    // memory; reads and writes happen at the same offsets and order as the original.
    unsafe {
        let s_var6 = ((param_2 as i32 as u32).wrapping_mul(DAT_009d8112 as i32 as u32)) as i16;
        let i_var8 = DAT_009a1fe8.byte_add((param_1 as usize).wrapping_mul(0x23));
        DAT_009ec604 = i_var8.byte_add(4).cast::<i32>().read_unaligned();
        DAT_00a081e8 = i_var8.byte_add(8).cast::<i32>().read_unaligned();
        DAT_00a08298 = i_var8.byte_add(0xc).cast::<i32>().read_unaligned();
        let mut p_s = i_var8.byte_add(0x1f).cast::<*mut u8>().read_unaligned();
        let w79 = p_s.byte_add(0x79).cast::<u16>().read_unaligned();
        let t = (w79 as u32).wrapping_mul(0x1234).wrapping_add(0x80);
        let a = (t as i32) >> 8;
        let prod = (a as u32).wrapping_mul(DAT_009ec5fe as u32);
        let mut u_var4: u32 = (((prod as i32) / 0xec) as u32) & 0xffff;
        if p_s.byte_add(0x8b).read() != 0 {
            let s12 = i_var8.byte_add(0x12).cast::<i16>().read_unaligned();
            let s10 = i_var8.byte_add(0x10).cast::<i16>().read_unaligned();
            FUN_00427bd0(s12, s10, i_var8 as i32);
        }
        let i_var3 = s_var6 as i32;
        p_s = i_var8.byte_add(0x1f).cast::<*mut u8>().read_unaligned();
        let s8c = p_s.byte_add(0x8c).cast::<i16>().read_unaligned();
        let fx = fx_mul(s8c, u_var4);
        DAT_009a1fd4 = (i_var3 as u32)
            .wrapping_mul(fx)
            .wrapping_add(DAT_009ec604 as u32) as i32;
        let s8e = p_s.byte_add(0x8e).cast::<i16>().read_unaligned();
        let fx = fx_mul(s8e, u_var4);
        DAT_009a1fd8 = fx
            .wrapping_mul(i_var3 as u32)
            .wrapping_add(DAT_00a081e8 as u32) as i32;
        let s90 = p_s.byte_add(0x90).cast::<i16>().read_unaligned();
        let fx = fx_mul(s90, u_var4);
        DAT_009a1fd0 = fx
            .wrapping_mul(i_var3 as u32)
            .wrapping_add(DAT_00a08298 as u32) as i32;

        let mut u_var7: u16 = DAT_00a07cec;
        if (u_var7 as i32) < DAT_009a1fe0 {
            loop {
                let p_e = DAT_009a1fe8.byte_add((u_var7 as usize).wrapping_mul(0x23));
                if p_e.byte_add(3).read() != 0 && u_var7 != param_1 {
                    let c = p_e.byte_add(2).read();
                    if c == 4 || c == 5 || c == 3 {
                        DAT_009d80c0 = p_e.byte_add(4).cast::<i32>().read_unaligned();
                        DAT_009d8c18 = p_e.byte_add(8).cast::<i32>().read_unaligned();
                        DAT_009d1150 = p_e.byte_add(0xc).cast::<i32>().read_unaligned();
                        p_s = p_e.byte_add(0x1f).cast::<*mut u8>().read_unaligned();
                        let w79 = p_s.byte_add(0x79).cast::<u16>().read_unaligned();
                        let arg2 = ((((DAT_009d80c0 as u32) >> 0x10) as u16 as u32) << 16)
                            | (DAT_009d8112 as u16 as u32);
                        u_var4 = FUN_00425d80(w79 as i16, arg2) & 0xffff;
                        p_s = p_e.byte_add(0x1f).cast::<*mut u8>().read_unaligned();
                        if p_s.byte_add(0x8b).read() != 0 {
                            let s12 = p_e.byte_add(0x12).cast::<i16>().read_unaligned();
                            let s10 = p_e.byte_add(0x10).cast::<i16>().read_unaligned();
                            FUN_00427bd0(s12, s10, p_e as i32);
                        }
                        p_s = p_e.byte_add(0x1f).cast::<*mut u8>().read_unaligned();
                        let s8c = p_s.byte_add(0x8c).cast::<i16>().read_unaligned();
                        let fx = fx_mul(s8c, u_var4);
                        DAT_00a07bdc = fx
                            .wrapping_mul(i_var3 as u32)
                            .wrapping_add(DAT_009d80c0 as u32)
                            as i32;
                        p_s = p_e.byte_add(0x1f).cast::<*mut u8>().read_unaligned();
                        let s8e = p_s.byte_add(0x8e).cast::<i16>().read_unaligned();
                        let fx = fx_mul(s8e, u_var4);
                        DAT_00a07c54 = fx
                            .wrapping_mul(i_var3 as u32)
                            .wrapping_add(DAT_009d8c18 as u32)
                            as i32;
                        p_s = p_e.byte_add(0x1f).cast::<*mut u8>().read_unaligned();
                        let s90 = p_s.byte_add(0x90).cast::<i16>().read_unaligned();
                        let fx = fx_mul(s90, u_var4);
                        DAT_00a07c58 = fx
                            .wrapping_mul(i_var3 as u32)
                            .wrapping_add(DAT_009d1150 as u32)
                            as i32;
                        let r = FUN_0041c1d0(param_1, u_var7);
                        if (r as i16) != 0 {
                            return u_var7;
                        }
                    }
                }
                u_var7 = u_var7.wrapping_add(1);
                if !((u_var7 as i32) < DAT_009a1fe0) {
                    break;
                }
            }
        }

        u_var4 = 0;
        p_s = DAT_009a1fe8
            .byte_add(0x1f)
            .byte_add((param_1 as usize).wrapping_mul(0x23))
            .cast::<*mut u8>()
            .read_unaligned();
        let b12 = p_s.byte_add(0x12).read();
        if b12 != 0 {
            loop {
                let w53 = p_s
                    .byte_add(0x53)
                    .byte_add((u_var4 as usize).wrapping_mul(2))
                    .cast::<u16>()
                    .read_unaligned();
                let u_var7 = w53;
                if DAT_00a07ce8 <= (u_var7 as i32)
                    && (u_var7 as i32)
                        < (DAT_009d1308 as u32).wrapping_add(DAT_00a07ce8 as u32) as i32
                {
                    let u_var5 = FUN_00446700(param_1, u_var7);
                    if (u_var5 as i16) != 0 {
                        return u_var7;
                    }
                }
                u_var4 = ((u_var4 as u16).wrapping_add(1)) as u32;
                if !(u_var4 < p_s.byte_add(0x12).read() as u32) {
                    break;
                }
            }
        }
        0xffff
    }
}
