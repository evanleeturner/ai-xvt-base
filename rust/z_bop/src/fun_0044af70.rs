// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005233cc: u32;
    static mut DAT_005233ec: u32;
    static mut DAT_005233f4: u32;
    static mut DAT_00523928: [u8; 0];
    static mut DAT_00523929: [u8; 0];
    static mut DAT_005569c0: [u16; 0];
    static mut DAT_00a00530: [u16; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040e190(param_1: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_00410ff0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044af70(param_1: u32, param_2: u32, param_3: u8) {
    let mut local_e: i16 = 7;
    let mut u_var4: u32 = 0;
    let mut u_var5: u16 = 0;
    // SAFETY: all globals and callees are the game's own memory and code;
    // the pointer written through is the game's video/buffer memory, accessed
    // where the original does.
    unsafe {
        loop {
            let a = (&raw mut DAT_00523929)
                .cast::<u8>()
                .add(u_var4 as usize)
                .read() as i8 as i32;
            let b = (&raw mut DAT_00523928)
                .cast::<u8>()
                .add(u_var4 as usize)
                .read() as i8 as i32;
            let i_var1 = FUN_0040e190(a.wrapping_add((param_2 & 0xffff) as i32));
            let mut u_var2: u32 = (i_var1
                .wrapping_add(b.wrapping_add((param_1 & 0xffff) as i32).wrapping_mul(2)))
                as u32;
            if DAT_005233ec != 0x13 && DAT_005233f4 == 0xa0000 {
                let divisor = DAT_005233cc;
                let rem: u32;
                core::arch::asm!(
                    "div {d}",
                    d = in(reg) divisor,
                    inout("eax") u_var2 => _,
                    inout("edx") 0u32 => rem,
                    options(nomem, nostack)
                );
                u_var2 = rem;
                FUN_00410ff0();
                FUN_00410ff0();
            }
            u_var4 = (u_var4.wrapping_add(2)) as u16 as u32;
            let p = u_var2.wrapping_add(DAT_005233f4 & 0xfffffffe) as usize as *mut u16;
            (&raw mut DAT_005569c0)
                .cast::<u16>()
                .add(u_var5 as usize)
                .write_unaligned(p.read_unaligned());
            p.write_unaligned(
                (&raw mut DAT_00a00530)
                    .cast::<u16>()
                    .add(param_3 as usize)
                    .read_unaligned(),
            );
            local_e = local_e.wrapping_sub(1);
            u_var5 = u_var5.wrapping_add(1);
            if local_e == 0 {
                break;
            }
        }
    }
}
