/* SPDX-License-Identifier: MIT */
/* z_xvt 00486000 */
void __cdecl FUN_00486000(int *param_1, int param_2);

void __cdecl FUN_00486000(int *param_1, int param_2)
{
  int *piVar1;
  int iVar2;
  int iVar3;
  int iVar4;

  if (*param_1 != 0) {
    *param_1 = *param_1 + param_2;
  }
  if (param_1[5] != 0) {
    param_1[5] = param_1[5] + param_2;
  }
  if ((param_1[1] == 0x14) && (piVar1 = (int *)param_1[5], piVar1[1] == 0)) {
    param_1[5] = param_1[5] + param_2;
  }
  iVar3 = 0;
  if ((param_1[3] != 0) && (param_1[3] = param_1[3] + param_2, 0 < param_1[2])) {
    iVar4 = 0;
    do {
      iVar2 = *(int *)(param_1[3] + iVar4);
      if (iVar2 != 0) {
        *(int *)(param_1[3] + iVar4) = iVar2 + param_2;
        FUN_00486000((int *)iVar2, param_2);
      }
      iVar4 = iVar4 + 4;
      iVar3 = iVar3 + 1;
    } while (iVar3 < param_1[2]);
  }
  return;
}
