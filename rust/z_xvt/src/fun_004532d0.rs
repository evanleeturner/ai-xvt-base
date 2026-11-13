// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048e480(param_1: i32, param_2: *mut i8) -> *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004532d0(param_1: *mut u32, param_2: i32, param_3: u32, param_4: u32) {
    let mut param_1 = param_1;
    if param_1.is_null() {
        return;
    }
    let dbl = f64::from_bits(((param_4 as u64) << 32) | (param_3 as u64));
    // SAFETY: param_1 and the structures it links to are the game's own
    // memory, read and written at the offsets the original uses.
    unsafe {
        while param_1.add(1).read_unaligned() == 7 {
            param_1 = FUN_0048e480(param_2, param_1.add(5).read_unaligned() as usize as *mut i8);
            if param_1.is_null() {
                return;
            }
        }
        let kind = param_1.add(1).read_unaligned() as i32;
        if kind == 1 {
            let mut n = param_1.add(4).read_unaligned() as i32;
            if 0 < n {
                let f = dbl as f32;
                let mut p = (param_1
                    .add(5)
                    .read_unaligned()
                    .wrapping_add(4)
                    .wrapping_add((n as u32).wrapping_mul(0x4c)))
                    as usize as *mut f32;
                loop {
                    n = n.wrapping_sub(1);
                    for k in 0..5 {
                        let q = p.add(k);
                        q.write_unaligned(q.read_unaligned() * f);
                    }
                    if n == 0 {
                        break;
                    }
                    let q = p.add(5);
                    q.write_unaligned(q.read_unaligned() * f);
                    p = p.add(6);
                }
                let q = p.add(5);
                q.write_unaligned(q.read_unaligned() * f);
            }
        } else if kind == 3 {
            let mut n = param_1.add(4).read_unaligned() as i32;
            if 0 < n {
                let f = dbl as f32;
                let mut p = param_1.add(5).read_unaligned() as usize as *mut f32;
                loop {
                    n = n.wrapping_sub(1);
                    for k in 0..2 {
                        let q = p.add(k);
                        q.write_unaligned(q.read_unaligned() * f);
                    }
                    if n == 0 {
                        break;
                    }
                    let q = p.add(2);
                    q.write_unaligned(q.read_unaligned() * f);
                    p = p.add(3);
                }
                let q = p.add(2);
                q.write_unaligned(q.read_unaligned() * f);
            }
        } else if kind == 0x15 {
            let mut n = param_1.add(2).read_unaligned() as i32;
            if 0 < n {
                let mut p = param_1.add(5).read_unaligned() as usize as *mut f32;
                let mut f1: f32;
                loop {
                    f1 = p.read_unaligned() / (dbl as f32);
                    n = n.wrapping_sub(1);
                    if n == 0 {
                        break;
                    }
                    p.write_unaligned(f1);
                    p = p.add(1);
                }
                p.write_unaligned(f1);
            }
        }
        let mut off: u32 = 0;
        let mut i: i32 = 0;
        if 0 < (param_1.add(2).read_unaligned() as i32) {
            loop {
                off = off.wrapping_add(4);
                i = i.wrapping_add(1);
                let base = param_1.add(3).read_unaligned();
                let child = (base.wrapping_add(off).wrapping_sub(4)) as usize as *const u32;
                FUN_004532d0(
                    child.read_unaligned() as usize as *mut u32,
                    param_2,
                    param_3,
                    param_4,
                );
                if !(i < (param_1.add(2).read_unaligned() as i32)) {
                    break;
                }
            }
        }
    }
}
