/* SPDX-License-Identifier: MIT */
/* z_xvt 00462590 */
void __cdecl FUN_004153d0(int param_1, int param_2, int param_3, unsigned int param_4);

void __cdecl FUN_00462590(int *param_1)
{
  unsigned char *pbVar1;
  unsigned char *pbVar2;
  int iVar3;
  unsigned int uVar4;
  int iVar5;
  unsigned int uVar6;
  unsigned int local_c04;
  unsigned char local_c00[3072];
  
  iVar3 = *param_1 + (int)param_1;
  if (param_1[0xb] == 0x18) {
    uVar6 = param_1[0xc];
    param_1[5] = iVar3 - (int)param_1;
    if (uVar6 < 0x400) {
      if (uVar6 != 0) {
        pbVar1 = local_c00;
        pbVar2 = (unsigned char *)(param_1[3] + (int)param_1);
        uVar4 = uVar6;
        do {
          *pbVar1 = *pbVar2 >> 2;
          pbVar1[1] = pbVar2[1] >> 2;
          uVar4 = uVar4 - 1;
          pbVar1[2] = pbVar2[2] >> 2;
          pbVar1 = pbVar1 + 3;
          pbVar2 = pbVar2 + 4;
        } while (uVar4 != 0);
      }
      FUN_004153d0((int)local_c00, iVar3, 0, uVar6);
      iVar3 = iVar3 + param_1[0xc] * 2;
    }
  }
  local_c04 = 0;
  if (param_1[6] != 0) {
    do {
      iVar5 = *(int *)(param_1[4] + local_c04 * 4 + (int)param_1) + (int)param_1;
      *(int *)(iVar5 + 0xc) = iVar3 - iVar5;
      if (*(int *)(iVar5 + 0x24) == 0x18) {
        if (*(unsigned int *)(iVar5 + 0x28) < 0x400) {
          uVar6 = 0;
          if (*(unsigned int *)(iVar5 + 0x28) != 0) {
            pbVar1 = (unsigned char *)(*(int *)(iVar5 + 4) + iVar5);
            pbVar2 = local_c00;
            do {
              *pbVar2 = *pbVar1 >> 2;
              uVar6 = uVar6 + 1;
              pbVar2[1] = pbVar1[1] >> 2;
              pbVar2[2] = pbVar1[2] >> 2;
              pbVar1 = pbVar1 + 4;
              pbVar2 = pbVar2 + 3;
            } while (uVar6 < *(unsigned int *)(iVar5 + 0x28));
          }
          FUN_004153d0((int)local_c00, iVar3, 0, *(int *)(iVar5 + 0x28));
          iVar3 = iVar3 + *(int *)(iVar5 + 0x28) * 2;
        }
      }
      local_c04 = local_c04 + 1;
    } while (local_c04 < (unsigned int)param_1[6]);
  }
  return;
}
