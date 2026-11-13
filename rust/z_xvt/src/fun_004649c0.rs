// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005f1d20: [u32; 0];
    static mut DAT_00527d21: [u8; 0];
    static mut DAT_00527d20: [u8; 0];
    static mut DAT_0052552c: u32;
    static mut DAT_00525534: u32;
    static mut DAT_0052550c: u32;
    static mut DAT_005f1118: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004649c0(param_1: u32, param_2: u32, param_3: u8) {
    let mut u_var4: u32 = 0;
    let mut s_var3: i16 = 7;
    let mut u_var5: u16 = 0;
    // SAFETY: all globals and tables are the game's own memory; the
    // addresses computed are read and written exactly where the original does.
    unsafe {
        let tab = (&raw mut DAT_005f1d20) as *mut u8 as u32;
        let a1 = (&raw mut DAT_00527d21) as *const u8;
        let a0 = (&raw mut DAT_00527d20) as *const u8;
        let save = (&raw mut DAT_005f1118) as *mut u8;
        loop {
            let c1 = a1.add(u_var4 as usize).read() as i8 as i32;
            let c0 = a0.add(u_var4 as usize).read() as i8 as i32;
            let idx = c1.wrapping_add((param_2 & 0xffff) as i32);
            let addr = tab.wrapping_add((idx as u32).wrapping_mul(16)) as *const u32;
            let mut u_var1: u32 = addr
                .read_unaligned()
                .wrapping_add(c0 as u32)
                .wrapping_add(param_1 & 0xffff);
            if DAT_0052552c != 0x13 && DAT_00525534 != 0xa0000 {
                let d = DAT_0052550c;
                let rem: u32;
                core::arch::asm!(
                    "div {d:e}",
                    d = in(reg) d,
                    inout("eax") u_var1 => _,
                    inout("edx") 0u32 => rem,
                );
                u_var1 = rem;
            }
            u_var4 = u_var4.wrapping_add(2);
            let p = u_var1.wrapping_add(DAT_00525534) as *mut u8;
            s_var3 = s_var3.wrapping_sub(1);
            save.add(u_var5 as usize).write(p.read());
            p.write(param_3);
            u_var5 = u_var5.wrapping_add(1);
            if s_var3 == 0 {
                break;
            }
        }
    }
}
