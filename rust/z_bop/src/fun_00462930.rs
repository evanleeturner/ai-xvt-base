// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a2000: u16;
    static mut DAT_009a20c0: [i8; 0];
    static mut DAT_009d9192: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00432fa0(param_1: u16, param_2: u16, param_3: u16) -> u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00462930(
    param_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: u16,
    param_5: u16,
) -> u32 {
    // SAFETY: the globals and the callee are the game's own memory; the
    // table offsets are read exactly where the original reads them.
    unsafe {
        let mut in_eax: u32 = DAT_009a2000 as u32;
        let mut u_var4: u16 = 0;
        if DAT_009a2000 > 0 {
            loop {
                let u_var3: u32 = u_var4 as u32;
                let base1 = (&raw mut DAT_009a20c0).cast::<u8>();
                let base2 = (&raw mut DAT_009d9192).cast::<u8>();
                let off1 = u_var3.wrapping_mul(0x126) as usize;
                let off2 = u_var3.wrapping_mul(0x566) as usize;
                if base1.byte_add(off1).byte_add(3).read() != 0
                    || base2.byte_add(off2).cast::<i32>().read_unaligned() != -1
                {
                    let mut u_var1 = FUN_00432fa0(u_var4, param_1, param_2);
                    let u_var2 = FUN_00432fa0(u_var4, param_4, param_5);
                    if param_3 == 1 {
                        u_var1 |= u_var2;
                    } else {
                        u_var1 &= u_var2;
                    }
                    if u_var1 != 0 {
                        if base1.byte_add(off1).read() == 0 {
                            return 1;
                        }
                        if base1.byte_add(off1).byte_add(1).read() != 0 {
                            return 1;
                        }
                    }
                }
                in_eax = DAT_009a2000 as u32;
                u_var4 = u_var4.wrapping_add(1);
                if !((u_var4 as u32 as i32) < (in_eax as i32)) {
                    break;
                }
            }
        }
        in_eax & 0xffff0000
    }
}
