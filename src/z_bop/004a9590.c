/* SPDX-License-Identifier: MIT */
/* z_bop 004a9590 */
extern unsigned char DAT_005233c8;
extern unsigned char DAT_009a807a;
extern unsigned char PTR_s_I_d_also_like_to_thank_Larry_for_00524040[];

void __cdecl FUN_004a9590(unsigned int param_1)
{
    if (param_1 > 0x3f && DAT_005233c8 != param_1) {
        DAT_009a807a = PTR_s_I_d_also_like_to_thank_Larry_for_00524040[param_1];
        return;
    }
    DAT_009a807a = (unsigned char)param_1;
    return;
}
