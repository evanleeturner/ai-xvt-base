// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00458780(param_1: i32) {
    // SAFETY: param_1 points at the caller's object; the pointer slots at
    // offsets 0x45e and 0x41e..0x45e are read and cleared where the original
    // does, and each non-null slot points at memory whose byte 3 is cleared.
    unsafe {
        let base = param_1 as usize as *mut u8;

        let slot = base.byte_add(0x45e);
        let mut local_val: i32 = slot.cast::<i32>().read_unaligned();
        if local_val != 0 {
            ((local_val as usize) as *mut u8).byte_add(3).write(0);
            local_val = 0;
            slot.cast::<i32>().write_unaligned(local_val);
        }

        let mut p_base = base.byte_add(0x41e);
        let mut i_var1: i32 = 0x10;
        loop {
            let mut v: i32 = p_base.cast::<i32>().read_unaligned();
            if v != 0 {
                ((v as usize) as *mut u8).byte_add(3).write(0);
                v = 0;
                p_base.cast::<i32>().write_unaligned(v);
            }
            p_base = p_base.byte_add(4);
            i_var1 = i_var1.wrapping_sub(1);
            if i_var1 == 0 {
                break;
            }
        }
    }
}
