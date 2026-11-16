/* SPDX-License-Identifier: MIT */
/* z_xvt 0048ed70 */
extern unsigned char DAT_00525508;
extern unsigned char DAT_006da9fa;
extern unsigned char s_Rating_points_awarded___d_to_pla_0052ad48[];

void __cdecl FUN_0048ed70(uint param_1)
{
  if (param_1 >= 0x40 && DAT_00525508 != param_1) {
    DAT_006da9fa = s_Rating_points_awarded___d_to_pla_0052ad48[param_1 + 0x38];
  } else {
    DAT_006da9fa = (unsigned char)param_1;
  }
  return;
}
