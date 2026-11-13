/* SPDX-License-Identifier: MIT */
/* z_bop 0040e7f0 */
extern unsigned int __cdecl FUN_0040e940(unsigned char *param_1, int param_2, unsigned int param_3, unsigned int param_4);
extern void __cdecl FUN_004453d0(int param_1, int param_2, int param_3, int param_4, int param_5);
extern int DAT_009e8f54;

void __cdecl FUN_0040e7f0(int *param_1)
{
  unsigned int uVar1;
  unsigned char *puVar2;
  unsigned char *puVar3;
  int iVar4;
  unsigned int uVar5;
  unsigned char *pbVar6;
  unsigned char local_8;
  unsigned char local_7;
  unsigned char local_6;
  unsigned int local_4;
  
  puVar2 = (unsigned char *)(*param_1 + (int)param_1);
  if (param_1[0xb] == 0x18) {
    uVar5 = 0;
    param_1[5] = (int)puVar2 - (int)param_1;
    puVar3 = puVar2;
    pbVar6 = (unsigned char *)(param_1[3] + (int)param_1);
    if (param_1[0xc] != 0) {
      do {
        local_8 = *pbVar6 >> 2;
        puVar2 = puVar3 + 1;
        uVar5 = uVar5 + 1;
        local_7 = pbVar6[1] >> 2;
        local_6 = pbVar6[2] >> 2;
        uVar1 = FUN_0040e940(&local_8, 0x9a7bc0, 0x40, 0x100);
        *puVar3 = (char)uVar1;
        puVar3 = puVar2;
        pbVar6 = pbVar6 + 4;
      } while (uVar5 < (unsigned int)param_1[0xc]);
    }
  }
  local_4 = 0;
  if (param_1[6] != 0) {
    do {
      iVar4 = *(int *)(param_1[4] + local_4 * 4 + (int)param_1) + (int)param_1;
      *(int *)(iVar4 + 0xc) = (int)puVar2 - iVar4;
      if (*(int *)(iVar4 + 0x24) == 0x18) {
        pbVar6 = (unsigned char *)(*(int *)(iVar4 + 4) + iVar4);
        if (DAT_009e8f54 != 0) {
          FUN_004453d0(*(int *)(iVar4 + 8) + iVar4, (int)pbVar6, *(int *)(iVar4 + 0x10),
                       *(int *)(iVar4 + 0x14), *(int *)(iVar4 + 0x20));
        }
        uVar5 = 0;
        puVar3 = puVar2;
        if (*(int *)(iVar4 + 0x28) != 0) {
          do {
            local_8 = *pbVar6 >> 2;
            puVar2 = puVar3 + 1;
            uVar5 = uVar5 + 1;
            local_7 = pbVar6[1] >> 2;
            local_6 = pbVar6[2] >> 2;
            uVar1 = FUN_0040e940(&local_8, 0x9a7bc0, 0x40, 0x100);
            *puVar3 = (char)uVar1;
            puVar3 = puVar2;
            pbVar6 = pbVar6 + 4;
          } while (uVar5 < *(unsigned int *)(iVar4 + 0x28));
        }
      }
      local_4 = local_4 + 1;
    } while (local_4 < (unsigned int)param_1[6]);
  }
  (void)local_7;
  (void)local_6;
  return;
}
