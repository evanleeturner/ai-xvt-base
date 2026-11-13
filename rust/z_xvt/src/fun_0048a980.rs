// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0048a980(param_1: i32, param_2: i32, param_3: i32) -> i32 {
    if param_1 == 0 {
        return 0;
    }
    if param_2 == 0 {
        return 0;
    }
    // SAFETY: the parameters are addresses of the game's own structures,
    // read at the offsets the original uses.
    unsafe {
        let rd = |addr: i32| -> i32 { (addr as usize as *const i32).read_unaligned() };
        if rd(param_2.wrapping_add(4)) != rd(param_1.wrapping_add(4)) {
            return 0;
        }
        if rd(param_1.wrapping_add(4)) == 0x14 {
            let p1 = rd(param_1.wrapping_add(0x14));
            let mut i2 = rd(p1.wrapping_add(0x14)).wrapping_mul(rd(p1.wrapping_add(0x10)));
            if rd(p1.wrapping_add(8)) == i2 {
                i2 = rd(p1.wrapping_add(0xc));
            }
            let i2 = p1.wrapping_add(i2).wrapping_add(0x18);
            if (rd(p1) == i2 || rd(p1.wrapping_add(4)) != 0) && i2 == param_3 {
                return param_2;
            }
        }
        if rd(param_2.wrapping_add(8)) != rd(param_1.wrapping_add(8)) {
            return 0;
        }
        let mut i4: i32 = 0;
        let mut i2: i32 = 0;
        if 0 < rd(param_1.wrapping_add(8)) {
            loop {
                let a = rd(rd(param_1.wrapping_add(0xc)).wrapping_add(i4));
                let b = rd(rd(param_2.wrapping_add(0xc)).wrapping_add(i4));
                let i3 = FUN_0048a980(a, b, param_3);
                if i3 != 0 {
                    return i3;
                }
                i4 = i4.wrapping_add(4);
                i2 = i2.wrapping_add(1);
                if !(i2 < rd(param_1.wrapping_add(8))) {
                    break;
                }
            }
        }
        0
    }
}
