/* SPDX-License-Identifier: MIT */
/* z_xvt 00433680 */
extern int DAT_00739290;
extern int DAT_00733798;

void __cdecl FUN_00433680(int param_1)
{
  int *piVar1;
  int uVar2;
  
  uVar2 = 0;
  if (DAT_00739290 != 0) {
    piVar1 = &DAT_00733798;
    while (*piVar1 != param_1) {
      piVar1 = piVar1 + 0x8e;
      uVar2 = uVar2 + 1;
      if (DAT_00739290 <= uVar2) {
        return;
      }
    }
  }
  return;
}
