/* SPDX-License-Identifier: MIT */
/* z_bop 00476490 */
extern int DAT_005233d8;
extern int * __cdecl FUN_00479de0(int param_1, char *param_2);
extern int __cdecl FUN_00476670(int param_1, int param_2, int param_3);
extern void __cdecl FUN_00476490(int *param_1, int param_2, int param_3);

void __cdecl FUN_00476490(int *param_1, int param_2, int param_3)
{
  int *piVar1;
  int iVar2;
  int iVar3;
  int iVar4;

  if (param_1 != (int *)0x0) {
    while (param_1[1] == 7) {
      param_1 = FUN_00479de0(param_2, (char *)param_1[5]);
      if (param_1 == (int *)0x0) {
        return;
      }
    }
    if (param_1[1] == 0x14) {
      piVar1 = (int *)param_1[5];
      if (piVar1[1] == 0) {
        iVar4 = *piVar1;
        iVar2 = iVar4;
        if (DAT_005233d8 == 2) {
          iVar2 = iVar4 + 0x1000;
        }
        iVar3 = piVar1[5] * piVar1[4];
        if (piVar1[2] == iVar3) {
          iVar3 = piVar1[3];
        }
        if (((int *)((iVar2 + -0x18) - iVar3) != piVar1) &&
           (iVar2 = FUN_00476670(param_2, param_3, iVar4), iVar2 != 0)) {
          iVar2 = *(int *)(iVar2 + 0x14);
          iVar4 = *(int *)(iVar2 + 0x14) * *(int *)(iVar2 + 0x10);
          if (*(int *)(iVar2 + 8) == iVar4) {
            iVar4 = *(int *)(iVar2 + 0xc);
          }
          iVar4 = iVar2 + 0x18 + iVar4;
          if (DAT_005233d8 == 2) {
            iVar4 = iVar4 + -0x1000;
          }
          *piVar1 = iVar4;
        }
      }
      else {
        piVar1[1] = 0;
        iVar2 = piVar1[5] * piVar1[4];
        if (piVar1[2] == iVar2) {
          iVar2 = piVar1[3];
        }
        iVar2 = (int)piVar1 + iVar2 + 0x18;
        if (DAT_005233d8 == 2) {
          iVar2 = iVar2 + -0x1000;
        }
        *piVar1 = iVar2;
      }
    }
    iVar2 = 0;
    iVar4 = 0;
    if (0 < (int)param_1[2]) {
      do {
        iVar2 = iVar2 + 4;
        iVar4 = iVar4 + 1;
        FUN_00476490(*(int **)(param_1[3] + -4 + iVar2), param_2, param_3);
      } while (iVar4 < (int)param_1[2]);
    }
  }
  return;
}
