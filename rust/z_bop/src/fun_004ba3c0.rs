// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00bb7351: [i8; 1];
    static mut DAT_00b6a2e0: [i8; 1];
    static mut DAT_00bb735d: u8;
    static mut DAT_0052436c: [i8; 1];

    static mut DAT_00bb72b0: [u8; 1];
    static mut DAT_00bb72b2: [u8; 1];
    static mut DAT_00bb72b4: [u8; 1];
    static mut DAT_00bb72b6: [u8; 1];
    static mut DAT_00bb72b8: [u8; 1];
    static mut DAT_00bb72ba: [u8; 1];
    static mut DAT_00bb72bc: [u8; 1];
    static mut DAT_00bb72be: [u8; 1];
    static mut DAT_00bb72c0: [u8; 1];
    static mut DAT_00bb72c2: [u8; 1];
    static mut DAT_00bb72c4: [u8; 1];
    static mut DAT_00bb72c6: [u8; 1];
    static mut DAT_00bb72c8: [u8; 1];
    static mut DAT_00bb72ca: [u8; 1];
    static mut DAT_00bb72cc: [u8; 1];
    static mut DAT_00bb72ce: [u8; 1];
    static mut DAT_00bb72d0: u8;
    static mut DAT_00bb72d1: [i8; 1];
    static mut DAT_00bb7311: [i8; 1];
    static mut DAT_00bb735f: [i8; 1];
    static mut DAT_00bb736f: u8;
    static mut DAT_00bb7370: u8;
    static mut DAT_00bb7371: u8;
    static mut DAT_00bb7372: u8;
    static mut DAT_00bb7373: u8;
    static mut DAT_00bb7374: u8;
    static mut DAT_00bb7375: u8;
    static mut DAT_00bb7376: u8;
    static mut DAT_00bb7377: u8;
    static mut DAT_00bb7378: u8;
    static mut DAT_00bb7379: u8;
    static mut DAT_00bb737a: u8;
    static mut DAT_00bb737b: u8;
    static mut DAT_00bb737c: u8;
    static mut DAT_00bb737d: u8;
    static mut DAT_00bb737e: u8;
    static mut DAT_00bb737f: u8;
    static mut DAT_00bb7380: u8;
    static mut DAT_00bb7381: u8;
    static mut DAT_00bb7382: u8;
    static mut DAT_00bb7383: u8;
    static mut DAT_00bb7384: u8;
    static mut DAT_00bb7385: u8;
    static mut DAT_00bb7386: u8;
    static mut DAT_00bb7387: u8;
    static mut DAT_00bb7388: u8;
    static mut DAT_00bb7389: u8;
    static mut DAT_00bb738a: u8;
    static mut DAT_00bb738b: u8;
    static mut DAT_00bb738c: u8;
    static mut DAT_00bb738d: u8;
    static mut DAT_00bb738e: u8;
    static mut DAT_00bb738f: u8;
    static mut DAT_00bb7390: u8;
    static mut DAT_00bb7391: u8;
    static mut DAT_00bb7392: u8;
    static mut DAT_00bb7393: u8;
    static mut DAT_00bb7394: u8;
    static mut DAT_00bb7395: u8;
    static mut DAT_00bb7396: u8;
    static mut DAT_00bb7397: u8;
    static mut DAT_00bb7398: u8;
    static mut DAT_00bb7399: u8;
    static mut DAT_00bb739a: u8;
    static mut DAT_00bb739b: u8;
    static mut DAT_00bb739c: u8;
    static mut DAT_00bb739d: u8;
    static mut DAT_00bb739e: u8;
    static mut DAT_00bb739f: u8;
    static mut DAT_00bb73a0: i32;
    static mut DAT_00bb73a4: u8;
    static mut DAT_00bb73a5: u8;
    static mut DAT_00bb73a7: u8;
    static mut DAT_00bb73a8: u8;
    static mut DAT_00bb73a9: [i8; 1];
    static mut DAT_00bb73ef: [i8; 1];
    static mut DAT_00bb7435: [i8; 1];
    static mut DAT_00bb747b: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    #[link_name = "strncpy"]
    fn _strncpy(dst: *mut i8, src: *const i8, n: u32) -> *mut i8;
    fn FUN_004cc4a0(name: *const i8, mode: *const i8) -> *mut core::ffi::c_void;
    #[link_name = "fprintf"]
    fn _fprintf(file: *mut core::ffi::c_void, fmt: *const i8, ...) -> i32;
    fn FUN_004cc590(file: *mut core::ffi::c_void);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ba3c0() {
    macro_rules! pa {
        ($f:expr, $s:literal, $i1:expr, $arr:ident, $i2:expr) => {
            _fprintf(
                $f,
                concat!($s, "\0").as_ptr().cast::<i8>(),
                $i1,
                u32::from(*(&raw mut $arr).cast::<u8>().add($i2 as usize)),
            );
        };
    }
    macro_rules! ps {
        ($f:expr, $s:literal, $d:ident) => {
            _fprintf($f, concat!($s, "\0").as_ptr().cast::<i8>(), u32::from($d));
        };
    }
    macro_rules! pstr {
        ($f:expr, $s:literal, $d:ident) => {
            _fprintf(
                $f,
                concat!($s, "\0").as_ptr().cast::<i8>(),
                (&raw const $d).cast::<i8>(),
            );
        };
    }
    // SAFETY: all globals and callees are the game's own memory and code;
    // format strings are NUL-terminated and arguments match their specifiers.
    unsafe {
        _strncpy(
            (&raw mut DAT_00bb7351).cast::<i8>(),
            (&raw const DAT_00b6a2e0).cast::<i8>(),
            0xc,
        );
        DAT_00bb735d = 0;
        let f = FUN_004cc4a0(
            c"config2.cfg".as_ptr(),
            (&raw const DAT_0052436c).cast::<i8>(),
        );
        if !f.is_null() {
            pstr!(f, "lastpilot %s\n", DAT_00b6a2e0);
            let mut i2: i32 = 0;
            loop {
                let i1 = i2.wrapping_add(1);
                pa!(f, "backdrop%d %d\n", i1, DAT_00bb72b0, i2);
                pa!(f, "stardensity%d %d\n", i1, DAT_00bb72b2, i2);
                pa!(f, "debris%d %d\n", i1, DAT_00bb72b4, i2);
                pa!(f, "locallights%d %d\n", i1, DAT_00bb72b6, i2);
                pa!(f, "specular%d %d\n", i1, DAT_00bb72b8, i2);
                pa!(f, "diffuse%d %d\n", i1, DAT_00bb72ba, i2);
                pa!(f, "dither%d %d\n", i1, DAT_00bb72bc, i2);
                pa!(f, "textureres%d %d\n", i1, DAT_00bb72be, i2);
                pa!(f, "mipmap%d %d\n", i1, DAT_00bb72c0, i2);
                pa!(f, "lod%d %d\n", i1, DAT_00bb72c2, i2);
                pa!(f, "screenres%d %d\n", i1, DAT_00bb72c4, i2);
                pa!(f, "windowsize%d %d\n", i1, DAT_00bb72c6, i2);
                pa!(f, "bpp%d %d\n", i1, DAT_00bb72c8, i2);
                pa!(f, "brightness%d %d\n", i1, DAT_00bb72ca, i2);
                pa!(f, "use_3d_hardware%d %d\n", i1, DAT_00bb72cc, i2);
                pa!(f, "bilinear%d %d\n", i1, DAT_00bb72ce, i2);
                i2 = i1;
                if !(i1 < 2) {
                    break;
                }
            }
            ps!(f, "networktype %d\n", DAT_00bb72d0);
            pstr!(f, "phonenumber %s\n", DAT_00bb72d1);
            pstr!(f, "ipaddress %s\n", DAT_00bb7311);
            ps!(f, "server_update_rate %d\n", DAT_00bb7370);
            ps!(f, "sfx_exterior %d\n", DAT_00bb7371);
            ps!(f, "sfx_interior %d\n", DAT_00bb7372);
            ps!(f, "sfx_engine %d\n", DAT_00bb7373);
            ps!(f, "sfx_datapad %d\n", DAT_00bb73a7);
            ps!(f, "voice_pilot %d\n", DAT_00bb7374);
            ps!(f, "voice_tactical_officer %d\n", DAT_00bb7375);
            ps!(f, "voice_commander %d\n", DAT_00bb7376);
            ps!(f, "voice_special %d\n", DAT_00bb7377);
            ps!(f, "music %d\n", DAT_00bb7378);
            ps!(f, "sfx_datapad_volume %d\n", DAT_00bb7379);
            ps!(f, "sfx_exterior_volume %d\n", DAT_00bb737a);
            ps!(f, "sfx_interior_volume %d\n", DAT_00bb737b);
            ps!(f, "sfx_engine_volume %d\n", DAT_00bb737c);
            ps!(f, "voice_volume %d\n", DAT_00bb737d);
            ps!(f, "music_volume %d\n", DAT_00bb737e);
            ps!(f, "datapad_music %d\n", DAT_00bb737f);
            ps!(f, "joybutton1 %d\n", DAT_00bb7380);
            ps!(f, "joybutton2 %d\n", DAT_00bb7381);
            ps!(f, "joybutton3 %d\n", DAT_00bb7382);
            ps!(f, "joybutton4 %d\n", DAT_00bb7383);
            ps!(f, "joybutton5 %d\n", DAT_00bb7384);
            ps!(f, "joybutton6 %d\n", DAT_00bb7385);
            ps!(f, "joybutton7 %d\n", DAT_00bb7386);
            ps!(f, "joybutton8 %d\n", DAT_00bb7387);
            ps!(f, "joybutton9 %d\n", DAT_00bb7388);
            ps!(f, "joybutton10 %d\n", DAT_00bb7389);
            ps!(f, "joybutton11 %d\n", DAT_00bb738a);
            ps!(f, "joybutton12 %d\n", DAT_00bb738b);
            ps!(f, "joybutton13 %d\n", DAT_00bb738c);
            ps!(f, "joybutton14 %d\n", DAT_00bb738d);
            ps!(f, "joybutton15 %d\n", DAT_00bb738e);
            ps!(f, "joybutton16 %d\n", DAT_00bb738f);
            ps!(f, "joybutton17 %d\n", DAT_00bb7390);
            ps!(f, "joybutton18 %d\n", DAT_00bb7391);
            ps!(f, "joybutton19 %d\n", DAT_00bb7392);
            ps!(f, "joybutton20 %d\n", DAT_00bb7393);
            ps!(f, "difficulty %d\n", DAT_00bb7394);
            ps!(f, "collisions %d\n", DAT_00bb7395);
            ps!(f, "craft_jumping %d\n", DAT_00bb7396);
            ps!(f, "random_setup %d\n", DAT_00bb7397);
            ps!(f, "handicapping %d\n", DAT_00bb7398);
            ps!(f, "require_password %d\n", DAT_00bb7399);
            ps!(f, "in_progress_join %d\n", DAT_00bb739a);
            ps!(f, "craft_selection %d\n", DAT_00bb739b);
            ps!(f, "locate_players %d\n", DAT_00bb739c);
            ps!(f, "craft_waves %d\n", DAT_00bb739d);
            ps!(f, "mission_time_limit %d\n", DAT_00bb739e);
            ps!(f, "last_time_limit %d\n", DAT_00bb739f);
            _fprintf(f, c"random_seed %d\n".as_ptr(), DAT_00bb73a0);
            pstr!(f, "password %s\n", DAT_00bb735f);
            ps!(f, "async_flag %d\n", DAT_00bb736f);
            ps!(f, "ai_opponents %d\n", DAT_00bb73a4);
            ps!(f, "help_on %d\n", DAT_00bb73a8);
            ps!(f, "combat_balance %d\n", DAT_00bb73a5);
            pstr!(f, "taunt1 %s\n", DAT_00bb73a9);
            pstr!(f, "taunt2 %s\n", DAT_00bb73ef);
            pstr!(f, "taunt3 %s\n", DAT_00bb7435);
            pstr!(f, "taunt4 %s\n", DAT_00bb747b);
            FUN_004cc590(f);
        }
    }
}
