/* SPDX-License-Identifier: MIT */
/* z_xvt 0044a6c0 */
extern short DAT_006da850[];
extern void *FUN_0048f4f0(int param_1);
extern void FUN_00485fa0(int *param_1);
extern int FUN_00449670(int param_1, int param_2);
extern void FUN_0048f510(void);

uint __cdecl FUN_0044a6c0(int param_1, int param_2)
{
  int iVar1;
  uint uVar2;
  int *piVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  int local_6;
  int *local_A;

  if (param_2 < 0) {
    return 0;
  }
  piVar3 = (int *)FUN_0048f4f0((((uint)param_1 >> 0x10) << 16) | (uint)DAT_006da850[param_1]);
  if ((int *)*piVar3 != piVar3) {
    FUN_00485fa0(piVar3);
  }
  __builtin_memcpy(&local_6, (char *)piVar3 + 6, 4);
  __builtin_memcpy(&local_A, (char *)piVar3 + 0xA, 4);
  if (*(int *)(*local_A + 4) == 0x14) {
    param_2 = param_2 + 1;
  }
  if (local_6 <= param_2) {
    param_2 = local_6 - 1;
  }
  iVar1 = local_A[param_2];
  if (iVar1 != 0) {
    if (*(int *)(iVar1 + 4) == 0x19) {
      iVar4 = *(int *)(iVar1 + 0x14);
      goto LAB_0044a74f;
    }
    iVar5 = 0;
    iVar6 = 0;
    if (0 < *(int *)(iVar1 + 8)) {
      do {
        iVar4 = *(int *)(*(int *)(iVar1 + 0xC) + iVar5);
        if (iVar4 != 0) {
          iVar4 = FUN_00449670(iVar4, (int)piVar3);
          if (iVar4 != 0) goto LAB_0044a74f;
        }
        iVar5 = iVar5 + 4;
        iVar6 = iVar6 + 1;
      } while (iVar6 < *(int *)(iVar1 + 8));
    }
  }
  iVar4 = 0;
LAB_0044a74f:
  uVar2 = *(uint *)(iVar4 + 4);
  FUN_0048f510();
  return uVar2 & 1;
}
