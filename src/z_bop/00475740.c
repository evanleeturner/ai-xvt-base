/* SPDX-License-Identifier: MIT */
/* z_bop 00475740 */
extern undefined4 * __cdecl FUN_00479de0(int param_1, char *param_2);

void __cdecl FUN_00475740(int param_1, undefined4 *param_2, int param_3, undefined4 param_4)
{
  float *pfVar1;
  float *pfVar2;
  int iVar3;
  int iVar4;
  int bVar5;
  int local_4;
  
  if (param_2 != (undefined4 *)0x0) {
    while (param_2[1] == 7) {
      param_2 = FUN_00479de0(param_3, (char *)param_2[5]);
      if (param_2 == (undefined4 *)0x0) {
        return;
      }
    }
    if (param_2[1] == 3) {
      pfVar1 = (float *)param_2[5];
      local_4 = 0;
      if (0 < (int)param_2[4]) {
        do {
          pfVar2 = *(float **)(param_1 + 0x14);
          iVar3 = 0;
          iVar4 = *(int *)(param_1 + 0x10);
          bVar5 = iVar4 == 0;
          if (0 < iVar4) {
            do {
              if (((*pfVar1 == *pfVar2) && (pfVar1[1] == pfVar2[1])) && (pfVar1[2] == pfVar2[2]))
                break;
              pfVar2 = pfVar2 + 3;
              iVar3 = iVar3 + 1;
            } while (iVar3 < iVar4);
            bVar5 = iVar3 == iVar4;
          }
          if (bVar5) {
            *pfVar2 = *pfVar1;
            pfVar2[1] = pfVar1[1];
            pfVar2[2] = pfVar1[2];
            *(int *)(param_1 + 0x10) = *(int *)(param_1 + 0x10) + 1;
          }
          pfVar1 = pfVar1 + 3;
          local_4 = local_4 + 1;
        } while (local_4 < (int)param_2[4]);
      }
    }
    iVar4 = 0;
    if (0 < (int)param_2[2]) {
      iVar3 = 0;
      do {
        iVar3 = iVar3 + 4;
        iVar4 = iVar4 + 1;
        FUN_00475740(param_1, *(undefined4 **)(param_2[3] - 4 + iVar3), param_3, param_4);
      } while (iVar4 < (int)param_2[2]);
    }
  }
  return;
}
