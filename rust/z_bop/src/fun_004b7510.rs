// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00664c88: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004b7570(param_1: *mut u8, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b7510(param_1: *mut u8, param_2: i32, param_3: i32) {
    // SAFETY: all pointers are the game's own memory; param_3 points at a
    // structure with two ints at offsets 4 and 8, DAT_00664c88 is a 0x80-int
    // table, and param_1 points at the caller's pixel bytes.
    unsafe {
        let i_var1 = (param_3 as u32 as usize as *const u8)
            .byte_add(4)
            .cast::<i32>()
            .read_unaligned();
        let mut i_var4 = (param_3 as u32 as usize as *const u8)
            .byte_add(8)
            .cast::<i32>()
            .read_unaligned();
        let mut p = &raw mut DAT_00664c88;
        let mut i_var3: i32 = 0x80;
        while i_var3 != 0 {
            p.write_unaligned(0x1000100);
            p = p.add(1);
            i_var3 = i_var3.wrapping_sub(1);
        }
        let mut param_1 = param_1;
        if i_var4 > 0 {
            loop {
                i_var3 = i_var1;
                let mut pb = param_1;
                if i_var1 > 0 {
                    loop {
                        param_1 = pb.add(1);
                        let b = pb.read() as u32;
                        let addr = param_2.wrapping_add(b.wrapping_mul(4) as i32);
                        let u_var2 = FUN_004b7570(addr as u32 as usize as *mut u8, b as i32);
                        pb.write(u_var2 as u8);
                        i_var3 = i_var3.wrapping_sub(1);
                        pb = param_1;
                        if i_var3 == 0 {
                            break;
                        }
                    }
                }
                i_var4 = i_var4.wrapping_sub(1);
                if i_var4 == 0 {
                    break;
                }
            }
        }
    }
}
