/* SPDX-License-Identifier: MIT */
/* z_bop 004b7510 */
extern int DAT_00664c88;

unsigned int __cdecl FUN_004b7570(unsigned char *param_1, int param_2);

void __cdecl FUN_004b7510(unsigned char *param_1, int param_2, int param_3)
{
  int iVar1;
  unsigned int uVar2;
  int iVar3;
  int iVar4;
  int *puVar5;
  unsigned char *pbVar6;

  iVar1 = *(int *)(param_3 + 4);
  iVar4 = *(int *)(param_3 + 8);
  puVar5 = &DAT_00664c88;
  for (iVar3 = 0x80; iVar3 != 0; iVar3 = iVar3 - 1) {
    *puVar5 = 0x1000100;
    puVar5 = puVar5 + 1;
  }
  if (iVar4 > 0) {
    do {
      iVar3 = iVar1;
      pbVar6 = param_1;
      if (iVar1 > 0) {
        do {
          param_1 = pbVar6 + 1;
          uVar2 = FUN_004b7570((unsigned char *)(param_2 + (unsigned int)*pbVar6 * 4), (unsigned int)*pbVar6);
          *pbVar6 = (unsigned char)uVar2;
          iVar3 = iVar3 - 1;
          pbVar6 = param_1;
        } while (iVar3 != 0);
      }
      iVar4 = iVar4 - 1;
    } while (iVar4 != 0);
  }
  return;
}
