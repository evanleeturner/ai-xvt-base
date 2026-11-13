/* SPDX-License-Identifier: MIT */
/* z_xvt 0048a980 */
int __cdecl FUN_0048a980(int param_1, int param_2, int param_3);

int __cdecl FUN_0048a980(int param_1, int param_2, int param_3)
{
  int *piVar1;
  int iVar2;
  int iVar3;
  int iVar4;

  if (param_1 == 0) {
    return 0;
  }
  if (param_2 != 0) {
    if (*(int *)(param_2 + 4) != *(int *)(param_1 + 4)) {
      return 0;
    }
    if (*(int *)(param_1 + 4) == 0x14) {
      piVar1 = *(int **)(param_1 + 0x14);
      iVar2 = piVar1[5] * piVar1[4];
      if (piVar1[2] == iVar2) {
        iVar2 = piVar1[3];
      }
      iVar2 = (int)piVar1 + iVar2 + 0x18;
      if (((*piVar1 == iVar2) || (piVar1[1] != 0)) && (iVar2 == param_3)) {
        return param_2;
      }
    }
    if (*(int *)(param_2 + 8) != *(int *)(param_1 + 8)) {
      return 0;
    }
    iVar4 = 0;
    iVar2 = 0;
    if (0 < *(int *)(param_1 + 8)) {
      do {
        iVar3 = FUN_0048a980(*(int *)(*(int *)(param_1 + 0xc) + iVar4),
                             *(int *)(*(int *)(param_2 + 0xc) + iVar4), param_3);
        if (iVar3 != 0) {
          return iVar3;
        }
        iVar4 = iVar4 + 4;
        iVar2 = iVar2 + 1;
      } while (iVar2 < *(int *)(param_1 + 8));
    }
    return 0;
  }
  return 0;
}
