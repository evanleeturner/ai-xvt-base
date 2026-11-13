// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_009e9670: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00405820(param_1: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00481ea0(param_1: u32, param_2: u32, param_3: i32) {
    // SAFETY: all addresses are the game's own memory, read and written
    // in the original's order; the callee and globals belong to the game.
    unsafe {
        let pu_var2 = (&raw mut DAT_009e9670)
            .cast::<u8>()
            .byte_offset(param_3.wrapping_mul(0x5bd) as isize)
            .cast::<u32>();
        let i_var1 = ((param_2 & 0xffff) as i32).wrapping_mul(4);
        let p_ref = pu_var2
            .read_unaligned()
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(DAT_009a1fe8 as u32) as usize as *const i32;
        let p_obj = (p_ref.read_unaligned() as u32).wrapping_add(0xa9) as usize as *const i32;
        let p_arr = (p_obj.read_unaligned() as u32).wrapping_add(0x211) as usize as *mut u8;

        let p_a = p_arr.byte_offset(i_var1 as isize).cast::<i32>();
        if 0 < p_a.read_unaligned() {
            let i_var7 = FUN_00405820(pu_var2.read_unaligned());
            let _ = p_obj.read_unaligned();
            let idx1 = ((param_1 & 0xffff) as i32).wrapping_mul(4);
            let pi_var3 = p_arr.byte_offset(idx1 as isize).cast::<i32>();
            let i_var5 = pi_var3.read_unaligned();
            let s_var6 = (i_var7 as i16).wrapping_sub(i_var5 as i16);

            if 0 < s_var6 {
                let i_var7 = s_var6 as i32;
                let i_var4 = p_a.read_unaligned();
                if i_var7 < i_var4 {
                    pi_var3.write_unaligned(i_var5.wrapping_add(i_var7));
                    let cur = p_a.read_unaligned();
                    p_a.write_unaligned(cur.wrapping_sub(i_var7));
                    return;
                }
                pi_var3.write_unaligned(i_var5.wrapping_add(i_var4));
                p_a.write_unaligned(0);
            }
        }
    }
}
