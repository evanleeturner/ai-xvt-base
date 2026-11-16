/* SPDX-License-Identifier: MIT */
/* z_bop 004b0b90 */
extern int DAT_00528c64;
extern int DAT_00528c68;
extern unsigned short DAT_00664550;

void __cdecl FUN_004b0b90(unsigned short *param_1, unsigned int param_2)
{
  int iVar1;
  int iVar2;
  unsigned short *puVar3;
  unsigned int uVar4;
  unsigned short uVar5;
  unsigned int *puVar6;
  unsigned int local_4;
  
  iVar2 = DAT_00528c68;
  iVar1 = DAT_00528c64;
  if (DAT_00528c68 == DAT_00528c64) {
    puVar6 = (unsigned int *)&DAT_00664550;
    for (uVar4 = (param_2 & 0x7fffffff) >> 1; uVar4 != 0; uVar4 = uVar4 - 1) {
      *puVar6 = *(unsigned int *)param_1;
      param_1 = param_1 + 2;
      puVar6 = puVar6 + 1;
    }
    for (uVar4 = (param_2 * 2) & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
      *(char *)puVar6 = (char)*param_1;
      param_1 = (unsigned short *)((int)param_1 + 1);
      puVar6 = (unsigned int *)((int)puVar6 + 1);
    }
    return;
  }
  if (0 < (int)param_2) {
    puVar3 = &DAT_00664550;
    local_4 = param_2;
    do {
      uVar5 = (unsigned short)((unsigned char)((char)(*param_1 >> (*(unsigned char *)(iVar1 + 0x14) & 0x1f)) <<
                             (*(unsigned char *)(iVar1 + 0x20) & 0x1f)) >> (*(unsigned char *)(iVar2 + 0x20) & 0x1f))
              << (*(unsigned char *)(iVar2 + 0x14) & 0x1f);
      *puVar3 = uVar5;
      uVar5 = (unsigned short)((unsigned char)((char)(*param_1 >> (*(unsigned char *)(iVar1 + 0x18) & 0x1f)) <<
                             (*(unsigned char *)(iVar1 + 0x24) & 0x1f)) >> (*(unsigned char *)(iVar2 + 0x24) & 0x1f))
              << (*(unsigned char *)(iVar2 + 0x18) & 0x1f) | uVar5;
      *puVar3 = uVar5;
      uVar5 = (unsigned short)((unsigned char)((char)(*param_1 >> (*(unsigned char *)(iVar1 + 0x1c) & 0x1f)) <<
                             (*(unsigned char *)(iVar1 + 0x28) & 0x1f)) >> (*(unsigned char *)(iVar2 + 0x28) & 0x1f))
              << (*(unsigned char *)(iVar2 + 0x1c) & 0x1f) | uVar5;
      *puVar3 = uVar5;
      if (puVar3 != &DAT_00664550) {
        *puVar3 = (unsigned short)(0xff >> (*(unsigned char *)(iVar2 + 0x34) & 0x1f)) <<
                  (*(unsigned char *)(iVar2 + 0x30) & 0x1f) | uVar5;
      }
      param_1 = param_1 + 1;
      puVar3 = puVar3 + 1;
      local_4 = local_4 - 1;
    } while (local_4 != 0);
  }
  return;
}
