/* SPDX-License-Identifier: MIT */
/* z_bop 004eec10 */
extern unsigned int DAT_00aa6af0;
extern unsigned int DAT_00aa6166;
extern unsigned int DAT_00aa62b4;
extern unsigned int DAT_00aa6ae4;
extern unsigned int DAT_00aa6ae8;
extern unsigned int DAT_00aa6aec;
extern unsigned int DAT_00aa5ac0;

extern unsigned int __cdecl FUN_004cef70(int param_1, unsigned int *param_2, unsigned int param_3);

unsigned int __stdcall FUN_004eec10(void)
{
  unsigned char uVar1;
  unsigned int uVar2;
  unsigned int uVar3;
  int iVar4;
  int iVar5;
  unsigned int *puVar6;
  unsigned int *puVar7;
  unsigned char *puVar8;
  unsigned char *puVar9;
  int iVar10;
  
  iVar5 = 1;
  *(unsigned char *)&DAT_00aa6af0 = 0x61;
  puVar7 = &DAT_00aa6166;
  do {
    uVar2 = puVar7[1];
    *(unsigned int *)(&DAT_00aa6af0 + iVar5 * 4) = *puVar7;
    uVar3 = puVar7[2];
    puVar6 = (unsigned int *)((int)puVar7 + 0x2a);
    *(unsigned int *)((iVar5 + 4) * 4 + (int)&DAT_00aa6ae4) = uVar2;
    iVar5 = iVar5 + 5;
    uVar2 = puVar7[3];
    *(unsigned int *)(iVar5 * 4 + (int)&DAT_00aa6ae4) = uVar3;
    uVar3 = puVar7[4];
    ((unsigned int *)&DAT_00aa6ae8)[iVar5] = uVar2;
    *(unsigned int *)(iVar5 * 4 + (int)&DAT_00aa6aec) = uVar3;
    puVar7 = puVar6;
  } while (puVar6 < (unsigned int *)((int)&DAT_00aa62b4 + 2));
  
  puVar9 = (unsigned char *)&DAT_00aa6af0 + iVar5 * 4;
  puVar7 = (unsigned int *)&DAT_00aa5ac0;
  iVar10 = 10;
  do {
    iVar4 = 8;
    puVar8 = puVar9;
    do {
      uVar1 = *(unsigned char *)puVar7;
      puVar9 = puVar8 + 1;
      puVar7 = (unsigned int *)((int)puVar7 + 1);
      iVar4 = iVar4 - 1;
      *puVar8 = uVar1;
      puVar8 = puVar9;
    } while (iVar4 != 0);
    iVar10 = iVar10 - 1;
  } while (iVar10 != 0);
  
  FUN_004cef70(0, (unsigned int *)&DAT_00aa6af0, iVar5 * 4 + 0x50);
  return 1;
}
