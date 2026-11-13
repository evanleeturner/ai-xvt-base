/* SPDX-License-Identifier: MIT */
/* z_xvt 00401000 */
extern unsigned int DAT_00732f70;
extern unsigned int DAT_00732f80;
extern unsigned int DAT_00732f50;
extern unsigned short DAT_006db6d8;
extern unsigned short DAT_0071f2d0;
extern unsigned short DAT_00732570;
extern unsigned int DAT_00732f74;
extern unsigned int DAT_00732f84;
extern unsigned int DAT_00732f54;
extern unsigned short DAT_006db6da;
extern unsigned short DAT_0071f2d2;
extern unsigned short DAT_00732572;
extern int DAT_0071b730;
extern int DAT_0071ecf4;
extern int DAT_006d4988;

void __stdcall FUN_00401000(void)
{
  unsigned int uVar4;
  short sVar3;
  int iVar5;
  unsigned short uVar1, uVar2;

  uVar4 = 2;
  do {
    *(unsigned int *)((char *)&DAT_00732f74 + uVar4 * 4) = *(unsigned int *)((char *)&DAT_00732f70 + uVar4 * 4);
    uVar1 = *(unsigned short *)((char *)&DAT_006db6d8 + uVar4 * 2);
    *(unsigned int *)((char *)&DAT_00732f84 + uVar4 * 4) = *(unsigned int *)((char *)&DAT_00732f80 + uVar4 * 4);
    uVar2 = *(unsigned short *)((char *)&DAT_0071f2d0 + uVar4 * 2);
    *(unsigned int *)((char *)&DAT_00732f54 + uVar4 * 4) = *(unsigned int *)((char *)&DAT_00732f50 + uVar4 * 4);
    *(unsigned short *)((char *)&DAT_006db6da + uVar4 * 2) = uVar1;
    sVar3 = (short)uVar4;
    *(unsigned short *)((char *)&DAT_0071f2d2 + uVar4 * 2) = uVar2;
    *(unsigned short *)((char *)&DAT_00732572 + uVar4 * 2) = *(unsigned short *)((char *)&DAT_00732570 + uVar4 * 2);
    uVar4 = (unsigned int)(unsigned short)(sVar3 - 1);
  } while (sVar3 != 0);

  iVar5 = *(int *)((int)&DAT_0071b730 + DAT_0071ecf4 * 0x5bd) * 0x23 + DAT_006d4988;
  DAT_00732f70 = *(unsigned int *)(*(int *)(iVar5 + 0x1f) + 6);
  DAT_00732f80 = *(unsigned int *)(*(int *)(iVar5 + 0x1f) + 10);
  DAT_00732f50 = *(unsigned int *)(*(int *)(iVar5 + 0x1f) + 0xe);
  DAT_006db6d8 = *(unsigned short *)(iVar5 + 0x14);
  DAT_0071f2d0 = *(unsigned short *)(iVar5 + 0x12);
  DAT_00732570 = *(unsigned short *)(iVar5 + 0x10);
  return;
}
