/* SPDX-License-Identifier: MIT */
/* z_xvt 0044a510 */
extern int __cdecl FUN_0048f4f0(int param_1);
extern void __cdecl FUN_00485fa0(int *param_1);
extern int __cdecl FUN_00449670(int param_1, int *param_2);
extern long __cdecl __ftol(void);
extern void __cdecl FUN_0048f510(void);
extern short DAT_006da850[];

int __cdecl FUN_0044a510(int param_1, int param_2)
{
  int *piVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  long lVar6;
  
  if (param_2 < 0) {
    return 0;
  }
  piVar1 = (int *)FUN_0048f4f0(((unsigned int)param_1 & 0xFFFF0000) | (unsigned short)DAT_006da850[param_1]);
  if ((int *)*piVar1 != piVar1) {
    FUN_00485fa0(piVar1);
  }
  if (*(int *)(**(int **)((int)piVar1 + 10) + 4) == 0x14) {
    param_2 = param_2 + 1;
  }
  if (*(int *)((int)piVar1 + 6) <= param_2) {
    param_2 = *(int *)((int)piVar1 + 6) + -1;
  }
  iVar3 = (*(int **)((int)piVar1 + 10))[param_2];
  if (((iVar3 != 0) && (*(int *)(iVar3 + 4) != 0x19)) && (iVar4 = 0, 0 < *(int *)(iVar3 + 8))) {
    iVar5 = 0;
    do {
      iVar2 = *(int *)(*(int *)(iVar3 + 0xc) + iVar5);
      if ((iVar2 != 0) && (iVar2 = FUN_00449670(iVar2, piVar1), iVar2 != 0)) break;
      iVar5 = iVar5 + 4;
      iVar4 = iVar4 + 1;
    } while (iVar4 < *(int *)(iVar3 + 8));
  }
  lVar6 = __ftol();
  iVar3 = (int)lVar6;
  lVar6 = __ftol();
  iVar4 = (int)lVar6;
  lVar6 = __ftol();
  iVar5 = (int)lVar6;
  if (((iVar4 < iVar3) || (iVar2 = iVar4, iVar4 < iVar5)) &&
     ((iVar2 = iVar3, iVar3 <= iVar5 && (iVar4 <= iVar5)))) {
    iVar2 = iVar5;
  }
  FUN_0048f510();
  return iVar2;
}
