// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006da850: [i32; 0];
    static mut in_EAX: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f4f0(param_1: i32) -> *mut core::ffi::c_void;
    fn FUN_00485fa0(param_1: *mut core::ffi::c_void);
    fn FUN_00449140(param_1: i32) -> i32;
    fn FUN_0048f510();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00449850(param_1: i32, mut param_2: i32) -> u32 {
    // SAFETY: all pointers are the game's own structures, read at the
    // offsets the original uses; globals and callees belong to the game.
    unsafe {
        let arr = (&raw mut DAT_006da850).cast::<i32>();
        let e = arr.offset(param_1 as isize).read() as u32;
        let pi_var3 = FUN_0048f4f0(((in_EAX & 0xFFFF0000) | (e & 0xFFFF)) as i32) as *mut i32;
        if pi_var3.read_unaligned() != pi_var3 as i32 {
            FUN_00485fa0(pi_var3 as *mut core::ffi::c_void);
        }
        let list = pi_var3.byte_add(10).read_unaligned() as *const i32;
        let first = list.read_unaligned() as *const i32;
        if first.byte_add(4).read_unaligned() == 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        let val_at_6 = pi_var3.byte_add(6).read_unaligned();
        if val_at_6 <= param_2 {
            param_2 = val_at_6.wrapping_sub(1);
        }
        let i_var1 = list.offset(param_2 as isize).read_unaligned();
        let i_var4: i32 = 'lab: {
            if i_var1 != 0 {
                let p1 = i_var1 as *const u8;
                if p1.add(4).cast::<i32>().read_unaligned() == 3 {
                    break 'lab i_var1;
                }
                let mut i_var6: i32 = 0;
                let mut i_var5: i32 = 0;
                if 0 < p1.add(8).cast::<i32>().read_unaligned() {
                    loop {
                        let base = p1.add(0xc).cast::<i32>().read_unaligned();
                        let v = ((base.wrapping_add(i_var6)) as *const i32).read_unaligned();
                        if v != 0 {
                            let r = FUN_00449140(v);
                            if r != 0 {
                                break 'lab r;
                            }
                        }
                        i_var6 = i_var6.wrapping_add(4);
                        i_var5 = i_var5.wrapping_add(1);
                        if !(i_var5 < p1.add(8).cast::<i32>().read_unaligned()) {
                            break;
                        }
                    }
                }
            }
            0
        };
        let u_var2 = ((i_var4.wrapping_add(0x10)) as *const u32).read_unaligned();
        FUN_0048f510();
        u_var2
    }
}
