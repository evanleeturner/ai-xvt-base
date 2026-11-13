/* SPDX-License-Identifier: MIT */
/* z_xvt 00449790 */
extern undefined4 __cdecl FUN_0048f4f0(undefined4 param_1);
extern undefined __cdecl FUN_00485fa0(int *param_1);
extern int __cdecl FUN_00449670(int param_1, undefined4 param_2);
extern undefined4 __cdecl FUN_0048f510(void);
extern short DAT_006da850[];

undefined4 __cdecl FUN_00449790(int param_1, int param_2)
{
  int iVar1;
  int iVar2;
  int *piVar3;
  undefined4 *puVar4;
  int iVar5;
  int iVar6;
  undefined4 uVar7;

  if (param_2 < 0) {
    return 0;
  }
  piVar3 = (int *)FUN_0048f4f0(((unsigned int)(param_1 >> 0x10) << 16) | (unsigned int)(DAT_006da850[param_1] & 0xFFFF));
  if ((int *)*piVar3 != piVar3) {
    FUN_00485fa0(piVar3);
  }
  if (*(int *)(**(int **)((int)piVar3 + 10) + 4) == 0x14) {
    param_2 = param_2 + 1;
  }
  if (*(int *)((int)piVar3 + 6) <= param_2) {
    param_2 = *(int *)((int)piVar3 + 6) + -1;
  }
  iVar1 = (*(int **)((int)piVar3 + 10))[param_2];
  if (iVar1 != 0) {
    if (*(int *)(iVar1 + 4) == 0x19) {
      puVar4 = *(undefined4 **)(iVar1 + 0x14);
      goto LAB_0044981f;
    }
    iVar6 = 0;
    iVar5 = 0;
    if (0 < *(int *)(iVar1 + 8)) {
      do {
        iVar2 = *(int *)(*(int *)(iVar1 + 0xc) + iVar6);
        if (iVar2 != 0) {
          puVar4 = (undefined4 *)FUN_00449670(iVar2, (undefined4)piVar3);
          if (puVar4 != (undefined4 *)0x0) goto LAB_00449823;
        }
        iVar6 = iVar6 + 4;
        iVar5 = iVar5 + 1;
      } while (iVar5 < *(int *)(iVar1 + 8));
    }
  }
  puVar4 = (undefined4 *)0x0;
LAB_0044981f:
  if (puVar4 == (undefined4 *)0x0) {
    uVar7 = 0;
  }
  else {
LAB_00449823:
    uVar7 = *puVar4;
  }
  FUN_0048f510();
  return uVar7;
}
