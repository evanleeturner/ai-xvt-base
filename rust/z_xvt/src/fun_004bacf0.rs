// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0085e771: [i8; 0];
    static mut DAT_0081bcf0: [i8; 0];
    static mut DAT_0085e77d: u8;
    static mut DAT_00525164: [i8; 0];

    static mut DAT_0085e6d0: [u8; 2];
    static mut DAT_0085e6d2: [u8; 2];
    static mut DAT_0085e6d4: [u8; 2];
    static mut DAT_0085e6d6: [u8; 2];
    static mut DAT_0085e6d8: [u8; 2];
    static mut DAT_0085e6da: [u8; 2];
    static mut DAT_0085e6dc: [u8; 2];
    static mut DAT_0085e6de: [u8; 2];
    static mut DAT_0085e6e0: [u8; 2];
    static mut DAT_0085e6e2: [u8; 2];
    static mut DAT_0085e6e4: [u8; 2];
    static mut DAT_0085e6e6: [u8; 2];
    static mut DAT_0085e6e8: [u8; 2];
    static mut DAT_0085e6ea: [u8; 2];
    static mut DAT_0085e6ec: [u8; 2];
    static mut DAT_0085e6ee: [u8; 2];

    static mut DAT_0085e6f0: u8;
    static mut DAT_0085e6f1: [i8; 0];
    static mut DAT_0085e731: [i8; 0];
    static mut DAT_0085e790: u8;
    static mut DAT_0085e791: u8;
    static mut DAT_0085e792: u8;
    static mut DAT_0085e793: u8;
    static mut DAT_0085e7c5: u8;
    static mut DAT_0085e794: u8;
    static mut DAT_0085e795: u8;
    static mut DAT_0085e796: u8;
    static mut DAT_0085e797: u8;
    static mut DAT_0085e798: u8;
    static mut DAT_0085e799: u8;
    static mut DAT_0085e79a: u8;
    static mut DAT_0085e79b: u8;
    static mut DAT_0085e79c: u8;
    static mut DAT_0085e79d: u8;
    static mut DAT_0085e79e: u8;
    static mut DAT_0085e79f: u8;
    static mut DAT_0085e7a0: u8;
    static mut DAT_0085e7a1: u8;
    static mut DAT_0085e7a2: u8;
    static mut DAT_0085e7a3: u8;
    static mut DAT_0085e7a4: u32;
    static mut DAT_0085e7a8: u32;
    static mut DAT_0085e7ac: u32;
    static mut DAT_0085e7b0: u32;
    static mut DAT_0085e7b4: u8;
    static mut DAT_0085e7b5: u8;
    static mut DAT_0085e7b6: u8;
    static mut DAT_0085e7b7: u8;
    static mut DAT_0085e7b8: u8;
    static mut DAT_0085e7b9: u8;
    static mut DAT_0085e7ba: u8;
    static mut DAT_0085e7bb: u8;
    static mut DAT_0085e7bc: u8;
    static mut DAT_0085e7bd: u8;
    static mut DAT_0085e7be: u8;
    static mut DAT_0085e7bf: u8;
    static mut DAT_0085e7c0: u32;
    static mut DAT_0085e77f: [i8; 0];
    static mut DAT_0085e78f: u8;
    static mut DAT_0085e7c4: u8;
    static mut DAT_0085e7c6: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _strncpy(dst: *mut i8, src: *const i8, n: u32) -> *mut i8;
    fn FUN_004be160(name: *const i8, mode: *const i8) -> *mut core::ffi::c_void;
    fn _fprintf(file: *mut core::ffi::c_void, fmt: *const i8, ...) -> i32;
    fn FUN_004be1f0(file: *mut core::ffi::c_void);
}

macro_rules! pf {
    ($f:expr, $fmt:literal $(, $a:expr)*) => {
        _fprintf($f, concat!($fmt, "\0").as_ptr().cast::<i8>() $(, $a)*)
    };
}

