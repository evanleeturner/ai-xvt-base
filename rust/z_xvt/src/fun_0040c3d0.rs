// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523a80: [u8; 1];
    static mut DAT_00523a68: [u8; 1];
    static mut DAT_00523a98: [u8; 1];
    static mut DAT_0073b7e0: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004784d0() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040c3d0(param_1: i32, param_2: i32) -> u32 {
    // SAFETY: the globals and the callee are the game's own; the tables are
    // indexed where the original indexes them, and the divide faults as the
    // machine's `div` does on a zero divisor.
    unsafe {
        let u_var3: u32 = (&raw mut DAT_00523a80)
            .cast::<u8>()
            .offset(param_1 as isize)
            .read() as u32;
        let b_var1: u8 = (&raw mut DAT_00523a68)
            .cast::<u8>()
            .offset(param_1 as isize)
            .read();
        let u_var4: u32 = FUN_004784d0();
        let n: u32 = u_var4 & 0xffff;
        let q: u32;
        core::arch::asm!(
            "div {d}",
            d = in(reg) u_var3,
            inout("eax") n => q,
            inout("edx") 0u32 => _,
        );
        let mut u_var5: u32 = q;
        if (u_var5 as u16) == 0 {
            u_var5 = 0;
        } else {
            u_var5 = n % u_var5;
        }
        let b_var2: u8 = (&raw mut DAT_00523a98)
            .cast::<u8>()
            .offset(param_1 as isize)
            .read();
        let table = (&raw mut DAT_0073b7e0).cast::<u8>();
        let row = param_2.wrapping_mul(0x61);
        if b_var2 != 0 {
            let idx = (b_var1 as u32)
                .wrapping_add(u_var5)
                .wrapping_add(row as u32) as i32;
            if b_var2 <= table.offset(idx as isize).read() {
                let mut u_var4 = u_var3;
                while u_var4 != 0 {
                    u_var5 = u_var5.wrapping_add(1);
                    if (u_var3 as i32) <= (u_var5 as i32) {
                        u_var5 = 0;
                    }
                    u_var4 = u_var4.wrapping_sub(1);
                    let idx = (b_var1 as u32)
                        .wrapping_add(u_var5)
                        .wrapping_add(row as u32) as i32;
                    if table.offset(idx as isize).read() < b_var2 {
                        return u_var5;
                    }
                }
                u_var5 = 0xffff_ffff;
            }
        }
        u_var5
    }
}
