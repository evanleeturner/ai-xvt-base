/* SPDX-License-Identifier: MIT */
/* z_xvt 00491c50 */
extern int DAT_00732e54;
extern int DAT_006d9d56;
extern int DAT_006d4988;
extern int DAT_006d9d50;
extern int DAT_0052a5a0;
extern int DAT_006d9d70;

unsigned int __cdecl FUN_0042b250(int param_1);

void __cdecl FUN_00491c50(unsigned int param_1)
{
  short *psVar1;
  short sVar2;
  unsigned int uVar3;
  int iVar4;
  unsigned short uVar5;
  
  uVar5 = *(unsigned short *)((char *)DAT_00732e54 + 0x6a);
  if (uVar5 == 0xffff) {
    iVar4 = param_1 * 8;
    if ((*(unsigned char *)((char *)DAT_006d9d56 + 0x35) & 1) != 0) {
      sVar2 = *(short *)((char *)DAT_006d4988 + param_1 * 0x23 + 0x10) - 0x4000;
      goto LAB_00491cd7;
    }
  }
  else {
    if ((*(unsigned char *)((char *)DAT_006d9d56 + 0x35) & 1) != 0) {
      sVar2 = *(short *)((char *)DAT_006d4988 + uVar5 * 0x23 + 0x10) - 0x4000;
      goto LAB_00491cd7;
    }
    iVar4 = param_1 * 8;
  }
  sVar2 = *(short *)((char *)DAT_006d4988 + (iVar4 - param_1) * 5 + 0x10) + 0x4000;
LAB_00491cd7:
  *(short *)((char *)DAT_006d9d56 + 0x32) = sVar2;
  uVar3 = FUN_0042b250(DAT_00732e54);
  psVar1 = (short *)((char *)DAT_006d4988 + (unsigned int)DAT_006d9d50 * 0x23 + 0x10);
  uVar5 = *psVar1 - *(short *)((char *)DAT_006d9d56 + 0x32);
  if (0x7fff < uVar5) {
    uVar5 = -uVar5;
  }
  if (uVar5 < 0x301) {
    *psVar1 = *(short *)((char *)DAT_006d9d56 + 0x32);
    *(unsigned char *)(*(int *)((char *)DAT_006d4988 + (unsigned int)DAT_006d9d50 * 0x23 + 0x1f) + 0x90) = 1;
    iVar4 = *(int *)((char *)DAT_006d4988 + (unsigned int)DAT_006d9d50 * 0x23 + 0x1f);
    *(unsigned char *)(iVar4 + 0x89) = *(unsigned char *)(iVar4 + 0x90);
    *(unsigned char *)((char *)DAT_00732e54 + 0xab) = 3;
  }
  else {
    *(unsigned char *)((char *)DAT_00732e54 + 0xab) = 2;
    *(unsigned short *)((char *)DAT_00732e54 + 0xac) = ((unsigned short)(uVar3 >> 1) & 0x7fff) + 0x8000;
  }
  *(short *)((char *)DAT_006d9d56 + 0x3a) = *(short *)((char *)DAT_0052a5a0 + (unsigned int)DAT_006d9d70 * 2) * 0xec;
  return;
}
