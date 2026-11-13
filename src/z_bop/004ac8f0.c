/* SPDX-License-Identifier: MIT */
/* z_bop 004ac8f0 */
void __cdecl FUN_004aa910(int *, int *, int *, int *, int *, int *, int *);

void __stdcall FUN_004ac8f0(short param_1, short *param_2, short *param_3, short *param_4, short *param_5)
{
  short local_34[2];
  short local_30[2];
  int local_2c;
  int local_28;
  int local_24[6];

  FUN_004aa910((int *)local_34, (int *)local_30, &local_2c, &local_28, local_24, local_24 + 3, local_24 + 6);
  *param_2 = (short)local_24[param_1];
  *param_3 = (short)local_24[param_1 + 3];
  *param_4 = local_34[0];
  *param_5 = local_30[0];
  return;
}
