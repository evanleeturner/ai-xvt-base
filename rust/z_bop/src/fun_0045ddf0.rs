// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a73c0: u16;
    static mut DAT_009a73d8: i32;
    static mut DAT_009a73da: i8;
    static mut DAT_009d1320: i8;
    static mut DAT_009a7a40: [u8; 1];
    static mut DAT_00524120: [u8; 1];
    static mut DAT_009d8cd2: [u8; 1];
    static mut s_disableldr1pln_0051a310: [i8; 1];
    static mut s_capescortersldr1pln_0051a334: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0045d1c0(param_1: u32) -> i16;
    fn FUN_0045deb0(param_1: u32) -> i16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045ddf0(param_1: u16) -> u8 {
    // SAFETY: all globals and callees belong to the game; the pointers
    // address its tables and strings, read where the original reads.
    unsafe {
        let c0 = DAT_009a73c0 as u32;
        let u_var3: u32 = ((c0.wrapping_mul(0x159) >> 16) << 16) | (param_1 as u32);
        DAT_009a73d8 = ((param_1 as u32).wrapping_mul(0x52) == 0) as i32;

        let idx = c0
            .wrapping_mul(0x566)
            .wrapping_add((param_1 as u32).wrapping_mul(0x52));
        let val1 = (&raw mut DAT_009d8cd2)
            .cast::<u8>()
            .add(idx as usize)
            .read() as u32;
        let val2 = (&raw mut DAT_00524120)
            .cast::<u8>()
            .add(val1 as usize)
            .read() as u32;
        let val3 = (&raw mut DAT_009a7a40)
            .cast::<u8>()
            .add(val2 as usize)
            .read() as u32;
        let mut pc5: *const i8 = (&raw mut DAT_009d1320)
            .cast::<i8>()
            .add(val3.wrapping_mul(0x55) as usize);

        let mut i_var2: i32 = 0xf;
        let mut pc4: *const i8 = pc5;
        let mut pc6: *const i8 = (&raw mut s_disableldr1pln_0051a310).cast::<i8>();
        loop {
            if i_var2 == 0 {
                break;
            }
            i_var2 -= 1;
            DAT_009a73d8 = (pc4.read() == pc6.read()) as i32;
            pc4 = pc4.add(1);
            pc6 = pc6.add(1);
            if DAT_009a73d8 == 0 {
                break;
            }
        }

        i_var2 = 0x14;
        DAT_009a73da = 2;
        pc4 = (&raw mut s_capescortersldr1pln_0051a334).cast::<i8>();
        let mut b_var7: bool = DAT_009a73d8 != 0;
        loop {
            if i_var2 == 0 {
                break;
            }
            i_var2 -= 1;
            b_var7 = pc5.read() == pc4.read();
            pc5 = pc5.add(1);
            pc4 = pc4.add(1);
            if !b_var7 {
                break;
            }
        }

        let s_var1 = if b_var7 {
            FUN_0045d1c0(u_var3)
        } else {
            FUN_0045deb0(u_var3)
        };
        (s_var1 != -1) as u8
    }
}
