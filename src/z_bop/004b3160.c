/* SPDX-License-Identifier: MIT */
/* z_bop 004b3160 */
extern int DAT_00528c8c;
extern int DAT_00528c90;
extern int _DAT_00528c88;
extern int DAT_00528c5c;

void __cdecl FUN_004b3160(int param_1)
{
  if (param_1 == DAT_00528c8c) {
    DAT_00528c8c = *(int *)(param_1 + 0x94);
    if (DAT_00528c8c == 0) {
      DAT_00528c90 = 0;
    }
    else {
      *(int *)(DAT_00528c8c + 0x90) = 0;
      if (*(int *)(DAT_00528c8c + 0x94) == 0) {
        DAT_00528c90 = DAT_00528c8c;
      }
    }
  }
  else if (param_1 == DAT_00528c90) {
    DAT_00528c90 = *(int *)(param_1 + 0x90);
    *(int *)(DAT_00528c90 + 0x94) = 0;
  }
  else {
    *(int *)(*(int *)(param_1 + 0x90) + 0x94) = *(int *)(param_1 + 0x94);
    *(int *)(*(int *)(param_1 + 0x94) + 0x90) = *(int *)(param_1 + 0x90);
  }
  _DAT_00528c88 = _DAT_00528c88 - 1;
  *(int *)(DAT_00528c5c + 0x14c) = *(int *)(DAT_00528c5c + 0x14c) + *(int *)(param_1 + 0x88);
  return;
}
