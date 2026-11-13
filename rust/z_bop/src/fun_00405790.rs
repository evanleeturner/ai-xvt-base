// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0051c573: [i8; 1];
    static mut DAT_00a08104: [i8; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00405790(_param_1: u32, param_2: u32, param_3: i16) {
    // SAFETY: both globals are the game's own memory; the offsets computed
    // here stay inside the game's tables, read and written as the original does.
    unsafe {
        let base = (&raw mut DAT_00a08104).cast::<u8>();
        let tbl = (&raw mut DAT_0051c573).cast::<u8>();
        let idx = u32::from(base.byte_add(4).read()) as usize;
        let i_var2: i32 = tbl
            .byte_add(idx.wrapping_mul(0x101))
            .cast::<i32>()
            .read_unaligned();
        let i_var1: u32 = (param_2 & 0xffff).wrapping_mul(4);
        let p = base
            .byte_add(0x211usize.wrapping_add(i_var1 as usize))
            .cast::<i32>();

        let mut tmp: i32 = p.read_unaligned();
        tmp = tmp.wrapping_add(param_3 as i32);
        p.write_unaligned(tmp);

        tmp = p.read_unaligned();
        if tmp < 0 {
            tmp = 0;
            p.write_unaligned(tmp);
        }

        tmp = p.read_unaligned();
        if i_var2.wrapping_mul(2) < tmp {
            tmp = i_var2.wrapping_mul(2);
            p.write_unaligned(tmp);
        }
    }
}
