// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0075d846: [u8; 1];
    static mut DAT_00759ad0: [u32; 256];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004db2f0(param_1: *mut i8, param_2: i32) -> i32;
    fn FUN_004db2c0(param_1: i32) -> i32;
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
pub unsafe extern "C" fn FUN_004daac0(
    param_1: i32,
    param_2: *mut i8,
    param_3: *mut i32,
    param_4: i32,
    param_5: i32,
    param_6: u32,
) -> u32 {
    if param_2.is_null() {
        return 0;
    }
    if !(-1 < param_1 && param_1 < 0x100) {
        return 0;
    }
    // SAFETY: the globals and callees are the game's own; the pointers passed
    // in point at the caller's memory, read and written where the original does.
    unsafe {
        let pi_var3: *mut i32 = (&raw mut DAT_0075d846)
            .cast::<u8>()
            .byte_add((param_1 as u32).wrapping_mul(4) as usize)
            .cast::<*mut i32>()
            .read_unaligned();
        if pi_var3.is_null() {
            return 0;
        }
        let mut i_var6 = FUN_004db2f0(param_2, param_1);
        let p0 = param_3.read_unaligned();
        if param_4 == 0 {
            i_var6 = p0;
        } else {
            let p2 = param_3.add(2).read_unaligned();
            i_var6 = (p2.wrapping_sub(p0) >> 1)
                .wrapping_sub(i_var6 >> 1)
                .wrapping_add(p0);
        }
        let local_420: i32;
        if param_5 == 0 {
            local_420 = param_3.add(1).read_unaligned();
        } else {
            let i_var8 = param_3.add(1).read_unaligned();
            let i_var4 = param_3.add(3).read_unaligned();
            let i_var9 = FUN_004db2c0(param_1);
            local_420 = (i_var4.wrapping_sub(i_var8) >> 1)
                .wrapping_add(i_var8)
                .wrapping_sub(i_var9 >> 1);
        }
        let mut i_var8: usize = 0;
        let mut c_var1: i8 = param_2.read();
        let mut local_418: u32 = 0;
        let mut local_41c: u32 = param_6;
        let base = pi_var3.cast::<u8>();
        while c_var1 != 0 && i_var6 < 0x280 {
            let b_var2: u8 = param_2.add(i_var8).read() as u8;
            if b_var2 == 1 {
                local_41c = param_6;
            } else if b_var2 < 2 || 7 < b_var2 {
                let u = b_var2 as usize;
                let mut local_414 = [0i32; 5];
                local_414[0] = base.add(u + 0x504).read() as i32;
                local_414[1] = base.add(u + 0x404).read() as i32;
                local_414[3] = 0;
                local_414[2] = 1;
                let a = pi_var3.add(u + 1).read_unaligned() as u32;
                let b = pi_var3.read_unaligned() as u32;
                local_414[4] = a.wrapping_add(b) as i32;
                let u_var7 = FUN_004d2190(local_414.as_mut_ptr(), i_var6, local_420, local_41c, 1);
                local_418 |= u_var7;
                let ch = param_2.add(i_var8).read() as u8 as usize;
                let w1 = base.add(ch + 0x504).read() as u32;
                let w2 = base.add(0x609).read() as u32;
                i_var6 = (i_var6 as u32).wrapping_add(w1).wrapping_add(w2) as i32;
            } else {
                local_41c = (&raw mut DAT_00759ad0)
                    .cast::<u32>()
                    .add(b_var2 as usize)
                    .read();
            }
            c_var1 = param_2.add(i_var8 + 1).read();
            i_var8 += 1;
        }
        local_418
    }
}
