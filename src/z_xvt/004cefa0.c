/* SPDX-License-Identifier: MIT */
/* z_xvt 004cefa0 */
extern uint __cdecl FUN_004cee90(int param_1);
extern int DAT_0078072e;
extern int DAT_0064f070;
extern int DAT_0064f078;
extern int DAT_0064f07c;
extern int DAT_0064f080;
extern int DAT_0064f430;
extern int DAT_0081b5be;
extern int DAT_00815cee;
extern int DAT_00815cf2;
extern int DAT_00815cf6;

uint __cdecl FUN_004cefa0(int param_1)
{
  uint uVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  
  if (DAT_0078072e == 0) {
    iVar3 = 0;
    piVar2 = &DAT_0064f070;
    do {
      if (*piVar2 == param_1) {
        uVar1 = (uint)((&DAT_0064f078)[iVar3 * 6]);
        if (uVar1 == 0) {
          uVar1 = 1;
        }
        uVar1 = (uint)(((&DAT_0064f07c)[iVar3 * 6] + (&DAT_0064f080)[iVar3 * 6] * 2) * 10000) / uVar1;
        if ((int)uVar1 < 0x2711) {
          return uVar1;
        }
        return 10000;
      }
      piVar2 = piVar2 + 6;
      iVar3 = iVar3 + 1;
    } while (piVar2 < &DAT_0064f430);
    uVar1 = 0;
  }
  else {
    uVar1 = FUN_004cee90(param_1);
    if ((uVar1 < (uint)DAT_0081b5be) && (uVar1 < 0x28)) {
      iVar4 = (&DAT_00815cee)[uVar1 * 0x8e];
      iVar3 = (&DAT_00815cf2)[uVar1 * 0x8e] + (&DAT_00815cf6)[uVar1 * 0x8e] * 2;
    }
    else {
      iVar4 = 0;
      iVar3 = 0;
    }
    piVar2 = &DAT_0064f070;
    do {
      if (*piVar2 == param_1) {
        iVar4 = iVar4 + piVar2[2];
        iVar3 = iVar3 + piVar2[3] + piVar2[4] * 2;
      }
      piVar2 = piVar2 + 6;
    } while (piVar2 < &DAT_0064f430);
    if (iVar4 == 0) {
      iVar4 = 1;
    }
    uVar1 = (uint)((iVar3 * 10000) / iVar4);
    if (10000 < (int)uVar1) {
      return 10000;
    }
  }
  return uVar1;
}
