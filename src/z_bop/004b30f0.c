/* SPDX-License-Identifier: MIT */
/* z_bop 004b30f0 */
extern int DAT_00528c8c;
extern int DAT_00528c90;
extern int _DAT_00528c88;
extern int DAT_00528c5c;

void __cdecl FUN_004b30f0(int param_1)
{
  if (DAT_00528c8c == 0) {
    DAT_00528c90 = param_1;
    DAT_00528c8c = param_1;
    *(int *)(param_1 + 0x90) = 0;
    *(int *)(param_1 + 0x94) = 0;
  }
  else {
    *(int *)(DAT_00528c90 + 0x94) = param_1;
    *(int *)(param_1 + 0x94) = 0;
    *(int *)(param_1 + 0x90) = DAT_00528c90;
    DAT_00528c90 = param_1;
  }
  _DAT_00528c88 = _DAT_00528c88 + 1;
  *(int *)(DAT_00528c5c + 0x14c) = *(int *)(DAT_00528c5c + 0x14c) - *(int *)(param_1 + 0x88);
  return;
}
