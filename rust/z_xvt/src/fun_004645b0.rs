// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005f1d20: i32;
    static mut DAT_0052552c: i32;
    static mut DAT_00525534: i32;
    static mut DAT_0052550c: u32;
    static mut DAT_0071f2e8: i16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004645b0(
    param_1: *mut u8,
    param_2: i32,
    param_3: i32,
    param_4: i16,
    param_5: i32,
) {
    let mut param_1 = param_1;
    let mut param_5 = param_5;
    if param_5 != 0 {
        let mut i_var4: i32 = param_3.wrapping_shl(2);
        loop {
            // SAFETY: the globals and the table they point at are the game's own
            // memory; param_1 and the source buffer are the caller's bytes, read
            // and written where the original does.
            unsafe {
                let entry = (DAT_005f1d20.wrapping_add(i_var4) as u32 as usize) as *const i32;
                let mut u_var2: u32 = (param_2 as u32).wrapping_add(entry.read_unaligned() as u32);
                if DAT_0052552c != 0x13 && DAT_00525534 != 0xa0000 {
                    let divisor = DAT_0052550c;
                    let rem: u32;
                    core::arch::asm!(
                        "div {d}",
                        d = in(reg) divisor,
                        inout("eax") u_var2 => _,
                        inout("edx") 0u32 => rem,
                        options(nomem, nostack)
                    );
                    u_var2 = rem;
                }
                let mut p_var3 = (u_var2.wrapping_add(DAT_00525534 as u32) as usize) as *const u8;
                DAT_0071f2e8 = param_4;
                while DAT_0071f2e8 != 0 {
                    let u_var1 = p_var3.read();
                    p_var3 = p_var3.byte_add(1);
                    param_1.write(u_var1);
                    param_1 = param_1.byte_add(1);
                    DAT_0071f2e8 = DAT_0071f2e8.wrapping_sub(1);
                }
                param_5 = param_5.wrapping_sub(1);
                i_var4 = i_var4.wrapping_add(4);
                DAT_0071f2e8 = 0;
            }
            if param_5 == 0 {
                break;
            }
        }
    }
}
