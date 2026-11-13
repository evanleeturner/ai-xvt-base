// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523c39: [i8; 1];
    static mut DAT_00523c38: [i8; 1];
    static mut DAT_00656c84: u16;
    static mut DAT_00656c82: u16;
    static mut DAT_0052552c: u32;
    static mut DAT_00525534: u32;
    static mut DAT_0052550c: u32;
    static mut DAT_0073277c: u16;
    static mut DAT_005369e0: [u16; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00461ff0(param_1: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004651f0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004171d0() {
    let mut local_2: i16 = 10;
    let mut u_var5: u32 = 0;
    let mut u_var6: u16 = 0;
    // SAFETY: the globals and callees are the game's own memory and code;
    // the pointer arithmetic mirrors the original's reads and writes.
    unsafe {
        loop {
            let c1 = (&raw const DAT_00523c39)
                .cast::<i8>()
                .add(u_var5 as usize)
                .read();
            let c0 = (&raw const DAT_00523c38)
                .cast::<i8>()
                .add(u_var5 as usize)
                .read();
            let a = (c1 as i32).wrapping_add(DAT_00656c84 as u32 as i32);
            let i_var2 = FUN_00461ff0(a);
            let b = (c0 as i32).wrapping_add(DAT_00656c82 as u32 as i32);
            let mut u_var3: u32 = (i_var2 as u32).wrapping_add((b as u32).wrapping_mul(2));
            if DAT_0052552c != 0x13 && DAT_00525534 == 0xa0000 {
                let divisor = DAT_0052550c;
                let rem: u32;
                core::arch::asm!(
                    "div {d}",
                    d = in(reg) divisor,
                    inout("eax") u_var3 => _,
                    inout("edx") 0u32 => rem,
                );
                u_var3 = rem;
                FUN_004651f0();
                FUN_004651f0();
            }
            let u_var1 = DAT_0073277c;
            u_var5 = u_var5.wrapping_add(2);
            let addr = u_var3.wrapping_add(DAT_00525534 & 0xffff_fffe);
            let p = addr as *mut u16;
            (&raw mut DAT_005369e0)
                .cast::<u16>()
                .add(u_var6 as usize)
                .write_unaligned(p.read_unaligned());
            p.write_unaligned(u_var1);
            local_2 = local_2.wrapping_sub(1);
            u_var6 = u_var6.wrapping_add(1);
            if local_2 == 0 {
                break;
            }
        }
    }
}
