/* SPDX-License-Identifier: MIT */
/* z_xvt 004cf480 */
extern unsigned int DAT_0081b5be;
extern int DAT_00815cf2[];
extern int DAT_0064f070[];
extern int DAT_0064f07c[];
extern int DAT_0064f430[];

int __cdecl FUN_004cee90(int param_1);

int __cdecl FUN_004cf480(int param_1)
{
  unsigned int uVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  
  uVar1 = FUN_004cee90(param_1);
  if ((uVar1 < DAT_0081b5be) && (uVar1 < 0x28)) {
    iVar4 = DAT_00815cf2[uVar1 * 0x8e];
  }
  else {
    iVar4 = 0;
  }
  iVar3 = 0;
  piVar2 = DAT_0064f070;
  do {
    if (*piVar2 == param_1) {
      FUN_004cee90(param_1);
      return DAT_0064f07c[iVar3 * 6] + iVar4;
    }
    piVar2 = piVar2 + 6;
    iVar3 = iVar3 + 1;
  } while (piVar2 < DAT_0064f430);
  return iVar4;
}
