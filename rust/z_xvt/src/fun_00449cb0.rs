// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006da850: [i16; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f4f0(param_1: i32) -> i32;
    fn FUN_00485fa0(param_1: *mut i32);
    fn FUN_00449670(param_1: i32, param_2: *mut i32) -> i32;
    fn __ftol() -> i64;
    fn FUN_0048f510(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00449cb0(param_1: i32, param_2: i32) -> i32 {
    let mut param_2 = param_2;
    if param_2 < 0 {
        return 0;
    }
    // SAFETY: all globals and callees are the game's own; the pointers read
    // here are the game's object structures, accessed at the original's
    // offsets.
    unsafe {
        let idx = (&raw mut DAT_006da850)
            .cast::<i16>()
            .offset(param_1 as isize)
            .read_unaligned();
        let arg = ((param_1 as u32) & 0xFFFF_0000) | (idx as u16 as u32);
        let pi_var2 = FUN_0048f4f0(arg as i32) as usize as *mut i32;
        if (pi_var2.read_unaligned() as usize as *mut i32) != pi_var2 {
            FUN_00485fa0(pi_var2);
        }

        let ptr_a = pi_var2.byte_add(10).read_unaligned() as usize as *mut i32;
        if ptr_a.byte_add(4).read_unaligned() == 0x14 {
            param_2 = param_2.wrapping_add(1);
        }

        let field_6 = pi_var2.byte_add(6).read_unaligned();
        if field_6 <= param_2 {
            param_2 = field_6.wrapping_sub(1);
        }

        let i_var1 = ptr_a.offset(param_2 as isize).read_unaligned();
        if i_var1 != 0 && (i_var1 as usize as *const i32).byte_add(4).read_unaligned() != 0x19 {
            let base = i_var1 as usize as *const i32;
            let mut i_var5: i32 = 0;
            let mut i_var4: i32 = 0;
            if 0 < base.byte_add(8).read_unaligned() {
                loop {
                    let list = base.byte_add(0xc).read_unaligned() as usize as *const i32;
                    let i_var3 = list.byte_add(i_var5 as usize).read_unaligned();
                    if i_var3 != 0 {
                        let r = FUN_00449670(i_var3, pi_var2);
                        if r != 0 {
                            break;
                        }
                    }
                    i_var5 = i_var5.wrapping_add(4);
                    i_var4 = i_var4.wrapping_add(1);
                    if !(i_var4 < base.byte_add(8).read_unaligned()) {
                        break;
                    }
                }
            }
        }
        let l_var6 = __ftol();
        FUN_0048f510(i_var1);
        l_var6 as i32
    }
}
