// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525568: i32;
    static mut DAT_006db730: i32;
    static mut DAT_0071b740: i8;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071f71d: [i8; 0];
    static mut DAT_0071f71e: [i8; 0];
    static mut DAT_0071f716: [i8; 0];
    static mut DAT_0071e528: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049b0d0(param_1: i32, param_2: i32, param_3: *mut core::ffi::c_void) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0049adc0(param_1: i32) {
    // SAFETY: all globals and the callee are the game's own memory; the
    // pointers are walked and dereferenced exactly as the original does.
    unsafe {
        let mut i_var4: i32 = 0;
        if DAT_00525568 == 0 {
            // local_8 and local_4 are adjacent on the stack.
            let mut loc: [i32; 2] = [0, 0];
            let mut i_var6: i32 = 0;
            let mut pi_var5: *mut i32 = &raw mut DAT_006db730;
            let mut pc_var7: *mut i8 = &raw mut DAT_0071b740;
            let end = &raw mut DAT_0071e528 as usize;
            loop {
                if *pc_var7 != 0 && i_var6 != DAT_0071ecf4 && *pi_var5 != 0 {
                    let i_var2 = (*pi_var5).wrapping_add(i_var4);
                    let i_var1 = i_var2.wrapping_mul(0x12) as isize;
                    let b0 = (&raw mut DAT_0071f71d as *mut i8).offset(i_var1).read();
                    let b1 = (&raw mut DAT_0071f71e as *mut i8).offset(i_var1).read();
                    let base = loc.as_mut_ptr().cast::<u8>();
                    base.write(b0 as u8);
                    base.add(5).write(b1 as u8);
                    let val_f716: i32 = (&raw mut DAT_0071f716 as *mut i8)
                        .offset(i_var1)
                        .cast::<i32>()
                        .read_unaligned();
                    let pi_var3 = FUN_0049b0d0(
                        i_var6,
                        val_f716.wrapping_add(param_1),
                        loc.as_mut_ptr().cast::<core::ffi::c_void>(),
                    );
                    if !pi_var3.is_null() {
                        pi_var3.write(0);
                        pi_var3.add(1).write(2);
                    }
                }
                i_var4 = i_var4.wrapping_add(0x1c2);
                pi_var5 = pi_var5.wrapping_add(1);
                pc_var7 = pc_var7.wrapping_add(0x5bd);
                i_var6 = i_var6.wrapping_add(1);
                if !((pc_var7 as usize) < end) {
                    break;
                }
            }
        }
    }
}
