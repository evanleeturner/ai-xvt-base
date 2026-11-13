// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071b740: u8;
    static mut DAT_0071b743: u8;
    static mut DAT_0071b74a: u8;
    static mut DAT_0071b741: u8;
    static mut DAT_0071b730: u8;
    static mut DAT_0071b776: u8;
    static mut DAT_0071bb40: u8;
    static mut DAT_0071bb44: u8;
    static mut DAT_0071bb46: u8;
    static mut DAT_0071bb27: u8;
    static mut DAT_0071bb29: u8;
    static mut DAT_0071bb23: u8;
    static mut DAT_0071bb1b: u8;
    static mut DAT_0071bb1f: u8;
    static mut DAT_0071bcdd: u8;
    static mut DAT_0071e528: u8;
    static mut DAT_006d4988: u8;
    static mut DAT_0052686c: u8;
    static mut DAT_007092c0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040cbf0(param_1: u32, param_2: i32);
    fn FUN_0040ae30();
    fn FUN_0040acd0();
    fn FUN_0040b190();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004441e0(param_1: i32) {
    // SAFETY: all globals and callees are the game's own memory and code;
    // offsets are applied exactly as the original does.
    unsafe {
        let mut pc: *mut u8 = &raw mut DAT_0071b740;
        let i_var1 = param_1.wrapping_mul(0x5bd);
        let mut i_var2: i32 = 0;
        (&raw mut DAT_0071b740)
            .byte_offset(i_var1 as isize)
            .write(2);
        loop {
            if pc.read() == 1 {
                i_var2 = i_var2.wrapping_add(1);
            }
            pc = pc.byte_add(0x5bd);
            if !(pc < (&raw mut DAT_0071e528)) {
                break;
            }
        }

        if i_var2 != 0 {
            (&raw mut DAT_0071b743)
                .byte_offset(i_var1 as isize)
                .write(0xff);
            FUN_0040cbf0(0x15, param_1);

            (&raw mut DAT_0071bb40)
                .byte_offset(i_var1 as isize)
                .cast::<u16>()
                .write_unaligned(1);
            (&raw mut DAT_0071bb44)
                .byte_offset(i_var1 as isize)
                .cast::<u16>()
                .write_unaligned(1);
            (&raw mut DAT_0071bb46)
                .byte_offset(i_var1 as isize)
                .cast::<u32>()
                .write_unaligned(0x40000);
            (&raw mut DAT_0071bb27)
                .byte_offset(i_var1 as isize)
                .cast::<u16>()
                .write_unaligned(0xffff);
            (&raw mut DAT_0071bb29)
                .byte_offset(i_var1 as isize)
                .cast::<u16>()
                .write_unaligned(0xffff);
            (&raw mut DAT_0071bb23)
                .byte_offset(i_var1 as isize)
                .cast::<u32>()
                .write_unaligned(0x40000);

            let local_ushort: u16 = (&raw mut DAT_0071b74a)
                .byte_offset(i_var1 as isize)
                .cast::<u16>()
                .read_unaligned();
            if local_ushort != 0xffff {
                let base =
                    (&raw mut DAT_006d4988).byte_add((local_ushort as usize).wrapping_mul(0x23));
                let v4: u32 = base.byte_add(4).cast::<u32>().read_unaligned();
                (&raw mut DAT_0071bb1b)
                    .byte_offset(i_var1 as isize)
                    .cast::<u32>()
                    .write_unaligned(v4);
                let v8: u32 = base.byte_add(8).cast::<u32>().read_unaligned();
                (&raw mut DAT_0071bb1f)
                    .byte_offset(i_var1 as isize)
                    .cast::<u32>()
                    .write_unaligned(v8);

                let local_byte: u8 = base.byte_add(3).read();
                let w: u16 = (&raw mut DAT_0052686c)
                    .byte_add((local_byte as usize).wrapping_mul(0x16))
                    .cast::<u16>()
                    .read_unaligned();
                (&raw mut DAT_0071bb46)
                    .byte_offset(i_var1 as isize)
                    .cast::<u32>()
                    .write_unaligned((w as u32) << 4);
            }

            (&raw mut DAT_0071b741)
                .byte_offset(i_var1 as isize)
                .write(0);
            (&raw mut DAT_0071b730)
                .byte_offset(i_var1 as isize)
                .cast::<u32>()
                .write_unaligned(0xffff_ffff);
            (&raw mut DAT_0071bcdd)
                .byte_offset(i_var1 as isize)
                .cast::<u32>()
                .write_unaligned(0);
            (&raw mut DAT_0071b776)
                .byte_offset(i_var1 as isize)
                .write(0);

            FUN_0040ae30();
            FUN_0040acd0();
            FUN_0040b190();
            return;
        }
        DAT_007092c0 = 1;
    }
}
