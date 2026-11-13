// SPDX-License-Identifier: MIT

#[inline(always)]
unsafe fn port_in(port: u16) -> u8 {
    let v: u8;
    // SAFETY: raw port read, as the original's `in` does.
    unsafe {
        core::arch::asm!("in al, dx", in("dx") port, out("al") v, options(nomem, nostack, preserves_flags));
    }
    v
}

#[inline(always)]
unsafe fn port_out(port: u16, value: u8) {
    // SAFETY: raw port write, as the original's `out` does.
    unsafe {
        core::arch::asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0048f5c0(param_1: *mut u8, param_2: u8, param_3: i16) {
    let mut p = param_1;
    let mut i_var2: i32 = param_3 as i32;
    if i_var2 != 0 {
        // SAFETY: port I/O on the VGA registers and reads of the caller's
        // palette bytes, in the same order as the original.
        unsafe {
            loop {
                let b = port_in(0x3da);
                if (b & 8) == 0 {
                    break;
                }
            }
            loop {
                let b = port_in(0x3da);
                if (b & 8) != 0 {
                    break;
                }
            }
            port_out(0x3c8, param_2);
            loop {
                port_out(0x3c9, p.read());
                port_out(0x3c9, p.byte_add(1).read());
                port_out(0x3c9, p.byte_add(2).read());
                p = p.byte_add(3);
                i_var2 = i_var2.wrapping_sub(1);
                if i_var2 == 0 {
                    break;
                }
            }
        }
    }
}