macro_rules! at {
    ($sym:ident, $i:expr) => {
        (&raw mut $sym).cast::<u8>().add($i as usize).read() as u32
    };
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004bacf0() {
    // SAFETY: all globals and callees are the game's own; array indices are
    // 0 or 1 within the two-byte arrays, and strings are NUL-terminated.
    unsafe {
        _strncpy(
            (&raw mut DAT_0085e771).cast::<i8>(),
            (&raw mut DAT_0081bcf0).cast::<i8>() as *const i8,
            0xc,
        );
        DAT_0085e77d = 0;
        let file = FUN_004be160(
            c"config.cfg".as_ptr(),
            (&raw mut DAT_00525164).cast::<i8>() as *const i8,
        );
        if file.is_null() {
            return;
        }
        pf!(file, "lastpilot %s\n", (&raw mut DAT_0081bcf0).cast::<i8>());
        let mut i2: i32 = 0;
        loop {
            let i1 = i2.wrapping_add(1);
            pf!(file, "backdrop%d %d\n", i1, at!(DAT_0085e6d0, i2));
            pf!(file, "stardensity%d %d\n", i1, at!(DAT_0085e6d2, i2));
            pf!(file, "debris%d %d\n", i1, at!(DAT_0085e6d4, i2));
            pf!(file, "locallights%d %d\n", i1, at!(DAT_0085e6d6, i2));
            pf!(file, "specular%d %d\n", i1, at!(DAT_0085e6d8, i2));
            pf!(file, "diffuse%d %d\n", i1, at!(DAT_0085e6da, i2));
            pf!(file, "dither%d %d\n", i1, at!(DAT_0085e6dc, i2));
            pf!(file, "textureres%d %d\n", i1, at!(DAT_0085e6de, i2));
            pf!(file, "mipmap%d %d\n", i1, at!(DAT_0085e6e0, i2));
            pf!(file, "lod%d %d\n", i1, at!(DAT_0085e6e2, i2));
            pf!(file, "screenres%d %d\n", i1, at!(DAT_0085e6e4, i2));
            pf!(file, "windowsize%d %d\n", i1, at!(DAT_0085e6e6, i2));
            pf!(file, "bpp%d %d\n", i1, at!(DAT_0085e6e8, i2));
            pf!(file, "brightness%d %d\n", i1, at!(DAT_0085e6ea, i2));
            pf!(file, "use_3d_hardware%d %d\n", i1, at!(DAT_0085e6ec, i2));
            pf!(file, "bilinear%d %d\n", i1, at!(DAT_0085e6ee, i2));
            i2 = i1;
            if !(i1 < 2) {
                break;
            }
        }
        pf!(file, "networktype %d\n", DAT_0085e6f0 as u32);
        pf!(
            file,
            "phonenumber %s\n",
            (&raw mut DAT_0085e6f1).cast::<i8>()
        );
        pf!(file, "ipaddress %s\n", (&raw mut DAT_0085e731).cast::<i8>());
        pf!(file, "server_update_rate %d\n", DAT_0085e790 as u32);
        pf!(file, "sfx_exterior %d\n", DAT_0085e791 as u32);
        pf!(file, "sfx_interior %d\n", DAT_0085e792 as u32);
        pf!(file, "sfx_engine %d\n", DAT_0085e793 as u32);
        pf!(file, "sfx_datapad %d\n", DAT_0085e7c5 as u32);
        pf!(file, "voice_pilot %d\n", DAT_0085e794 as u32);
        pf!(file, "voice_tactical_officer %d\n", DAT_0085e795 as u32);
        pf!(file, "voice_commander %d\n", DAT_0085e796 as u32);
        pf!(file, "voice_special %d\n", DAT_0085e797 as u32);
        pf!(file, "music %d\n", DAT_0085e798 as u32);
        pf!(file, "sfx_datapad_volume %d\n", DAT_0085e799 as u32);
        pf!(file, "sfx_exterior_volume %d\n", DAT_0085e79a as u32);
        pf!(file, "sfx_interior_volume %d\n", DAT_0085e79b as u32);
        pf!(file, "sfx_engine_volume %d\n", DAT_0085e79c as u32);
        pf!(file, "voice_volume %d\n", DAT_0085e79d as u32);
        pf!(file, "music_volume %d\n", DAT_0085e79e as u32);
        pf!(file, "datapad_music %d\n", DAT_0085e79f as u32);
        pf!(file, "joybutton1 %d\n", DAT_0085e7a0 as u32);
        pf!(file, "joybutton2 %d\n", DAT_0085e7a1 as u32);
        pf!(file, "joybutton3 %d\n", DAT_0085e7a2 as u32);
        pf!(file, "joybutton4 %d\n", DAT_0085e7a3 as u32);
        pf!(file, "joybutton5 %d\n", (DAT_0085e7a4 as u8) as u32);
        pf!(file, "joybutton6 %d\n", ((DAT_0085e7a4 >> 8) as u8) as u32);
        pf!(file, "joybutton7 %d\n", ((DAT_0085e7a4 >> 16) as u8) as u32);
        pf!(file, "joybutton8 %d\n", ((DAT_0085e7a4 >> 24) as u8) as u32);
        pf!(file, "joybutton9 %d\n", (DAT_0085e7a8 as u8) as u32);
        pf!(file, "joybutton10 %d\n", ((DAT_0085e7a8 >> 8) as u8) as u32);
        pf!(
            file,
            "joybutton11 %d\n",
            ((DAT_0085e7a8 >> 16) as u8) as u32
        );
        pf!(
            file,
            "joybutton12 %d\n",
            ((DAT_0085e7a8 >> 24) as u8) as u32
        );
        pf!(file, "joybutton13 %d\n", (DAT_0085e7ac as u8) as u32);
        pf!(file, "joybutton14 %d\n", ((DAT_0085e7ac >> 8) as u8) as u32);
        pf!(
            file,
            "joybutton15 %d\n",
            ((DAT_0085e7ac >> 16) as u8) as u32
        );
        pf!(
            file,
            "joybutton16 %d\n",
            ((DAT_0085e7ac >> 24) as u8) as u32
        );
        pf!(file, "joybutton17 %d\n", (DAT_0085e7b0 as u8) as u32);
        pf!(file, "joybutton18 %d\n", ((DAT_0085e7b0 >> 8) as u8) as u32);
        pf!(
            file,
            "joybutton19 %d\n",
            ((DAT_0085e7b0 >> 16) as u8) as u32
        );
        pf!(
            file,
            "joybutton20 %d\n",
            ((DAT_0085e7b0 >> 24) as u8) as u32
        );
        pf!(file, "difficulty %d\n", DAT_0085e7b4 as u32);
        pf!(file, "collisions %d\n", DAT_0085e7b5 as u32);
        pf!(file, "craft_jumping %d\n", DAT_0085e7b6 as u32);
        pf!(file, "random_setup %d\n", DAT_0085e7b7 as u32);
        pf!(file, "handicapping %d\n", DAT_0085e7b8 as u32);
        pf!(file, "require_password %d\n", DAT_0085e7b9 as u32);
        pf!(file, "in_progress_join %d\n", DAT_0085e7ba as u32);
        pf!(file, "craft_selection %d\n", DAT_0085e7bb as u32);
        pf!(file, "locate_players %d\n", DAT_0085e7bc as u32);
        pf!(file, "craft_waves %d\n", DAT_0085e7bd as u32);
        pf!(file, "mission_time_limit %d\n", DAT_0085e7be as u32);
        pf!(file, "last_time_limit %d\n", DAT_0085e7bf as u32);
        pf!(file, "random_seed %d\n", DAT_0085e7c0);
        pf!(file, "password %s\n", (&raw mut DAT_0085e77f).cast::<i8>());
        pf!(file, "async_flag %d\n", DAT_0085e78f as u32);
        pf!(file, "ai_opponents %d\n", DAT_0085e7c4 as u32);
        pf!(file, "help_on %d\n", DAT_0085e7c6 as u32);
        FUN_004be1f0(file);
    }
}
