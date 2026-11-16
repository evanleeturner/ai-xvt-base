// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_009fd39c: i32;
    static mut DAT_009e9a5b: i32;
    static mut DAT_009fd3a0: i32;
    static mut DAT_009e9a5f: i32;
    static mut DAT_009fd398: i32;
    static mut DAT_009e9a63: i32;
    static mut DAT_009a8e34: i32;
    static mut DAT_009a8e2c: i32;
    static mut DAT_009a8e30: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00427500(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_00427420(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_00427490(param_1: i32, param_2: i32, param_3: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00436470(param_1: i32, param_2: u32, param_3: i32) -> i32 {
    let mut param_2 = param_2;
    // SAFETY: globals and callees are the game's own memory; the record
    // pointers are read at the same offsets the original uses (int-sized
    // pointer arithmetic on the DAT_ bases, as in the C).
    unsafe {
        let i_var2 = param_1.wrapping_mul(0x23).wrapping_add(DAT_009a1fe8);
        let idx = param_3.wrapping_mul(0x5bd) as isize;
        let a = (&raw mut DAT_009e9a5b)
            .wrapping_offset(idx)
            .read_unaligned();
        DAT_009fd39c = ((i_var2.wrapping_add(4)) as *const i32)
            .read_unaligned()
            .wrapping_sub(a);
        let b = (&raw mut DAT_009e9a5f)
            .wrapping_offset(idx)
            .read_unaligned();
        DAT_009fd3a0 = ((i_var2.wrapping_add(8)) as *const i32)
            .read_unaligned()
            .wrapping_sub(b);
        let c = (&raw mut DAT_009e9a63)
            .wrapping_offset(idx)
            .read_unaligned();
        DAT_009fd398 = ((i_var2.wrapping_add(0xc)) as *const i32)
            .read_unaligned()
            .wrapping_sub(c);
        DAT_009a8e34 = FUN_00427500(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
        let i_var2 = (DAT_009a8e34 as u32).wrapping_add(param_2) as i32;
        if i_var2 < 0 {
            return 0;
        }
        if param_2 < (i_var2 >> 4) as u32 {
            param_2 = (i_var2 >> 4) as u32;
        }
        DAT_009a8e2c = FUN_00427420(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
        let mut i_var1 = DAT_009a8e2c;
        if DAT_009a8e2c < 0 {
            i_var1 = DAT_009a8e2c.wrapping_neg();
        }
        if i_var2 < (i_var1 as u32).wrapping_sub(param_2) as i32 {
            return 0;
        }
        i_var1 = FUN_00427490(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
        DAT_009a8e30 = i_var1;
        if i_var1 < 0 {
            i_var1 = i_var1.wrapping_neg();
        }
        ((i_var1 as u32).wrapping_sub(param_2) as i32 <= i_var2) as i32
    }
}
