/* SPDX-License-Identifier: MIT */
/* z_bop 00470020 */
extern int DAT_0099f6e0;
extern int DAT_00999be8;

void __cdecl FUN_00470020(int param_1)
{
  int *piVar1;
  int uVar2;

  uVar2 = 0;
  if (DAT_0099f6e0 != 0) {
    piVar1 = &DAT_00999be8;
    while (*piVar1 != param_1) {
      piVar1 = piVar1 + 0x8e;
      uVar2 = uVar2 + 1;
      if (DAT_0099f6e0 <= uVar2) {
        return;
      }
    }
  }
  return;
}
