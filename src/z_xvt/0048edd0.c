/* SPDX-License-Identifier: MIT */
/* z_xvt 0048edd0 */
extern unsigned char DAT_00525508;
extern char DAT_0071b012;
extern char s_Rating_points_awarded___d_to_pla_0052ad48[];

void __cdecl FUN_0048edd0(uint param_1)
{
    if ((0x3f < param_1) && (DAT_00525508 != param_1)) {
        DAT_0071b012 = s_Rating_points_awarded___d_to_pla_0052ad48[param_1 + 0x38];
        return;
    }
    DAT_0071b012 = (char)param_1;
    return;
}
