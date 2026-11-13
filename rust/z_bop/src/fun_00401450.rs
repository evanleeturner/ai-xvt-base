// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052361c: i32;
    static mut DAT_009ed664: u16;
    static mut DAT_009a1fe8: u32;
    static mut DAT_009fd39c: i32;
    static mut DAT_009e9a5b: i32;
    static mut DAT_009ecc34: u32;
    static mut DAT_009fd3a0: i32;
    static mut DAT_009e9a5f: i32;
    static mut DAT_009fd398: i32;
    static mut DAT_009e9a63: i32;
    static mut DAT_009a8e34: u32;
    static mut DAT_00521f1c: u32;
    static mut DAT_00521f24: u16;
    static mut DAT_00527e90: i32;
    static mut unaff_retaddr: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004243d0(param_1: i32, param_2: u32, param_3: u32) -> i32;
    fn FUN_004ac720(param_1: u32) -> i32;
    fn FUN_0040bbf0(param_1: u16, param_2: i32, param_3: i32, param_4: u32, param_5: i32);
    fn FUN_00421850(param_1: u16);
    fn FUN_00421930(param_1: i32);
    fn FUN_00421560(param_1: u16, param_2: u16, param_3: u16, param_4: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004ac740();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00401450(param_1: *mut u16) -> () {
    // SAFETY: all globals and callees are the game's own; param_1 and the
    // computed addresses point into the game's tables, read as the original does.
    unsafe {
        let u_var1: u16 = param_1.add(1).read_unaligned();
        DAT_0052361c = 1;
        let u_var3: u32 = ((u_var1 & 0x7fff) >> 7) as u32;
        DAT_009ed664 = param_1.read_unaligned();
        let i_var4: u32 = (DAT_009ed664 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(DAT_009a1fe8);

        let off = DAT_009ecc34.wrapping_mul(0x5bd) as usize;

        let tmp_9e9a5b: i32 = (&raw mut DAT_009e9a5b)
            .cast::<u8>()
            .byte_add(off)
            .cast::<i32>()
            .read_unaligned();
        DAT_009fd39c = ((i_var4.wrapping_add(4)) as usize as *const i32)
            .read_unaligned()
            .wrapping_sub(tmp_9e9a5b);

        let off = DAT_009ecc34.wrapping_mul(0x5bd) as usize;
        let tmp_9e9a5f: i32 = (&raw mut DAT_009e9a5f)
            .cast::<u8>()
            .byte_add(off)
            .cast::<i32>()
            .read_unaligned();
        let mut i_var2: i32 = tmp_9e9a5f;
        DAT_009fd3a0 = ((i_var4.wrapping_add(8)) as usize as *const i32)
            .read_unaligned()
            .wrapping_sub(i_var2);

        let off = DAT_009ecc34.wrapping_mul(0x5bd) as usize;
        let tmp_9e9a63: i32 = (&raw mut DAT_009e9a63)
            .cast::<u8>()
            .byte_add(off)
            .cast::<i32>()
            .read_unaligned();
        DAT_009fd398 = ((i_var4.wrapping_add(0xc)) as usize as *const i32)
            .read_unaligned()
            .wrapping_sub(tmp_9e9a63);

        let tmp_param_5: u32 = param_1
            .cast::<u8>()
            .byte_add(10)
            .cast::<u32>()
            .read_unaligned();
        DAT_009a8e34 = tmp_param_5;

        let tmp_521f1c: u32 = (&raw mut DAT_00521f1c)
            .cast::<u8>()
            .byte_add(u_var3.wrapping_mul(0x1a) as usize)
            .cast::<u32>()
            .read_unaligned();
        let i_var4: i32 = FUN_004243d0(
            tmp_param_5 as i32,
            tmp_521f1c,
            param_1.add(2).read_unaligned() as u32,
        );

        let tmp_521f24: u16 = (&raw mut DAT_00521f24)
            .cast::<u8>()
            .byte_add(u_var3.wrapping_mul(0xd) as usize)
            .cast::<u16>()
            .read_unaligned();
        i_var2 = FUN_004ac720(((i_var2 as u32) & 0xFFFF0000) | (tmp_521f24 as u32));

        FUN_004ac740();

        let a: i32 = (i_var2.wrapping_add(0x10) as usize as *const i32).read_unaligned();
        let idx: i32 = (((u_var1 & 0x7fff) & 0x7f) as i32).wrapping_mul(4);
        let addr: i32 = a.wrapping_add(idx).wrapping_add(i_var2);
        i_var2 = (addr as usize as *const i32)
            .read_unaligned()
            .wrapping_add(i_var2);

        if DAT_00527e90 != 0 {
            FUN_0040bbf0(
                param_1.add(7).read_unaligned(),
                param_1.add(3).read_unaligned() as i16 as i32,
                param_1.add(4).read_unaligned() as i16 as i32,
                ((unaff_retaddr as u32) << 16) | (i_var4 as u16 as u32),
                i_var2,
            );
            return;
        }
        FUN_00421850(param_1.add(7).read_unaligned());
        FUN_00421930(i_var2);
        FUN_00421560(
            param_1.add(3).read_unaligned(),
            param_1.add(4).read_unaligned(),
            i_var4 as u16,
            i_var2,
        );
    }
}
