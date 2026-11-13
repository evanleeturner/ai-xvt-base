/* SPDX-License-Identifier: MIT */
/* z_xvt 004cf410 */
extern unsigned int FUN_004cee90(int param_1);
extern unsigned int DAT_0081b5be;
extern int DAT_00815cee[];
extern int DAT_0064f070[];
extern int DAT_0064f078[];
extern int DAT_0064f430[];

int __cdecl FUN_004cf410(int param_1)
{
  unsigned int uVar1;
  int iVar2;
  int *piVar3;
  int iVar4;
  
  uVar1 = FUN_004cee90(param_1);
  if ((uVar1 < DAT_0081b5be) && (uVar1 < 0x28)) {
    iVar4 = DAT_00815cee[uVar1 * 0x8e];
  }
  else {
    iVar4 = 0;
  }
  iVar2 = 0;
  piVar3 = &DAT_0064f070[0];
  do {
    if (*piVar3 == param_1) {
      return DAT_0064f078[iVar2 * 6] + iVar4;
    }
    piVar3 = piVar3 + 6;
    iVar2 = iVar2 + 1;
  } while (piVar3 < &DAT_0064f430[0]);
  return iVar4;
}
