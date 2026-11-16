// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0075d846: [u8; 0x400];
    static mut DAT_00759ad0: [u32; 8];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004db2f0(param_1: *mut i8, param_2: i32) -> i32;
    fn FUN_004db2c0(param_1: i32) -> u32;
    fn FUN_004d2190(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: u32,
        param_5: i32,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004da920(
    param_1: i32,
    param_2: *mut i8,
    param_3: *mut i32,
    param_4: u32,
) -> u32 {
    if param_2.is_null() {
        return 0;
    }
    if !(-1 < param_1 && param_1 < 0x100) {
        return 0;
    }
    // SAFETY: the globals and callees are the game's own; param_2 and param_3
    // point at the caller's buffers, read where the original reads them.
    unsafe {
        let pi_var3: *mut i32 = (&raw mut DAT_0075d846)
            .cast::<u8>()
            .add((param_1 as usize) * 4)
            .cast::<*mut i32>()
            .read_unaligned();
        if pi_var3.is_null() {
            return 0;
        }
        let pb_base = pi_var3.cast::<u8>();
        let i_var6 = FUN_004db2f0(param_2, param_1);
        let p0 = param_3.read_unaligned();
        let p1 = param_3.byte_add(4).read_unaligned();
        let p2 = param_3.byte_add(8).read_unaligned();
        let i_var4 = param_3.byte_add(12).read_unaligned();
        let mut i_var8: i32 = (p2.wrapping_sub(p0) >> 1)
            .wrapping_sub(i_var6 >> 1)
            .wrapping_add(p0);
        let i_var6 = p1;
        let u_var5 = FUN_004db2c0(param_1);
        let mut i_var9: usize = 0;
        let mut c_var1 = param_2.read();
        let mut local_418: u32 = 0;
        let mut local_420: u32 = param_4;
        let mut local_414: [u32; 5] = [0; 5];
        while c_var1 != 0 && i_var8 < 0x280 {
            let b_var2 = param_2.add(i_var9).read() as u8;
            if b_var2 == 1 {
                local_420 = param_4;
            } else if b_var2 < 2 || 7 < b_var2 {
                let u_var7 = b_var2 as u32;
                local_414[0] = pb_base.add(u_var7 as usize + 0x504).read() as u32;
                local_414[1] = pb_base.add(u_var7 as usize + 0x404).read() as u32;
                local_414[3] = 0;
                local_414[2] = 1;
                let elem = pi_var3
                    .byte_add((u_var7.wrapping_add(1) as usize).wrapping_mul(4))
                    .read_unaligned() as u32;
                local_414[4] = elem.wrapping_add(pi_var3.read_unaligned() as u32);
                let third = ((i_var4.wrapping_sub(i_var6) >> 1) as u32)
                    .wrapping_add(i_var6 as u32)
                    .wrapping_sub(u_var5 >> 1) as i32;
                let u_var7 = FUN_004d2190(
                    local_414.as_mut_ptr().cast::<i32>(),
                    i_var8,
                    third,
                    local_420,
                    1,
                );
                local_418 |= u_var7;
                let ch = param_2.add(i_var9).read() as u8;
                i_var8 = i_var8
                    .wrapping_add(pb_base.add(ch as usize + 0x504).read() as i32)
                    .wrapping_add(pb_base.add(0x609).read() as i32);
            } else {
                local_420 = (&raw mut DAT_00759ad0)
                    .cast::<u32>()
                    .add(b_var2 as usize)
                    .read();
            }
            c_var1 = param_2.add(i_var9 + 1).read();
            i_var9 += 1;
        }
        local_418
    }
}
