// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052550c: i32;
    static mut DAT_0052552c: i32;
    static mut DAT_00525534: i32;
    static mut DAT_005f1d20: i32;
    static mut DAT_0071f2e8: i16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00464640(
    param_1: *const u8,
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
            // SAFETY: the globals and the table and video memory they point
            // at are the game's own; param_1 is the caller's source buffer.
            unsafe {
                let mut u_var3: u32 = ((DAT_005f1d20.wrapping_add(i_var4)) as *const i32)
                    .read_unaligned()
                    .wrapping_add(param_2) as u32;
                if DAT_0052552c != 0x13 && DAT_00525534 == 0xa0000 {
                    let divisor = DAT_0052550c as u32;
                    let rem: u32;
                    let _q: u32;
                    core::arch::asm!(
                        "xor edx, edx",
                        "div {d}",
                        d = in(reg) divisor,
                        inout("eax") u_var3 => _q,
                        out("edx") rem,
                        options(nostack),
                    );
                    u_var3 = rem;
                }
                let mut puvar2 = u_var3.wrapping_add(DAT_00525534 as u32) as *mut u8;
                DAT_0071f2e8 = param_4;
                while DAT_0071f2e8 != 0 {
                    let u_var1 = *param_1;
                    param_1 = param_1.add(1);
                    *puvar2 = u_var1;
                    puvar2 = puvar2.add(1);
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
