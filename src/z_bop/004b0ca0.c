/* SPDX-License-Identifier: MIT */
/* z_bop 004b0ca0 */
extern int DAT_00528c64;
extern int DAT_00528c6c;
extern unsigned short DAT_006642f0;

void __cdecl FUN_004b0ca0(unsigned short *param_1, unsigned int param_2)
{
  int iVar1;
  int iVar2;
  unsigned int uVar3;
  unsigned short uVar4;
  unsigned short *puVar5;
  unsigned int *puVar6;
  unsigned int local_4;
  
  iVar2 = DAT_00528c6c;
  iVar1 = DAT_00528c64;
  if (DAT_00528c6c == DAT_00528c64) {
    puVar6 = (unsigned int *)&DAT_006642f0;
    for (uVar3 = (param_2 & 0x7fffffff) >> 1; uVar3 != 0; uVar3 = uVar3 - 1) {
      *puVar6 = *(unsigned int *)param_1;
      param_1 = param_1 + 2;
      puVar6 = puVar6 + 1;
    }
    for (uVar3 = param_2 * 2 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
      *(char *)puVar6 = (char)*param_1;
      param_1 = (unsigned short *)((int)param_1 + 1);
      puVar6 = (unsigned int *)((int)puVar6 + 1);
    }
    return;
  }
  if (0 < (int)param_2) {
    puVar5 = &DAT_006642f0;
    local_4 = param_2;
    do {
      uVar4 = (unsigned short)((unsigned char)((char)(*param_1 >> (*(unsigned char *)(iVar1 + 0x14) & 0x1f)) <<
                             (*(unsigned char *)(iVar1 + 0x20) & 0x1f)) >> (*(unsigned char *)(iVar2 + 0x20) & 0x1f))
              << (*(unsigned char *)(iVar2 + 0x14) & 0x1f);
      *puVar5 = uVar4;
      uVar4 = (unsigned short)((unsigned char)((char)(*param_1 >> (*(unsigned char *)(iVar1 + 0x18) & 0x1f)) <<
                             (*(unsigned char *)(iVar1 + 0x24) & 0x1f)) >> (*(unsigned char *)(iVar2 + 0x24) & 0x1f))
              << (*(unsigned char *)(iVar2 + 0x18) & 0x1f) | uVar4;
      *puVar5 = uVar4;
      uVar4 = (unsigned short)((unsigned char)((char)(*param_1 >> (*(unsigned char *)(iVar1 + 0x1c) & 0x1f)) <<
                             (*(unsigned char *)(iVar1 + 0x28) & 0x1f)) >> (*(unsigned char *)(iVar2 + 0x28) & 0x1f))
              << (*(unsigned char *)(iVar2 + 0x1c) & 0x1f) | uVar4;
      *puVar5 = uVar4;
      if (puVar5 != &DAT_006642f0) {
        *puVar5 = (unsigned short)(0xff >> (*(unsigned char *)(iVar2 + 0x34) & 0x1f)) <<
                  (*(unsigned char *)(iVar2 + 0x30) & 0x1f) | uVar4;
      }
      param_1 = param_1 + 1;
      puVar5 = puVar5 + 1;
      local_4 = local_4 - 1;
    } while (local_4 != 0);
  }
  return;
}
