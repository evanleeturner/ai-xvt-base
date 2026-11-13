// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00449410(param_1: i32, param_2: i32, param_3: i32, param_4: u32);
}

#[inline(always)]
unsafe fn rd(a: i32) -> i32 {
    // SAFETY: the caller passes an address inside the game's own data.
    unsafe { (a as u32 as usize as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040e6c0(param_1: *mut i32) {
    let mut local_c00 = [0u8; 3072];
    // SAFETY: param_1 points at the game's structure; offsets are read and
    // written where the original does, and the scratch buffer is local.
    unsafe {
        let base = param_1 as usize as u32 as i32;
        let mut i_var3: i32 = param_1.read_unaligned().wrapping_add(base);
        if param_1.add(0xb).read_unaligned() == 0x18 {
            let u_var6 = param_1.add(0xc).read_unaligned() as u32;
            param_1.add(5).write_unaligned(i_var3.wrapping_sub(base));
            if u_var6 < 0x400 {
                if u_var6 != 0 {
                    let mut pb1 = local_c00.as_mut_ptr();
                    let mut pb2 = (param_1.add(3).read_unaligned().wrapping_add(base)) as u32
                        as usize as *const u8;
                    let mut u_var4 = u_var6;
                    loop {
                        *pb1 = *pb2 >> 2;
                        *pb1.add(1) = *pb2.add(1) >> 2;
                        u_var4 = u_var4.wrapping_sub(1);
                        *pb1.add(2) = *pb2.add(2) >> 2;
                        pb1 = pb1.add(3);
                        pb2 = pb2.add(4);
                        if u_var4 == 0 {
                            break;
                        }
                    }
                }
                FUN_00449410(
                    local_c00.as_mut_ptr() as usize as u32 as i32,
                    i_var3,
                    0,
                    u_var6,
                );
                i_var3 = i_var3.wrapping_add(param_1.add(0xc).read_unaligned().wrapping_mul(2));
            }
        }
        let mut local_c04: u32 = 0;
        if param_1.add(6).read_unaligned() != 0 {
            loop {
                let i_var5: i32 = rd(param_1
                    .add(4)
                    .read_unaligned()
                    .wrapping_add((local_c04.wrapping_mul(4)) as i32)
                    .wrapping_add(base))
                .wrapping_add(base);
                ((i_var5.wrapping_add(0xc)) as u32 as usize as *mut i32)
                    .write_unaligned(i_var3.wrapping_sub(i_var5));
                if rd(i_var5.wrapping_add(0x24)) == 0x18
                    && (rd(i_var5.wrapping_add(0x28)) as u32) < 0x400
                {
                    let mut u_var6: u32 = 0;
                    if rd(i_var5.wrapping_add(0x28)) as u32 != 0 {
                        let mut pb1 = local_c00.as_mut_ptr();
                        let mut pb2 = rd(i_var5.wrapping_add(4)).wrapping_add(i_var5) as u32
                            as usize as *const u8;
                        loop {
                            *pb1 = *pb2 >> 2;
                            u_var6 = u_var6.wrapping_add(1);
                            *pb1.add(1) = *pb2.add(1) >> 2;
                            *pb1.add(2) = *pb2.add(2) >> 2;
                            pb1 = pb1.add(3);
                            pb2 = pb2.add(4);
                            if u_var6 >= rd(i_var5.wrapping_add(0x28)) as u32 {
                                break;
                            }
                        }
                    }
                    FUN_00449410(
                        local_c00.as_mut_ptr() as usize as u32 as i32,
                        i_var3,
                        0,
                        rd(i_var5.wrapping_add(0x28)) as u32,
                    );
                    i_var3 = i_var3.wrapping_add(rd(i_var5.wrapping_add(0x28)).wrapping_mul(2));
                }
                local_c04 = local_c04.wrapping_add(1);
                if local_c04 >= param_1.add(6).read_unaligned() as u32 {
                    break;
                }
            }
        }
    }
}
