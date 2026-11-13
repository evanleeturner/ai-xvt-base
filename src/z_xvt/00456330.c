/* SPDX-License-Identifier: MIT */
/* z_xvt 00456330 */
extern unsigned char DAT_005248f8[];
extern unsigned char DAT_0070ad5d[];
extern unsigned char DAT_00524900[];
extern unsigned int DAT_007092d4;
extern unsigned char DAT_0070b252[];
extern unsigned short DAT_006d9d60;
extern unsigned char DAT_0070ad8d[];
extern unsigned char DAT_0070ad8c[];
extern unsigned char DAT_006d4a60[];
extern unsigned short DAT_00732a3c;
extern unsigned char DAT_006d4a61[];
extern int DAT_006d4980;
extern int DAT_006d4988;
extern int DAT_006d49a0;

unsigned int __stdcall FUN_00456330(void)
{
  unsigned short uVar1;
  unsigned int uVar2;
  int iVar3;
  
  uVar2 = 0;
  if (0 < DAT_006d49a0) {
    do {
      iVar3 = uVar2 * 0x566;
      if (((((DAT_005248f8[(unsigned char)DAT_0070ad5d[iVar3]] & DAT_00524900[DAT_007092d4]) != 0)
           || (*(int *)(DAT_0070b252 + iVar3) != -1)) && ((unsigned short)uVar2 != DAT_006d9d60)) &&
         ((DAT_0070ad8d[iVar3] != '\0' && ((unsigned char)DAT_0070ad8c[iVar3] == DAT_006d9d60)))) {
        if ((DAT_006d4a60[uVar2 * 0x126] == '\0') ||
           (uVar1 = DAT_00732a3c, DAT_006d4a61[uVar2 * 0x126] != '\0')) {
          return 0;
        }
        for (; (int)(unsigned int)uVar1 < DAT_006d4980; uVar1 = uVar1 + 1) {
          iVar3 = (unsigned int)uVar1 * 0x23 + DAT_006d4988;
          if ((*(char *)(iVar3 + 3) != '\0') && (*(unsigned char *)(iVar3 + 0x16) == uVar2)) {
            return 0;
          }
        }
      }
      uVar1 = (unsigned short)uVar2 + 1;
      uVar2 = (unsigned int)uVar1;
    } while ((int)(unsigned int)uVar1 < (int)DAT_006d49a0);
  }
  return 1;
}
