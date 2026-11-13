// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071e520: u16;
    static mut DAT_007327f0: u32;
    static mut DAT_00524368: [u16; 1];
    static mut DAT_0052436a: [u16; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00419000(
    param_1: u32,
    param_2: u32,
    param_3: *mut u16,
    param_4: *mut u16,
) {
    let mut param_1 = param_1;
    let mut u_var6: u32 = 0;
    let i_var2: i32;
    // SAFETY: the globals and tables are the game's own memory; param_3 and
    // param_4 point at the caller's u16 outputs.
    unsafe {
        DAT_0071e520 = 0;
        if param_2 == param_1 {
            i_var2 = 0x100;
            DAT_007327f0 = param_1;
        } else {
            let b_var1: i32 = ((param_1 as i32) <= (param_2 as i32)) as i32;
            let mut u_var3 = param_2;
            if b_var1 != 0 {
                u_var3 = param_1;
                param_1 = param_2;
            }
            DAT_0071e520 = b_var1 as u16;
            DAT_007327f0 = param_1;
            if param_1 == 0 {
                u_var6 = u_var3 >> 0x10;
                i_var2 = 0;
            } else {
                u_var6 = u_var3;
                let mut u_var5 = param_1;
                if (param_1 & 0xff000000) == 0 {
                    u_var5 = param_1 << 8;
                    u_var6 = u_var3 << 8;
                    if (u_var5 & 0xff000000) == 0 {
                        u_var5 = param_1 << 0x10;
                        u_var6 = u_var3 << 0x10;
                    }
                }
                if u_var6 == u_var5 {
                    u_var6 >>= 0x10;
                    i_var2 = 0x100;
                } else {
                    u_var6 = u_var6 / (u_var5 >> 0x10);
                    i_var2 = ((u_var6 & 0xffff) as i32) >> 8;
                    u_var6 = (u_var6 & 0xff) << 8;
                }
            }
        }
        *param_4 = i_var2 as u16;
        let t1 = (&raw mut DAT_0052436a).cast::<u16>();
        let t0 = (&raw mut DAT_00524368).cast::<u16>();
        let mut u_var4: u16 = *t1.add(i_var2 as usize);
        *param_3 = u_var4;
        u_var4 = u_var4.wrapping_sub(*t0.add(*param_4 as usize));
        *param_3 = u_var4;
        u_var4 = ((u_var4 as u32).wrapping_mul(u_var6 & 0xff00) >> 0x10) as u16;
        *param_3 = u_var4;
        u_var4 = (*t0.add(*param_4 as usize)).wrapping_add(u_var4);
        *param_3 = u_var4;
        if DAT_0071e520 != 0 {
            u_var4 = 0u16.wrapping_sub(u_var4);
            *param_3 = u_var4;
            *param_3 = u_var4.wrapping_add(0x4000);
        }
    }
}
