// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525560: i32;
    static mut DAT_0070aaf0: [i32; 0];
    static mut DAT_0071b740: [i8; 0];
    static mut DAT_0071ecf4: i32;
    static mut DAT_00645a90: [i32; 0];
    static mut DAT_00645778: i32;
    static mut DAT_00645a84: i32;
    static mut DAT_0070a150: [u32; 0];
    static mut DAT_0071e528: i8;
    static mut DAT_0052555c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00432c10(param_1: i32) -> i32;
    fn FUN_0042e080(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_0042dec0(param_1: i32, param_2: i32);
    fn FUN_00431d30() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0049bde0(param_1: i32, param_2: i32) {
    // SAFETY: the globals and callees are the game's own memory; param_2 is an
    // address of a caller-owned structure, read where the original reads.
    unsafe {
        let base = param_2 as u32 as usize;
        if ((base + 4) as *const i32).read_unaligned() == DAT_00525560 {
            let i_var3 = FUN_00432c10(param_1);
            let a = (&raw mut DAT_0070aaf0).cast::<i32>();
            let c = (&raw mut DAT_0071b740).cast::<i8>();
            if a.offset(i_var3 as isize).read_unaligned() == 0
                && c.offset(i_var3.wrapping_mul(0x5bd) as isize).read() != 0
            {
                let mut b_var1: i32 = 0;
                if DAT_0071ecf4 != i_var3 {
                    let mut pi_var8 = (base + 8) as *const i32;
                    let mut i_var6: i32 = 0;
                    let mut b_var2: i32 = b_var1;
                    loop {
                        let p = (&raw mut DAT_00645a90)
                            .cast::<u8>()
                            .offset(i_var6 as isize)
                            .cast::<i32>();
                        if p.read_unaligned() != pi_var8.read_unaligned() {
                            b_var2 = 1;
                        }
                        i_var6 = i_var6.wrapping_add(4);
                        pi_var8 = pi_var8.add(1);
                        if i_var6 >= 0x40 {
                            break;
                        }
                    }
                    b_var1 = 0;
                    if b_var2 != 0 {
                        let i_var6 = FUN_0042e080(param_1, DAT_00645778, DAT_00645a84);
                        if i_var6 != 0 {
                            FUN_0042dec0(param_1, DAT_00645a84);
                        }
                        b_var1 = 1;
                    }
                }
                let i_var6 = FUN_00431d30();
                if i_var6 != 0 {
                    let arr = (&raw mut DAT_0070a150).cast::<u32>();
                    arr.offset(i_var3 as isize).write_unaligned(2);
                    if b_var1 == 0 {
                        arr.offset(i_var3 as isize).write_unaligned(1);
                    }
                    let mut u_var7: u32 = 3;
                    let mut pu_var5 = arr as *const u32;
                    let mut pc_var4 = c as *const i8;
                    let end = (&raw mut DAT_0071e528) as usize;
                    loop {
                        if pc_var4.read() != 0 {
                            u_var7 &= pu_var5.read_unaligned();
                        }
                        pu_var5 = pu_var5.add(1);
                        pc_var4 = pc_var4.wrapping_add(0x5bd);
                        if (pc_var4 as usize) >= end {
                            break;
                        }
                    }
                    if (u_var7 & 1) != 0 {
                        DAT_0052555c = 0;
                    }
                }
            }
        }
    }
}
