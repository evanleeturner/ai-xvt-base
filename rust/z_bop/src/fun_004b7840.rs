// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b7840(param_1: *mut i32, param_2: *mut u8, param_3: i32) -> i32 {
    // SAFETY: the caller supplies an output buffer at param_1 and an input
    // buffer of param_3 bytes at param_2; reads and writes follow the original.
    unsafe {
        let mut param_2 = param_2;
        let mut i_var1: i32 = 0;
        let mut i_var9: i32 = 0;
        let mut b_var6: u8 = *param_2;
        let mut local_c: *mut u8 = param_1.add(1) as *mut u8;
        *local_c = 0;
        let mut pbvar2: *mut u8 = local_c;
        if param_3 < 1 {
            *local_c = 0x80;
            *param_1 = 5;
            return 5;
        }
        loop {
            let mut i_var5: i32 = 0;
            loop {
                if param_3 <= i_var9 || *param_2.add(i_var5 as usize) != b_var6 {
                    break;
                }
                i_var9 = i_var9.wrapping_add(1);
                i_var5 = i_var5.wrapping_add(1);
                if i_var5 >= 0x3f {
                    break;
                }
            }
            let b_var4 = i_var5 as u8;
            let mut pbvar3: *mut u8;
            if b_var6 == 0 {
                i_var1 = i_var1.wrapping_add(1);
                *pbvar2 = b_var4 | 0x40;
                pbvar3 = pbvar2.add(1);
                local_c = pbvar2;
            } else if i_var5 < 3 {
                if (*local_c & 0x80) == 0 {
                    i_var1 = i_var1.wrapping_add(1);
                    *pbvar2 = b_var4 | 0x80;
                    pbvar3 = pbvar2.add(1);
                    local_c = pbvar2;
                    let mut k = 0;
                    while k < i_var5 {
                        *pbvar3.add(k as usize) = *param_2.add(k as usize);
                        k += 1;
                    }
                } else {
                    b_var6 = (*local_c & 0x7f).wrapping_add(b_var4);
                    if b_var6 < 0x80 {
                        *local_c = b_var6 | 0x80;
                        pbvar3 = pbvar2;
                        let mut k = 0;
                        while k < i_var5 {
                            *pbvar2.add(k as usize) = *param_2.add(k as usize);
                            k += 1;
                        }
                    } else {
                        i_var1 = i_var1.wrapping_add(1);
                        *pbvar2 = b_var4 | 0x80;
                        pbvar3 = pbvar2.add(1);
                        local_c = pbvar2;
                        let mut k = 0;
                        while k < i_var5 {
                            *pbvar3.add(k as usize) = *param_2.add(k as usize);
                            k += 1;
                        }
                    }
                }
                pbvar3 = pbvar3.add(i_var5 as usize);
                i_var1 = i_var1.wrapping_add(i_var5);
            } else {
                *pbvar2 = b_var4;
                i_var1 = i_var1.wrapping_add(2);
                *pbvar2.add(1) = b_var6;
                pbvar3 = pbvar2.add(2);
                local_c = pbvar2;
            }
            if param_3 <= i_var9 {
                *pbvar3 = 0x80;
                *param_1 = i_var1.wrapping_add(5);
                return i_var1.wrapping_add(5);
            }
            param_2 = param_2.add(i_var5 as usize);
            b_var6 = *param_2;
            pbvar2 = pbvar3;
        }
    }
}
