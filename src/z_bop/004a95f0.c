/* SPDX-License-Identifier: MIT */
/* z_bop 004a95f0 */
extern uchar DAT_005233c8;
extern uchar DAT_009e8f52;
extern uchar PTR_s_I_d_also_like_to_thank_Larry_for_00524040[];

void __cdecl FUN_004a95f0(uint param_1)
{
    if ((0x3f < param_1) && (DAT_005233c8 != param_1)) {
        DAT_009e8f52 = PTR_s_I_d_also_like_to_thank_Larry_for_00524040[param_1];
        return;
    }
    DAT_009e8f52 = (uchar)param_1;
    return;
}
