/* SPDX-License-Identifier: MIT */
/* z_xvt 004cf4f0 */
extern int FUN_004cee90(int);
extern int DAT_0081b5be;
extern int DAT_00815cf6;
extern int DAT_0064f070;
extern int DAT_0064f080;
extern int DAT_0064f430;

int __cdecl FUN_004cf4f0(int param_1)
{
  unsigned int uVar1;
  int iVar2;
  int *piVar3;
  int iVar4;
  
  uVar1 = FUN_004cee90(param_1);
  if ((uVar1 < (unsigned int)DAT_0081b5be) && (uVar1 < 0x28)) {
    iVar4 = (&DAT_00815cf6)[uVar1 * 0x8e];
  }
  else {
    iVar4 = 0;
  }
  iVar2 = 0;
  piVar3 = &DAT_0064f070;
  do {
    if (*piVar3 == param_1) {
      return (&DAT_0064f080)[iVar2 * 6] + iVar4;
    }
    piVar3 = piVar3 + 6;
    iVar2 = iVar2 + 1;
  } while (piVar3 < &DAT_0064f430);
  return iVar4;
}
