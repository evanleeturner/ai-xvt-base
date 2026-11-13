/* SPDX-License-Identifier: MIT */
/* z_xvt 004ced20 */
extern int DAT_0064f070;
extern int DAT_0064f074;
extern int DAT_0064f430;

uint __cdecl FUN_004ced20(int param_1)
{
  uint uVar1;
  int iVar2;
  int *piVar3;
  
  iVar2 = 0;
  piVar3 = &DAT_0064f070;
  do {
    if (*piVar3 == param_1) {
      uVar1 = *(uint *)(iVar2 * 0x18 + 0x64f084);
      if (uVar1 == 0) {
        return 1;
      }
      return (uint)(&DAT_0064f074)[iVar2 * 6] / uVar1;
    }
    piVar3 = piVar3 + 6;
    iVar2 = iVar2 + 1;
  } while (piVar3 < &DAT_0064f430);
  return 0;
}
