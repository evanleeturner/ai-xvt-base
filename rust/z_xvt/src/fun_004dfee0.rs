// SPDX-License-Identifier: MIT

unsafe fn cmp_str(mut p5: *const u8, mut p4: *const u8) -> i32 {
    // SAFETY: the caller passes pointers to NUL-terminated byte strings,
    // read exactly as the original does.
    unsafe {
        loop {
            let mut b1 = p5.read();
            let mut b6 = (b1 < p4.read()) as u32;
            if b1 != p4.read() {
                return (1u32.wrapping_sub(b6)).wrapping_sub((b6 != 0) as u32) as i32;
            }
            if b1 == 0 {
                break;
            }
            b1 = p5.add(1).read();
            b6 = (b1 < p4.add(1).read()) as u32;
            if b1 != p4.add(1).read() {
                return (1u32.wrapping_sub(b6)).wrapping_sub((b6 != 0) as u32) as i32;
            }
            p5 = p5.add(2);
            p4 = p4.add(2);
            if b1 == 0 {
                break;
            }
        }
        0
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dfee0(param_1: *mut u32, param_2: *mut u32) {
    // SAFETY: param_1 and param_2 point at the game's list head and node
    // records (word fields: string pointer, next pointer); they are read and
    // written where the original does.
    unsafe {
        let mut pu_var2 = param_1.read_unaligned() as usize as *mut u32;
        let s2 = param_2.read_unaligned() as usize as *const u8;
        let s1 = pu_var2.read_unaligned() as usize as *const u8;
        let mut i_var3 = cmp_str(s2, s1);
        if i_var3 < 0 {
            param_2.add(1).write_unaligned(pu_var2 as usize as u32);
            param_1.write_unaligned(param_2 as usize as u32);
            let c = param_1.add(1).read_unaligned();
            param_1.add(1).write_unaligned(c.wrapping_add(1));
            return;
        }
        i_var3 = pu_var2.add(1).read_unaligned() as i32;
        loop {
            if i_var3 == 0 {
                pu_var2.add(1).write_unaligned(param_2 as usize as u32);
                let c = param_1.add(1).read_unaligned();
                param_1.add(1).write_unaligned(c.wrapping_add(1));
                return;
            }
            let p5 = param_2.read_unaligned() as usize as *const u8;
            let next = pu_var2.add(1).read_unaligned() as usize as *const u32;
            let p4 = next.read_unaligned() as usize as *const u8;
            let r = cmp_str(p5, p4);
            if r < 0 {
                let n = pu_var2.add(1).read_unaligned();
                param_2.add(1).write_unaligned(n);
                pu_var2.add(1).write_unaligned(param_2 as usize as u32);
                let c = param_1.add(1).read_unaligned();
                param_1.add(1).write_unaligned(c.wrapping_add(1));
                return;
            }
            pu_var2 = pu_var2.add(1).read_unaligned() as usize as *mut u32;
            i_var3 = pu_var2.add(1).read_unaligned() as i32;
        }
    }
}
