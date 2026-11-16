// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005250d8: i32;
    static mut DAT_005373c8: i32;
    static mut DAT_005250dc: i32;
    static mut DAT_0052554c: i32;
    static mut DAT_00525558: i32;
    static mut DAT_0071f2ec: i32;
    static mut DAT_0071b740: i8;
    static mut DAT_0071e528: i8;
    static mut DAT_005250e0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049b200(param_1: i32) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042d950(param_1: i32) -> i32 {
    // SAFETY: the globals and the callee are the game's own; the scanned
    // pointer walks the game's table and is only read where the original reads.
    unsafe {
        if DAT_005250d8 != 0 {
            return 0;
        }
        if DAT_005373c8 != 0 {
            return 0;
        }
        if DAT_005250dc == 0 {
            DAT_005250dc = (DAT_0071f2ec >> 3)
                .wrapping_add(param_1)
                .wrapping_add(DAT_0052554c);
        }
        let i_var2 = param_1
            .wrapping_add(DAT_0052554c)
            .wrapping_sub(DAT_005250dc);
        if i_var2 < DAT_00525558 {
            return 0;
        }
        if DAT_00525558.wrapping_mul(5) < i_var2 {
            DAT_005250dc = DAT_005250dc.wrapping_add(DAT_00525558);
            return 1;
        }
        let mut i_var2: i32 = 0x7fffffff;
        let mut i_var4: i32 = 0;
        let mut pc_var3: *mut i8 = &raw mut DAT_0071b740;
        let end: *mut i8 = &raw mut DAT_0071e528;
        loop {
            if pc_var3.read() != 0 {
                let pi_var1 = FUN_0049b200(i_var4);
                if pi_var1.is_null() {
                    i_var2 = 0;
                    break;
                }
                let v = pi_var1.byte_add(8).read_unaligned();
                if v < i_var2 {
                    i_var2 = v;
                }
            }
            pc_var3 = pc_var3.wrapping_byte_add(0x5bd);
            i_var4 = i_var4.wrapping_add(1);
            if (pc_var3 as usize) >= (end as usize) {
                break;
            }
        }
        if i_var2 <= DAT_00525558.wrapping_add(DAT_005250e0) {
            return 0;
        }
        DAT_005250dc = DAT_005250dc.wrapping_add(DAT_00525558);
        1
    }
}
