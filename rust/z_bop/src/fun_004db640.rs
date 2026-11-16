// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aabd11: [u8; 1];
    static mut DAT_00aa7f9b: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dbfa0(param_1: *mut i8, param_2: i32) -> i32;
    fn FUN_004dbf70(param_1: i32) -> u32;
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
pub unsafe extern "C" fn FUN_004db640(
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
    // SAFETY: the globals, callees and pointers belong to the game; reads
    // and writes happen in the original's order.
    unsafe {
        let tbl = (&raw mut DAT_00aabd11).cast::<u8>();
        let pi_var3: *mut i32 = tbl
            .add((param_1 as u32).wrapping_mul(4) as usize)
            .cast::<u32>()
            .read_unaligned() as usize as *mut i32;
        if pi_var3.is_null() {
            return 0;
        }
        let i_var6 = FUN_004dbfa0(param_2, param_1);
        let i_var4 = param_3.add(3).read_unaligned();
        let p2 = param_3.add(2).read_unaligned() as u32;
        let p0 = param_3.read_unaligned() as u32;
        let half = ((p2.wrapping_sub(p0) as i32) >> 1) as u32;
        let mut i_var8: i32 = half.wrapping_sub((i_var6 >> 1) as u32).wrapping_add(p0) as i32;
        let i_var6 = param_3.add(1).read_unaligned();
        let mut i_var9: i32 = 0;
        let u_var5 = FUN_004dbf70(param_1);
        let mut local_418: u32 = 0;
        let mut c_var1 = param_2.read();
        let mut local_420: u32 = param_4;
        let base = pi_var3.cast::<u8>();
        while c_var1 != 0 && i_var8 < 0x280 {
            let b_var2 = (param_2.offset(i_var9 as isize).read()) as u8;
            if b_var2 == 1 {
                local_420 = param_4;
            } else if b_var2 < 2 || 7 < b_var2 {
                let u_var7 = b_var2 as u32;
                let mut local_414: [u32; 5] = [0; 5];
                local_414[0] = base.add(u_var7.wrapping_add(0x504) as usize).read() as u32;
                local_414[1] = base.add(u_var7.wrapping_add(0x404) as usize).read() as u32;
                local_414[3] = 0;
                local_414[2] = 1;
                local_414[4] = (pi_var3
                    .add(u_var7.wrapping_add(1) as usize)
                    .read_unaligned() as u32)
                    .wrapping_add(pi_var3.read_unaligned() as u32);
                let arg3 = (((i_var4 as u32).wrapping_sub(i_var6 as u32) as i32) >> 1) as u32;
                let arg3 = arg3.wrapping_add(i_var6 as u32).wrapping_sub(u_var5 >> 1) as i32;
                let r = FUN_004b6780(
                    local_414.as_mut_ptr().cast::<i32>(),
                    i_var8,
                    arg3,
                    local_420,
                    1,
                );
                local_418 |= r;
                let ch = param_2.offset(i_var9 as isize).read() as u8 as u32;
                let a = base.add(ch.wrapping_add(0x504) as usize).read() as u32;
                let b = base.add(0x609).read() as u32;
                i_var8 = (i_var8 as u32).wrapping_add(a).wrapping_add(b) as i32;
            } else {
                local_420 = (&raw mut DAT_00aa7f9b)
                    .cast::<u8>()
                    .add((b_var2 as u32).wrapping_mul(4) as usize)
                    .cast::<u32>()
                    .read_unaligned();
            }
            i_var9 = i_var9.wrapping_add(1);
            c_var1 = param_2.offset(i_var9 as isize).read();
        }
        local_418
    }
}
