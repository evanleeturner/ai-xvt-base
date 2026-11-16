// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aabd11: u8;
    static mut DAT_00aa7f9b: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004b6780(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: u32,
        param_5: i32,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004db4e0(
    param_1: i32,
    param_2: *mut i8,
    param_3: i32,
    param_4: i32,
    param_5: u32,
) -> u32 {
    let mut param_3 = param_3;
    if param_2.is_null() {
        return 0;
    }
    if !(-1 < param_1 && param_1 < 0x100) {
        return 0;
    }
    // SAFETY: the tables are the game's own memory; param_2 points at a
    // NUL-terminated string, and the glyph table pointer read from the
    // game's table is used as the original does.
    unsafe {
        let pi_var3: *mut i32 = (&raw mut DAT_00aabd11)
            .byte_add((param_1 as usize).wrapping_mul(4))
            .cast::<*mut i32>()
            .read_unaligned();
        if pi_var3.is_null() {
            return 0;
        }
        let mut i_var6: usize = 0;
        let mut local_418: u32 = 0;
        let mut c_var1: i8 = *param_2;
        let mut u_var4: u32 = param_5;
        while c_var1 != 0 && param_3 < 0x280 {
            let b_var2: u8 = param_2.add(i_var6).read() as u8;
            let mut u_var5: u32 = param_5;
            if b_var2 != 1 {
                if b_var2 < 2 || 7 < b_var2 {
                    u_var5 = b_var2 as u32;
                    let mut local_414: [u32; 5] = [0; 5];
                    local_414[0] = pi_var3
                        .byte_add(u_var5 as usize + 0x504)
                        .cast::<u8>()
                        .read() as u32;
                    local_414[1] = pi_var3
                        .byte_add(u_var5 as usize + 0x404)
                        .cast::<u8>()
                        .read() as u32;
                    local_414[3] = 0;
                    local_414[2] = 1;
                    let a = pi_var3
                        .byte_add((u_var5 as usize).wrapping_add(1).wrapping_mul(4))
                        .read_unaligned() as u32;
                    let b = pi_var3.read_unaligned() as u32;
                    local_414[4] = a.wrapping_add(b);
                    u_var5 = FUN_004b6780(
                        local_414.as_mut_ptr().cast::<i32>(),
                        param_3,
                        param_4,
                        u_var4,
                        1,
                    );
                    local_418 |= u_var5;
                    let w1 = pi_var3
                        .byte_add(b_var2 as usize + 0x504)
                        .cast::<u8>()
                        .read() as u32;
                    let w2 = pi_var3.byte_add(0x609).cast::<u8>().read() as u32;
                    param_3 = (param_3 as u32).wrapping_add(w1.wrapping_add(w2)) as i32;
                    u_var5 = u_var4;
                } else {
                    u_var5 = (&raw mut DAT_00aa7f9b)
                        .byte_add(b_var2 as usize * 4)
                        .cast::<u32>()
                        .read_unaligned();
                }
            }
            i_var6 += 1;
            c_var1 = param_2.add(i_var6).read();
            u_var4 = u_var5;
        }
        local_418
    }
}
