/* SPDX-License-Identifier: MIT */
/* z_xvt 0044a2a0 */
extern int FUN_0048f4f0(int param_1);
extern void FUN_00485fa0(int *param_1);
extern int FUN_00449670(int param_1, int param_2);
extern long long __ftol(void);
extern void FUN_0048f510(int param_1);
extern short DAT_006da850[];

int __cdecl FUN_0044a2a0(int param_1, int param_2)
{
  int iVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  long long lVar6;
  
  if (param_2 < 0) {
    return 0;
  }
  piVar2 = (int *)FUN_0048f4f0(((unsigned int)param_1 >> 0x10) | (unsigned short)DAT_006da850[param_1]);
  if (*piVar2 != (int)piVar2) {
    FUN_00485fa0(piVar2);
  }
  if (*(int *)(**(int **)((int)piVar2 + 10) + 4) == 0x14) {
    param_2 = param_2 + 1;
  }
  if (*(int *)((int)piVar2 + 6) <= param_2) {
    param_2 = *(int *)((int)piVar2 + 6) - 1;
  }
  iVar1 = (*(int **)((int)piVar2 + 10))[param_2];
  if ((iVar1 != 0) && (*(int *)(iVar1 + 4) != 0x19)) {
    iVar5 = 0;
    iVar4 = 0;
    if (0 < *(int *)(iVar1 + 8)) {
      do {
        iVar3 = *(int *)(*(int *)(iVar1 + 0xc) + iVar5);
        if ((iVar3 != 0) && (iVar3 = FUN_00449670(iVar3, (int)piVar2), iVar3 != 0)) break;
        iVar5 = iVar5 + 4;
        iVar4 = iVar4 + 1;
      } while (iVar4 < *(int *)(iVar1 + 8));
    }
  }
  lVar6 = __ftol();
  FUN_0048f510(0);
  return (int)lVar6;
}
