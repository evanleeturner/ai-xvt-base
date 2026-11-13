/* SPDX-License-Identifier: MIT */
/* z_bop 004af180 */
extern undefined4 * __cdecl FUN_00479de0(int param_1, char * param_2);
extern undefined4 DAT_005270b0;

uint __cdecl FUN_004af180(undefined4 *param_1, int param_2)
{
  undefined4 *puVar1;
  uint uVar2;
  int iVar3;
  int iVar4;
  uint local_4;
  
  if (param_1 == (undefined4 *)0x0) {
    return 0;
  }
  do {
    if (param_1[1] != 7) {
      local_4 = (uint)(param_1[1] == 0x16);
      iVar4 = 0;
      if (0 < (int)param_1[2]) {
        iVar3 = 0;
        do {
          puVar1 = (undefined4 *)(param_1[3] + iVar3);
          iVar3 = iVar3 + 4;
          iVar4 = iVar4 + 1;
          uVar2 = FUN_004af180((undefined4 *)*puVar1, param_2);
          local_4 = local_4 + uVar2;
        } while (iVar4 < (int)param_1[2]);
      }
      return local_4;
    }
    if (DAT_005270b0 == 0) {
      param_1 = FUN_00479de0(param_2, (char *)param_1[5]);
    }
    else if (*(char *)param_1[5] == '\0') {
      param_1 = (undefined4 *)*param_1;
    }
    else {
      puVar1 = FUN_00479de0(param_2, (char *)param_1[5]);
      *param_1 = (undefined4)puVar1;
      *(undefined1 *)param_1[5] = 0;
      param_1 = (undefined4 *)*param_1;
    }
  } while (param_1 != (undefined4 *)0x0);
  return 0;
}
