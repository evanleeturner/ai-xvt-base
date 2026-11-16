// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dcad0() -> u32;
    fn FUN_004dca90(param_1: u8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004bd770() -> u32 {
    // SAFETY: the callees are the game's own key-state functions; they take
    // and return plain integers and touch no memory of ours.
    unsafe {
        let u_var2 = FUN_004dcad0();
        if (u_var2 as u8) != 0 {
            return u_var2;
        }
        if FUN_004dca90(0x12) != 0 {
            let mut i: i32 = 0;
            loop {
                if FUN_004dca90(i.wrapping_add(0x41) as u8) != 0 {
                    return (i as u32).wrapping_add(0x80);
                }
                i = i.wrapping_add(1);
                if i >= 0x1a {
                    break;
                }
            }
            i = 0;
            loop {
                if FUN_004dca90(i.wrapping_add(0x30) as u8) != 0 {
                    return (i as u32).wrapping_sub(0x66);
                }
                i = i.wrapping_add(1);
                if i >= 10 {
                    break;
                }
            }
            return 0;
        }
        if FUN_004dca90(0x10) != 0 {
            let mut i: i32 = 0;
            loop {
                if FUN_004dca90(i.wrapping_add(0x70) as u8) != 0 {
                    return (i as u32).wrapping_sub(0x31);
                }
                i = i.wrapping_add(1);
                if i >= 0xc {
                    break;
                }
            }
            return 0;
        }
        let mut i: i32 = 0;
        loop {
            if FUN_004dca90(i.wrapping_add(0x70) as u8) != 0 {
                return (i as u32).wrapping_sub(0x3d);
            }
            i = i.wrapping_add(1);
            if i >= 0xc {
                break;
            }
        }
        const TABLE: [(u8, u32); 33] = [
            (0x25, 0xa4),
            (0x27, 0xa5),
            (0x26, 0xa6),
            (0x28, 0xa7),
            (0x2d, 0xa8),
            (0x2e, 0xa9),
            (0x24, 0xaa),
            (0x23, 0xab),
            (0x21, 0xac),
            (0x22, 0xad),
            (0x2c, 0xae),
            (0x91, 0xaf),
            (0x14, 0xb1),
            (0x60, 0xb2),
            (0x61, 0xb3),
            (0x62, 0xb4),
            (99, 0xb5),
            (100, 0xb6),
            (0x65, 0xb7),
            (0x66, 0xb8),
            (0x67, 0xb9),
            (0x68, 0xba),
            (0x69, 0xbb),
            (0x90, 0xbc),
            (0x6a, 0xbe),
            (0x6b, 0xc0),
            (0x6d, 0xbf),
            (0x6e, 0xc2),
            (0x6f, 0xbd),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
        ];
        let mut k = 0;
        while k < 29 {
            let (key, ret) = TABLE[k];
            if FUN_004dca90(key) != 0 {
                return ret;
            }
            k += 1;
        }
        0
    }
}
