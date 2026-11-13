/* SPDX-License-Identifier: MIT */
/* z_bop 00435ad0 */
extern int DAT_009a1fe8;
extern unsigned char DAT_0051c56e[];
extern unsigned char DAT_0051c5ac[];
extern unsigned char DAT_0051c5aa[];
extern unsigned short DAT_0051a3de[];
extern int DAT_00521188[];
extern int DAT_00521170[];
extern unsigned int __cdecl FUN_004269e0(unsigned int);

int __cdecl FUN_00435ad0(int param_1)
{
  int iVar1;
  int iVar2;
  int iVar3;
  unsigned int uVar4;
  int iVar5;
  unsigned int uVar6;
  
  iVar2 = param_1 * 0x23;
  iVar1 = *(int *)(*(int *)(DAT_009a1fe8 + 0x1f + iVar2) + 0xa9);
  uVar4 = FUN_004269e0((((unsigned int)DAT_009a1fe8 >> 8) << 8 | *(unsigned char *)(DAT_009a1fe8 + iVar2 + 3)) & 0xffff00ff);
  iVar3 = (uVar4 & 0xffff) * 0x101;
  iVar5 = (unsigned int)DAT_0051c56e[iVar3] * 0x28;
  if (*(char *)(DAT_009a1fe8 + 3 + iVar2) == '6') {
    iVar5 = (unsigned int)DAT_0051c56e[iVar3] * 0x50;
  }
  if (*(unsigned char *)(iVar1 + 0x232) != 0) {
    uVar4 = 0;
    do {
      uVar6 = uVar4 + 1;
      iVar5 = iVar5 + ((unsigned int)*(unsigned char *)(iVar1 + 0x372 + (unsigned int)DAT_0051c5ac[iVar3 + uVar4] * 4) +
                       (unsigned int)*(unsigned char *)(iVar1 + 0x372 + (unsigned int)DAT_0051c5aa[iVar3 + uVar4] * 4)) *
                       (unsigned int)DAT_0051a3de[(unsigned int)*(unsigned char *)(uVar4 + 0x233 + iVar1)];
      uVar4 = uVar6;
    } while (uVar6 < *(unsigned char *)(iVar1 + 0x232));
  }
  return iVar5 + DAT_00521188[(unsigned int)*(unsigned char *)(iVar1 + 0x246)] +
         DAT_00521170[(unsigned int)*(unsigned char *)(iVar1 + 0x23d)];
}
