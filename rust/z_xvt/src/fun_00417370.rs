// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052550c: u32;
    static mut DAT_0052552c: u32;
    static mut DAT_00525534: u32;
    static mut DAT_00523c50: [u8; 16];
    static mut DAT_00523c51: [u8; 16];
    static mut DAT_00536a08: [u16; 16];
    static mut DAT_007325e0: [u16; 256];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00461ff0(param_1: i32) -> i32;
    fn FUN_004651f0(arg1: u32, arg2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00417370(param_1: u32, param_2: u32, param_3: u8) {
    let mut local_e: i16 = 7;
    let mut u_var4: u32 = 0;
    let mut u_var5: u16 = 0;
    // SAFETY: all globals and callees are the game's own memory; the
    // computed address points into the game's video/back buffer as in the original.
    unsafe {
        loop {
            let b1 = (&raw mut DAT_00523c51)
                .cast::<u8>()
                .add(u_var4 as usize)
                .read() as i8;
            let b0 = (&raw mut DAT_00523c50)
                .cast::<u8>()
                .add(u_var4 as usize)
                .read() as i8;
            let i_var1 = FUN_00461ff0((b1 as i32).wrapping_add((param_2 & 0xffff) as i32));
            let mut u_var2: u32 = (i_var1 as u32)
                .wrapping_add(((b0 as i32 as u32).wrapping_add(param_1 & 0xffff)).wrapping_mul(2));
            if DAT_0052552c != 0x13 && DAT_00525534 == 0xa0000 {
                let divisor = DAT_0052550c;
                let q: u32;
                core::arch::asm!(
                    "div {d}",
                    d = in(reg) divisor,
                    inout("eax") u_var2 => q,
                    inout("edx") 0u32 => _,
                );
                u_var2 = q;
                FUN_004651f0(0, 0);
                FUN_004651f0(0, 0);
            }
            u_var4 = (u_var4 as u16).wrapping_add(2) as u32;
            let p = u_var2.wrapping_add(DAT_00525534 & 0xffff_fffe) as *mut u16;
            let v = p.read_unaligned();
            (&raw mut DAT_00536a08)
                .cast::<u16>()
                .add(u_var5 as usize)
                .write(v);
            let w = (&raw mut DAT_007325e0)
                .cast::<u16>()
                .add(param_3 as usize)
                .read();
            p.write_unaligned(w);
            local_e = local_e.wrapping_sub(1);
            u_var5 = u_var5.wrapping_add(1);
            if local_e == 0 {
                break;
            }
        }
    }
}
