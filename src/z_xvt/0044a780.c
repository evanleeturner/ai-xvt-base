/* SPDX-License-Identifier: MIT */
/* z_xvt 0044a780 */
extern int *FUN_0048f4f0(int param_1);
extern void FUN_00485fa0(int *param_1);
extern int FUN_00449190(int param_1);
extern void FUN_0048f510(void);
extern unsigned short DAT_006da850[];

unsigned int FUN_0044a780(int param_1, int param_2)
{
  int *piVar2;
  int iVar1;
  int iVar3;
  int iVar4;
  int iVar5;
  unsigned int uVar6;

  piVar2 = FUN_0048f4f0((int)(unsigned short)DAT_006da850[param_1]);
  if ((int *)*piVar2 != piVar2) {
    FUN_00485fa0(piVar2);
  }
  if (*(int *)(**(int **)((char *)piVar2 + 10) + 4) == 0x14) {
    param_2 = param_2 + 1;
  }
  {
    int field_6;
    __builtin_memcpy(&field_6, (void *)((char *)piVar2 + 6), 4);
    if (field_6 <= param_2) {
      param_2 = field_6 - 1;
    }
  }
  iVar1 = (*(int **)((char *)piVar2 + 10))[param_2];
  if (iVar1 != 0) {
    iVar3 = iVar1;
    if (*(int *)(iVar1 + 4) == 0x17) goto LAB_0044a7fe;
    iVar5 = 0;
    iVar4 = 0;
    if (0 < *(int *)(iVar1 + 8)) {
      do {
        iVar3 = *(int *)((char *)(*(int *)(iVar1 + 0xc)) + iVar5);
        if ((iVar3 != 0) && (iVar3 = FUN_00449190(iVar3), iVar3 != 0)) goto LAB_0044a7fe;
        iVar5 = iVar5 + 4;
        iVar4 = iVar4 + 1;
      } while (iVar4 < *(int *)(iVar1 + 8));
    }
  }
  iVar3 = 0;
LAB_0044a7fe:
  if (iVar3 == 0) {
    uVar6 = 0;
  }
  else {
    uVar6 = *(unsigned int *)(iVar3 + 0x14);
  }
  FUN_0048f510();
  return uVar6;
}
