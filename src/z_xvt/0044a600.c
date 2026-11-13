/* SPDX-License-Identifier: MIT */
/* z_xvt 0044a600 */
extern int __cdecl FUN_0048f4f0(int);
extern void __cdecl FUN_00485fa0(int *);
extern int __cdecl FUN_00449670(int, int *);
extern void __cdecl FUN_0048f510(void);
extern short DAT_006da850[];

unsigned int __cdecl FUN_0044a600(int param_1, int param_2)
{
  int iVar1;
  unsigned int uVar2;
  int *piVar3;
  int iVar4;
  int iVar5;
  int iVar6;

  if (param_2 < 0) {
    return 0;
  }

  piVar3 = (int *)FUN_0048f4f0(((unsigned int)param_1 >> 0x10) | DAT_006da850[param_1]);

  if ((int *)*piVar3 != piVar3) {
    FUN_00485fa0(piVar3);
  }

  if (*(int *)(**(int **)((char *)piVar3 + 0xA) + 4) == 0x14) {
    param_2 = param_2 + 1;
  }

  if (*(int *)((char *)piVar3 + 6) <= param_2) {
    param_2 = *(int *)((char *)piVar3 + 6) - 1;
  }

  iVar1 = (*(int **)((char *)piVar3 + 0xA))[param_2];

  if (iVar1 != 0) {
    if (*(int *)(iVar1 + 4) == 0x19) {
      iVar4 = *(int *)(iVar1 + 0x14);
      goto LAB_0044a68f;
    }
    iVar6 = 0;
    iVar5 = 0;
    if (0 < *(int *)(iVar1 + 8)) {
      do {
        iVar4 = *(int *)(*(int *)(iVar1 + 0xC) + iVar6);
        if (iVar4 != 0) {
          iVar4 = FUN_00449670(iVar4, piVar3);
          if (iVar4 != 0) goto LAB_0044a68f;
        }
        iVar6 = iVar6 + 4;
        iVar5 = iVar5 + 1;
      } while (iVar5 < *(int *)(iVar1 + 8));
    }
  }
  iVar4 = 0;
LAB_0044a68f:
  uVar2 = *(unsigned int *)(iVar4 + 4);
  FUN_0048f510();
  return uVar2 & 2;
}
