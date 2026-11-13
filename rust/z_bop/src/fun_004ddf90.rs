// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa6cfc: u32;
    static mut DAT_00aa7356: u32;
    static mut DAT_00aa734e: i32;
    static mut DAT_00aa7b1f: u32;
    static mut DAT_00aa7b13: i32;
    static mut DAT_00aa7215: i32;
    static mut DAT_00aa7211: i32;
    static mut DAT_00aa734a: *mut u32;
    static mut DAT_00aa735a: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d48e0() -> u32;
    fn FUN_004d4970();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ddf90() {
    // SAFETY: the globals and callees are the game's own; the pointers are the
    // game's surface and source buffers, accessed exactly as the original does.
    unsafe {
        DAT_00aa6cfc = FUN_004d48e0();
        let u_var2: u32 = DAT_00aa7356;
        let i_var1: i32 = DAT_00aa734e;
        if DAT_00aa7b1f == 8 {
            let addr = DAT_00aa6cfc
                .wrapping_add(DAT_00aa7b13.wrapping_mul(DAT_00aa7215) as u32)
                .wrapping_add(DAT_00aa7211 as u32);
            let mut p_var3 = addr as usize as *mut u8;
            let mut p_var5: *mut u8 = DAT_00aa734a as *mut u8;
            let mut i_var6: i32 = DAT_00aa735a;
            if 0 < DAT_00aa735a {
                loop {
                    let mut p_var7 = p_var5;
                    let mut p_var9 = p_var3;
                    let mut u_var4 = u_var2 >> 2;
                    while u_var4 != 0 {
                        p_var9
                            .cast::<u32>()
                            .write_unaligned(p_var7.cast::<u32>().read_unaligned());
                        p_var7 = p_var7.wrapping_byte_add(4);
                        p_var9 = p_var9.wrapping_byte_add(4);
                        u_var4 -= 1;
                    }
                    u_var4 = u_var2 & 3;
                    while u_var4 != 0 {
                        p_var9.write(p_var7.read());
                        p_var7 = p_var7.wrapping_byte_add(1);
                        p_var9 = p_var9.wrapping_byte_add(1);
                        u_var4 -= 1;
                    }
                    p_var3 = p_var3.wrapping_byte_offset(DAT_00aa7b13 as isize);
                    i_var6 = i_var6.wrapping_sub(1);
                    p_var5 = p_var5.wrapping_byte_offset(i_var1 as isize);
                    if i_var6 == 0 {
                        break;
                    }
                }
            }
        } else if DAT_00aa7b1f == 0x10 {
            let addr = DAT_00aa7b13
                .wrapping_mul(DAT_00aa7215)
                .wrapping_add(DAT_00aa7211.wrapping_mul(2))
                .wrapping_add(DAT_00aa6cfc as i32);
            let mut p_var8 = addr as u32 as usize as *mut u8;
            if 0 < DAT_00aa735a {
                let mut local_4: i32 = DAT_00aa735a;
                let mut p_var5: *mut u8 = DAT_00aa734a as *mut u8;
                loop {
                    let mut u_var4 = u_var2;
                    let mut p_var3 = p_var5;
                    let mut p_var10 = p_var8;
                    if 0 < (u_var2 as i32) {
                        loop {
                            p_var10
                                .cast::<u16>()
                                .write_unaligned(p_var3.cast::<u16>().read_unaligned());
                            u_var4 = u_var4.wrapping_sub(1);
                            p_var3 = p_var3.wrapping_byte_add(2);
                            p_var10 = p_var10.wrapping_byte_add(2);
                            if u_var4 == 0 {
                                break;
                            }
                        }
                    }
                    p_var5 = p_var5.wrapping_byte_offset(i_var1.wrapping_mul(2) as isize);
                    p_var8 = p_var8
                        .wrapping_byte_offset((DAT_00aa7b13 & (0xfffffffeu32 as i32)) as isize);
                    local_4 = local_4.wrapping_sub(1);
                    if local_4 == 0 {
                        break;
                    }
                }
            }
        }
        FUN_004d4970();
    }
}
