// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732f70: u32;
    static mut DAT_00732f80: u32;
    static mut DAT_00732f50: u32;
    static mut DAT_006db6d8: u16;
    static mut DAT_0071f2d0: u16;
    static mut DAT_00732570: u16;
    static mut DAT_00732f74: u32;
    static mut DAT_00732f84: u32;
    static mut DAT_00732f54: u32;
    static mut DAT_006db6da: u16;
    static mut DAT_0071f2d2: u16;
    static mut DAT_00732572: u16;
    static mut DAT_0071b730: i32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_006d4988: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00401000() {
    // SAFETY: all globals are the game's own memory; offsets and pointers
    // are dereferenced exactly as the original does.
    unsafe {
        let mut u_var4: u32 = 2;
        loop {
            let o4 = u_var4.wrapping_mul(4) as usize;
            let o2 = u_var4.wrapping_mul(2) as usize;

            let v = (&raw mut DAT_00732f70)
                .cast::<u8>()
                .byte_add(o4)
                .cast::<u32>()
                .read_unaligned();
            (&raw mut DAT_00732f74)
                .cast::<u8>()
                .byte_add(o4)
                .cast::<u32>()
                .write_unaligned(v);

            let u_var1 = (&raw mut DAT_006db6d8)
                .cast::<u8>()
                .byte_add(o2)
                .cast::<u16>()
                .read_unaligned();

            let v = (&raw mut DAT_00732f80)
                .cast::<u8>()
                .byte_add(o4)
                .cast::<u32>()
                .read_unaligned();
            (&raw mut DAT_00732f84)
                .cast::<u8>()
                .byte_add(o4)
                .cast::<u32>()
                .write_unaligned(v);

            let u_var2 = (&raw mut DAT_0071f2d0)
                .cast::<u8>()
                .byte_add(o2)
                .cast::<u16>()
                .read_unaligned();

            let v = (&raw mut DAT_00732f50)
                .cast::<u8>()
                .byte_add(o4)
                .cast::<u32>()
                .read_unaligned();
            (&raw mut DAT_00732f54)
                .cast::<u8>()
                .byte_add(o4)
                .cast::<u32>()
                .write_unaligned(v);

            (&raw mut DAT_006db6da)
                .cast::<u8>()
                .byte_add(o2)
                .cast::<u16>()
                .write_unaligned(u_var1);
            let s_var3 = u_var4 as i16;
            (&raw mut DAT_0071f2d2)
                .cast::<u8>()
                .byte_add(o2)
                .cast::<u16>()
                .write_unaligned(u_var2);

            let v = (&raw mut DAT_00732570)
                .cast::<u8>()
                .byte_add(o2)
                .cast::<u16>()
                .read_unaligned();
            (&raw mut DAT_00732572)
                .cast::<u8>()
                .byte_add(o2)
                .cast::<u16>()
                .write_unaligned(v);

            u_var4 = s_var3.wrapping_sub(1) as u16 as u32;
            if s_var3 == 0 {
                break;
            }
        }

        let base = (&raw mut DAT_0071b730)
            .cast::<u8>()
            .byte_offset(DAT_0071ecf4.wrapping_mul(0x5bd) as isize)
            .cast::<i32>()
            .read_unaligned();
        let i_var5 = base.wrapping_mul(0x23).wrapping_add(DAT_006d4988);

        let p = (i_var5.wrapping_add(0x1f) as usize as *const i32).read_unaligned();
        DAT_00732f70 = ((p.wrapping_add(6)) as usize as *const u32).read_unaligned();
        let p = (i_var5.wrapping_add(0x1f) as usize as *const i32).read_unaligned();
        DAT_00732f80 = ((p.wrapping_add(10)) as usize as *const u32).read_unaligned();
        let p = (i_var5.wrapping_add(0x1f) as usize as *const i32).read_unaligned();
        DAT_00732f50 = ((p.wrapping_add(0xe)) as usize as *const u32).read_unaligned();
        DAT_006db6d8 = (i_var5.wrapping_add(0x14) as usize as *const u16).read_unaligned();
        DAT_0071f2d0 = (i_var5.wrapping_add(0x12) as usize as *const u16).read_unaligned();
        DAT_00732570 = (i_var5.wrapping_add(0x10) as usize as *const u16).read_unaligned();
    }
}
