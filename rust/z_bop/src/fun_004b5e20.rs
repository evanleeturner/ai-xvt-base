// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b1f: i32;
    static mut DAT_00aa7b13: i32;
    static mut DAT_00aa6cfc: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6480(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004d64e0(param_1: *mut i32) -> u32;
    fn FUN_004b5fe0(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    );
    fn FUN_004b6340(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b5e20(param_1: *mut i32, param_2: i32, param_3: i32) -> u32 {
    if param_1.is_null() {
        return 0;
    }
    // SAFETY: param_1 points at the caller's image header; globals and callees
    // are the game's own; raw addresses are the framebuffer/image memory the
    // original reads and writes.
    unsafe {
        // local_24, local_20, local_1c, local_18 are contiguous on the stack
        let mut rect: [i32; 4] = [0; 4];
        // local_10, local_c
        let mut pt: [i32; 2] = [0; 2];
        let local_8: i32 = 0;
        let local_4: i32 = 0;

        rect[0] = param_2;
        rect[1] = param_3;
        rect[2] = param_1.read().wrapping_add(param_2).wrapping_sub(1);
        rect[3] = param_1.add(1).read().wrapping_add(param_3).wrapping_sub(1);
        FUN_004d6480(pt.as_mut_ptr(), rect.as_mut_ptr());
        let u_var2 = FUN_004d64e0(rect.as_mut_ptr());
        if rect[0] <= rect[2] {
            if rect[3] < rect[1] {
                return u_var2;
            }
            pt[0] = rect[0].wrapping_sub(pt[0]);
            pt[1] = rect[1].wrapping_sub(rect[2]);
            let local_10 = pt[0];
            let local_c = pt[1];
            let width = param_1.read();
            let mut i_var3 = width;
            let i_var4 = rect[2]
                .wrapping_sub(local_10)
                .wrapping_sub(local_8)
                .wrapping_add(i_var3);
            let mut i_var7 = param_1
                .add(1)
                .read()
                .wrapping_sub(local_c)
                .wrapping_sub(local_4)
                .wrapping_add(rect[3]);
            if param_1.add(2).read() == 0 {
                if DAT_00aa7b1f == 8 {
                    i_var3 = i_var3
                        .wrapping_mul(local_c)
                        .wrapping_add(param_1.add(4).read())
                        .wrapping_add(local_10);
                    let mut dst = local_c
                        .wrapping_add(param_3)
                        .wrapping_mul(DAT_00aa7b13)
                        .wrapping_add(param_2)
                        .wrapping_add(DAT_00aa6cfc)
                        .wrapping_add(local_10);
                    if 0 < i_var7 {
                        loop {
                            let mut i_var5: i32 = 0;
                            if 0 < i_var4 {
                                loop {
                                    let src = i_var3.wrapping_add(i_var5) as usize as *const u8;
                                    i_var5 = i_var5.wrapping_add(1);
                                    let d = dst.wrapping_sub(1).wrapping_add(i_var5) as usize
                                        as *mut u8;
                                    d.write(src.read());
                                    if !(i_var5 < i_var4) {
                                        break;
                                    }
                                }
                            }
                            i_var3 = i_var3.wrapping_add(param_1.read());
                            dst = dst.wrapping_add(DAT_00aa7b13);
                            i_var7 = i_var7.wrapping_sub(1);
                            if i_var7 == 0 {
                                break;
                            }
                        }
                    }
                } else if DAT_00aa7b1f == 0x10 {
                    let mut i_var5 = i_var3
                        .wrapping_mul(local_c)
                        .wrapping_add(param_1.add(4).read())
                        .wrapping_add(local_10);
                    let mut i_var3b = local_c
                        .wrapping_add(param_3)
                        .wrapping_mul(DAT_00aa7b13)
                        .wrapping_add(local_10.wrapping_add(param_2).wrapping_mul(2))
                        .wrapping_add(DAT_00aa6cfc);
                    if 0 < i_var7 {
                        loop {
                            let mut i_var6: i32 = 0;
                            if 0 < i_var4 {
                                loop {
                                    i_var6 = i_var6.wrapping_add(1);
                                    let idx = (i_var5.wrapping_sub(1).wrapping_add(i_var6) as usize
                                        as *const u8)
                                        .read()
                                        as usize;
                                    let val = param_1.add(idx + 5).read();
                                    let d = i_var3b
                                        .wrapping_sub(2)
                                        .wrapping_add(i_var6.wrapping_mul(2))
                                        as usize
                                        as *mut i16;
                                    d.write_unaligned(val as i16);
                                    if !(i_var6 < i_var4) {
                                        break;
                                    }
                                }
                            }
                            i_var5 = i_var5.wrapping_add(param_1.read());
                            i_var3b = i_var3b.wrapping_add(DAT_00aa7b13 & -2i32);
                            i_var7 = i_var7.wrapping_sub(1);
                            if i_var7 == 0 {
                                break;
                            }
                        }
                    }
                }
            } else if DAT_00aa7b1f == 8 {
                FUN_004b5fe0(
                    param_1,
                    local_10.wrapping_add(param_2),
                    local_c.wrapping_add(param_3),
                    local_10,
                    local_c,
                    i_var4,
                    i_var7,
                );
            } else if DAT_00aa7b1f == 0x10 {
                FUN_004b6340(
                    param_1,
                    local_10.wrapping_add(param_2),
                    local_c.wrapping_add(param_3),
                    local_10,
                    local_c,
                    i_var4,
                    i_var7,
                );
            }
        }
        u_var2
    }
}
