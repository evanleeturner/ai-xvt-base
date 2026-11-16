/* SPDX-License-Identifier: MIT */
/* z_bop 004377b0 */
extern short DAT_009a6fe0;
extern int DAT_005233d8;
extern unsigned short DAT_00a00530[];
extern short DAT_00a0813c;
extern int DAT_009d8c08;
extern int DAT_005233f4;

void __cdecl FUN_004377b0(int param_1, int param_2, int param_3, unsigned char param_4)
{
  unsigned short uVar1;
  int iVar2;
  unsigned int uVar3;
  int iVar4;
  unsigned int *puVar5;
  unsigned char *pbVar6;
  
  iVar2 = param_1 + DAT_009a6fe0;
  iVar4 = param_2 + DAT_009a6fe0;
  if (DAT_005233d8 == 2) {
    uVar1 = DAT_00a00530[param_4];
    if (iVar2 < iVar4) {
      puVar5 = (unsigned int *)((param_3 + DAT_00a0813c) * DAT_009d8c08 + DAT_005233f4 + iVar2 * 2);
      for (uVar3 = (unsigned int)(iVar4 - iVar2) >> 1; uVar3 != 0; uVar3 = uVar3 - 1) {
        *puVar5 = (unsigned int)uVar1 | ((unsigned int)uVar1 << 16);
        puVar5 = puVar5 + 1;
      }
      for (uVar3 = (unsigned int)((iVar4 - iVar2) & 1); uVar3 != 0; uVar3 = uVar3 - 1) {
        *(unsigned short *)puVar5 = uVar1;
        puVar5 = (unsigned int *)((int)puVar5 + 2);
      }
      return;
    }
  }
  else if (iVar2 < iVar4) {
    pbVar6 = (unsigned char *)(iVar2 + (param_3 + DAT_00a0813c) * DAT_009d8c08 + DAT_005233f4);
    for (uVar3 = (unsigned int)(iVar4 - iVar2) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
      *(unsigned int *)pbVar6 = (unsigned int)param_4 | ((unsigned int)param_4 << 8) | ((unsigned int)param_4 << 16) | ((unsigned int)param_4 << 24);
      pbVar6 = pbVar6 + 4;
    }
    for (uVar3 = (unsigned int)((iVar4 - iVar2) & 3); uVar3 != 0; uVar3 = uVar3 - 1) {
      *pbVar6 = param_4;
      pbVar6 = pbVar6 + 1;
    }
  }
  return;
}
