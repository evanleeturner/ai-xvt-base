/* SPDX-License-Identifier: MIT */
/* z_xvt 0048eda0 */
extern unsigned char DAT_00525508;
extern unsigned char DAT_00732e50;
extern unsigned char s_Rating_points_awarded___d_to_pla_0052ad48[];

void __cdecl FUN_0048eda0(uint param_1)
{
    if ((0x3f < param_1) && (DAT_00525508 != param_1)) {
        DAT_00732e50 = s_Rating_points_awarded___d_to_pla_0052ad48[param_1 + 0x38];
        return;
    }
    DAT_00732e50 = (unsigned char)param_1;
    return;
}
