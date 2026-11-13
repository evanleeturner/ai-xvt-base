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
    fn FUN_0048f510();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044a600(param_1: i32, param_2: i32) -> u32 {
    let mut param_2 = param_2;
    if param_2 < 0 {
        return 0;
    }
    // SAFETY: all pointers and globals belong to the game; reads and calls
    // happen in the original's order on its own structures.
    unsafe {
        let idx = *(&raw mut DAT_006da850)
            .cast::<i16>()
            .offset(param_1 as isize);
        let key = ((param_1 as u32) >> 0x10) | (idx as i32 as u32);
        let pi_var3 = FUN_0048f4f0(key as i32) as usize as *mut i32;

        if (pi_var3.read_unaligned() as u32 as usize) != (pi_var3 as usize) {
            FUN_00485fa0(pi_var3);
        }

        let arr = pi_var3.byte_add(0xA).cast::<i32>().read_unaligned() as usize as *const i32;
        if (arr.read_unaligned() as usize as *const i32)
            .byte_add(4)
            .read_unaligned()
            == 0x14
        {
            param_2 = param_2.wrapping_add(1);
        }

        let cnt = pi_var3.byte_add(6).read_unaligned();
        if cnt <= param_2 {
            param_2 = cnt.wrapping_sub(1);
        }

        let i_var1 = arr.offset(param_2 as isize).read_unaligned();
        let i_var4: i32;
        'lab: {
            if i_var1 != 0 {
                let p1 = i_var1 as usize as *const u8;
                if p1.add(4).cast::<i32>().read_unaligned() == 0x19 {
                    i_var4 = p1.add(0x14).cast::<i32>().read_unaligned();
                    break 'lab;
                }
                let mut i_var6: i32 = 0;
                let mut i_var5: i32 = 0;
                if 0 < p1.add(8).cast::<i32>().read_unaligned() {
                    loop {
                        let base = p1.add(0xC).cast::<i32>().read_unaligned();
                        let v = (base.wrapping_add(i_var6) as usize as *const i32).read_unaligned();
                        if v != 0 {
                            let r = FUN_00449670(v, pi_var3);
                            if r != 0 {
                                i_var4 = r;
                                break 'lab;
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
            i_var4 = 0;
        }
        let u_var2 = (i_var4 as usize as *const u8)
            .add(4)
            .cast::<u32>()
            .read_unaligned();
        FUN_0048f510();
        u_var2 & 2
    }
}
