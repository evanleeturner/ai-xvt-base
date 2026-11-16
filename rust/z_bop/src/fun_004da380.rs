// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052c018: i32;
    static mut DAT_0052c014: i32;
    static mut DAT_00665464: i32;
    static mut DAT_00665680: i32;
    static mut DAT_0052c000: i32;
    static mut PTR_DAT_0052da78: *mut u8;
    static mut DAT_0052dc84: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6e30(a: i32, b: i32, c: i32) -> i32;
    fn FUN_004de090(a: *mut i32, b: *mut i32);
    fn FUN_004d6b90(a: *mut i32, b: i32, c: i32) -> i32;
    fn FUN_004dc220() -> u8;
    fn FUN_004dc240() -> u8;
    fn FUN_004dcb10() -> u32;
    fn FUN_004dcb50();
    fn __isctype(c: i32, mask: i32) -> i32;
    fn FUN_004dbfa0(s: *mut i8, a: i32) -> i32;
    fn FUN_004d4cf0(r: *mut i32);
    fn FUN_004d6460(r: *mut i32, a: i32, b: i32, c: i32, d: i32);
    fn FUN_004d4d20(r: *mut i32);
    fn FUN_004db4e0(a: i32, s: *mut i8, b: i32, c: i32, d: i32);
    fn FUN_004d4de0() -> i32;
    fn FUN_004d6480(r: *mut i32, src: *mut i32);
    fn FUN_004d67e0(r: *mut i32, a: i32, b: i32, c: i16, d: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004da380(
    param_1: *mut i32,
    param_2: *mut i8,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: *mut i8,
) -> i32 {
    let mut local_2c: i32;
    let mut local_28: i32 = 0;
    let mut local_24: [i32; 4] = [0; 4];
    let mut local_14: i32 = 0;
    let mut local_10: [i32; 4] = [0; 4];

    // SAFETY: the globals and callees are the game's own; the pointers are
    // the caller's buffers, accessed where the original accesses them.
    unsafe {
        if DAT_0052c018 == 0 {
            DAT_0052c014 = FUN_004d6e30(0xff, 0xff, 0xff);
            DAT_0052c018 = 1;
        }
        let mut len: u32 = 0;
        while *param_2.add(len as usize) != 0 {
            len = len.wrapping_add(1);
        }
        DAT_00665464 = len as i32;
        DAT_00665680 = len as i32;
        local_28 = 0;
        local_2c = 0;
        FUN_004de090(&raw mut local_2c, &raw mut local_14);
        let mut iv = FUN_004d6b90(param_1, local_2c, local_14);
        if iv != 0 && (FUN_004dc220() != 0 || FUN_004dc240() != 0) {
            DAT_0052c000 = param_4;
        }
        if DAT_0052c000 == param_4 {
            let u = FUN_004dcb10();
            let mut b6 = u as u8;
            if !param_6.is_null() {
                let mut i: i32 = 0;
                let mut c = *param_6;
                while c != 0 {
                    if (*param_6.offset(i as isize) as i32) == (b6 as i32) {
                        b6 = 0;
                        FUN_004dcb50();
                        break;
                    }
                    i = i.wrapping_add(1);
                    c = *param_6.offset(i as isize);
                }
            }
            'lab: {
                if b6 != 0 {
                    let mut u5 = b6 as u32;
                    if u5 != 0x1b {
                        FUN_004dcb50();
                    }
                    if DAT_0052dc84 < 2 {
                        u5 = (PTR_DAT_0052da78
                            .byte_add((u5 as usize) * 2)
                            .cast::<u16>()
                            .read_unaligned() as u32)
                            & 0x157;
                    } else {
                        u5 = __isctype(u5 as i32, 0x157) as u32;
                    }
                    if u5 == 0 && b6 != 0x20 {
                        if b6 == 0xd || b6 == 9 {
                            local_28 = 1;
                            break 'lab;
                        }
                        if b6 != 8 || DAT_00665680 == 0 {
                            break 'lab;
                        }
                        DAT_00665680 = DAT_00665680.wrapping_sub(1);
                        DAT_00665464 = DAT_00665464.wrapping_sub(1);
                    } else {
                        if param_3.wrapping_sub(1) <= DAT_00665680 {
                            break 'lab;
                        }
                        *param_2.offset(DAT_00665680 as isize) = b6 as i8;
                        DAT_00665680 = DAT_00665680.wrapping_add(1);
                        DAT_00665464 = DAT_00665464.wrapping_add(1);
                    }
                    *param_2.offset(DAT_00665680 as isize) = 0;
                }
            }
        }
        // LAB_004da51b
        iv = FUN_004dbfa0(param_2, param_5);
        local_2c = 0;
        let p0 = param_1.read();
        let p1 = param_1.add(1).read();
        let p2 = param_1.add(2).read();
        let p3 = param_1.add(3).read();
        if p2.wrapping_sub(p0).wrapping_add(1) < iv {
            local_2c = p2
                .wrapping_sub(p0)
                .wrapping_sub(param_5)
                .wrapping_sub(iv)
                .wrapping_add(1);
        }
        FUN_004d4cf0(local_24.as_mut_ptr());
        FUN_004d6460(
            local_10.as_mut_ptr(),
            p0.wrapping_add(2),
            p1.wrapping_add(2),
            p2,
            p3,
        );
        FUN_004d4d20(local_10.as_mut_ptr());
        FUN_004db4e0(
            param_5,
            param_2,
            p0.wrapping_add(local_2c).wrapping_add(2),
            p1.wrapping_add(2),
            DAT_0052c014,
        );
        FUN_004d4d20(local_24.as_mut_ptr());
        if DAT_0052c000 == param_4 {
            let cv = *param_2.offset(DAT_00665464 as isize);
            *param_2.offset(DAT_00665464 as isize) = 0;
            iv = FUN_004dbfa0(param_2, param_5);
            *param_2.offset(DAT_00665464 as isize) = cv;
            let i4 = FUN_004d4de0();
            if i4 % 10 < 5 {
                FUN_004d6480(local_24.as_mut_ptr(), param_1);
                local_24[1] = local_24[1].wrapping_add(1);
                local_24[0] = local_24[0]
                    .wrapping_add(local_2c)
                    .wrapping_add(iv)
                    .wrapping_add(1);
                local_24[2] = local_24[0].wrapping_add(1);
                local_24[3] = local_24[1].wrapping_add(param_5);
                FUN_004d67e0(local_24.as_mut_ptr(), 0, 0, DAT_0052c014 as i16, 1);
            }
        }
    }
    local_28
}
