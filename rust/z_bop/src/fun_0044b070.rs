// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523928: [i8; 1];
    static mut DAT_00523929: [i8; 1];
    static mut DAT_005233cc: u32;
    static mut DAT_005233ec: u32;
    static mut DAT_005233f4: u32;
    static mut DAT_005569c0: [u16; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040e190(param_1: i32) -> i32;
    fn FUN_00410ff0(param_1: u32, param_2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044b070(param_1: u32, param_2: u32) {
    let mut s_var4: i16 = 7;
    let mut u_var5: u32 = 0;
    let mut u_var3: u16 = 0;
    // SAFETY: the globals and callees are the game's own memory; the
    // pointer writes go to the addresses the original computes.
    unsafe {
        loop {
            let p9 = (&raw mut DAT_00523929).cast::<i8>();
            let p8 = (&raw mut DAT_00523928).cast::<i8>();
            let i_var1 = FUN_0040e190(
                (*p9.add(u_var5 as usize) as i32).wrapping_add((param_2 & 0xffff) as i32),
            );
            let t = (*p8.add(u_var5 as usize) as i32).wrapping_add((param_1 & 0xffff) as i32);
            let mut u_var2: u32 = i_var1.wrapping_add(t.wrapping_mul(2)) as u32;
            if DAT_005233ec != 0x13 && DAT_005233f4 == 0xa0000 {
                let dv = DAT_005233cc;
                let rem: u32;
                core::arch::asm!(
                    "div {d:e}",
                    d = in(reg) dv,
                    inout("eax") u_var2 => _,
                    inout("edx") 0u32 => rem,
                );
                u_var2 = rem;
                FUN_00410ff0(u_var2, DAT_005233f4 & 0xfffffffe);
            }
            u_var5 = (u_var5 as u16).wrapping_add(2) as u32;
            s_var4 = s_var4.wrapping_sub(1);
            let src = (&raw mut DAT_005569c0).cast::<u16>().add(u_var3 as usize);
            let val = src.read_unaligned();
            let addr = u_var2.wrapping_add(DAT_005233f4 & 0xfffffffe);
            (addr as *mut u16).write_unaligned(val);
            u_var3 = u_var3.wrapping_add(1);
            if s_var4 == 0 {
                break;
            }
        }
    }
}
