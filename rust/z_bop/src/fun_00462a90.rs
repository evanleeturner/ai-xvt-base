// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a7ba4: u32;
    static mut DAT_00a08300: u16;
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_009a8da0: *mut u8;
    static mut DAT_00a07ce8: u32;
    static mut DAT_009d767c: u32;
    static mut DAT_009d6820: *mut u8;
    static mut DAT_009d1308: u32;
    static mut DAT_009eca28: u32;
    static mut DAT_00a082c0: u16;
    static mut DAT_00a07bd0: *mut u8;
    static mut DAT_009fd430: u32;
    static mut DAT_00a082d8: u16;
    static mut DAT_009a8e18: *mut u8;
    static mut DAT_009ed228: u32;
    static mut DAT_009d8b70: u32;
    static mut DAT_009a2000: i16;
    static mut DAT_009a20c0: u32;
    static mut DAT_009d6940: u32;
    static mut DAT_009a8d40: u16;
    static mut DAT_009ec5e0: u32;
    static mut DAT_00a08294: i32;
    static mut DAT_009d8c30: u32;
    static mut DAT_009fe800: u32;
    static mut DAT_009a8080: u32;
    static mut DAT_009a8bfc: u32;
    static mut DAT_009fd394: u32;
    static mut DAT_009d8c20: u8;
    static mut DAT_00a0814c: u32;
    static mut DAT_009a7b50: u32;
    static mut _DAT_00a07cec: u32;
    static mut DAT_009a1fe0: u32;
    static mut DAT_009d1144: u32;
    static mut DAT_009fe7dc: u32;
    static mut DAT_009e9640: u32;
    static mut DAT_00a00854: u32;
    static mut DAT_009a7b58: u32;
    static mut DAT_009d1140: u32;
    static mut DAT_009d1310: u32;
    static mut DAT_009ec47c: u32;
    static mut DAT_009a8e24: u32;
    static mut DAT_009cd274: u32;
    static mut DAT_009d1320: u32;
    static mut DAT_009a8068: u32;
    static mut DAT_009a8e40: u32;
    static mut DAT_009a73a4: u32;
    static mut DAT_009d113c: u16;
    static mut DAT_00a08138: u32;
    static mut DAT_009e9680: i8;
    static mut DAT_009ec468: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004630c0(param_1: *mut u32, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00462a90() -> u32 {
    // SAFETY: all globals and the callee are the game's own memory; pointer
    // globals point at the game's tables, read exactly as the original does.
    unsafe {
        let mut local_4: u32 = 0;
        let mut i: i32 = 0;
        if 0 < DAT_009a7ba4 as i32 {
            let mut o6: i32 = 0;
            let mut o3: i32 = (DAT_00a08300 as u32).wrapping_mul(0x23) as i32;
            loop {
                if *DAT_009a1fe8.wrapping_offset(3).wrapping_offset(o3 as isize) != 0 {
                    let v = FUN_004630c0(
                        DAT_009a8da0.wrapping_offset(o6 as isize).cast::<u32>(),
                        0x4c,
                    );
                    local_4 = (local_4 ^ v).rotate_left(1);
                }
                o6 = o6.wrapping_add(0x4c);
                i = i.wrapping_add(1);
                o3 = o3.wrapping_add(0x23);
                if !(i < DAT_009a7ba4 as i32) {
                    break;
                }
            }
        }
        i = 0;
        if 0 < DAT_00a07ce8.wrapping_sub(DAT_009d767c) as i32 {
            let mut o3: i32 = 0;
            let mut o6: i32 = 0;
            loop {
                if *DAT_009a1fe8.wrapping_offset(3).wrapping_offset(o6 as isize) != 0 {
                    let v = FUN_004630c0(
                        DAT_009d6820.wrapping_offset(o3 as isize).cast::<u32>(),
                        0x8b,
                    );
                    local_4 = (local_4 ^ v).rotate_left(1);
                }
                o3 = o3.wrapping_add(0xb1);
                i = i.wrapping_add(1);
                o6 = o6.wrapping_add(0x23);
                if !(i < DAT_00a07ce8.wrapping_sub(DAT_009d767c) as i32) {
                    break;
                }
            }
        }
        i = 0;
        if 0 < DAT_00a07ce8.wrapping_sub(DAT_009d767c) as i32 {
            let mut o3: i32 = 0;
            loop {
                if *DAT_009a1fe8.wrapping_offset(o3 as isize).wrapping_offset(3) != 0 {
                    let v = FUN_004630c0(
                        DAT_009a1fe8.wrapping_offset(o3 as isize).cast::<u32>(),
                        0x1f,
                    );
                    local_4 = (local_4 ^ v).rotate_left(1);
                }
                o3 = o3.wrapping_add(0x23);
                i = i.wrapping_add(1);
                if !(i < DAT_00a07ce8.wrapping_sub(DAT_009d767c) as i32) {
                    break;
                }
            }
        }
        if (DAT_00a07ce8 as i32) < DAT_009d1308.wrapping_add(DAT_00a07ce8) as i32 {
            let mut o4: i32 = DAT_00a07ce8.wrapping_mul(0x23) as i32;
            let mut u1: u32 = DAT_00a07ce8;
            loop {
                if *DAT_009a1fe8.wrapping_offset(o4 as isize).wrapping_offset(3) != 0 {
                    let v = FUN_004630c0(
                        DAT_009a1fe8.wrapping_offset(o4 as isize).cast::<u32>(),
                        0x1f,
                    );
                    local_4 = (local_4 ^ v).rotate_left(1);
                }
                o4 = o4.wrapping_add(0x23);
                u1 = u1.wrapping_add(1);
                if !((u1 as i32) < DAT_009d1308.wrapping_add(DAT_00a07ce8) as i32) {
                    break;
                }
            }
        }
        let mut o3: i32 = 0;
        i = 0;
        if 0 < DAT_009eca28 as i32 {
            let mut o6: i32 = (DAT_00a082c0 as u32).wrapping_mul(0x23) as i32;
            loop {
                if *DAT_009a1fe8.wrapping_offset(3).wrapping_offset(o6 as isize) != 0 {
                    let v = FUN_004630c0(
                        DAT_00a07bd0.wrapping_offset(o3 as isize).cast::<u32>(),
                        0x412,
                    );
                    local_4 = (local_4 ^ v).rotate_left(1);
                }
                o3 = o3.wrapping_add(0x462);
                i = i.wrapping_add(1);
                o6 = o6.wrapping_add(0x23);
                if !(i < DAT_009eca28 as i32) {
                    break;
                }
            }
        }
        o3 = 0;
        i = 0;
        if 0 < DAT_009fd430 as i32 {
            let mut o6: i32 = (DAT_00a082d8 as u32).wrapping_mul(0x23) as i32;
            loop {
                if *DAT_009a1fe8.wrapping_offset(3).wrapping_offset(o6 as isize) != 0 {
                    let v =
                        FUN_004630c0(DAT_009a8e18.wrapping_offset(o3 as isize).cast::<u32>(), 10);
                    local_4 = (local_4 ^ v).rotate_left(1);
                }
                o3 = o3.wrapping_add(10);
                i = i.wrapping_add(1);
                o6 = o6.wrapping_add(0x23);
                if !(i < DAT_009fd430 as i32) {
                    break;
                }
            }
        }
        let mut u1 = FUN_004630c0(&raw mut DAT_009ed228, 8);
        let mut u2 = FUN_004630c0(&raw mut DAT_009d8b70, 8);
        u2 = (local_4 ^ u1).rotate_left(1) ^ u2;
        local_4 = u2.rotate_left(1);
        i = 0;
        if 0 < DAT_009a2000 as i32 {
            let mut p: *mut u32 = &raw mut DAT_009a20c0;
            loop {
                let v = FUN_004630c0(p, 0x126);
                local_4 = (local_4 ^ v).rotate_left(1);
                p = p.byte_add(0x126);
                i = i.wrapping_add(1);
                if !(i < DAT_009a2000 as i32) {
                    break;
                }
            }
        }
        u1 = FUN_004630c0(&raw mut DAT_009d6940, 0xd30);
        u2 = (local_4 ^ u1).rotate_left(1) ^ (DAT_009a8d40 as u32);
        u1 = FUN_004630c0(&raw mut DAT_009ec5e0, 0x16);
        u1 = u2.rotate_left(1) ^ u1;
        u1 = u1.rotate_left(1) ^ (DAT_00a08294 as u32);
        u2 = FUN_004630c0((&raw mut DAT_009a2000).cast::<u32>(), 0xa2);
        u2 = u1.rotate_left(1) ^ u2;
        local_4 = u2.rotate_left(1);
        i = 0;
        if 0 < DAT_009a2000 as i32 {
            let mut p: *mut u32 = &raw mut DAT_009d8c30;
            loop {
                let v = FUN_004630c0(p, 0x562);
                local_4 = (local_4 ^ v).rotate_left(1);
                p = p.byte_add(0x566);
                i = i.wrapping_add(1);
                if !(i < DAT_009a2000 as i32) {
                    break;
                }
            }
        }
        u1 = FUN_004630c0(&raw mut DAT_009fe800, 0x1c80);
        local_4 = (local_4 ^ u1).rotate_left(1);
        let mut p: *mut u32 = &raw mut DAT_009a8080;
        loop {
            let v = FUN_004630c0(p, 0x126);
            local_4 = (local_4 ^ v).rotate_left(1);
            p = p.byte_add(0x126);
            if !((p as usize) < ((&raw mut DAT_009a8bfc) as usize)) {
                break;
            }
        }
        u1 = (local_4 ^ DAT_009fd394).rotate_left(1) ^ (DAT_009d8c20 as u32);
        u1 = u1.rotate_left(1) ^ DAT_009eca28;
        u1 = u1.rotate_left(1) ^ DAT_009a7ba4;
        u1 = u1.rotate_left(1) ^ DAT_009fd430;
        u1 = u1.rotate_left(1) ^ DAT_00a0814c;
        u1 = u1.rotate_left(1) ^ DAT_009a7b50;
        u1 = u1.rotate_left(1) ^ DAT_009d767c;
        u1 = u1.rotate_left(1) ^ _DAT_00a07cec;
        u1 = u1.rotate_left(1) ^ DAT_009a1fe0;
        u1 = u1.rotate_left(1) ^ DAT_009d1144;
        u1 = u1.rotate_left(1) ^ DAT_009fe7dc;
        u1 = u1.rotate_left(1) ^ DAT_009e9640;
        u1 = u1.rotate_left(1) ^ DAT_00a00854;
        u1 = u1.rotate_left(1) ^ DAT_009a7b58;
        u1 = u1.rotate_left(1) ^ DAT_009d1140;
        u1 = u1.rotate_left(1) ^ DAT_009d1310;
        u1 = u1.rotate_left(1) ^ DAT_009ec47c;
        u1 = u1.rotate_left(1) ^ DAT_009a8e24;
        u1 = u1.rotate_left(1) ^ DAT_009cd274;
        u1 = u1.rotate_left(1) ^ DAT_00a07ce8;
        u2 = u1.rotate_left(1) ^ DAT_009d1308;
        u1 = FUN_004630c0(&raw mut DAT_009d1320, 0x5500);
        u1 = u2.rotate_left(1) ^ u1;
        u2 = u1.rotate_left(1) ^ DAT_009a8068;
        u1 = FUN_004630c0(&raw mut DAT_009a8e40, 0x1ffff);
        u1 = u2.rotate_left(1) ^ u1;
        u1 = u1.rotate_left(1) ^ DAT_009a73a4;
        u1 = u1.rotate_left(1) ^ (DAT_009d113c as u32);
        u1 = u1.rotate_left(1) ^ DAT_00a08138;
        local_4 = u1.rotate_left(1);
        let mut pc: *mut i8 = &raw mut DAT_009e9680;
        loop {
            if *pc != 0 {
                let v = FUN_004630c0(pc.wrapping_offset(-0x10).cast::<u32>(), 0x5bd);
                local_4 = (local_4 ^ v).rotate_left(1);
            }
            pc = pc.wrapping_offset(0x5bd);
            if !((pc as usize) < ((&raw mut DAT_009ec468) as usize)) {
                break;
            }
        }
        local_4
    }
}
