// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d3320(param_1: *mut i32, param_2: *mut u8, param_3: i32) -> i32 {
    // SAFETY: the caller supplies the output buffer and input bytes; they are
    // read and written exactly where the original does.
    unsafe {
        let mut param_2 = param_2;
        let mut i_var1: i32 = 0;
        let mut i_var9: i32 = 0;
        let mut b_var6: u8 = *param_2;
        let mut local_c: *mut u8 = param_1.add(1).cast::<u8>();
        let mut pbvar3: *mut u8 = local_c;

        *local_c = 0;

        if param_3 < 1 {
            *local_c = 0x80;
            *param_1 = 5;
            return 5;
        }

        loop {
            let mut i_var5: i32 = 0;
            loop {
                if param_3 <= i_var9 || *param_2.offset(i_var5 as isize) != b_var6 {
                    break;
                }
                i_var9 = i_var9.wrapping_add(1);
                i_var5 = i_var5.wrapping_add(1);
                if i_var5 >= 0x3f {
                    break;
                }
            }
            let b_var4: u8 = i_var5 as u8;
            let mut pbvar2: *mut u8;

            if b_var6 == 0 {
                i_var1 = i_var1.wrapping_add(1);
                *pbvar3 = b_var4 | 0x40;
                pbvar2 = pbvar3.add(1);
                local_c = pbvar3;
            } else if i_var5 < 3 {
                if (*local_c & 0x80) == 0 {
                    i_var1 = i_var1.wrapping_add(1);
                    *pbvar3 = b_var4 | 0x80;
                    pbvar2 = pbvar3.add(1);
                    local_c = pbvar3;
                    let mut k: i32 = 0;
                    while k < i_var5 {
                        *pbvar2.offset(k as isize) = *param_2.offset(k as isize);
                        k += 1;
                    }
                } else {
                    b_var6 = (*local_c & 0x7f).wrapping_add(b_var4);
                    if b_var6 < 0x80 {
                        *local_c = b_var6 | 0x80;
                        pbvar2 = pbvar3;
                        let mut k: i32 = 0;
                        while k < i_var5 {
                            *pbvar3.offset(k as isize) = *param_2.offset(k as isize);
                            k += 1;
                        }
                    } else {
                        i_var1 = i_var1.wrapping_add(1);
                        *pbvar3 = b_var4 | 0x80;
                        pbvar2 = pbvar3.add(1);
                        local_c = pbvar3;
                        let mut k: i32 = 0;
                        while k < i_var5 {
                            *pbvar2.offset(k as isize) = *param_2.offset(k as isize);
                            k += 1;
                        }
                    }
                }
                pbvar2 = pbvar2.offset(i_var5 as isize);
                i_var1 = i_var1.wrapping_add(i_var5);
            } else {
                *pbvar3 = b_var4;
                i_var1 = i_var1.wrapping_add(2);
                *pbvar3.add(1) = b_var6;
                pbvar2 = pbvar3.add(2);
                local_c = pbvar3;
            }

            if param_3 <= i_var9 {
                *pbvar2 = 0x80;
                *param_1 = i_var1.wrapping_add(5);
                return i_var1.wrapping_add(5);
            }

            param_2 = param_2.offset(i_var5 as isize);
            b_var6 = *param_2;
            pbvar3 = pbvar2;
        }
    }
}
