// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0051a820: [i8; 0];
    static mut DAT_0051a821: [i8; 0];
    static mut DAT_005233cc: i32;
    static mut DAT_005233ec: i32;
    static mut DAT_005233f4: i32;
    static mut DAT_0054f9d8: [i8; 0];
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
pub unsafe extern "C" fn FUN_00410860(param_1: u32, param_2: u32) {
    let mut local_6: i16 = 7;
    let mut u_var4: u32 = 0;
    let mut u_var3: u16 = 0;
    // SAFETY: the globals, arrays and callees are the game's own memory and
    // code; the destination address is computed and written as the original does.
    unsafe {
        let a820 = (&raw mut DAT_0051a820).cast::<i8>();
        let a821 = (&raw mut DAT_0051a821).cast::<i8>();
        let a9d8 = (&raw mut DAT_0054f9d8).cast::<i8>();
        loop {
            let i_var1 = FUN_0040e190(
                (a821.add(u_var4 as usize).read() as i32).wrapping_add((param_2 & 0xffff) as i32),
            );
            let mut u_var2: u32 = (i_var1 as u32)
                .wrapping_add(a820.add(u_var4 as usize).read() as i32 as u32)
                .wrapping_add(param_1 & 0xffff);
            if DAT_005233ec != 0x13 && DAT_005233f4 == 0xa0000 {
                let divisor = DAT_005233cc as u32;
                let rem: u32;
                core::arch::asm!(
                    "div {d}",
                    d = in(reg) divisor,
                    inout("eax") u_var2 => _,
                    inout("edx") 0u32 => rem,
                );
                u_var2 = rem;
                FUN_00410ff0();
            }
            u_var4 = u_var4.wrapping_add(2);
            local_6 = local_6.wrapping_sub(1);
            let addr = u_var2.wrapping_add(DAT_005233f4 as u32) as *mut u8;
            addr.write(a9d8.add(u_var3 as usize).read() as u8);
            u_var3 = u_var3.wrapping_add(1);
            if local_6 == 0 {
                break;
            }
        }
    }
}
