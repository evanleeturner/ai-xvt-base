// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00479de0(param_1: i32, param_2: *mut i8) -> *mut u32;
}

unsafe fn rdf(p: *mut f32, i: usize) -> f32 {
    // SAFETY: caller guarantees p+i points at game memory holding a float.
    unsafe { p.add(i).read_unaligned() }
}

unsafe fn wrf(p: *mut f32, i: usize, v: f32) {
    // SAFETY: caller guarantees p+i points at writable game memory.
    unsafe { p.add(i).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042aa60(param_1: *mut u32, param_2: i32, param_3: u32, param_4: u32) {
    let mut param_1 = param_1;
    if param_1.is_null() {
        return;
    }
    let d = f64::from_bits(((param_4 as u64) << 32) | (param_3 as u64));
    // SAFETY: param_1 and the structures it links to are the game's own
    // memory, read and written where the original does.
    unsafe {
        while param_1.add(1).read_unaligned() == 7 {
            param_1 = FUN_00479de0(param_2, param_1.add(5).read_unaligned() as *mut i8);
            if param_1.is_null() {
                return;
            }
        }
        let t = param_1.add(1).read_unaligned() as i32;
        if t == 1 {
            let mut i3 = param_1.add(4).read_unaligned() as i32;
            if 0 < i3 {
                let f1 = d as f32;
                let mut p = param_1
                    .add(5)
                    .read_unaligned()
                    .wrapping_add(4)
                    .wrapping_add((i3 as u32).wrapping_mul(0x4c))
                    as *mut f32;
                loop {
                    i3 = i3.wrapping_sub(1);
                    wrf(p, 0, rdf(p, 0) / f1);
                    wrf(p, 1, rdf(p, 1) / f1);
                    wrf(p, 2, rdf(p, 2) / f1);
                    wrf(p, 3, rdf(p, 3) / f1);
                    wrf(p, 4, rdf(p, 4) / f1);
                    if i3 == 0 {
                        break;
                    }
                    wrf(p, 5, rdf(p, 5) / f1);
                    p = p.wrapping_add(6);
                }
                wrf(p, 5, rdf(p, 5) / f1);
            }
        } else if t == 3 {
            let mut i3 = param_1.add(4).read_unaligned() as i32;
            if 0 < i3 {
                let f1 = d as f32;
                let mut p = param_1.add(5).read_unaligned() as *mut f32;
                loop {
                    i3 = i3.wrapping_sub(1);
                    wrf(p, 0, rdf(p, 0) / f1);
                    wrf(p, 1, rdf(p, 1) / f1);
                    if i3 == 0 {
                        break;
                    }
                    wrf(p, 2, rdf(p, 2) / f1);
                    p = p.wrapping_add(3);
                }
                wrf(p, 2, rdf(p, 2) / f1);
            }
        } else if t == 0x15 {
            let mut i3 = param_1.add(2).read_unaligned() as i32;
            if 0 < i3 {
                let mut p = param_1.add(5).read_unaligned() as *mut f32;
                let mut f1;
                loop {
                    f1 = rdf(p, 0) * (d as f32);
                    i3 = i3.wrapping_sub(1);
                    if i3 == 0 {
                        break;
                    }
                    wrf(p, 0, f1);
                    p = p.wrapping_add(1);
                }
                wrf(p, 0, f1);
            }
        }
        let mut i4: u32 = 0;
        let mut i3: i32 = 0;
        if 0 < (param_1.add(2).read_unaligned() as i32) {
            loop {
                i4 = i4.wrapping_add(4);
                i3 = i3.wrapping_add(1);
                let addr = param_1
                    .add(3)
                    .read_unaligned()
                    .wrapping_sub(4)
                    .wrapping_add(i4) as *const u32;
                let child = addr.read_unaligned() as *mut u32;
                FUN_0042aa60(child, param_2, param_3, param_4);
                if !(i3 < (param_1.add(2).read_unaligned() as i32)) {
                    break;
                }
            }
        }
    }
}
