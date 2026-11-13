// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525738: i32;
    static mut DAT_00732e42: u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004068d0(param_1: u16, param_2: i32, param_3: *mut u16) {
    // SAFETY: param_3 and param_2 point at the caller's buffers, accessed at the
    // same offsets as the original; the global is the game's own.
    unsafe {
        let b = param_3 as *mut u8;
        let p2 = param_2 as usize as *const u8;
        *param_3 = param_1;
        let puvar1 = param_3.add(0x204);
        if DAT_00525738 == 1 {
            puvar1.write_unaligned(0x100);
            param_3.add(0x205).write_unaligned(0x100);
            DAT_00732e42 = 0x2000;
        } else {
            puvar1.write_unaligned(0xe9);
            param_3.add(0x205).write_unaligned(0x11a);
            DAT_00732e42 = 0x2200;
        }
        let u_var3: u32 =
            (p2.add(10).cast::<u16>().read_unaligned() as u32).wrapping_mul(param_1 as u32);
        let mut u_var4: u32 = u_var3 >> 16;
        b.add(4).write(u_var4 as u8);
        b.add(5).write((u_var3 >> 24) as u8);
        if p2.add(0x18).cast::<i16>().read_unaligned() != 0 {
            let t = (puvar1.read_unaligned() as u32).wrapping_mul(u_var4);
            b.add(4).write((t >> 8) as u8);
            b.add(5).write((t >> 16) as u8);
        }
        u_var4 = u_var4.wrapping_add(
            (p2.add(0x1930).cast::<u16>().read_unaligned() as u32).wrapping_mul(u_var4) >> 8,
        );
        if p2.add(0x1a).cast::<i16>().read_unaligned() == 0 {
            u_var4 = (param_3.add(0x205).read_unaligned() as u32).wrapping_mul(u_var4) >> 8;
        }
        let b_var2: u8 = b.add(4).read();
        b.add(6).write(u_var4 as u8);
        b.add(7).write((u_var4 >> 8) as u8);
        if b.add(2).read() != b_var2 || b.add(3).read() != b.add(5).read() {
            b.add(2).write(b_var2);
            b.add(3).write(b.add(5).read());
            let u_var4: u32 = ((b.add(5).read() as u32) << 16) | ((b_var2 as u32) << 8);
            let mut u_var3 = u_var4;
            let mut u_var5: u16 = 0;
            loop {
                let u_var6 = u_var5.wrapping_add(1);
                param_3
                    .add(u_var5 as usize + 4)
                    .write_unaligned(u_var3 as u16);
                param_3
                    .add(u_var5 as usize + 0x104)
                    .write_unaligned((u_var3 >> 16) as u16);
                u_var3 = u_var3.wrapping_add(u_var4);
                u_var5 = u_var6;
                if !(u_var6 < 0x100) {
                    break;
                }
            }
        }
    }
}
