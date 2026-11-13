/* SPDX-License-Identifier: MIT */
/* z_xvt 00488a90 */
int __cdecl FUN_00488a90(char *param_1, int param_2, int param_3)
{
  int *piVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  char *pcVar5;
  char *pcVar6;
  int bVar7;

  if (param_2 == 0) {
    return 0;
  }
  if (*(int *)(param_2 + 4) == 0x14) {
    piVar1 = *(int **)(param_2 + 0x14);
    iVar3 = piVar1[5] * piVar1[4];
    if (piVar1[2] == iVar3) {
      iVar3 = piVar1[3];
    }
    iVar3 = (int)piVar1 + iVar3 + 0x18;
    if (piVar1[1] == 0) {
      if (*piVar1 == iVar3) {
        param_1 = param_1 + 0x1000;
        bVar7 = (char *)(iVar3 + 0x1000) == (char *)0x0;
        iVar4 = 0x2000;
        pcVar5 = param_1;
        pcVar6 = (char *)(iVar3 + 0x1000);
        do {
          if (iVar4 == 0) break;
          iVar4 = iVar4 - 1;
          bVar7 = *pcVar5 == *pcVar6;
          pcVar5 = pcVar5 + 1;
          pcVar6 = pcVar6 + 1;
        } while (bVar7);
        if (bVar7) {
          return iVar3;
        }
      }
    }
    else {
      param_1 = param_1 + 0x1000;
      bVar7 = (char *)(iVar3 + 0x1000) == (char *)0x0;
      iVar4 = 0x2000;
      pcVar5 = param_1;
      pcVar6 = (char *)(iVar3 + 0x1000);
      do {
        if (iVar4 == 0) break;
        iVar4 = iVar4 - 1;
        bVar7 = *pcVar5 == *pcVar6;
        pcVar5 = pcVar5 + 1;
        pcVar6 = pcVar6 + 1;
      } while (bVar7);
      if (bVar7) {
        return iVar3;
      }
    }
  }
  iVar3 = 0;
  iVar4 = 0;
  if (0 < *(int *)(param_2 + 8)) {
    do {
      iVar2 = *(int *)(*(int *)(param_2 + 0xc) + iVar3);
      if (param_3 == iVar2) {
        return 0;
      }
      iVar2 = FUN_00488a90(param_1, iVar2, param_3);
      if (iVar2 != 0) {
        return iVar2;
      }
      iVar3 = iVar3 + 4;
      iVar4 = iVar4 + 1;
    } while (iVar4 < *(int *)(param_2 + 8));
  }
  return 0;
}
