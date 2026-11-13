/* SPDX-License-Identifier: MIT */
/* z_xvt 004a1400 */
extern char DAT_006d4988;
extern char DAT_0070b252;
extern char DAT_0070ad49;
extern char DAT_0071badb;
extern char DAT_0071b738;
extern char DAT_0052ad08;
extern char DAT_0071baa9;
extern char DAT_006d4a02;
extern char DAT_0071b7a1;

extern unsigned int __cdecl FUN_00478420(unsigned int, unsigned int);

void __cdecl FUN_004a1400(int param_1, int param_2, int param_3)
{
  int iVar5;
  int iVar6;
  int iVar7;
  unsigned int uVar3;
  unsigned int uVar4;
  unsigned char bVar2;

  if ((param_3 != 2) && (param_3 != 3)) {
    return;
  }
  iVar5 = param_2 * 0x23 + DAT_006d4988;
  iVar6 = *(int *)(iVar5 + 0x1b);
  if ((iVar6 == -1) &&
      (iVar6 = *(int *)((int)&DAT_0070b252 + (unsigned int)*(unsigned char *)(iVar5 + 0x16) * 0x566), iVar6 == -1)) {
    return;
  }
  iVar7 = 0;
  iVar5 = *(int *)(*(int *)(iVar5 + 0x1f) + 0xa7);
  uVar3 = FUN_00478420(*(unsigned int *)(iVar5 + 0xd1), *(unsigned int *)(iVar5 + 0xcd));
  if ((unsigned short)uVar3 < 0x8000) {
    return;
  }
  iVar5 = *(int *)((int)&DAT_0070b252 + param_1 * 0x566);
  if (iVar5 == -1) {
    bVar2 = (&DAT_0070ad49)[param_1 * 0x566];
    if (param_3 == 3) {
      *(short *)(&DAT_0071badb + (unsigned int)bVar2 * 2 + iVar6 * 0x5bd) =
           *(short *)(&DAT_0071badb + (unsigned int)bVar2 * 2 + iVar6 * 0x5bd) + 1;
    }
    uVar3 = (unsigned int)*(unsigned short *)((int)&DAT_0071b738 + iVar6 * 0x5bd);
    if ((int)uVar3 <= *(int *)(&DAT_0052ad08 + (unsigned int)bVar2 * 4) + 4) goto LAB_004a156f;
    iVar7 = uVar3 - *(int *)(&DAT_0052ad08 + (unsigned int)bVar2 * 4);
  }
  else {
    uVar3 = (unsigned int)*(unsigned short *)((int)&DAT_0071b738 + iVar5 * 0x5bd);
    if (param_3 == 3) {
      *(short *)(&DAT_0071baa9 + uVar3 * 2 + iVar6 * 0x5bd) =
           *(short *)(&DAT_0071baa9 + uVar3 * 2 + iVar6 * 0x5bd) + 1;
    }
    uVar4 = (unsigned int)*(unsigned short *)((int)&DAT_0071b738 + iVar6 * 0x5bd);
    if (uVar4 <= uVar3 + 4) goto LAB_004a156f;
    iVar7 = uVar4 - uVar3;
  }
  iVar7 = iVar7 * 2;
  if (param_3 == 2) {
    iVar7 = iVar7 / 2;
  }
LAB_004a156f:
  if (DAT_006d4a02 != '\x02') {
    iVar7 = (int)(iVar7 + (iVar7 >> 0x1f & 3U)) >> 2;
  }
  *(int *)((int)&DAT_0071b7a1 + iVar6 * 0x5bd) = *(int *)((int)&DAT_0071b7a1 + iVar6 * 0x5bd) - iVar7;
  return;
}
