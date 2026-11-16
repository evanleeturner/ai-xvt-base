/* SPDX-License-Identifier: MIT */
/* z_bop 004a95c0 */
extern uchar DAT_005233c8;
extern uchar DAT_00a08100;
extern uchar DAT_00524040[];

void __cdecl FUN_004a95c0(uint param_1)
{
  if ((0x3f < param_1) && ((uint)DAT_005233c8 != param_1)) {
    DAT_00a08100 = DAT_00524040[param_1];
    return;
  }
  DAT_00a08100 = (uchar)param_1;
  return;
}
