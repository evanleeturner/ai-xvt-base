// SPDX-License-Identifier: MIT

unsafe fn cmp(mut a: *const u8, mut b: *const u8) -> i32 {
    // SAFETY: a and b point at NUL-terminated strings owned by the game.
    unsafe {
        loop {
            let mut c = a.read();
            let d = b.read();
            if c != d {
                return if c < d { -1 } else { 1 };
            }
            if c == 0 {
                return 0;
            }
            c = a.add(1).read();
            let d = b.add(1).read();
            if c != d {
                return if c < d { -1 } else { 1 };
            }
            a = a.add(2);
            b = b.add(2);
            if c == 0 {
                return 0;
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dfc70(param_1: *mut u32, param_2: *mut u32) {
    // SAFETY: param_1 is the game's list header (head, count) and param_2 a
    // node (key, next); all pointers are the game's own 32-bit addresses.
    unsafe {
        let mut node = param_1.read_unaligned() as usize as *mut u32;
        let key = param_2.read_unaligned() as usize as *const u8;
        let nkey = node.read_unaligned() as usize as *const u8;
        if cmp(key, nkey) < 0 {
            param_2.add(1).write_unaligned(node as usize as u32);
            param_1.write_unaligned(param_2 as usize as u32);
            let c = param_1.add(1).read_unaligned();
            param_1.add(1).write_unaligned(c.wrapping_add(1));
            return;
        }
        let mut next = node.add(1).read_unaligned();
        loop {
            if next == 0 {
                node.add(1).write_unaligned(param_2 as usize as u32);
                let c = param_1.add(1).read_unaligned();
                param_1.add(1).write_unaligned(c.wrapping_add(1));
                return;
            }
            let key = param_2.read_unaligned() as usize as *const u8;
            let nnode = next as usize as *const u32;
            let nkey = nnode.read_unaligned() as usize as *const u8;
            if cmp(key, nkey) < 0 {
                param_2.add(1).write_unaligned(next);
                node.add(1).write_unaligned(param_2 as usize as u32);
                let c = param_1.add(1).read_unaligned();
                param_1.add(1).write_unaligned(c.wrapping_add(1));
                return;
            }
            node = next as usize as *mut u32;
            next = node.add(1).read_unaligned();
        }
    }
}
