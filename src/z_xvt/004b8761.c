/* SPDX-License-Identifier: MIT */
/* z_xvt 004b8761 */
extern int *DAT_0052f1e8;
extern int *DAT_0052f1ec;
extern int _DAT_0052f1e4;
extern int DAT_00742820;

void __cdecl FUN_004b8761(int *param_1)
{
  if (DAT_0052f1e8 == (int *)0x0) {
    DAT_0052f1ec = param_1;
    DAT_0052f1e8 = param_1;
    *param_1 = 0;
    param_1[1] = 0;
  }
  else {
    *(int **)((int)DAT_0052f1ec + 4) = param_1;
    *param_1 = (int)DAT_0052f1ec;
    param_1[1] = 0;
    DAT_0052f1ec = param_1;
  }
  _DAT_0052f1e4 = _DAT_0052f1e4 + 1;
  DAT_00742820 = DAT_00742820 - param_1[4];
  return;
}
