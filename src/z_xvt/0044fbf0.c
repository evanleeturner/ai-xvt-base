/* SPDX-License-Identifier: MIT */
/* z_xvt 0044fbf0 */
extern unsigned short DAT_006d49a0;
extern unsigned short DAT_00732a3c;
extern int DAT_006d4980;
extern unsigned int DAT_006d4988;
extern unsigned short DAT_006d9d50;
extern unsigned int DAT_006d499c;
extern char DAT_006d497c;
extern unsigned int DAT_00732a38;
extern unsigned int DAT_00703c88;

extern unsigned short __cdecl FUN_0049ec30(unsigned short param_1, unsigned short param_2, unsigned short param_3);
extern void __cdecl FUN_0042a1a0(unsigned int param_1, unsigned int param_2);

unsigned short __cdecl FUN_0044fbf0(unsigned short param_1, unsigned short param_2, short param_3, unsigned short param_4, unsigned short param_5)
{
  unsigned short uVar1;
  unsigned short uVar2;
  unsigned int iVar3;
  unsigned short uVar4;
  unsigned short uVar5;
  unsigned int uVar6;

  uVar4 = 0;
  uVar5 = 0xffff;
  uVar6 = 0xffffffff;
  uVar1 = 0xffff;
  if (0 < DAT_006d49a0) {
    do {
      uVar5 = uVar1;
      uVar1 = FUN_0049ec30(uVar4, param_1, param_2);
      uVar2 = FUN_0049ec30(uVar4, param_4, param_5);
      if (param_3 == 1) {
        uVar1 = uVar1 | uVar2;
      }
      else {
        uVar1 = uVar1 & uVar2;
      }
      if (uVar1 != 0) {
        uVar1 = DAT_00732a3c;
        if ((int)(unsigned int)DAT_00732a3c < DAT_006d4980) {
          do {
            iVar3 = (unsigned int)uVar1 * 0x23 + DAT_006d4988;
            if ((*(char *)(iVar3 + 3) != '\0') && (*(unsigned char *)(iVar3 + 0x16) == uVar4)) {
              FUN_0042a1a0((unsigned int)DAT_006d9d50, (unsigned int)uVar1);
              if (DAT_006d499c < uVar6) {
                uVar6 = DAT_006d499c;
                DAT_006d497c = (char)uVar4;
                uVar5 = uVar1;
              }
            }
            uVar1 = uVar1 + 1;
          } while ((int)(unsigned int)uVar1 < DAT_006d4980);
        }
        uVar1 = (unsigned short)DAT_00732a38;
        if ((int)(DAT_00732a38 & 0xffff) < (int)(DAT_00703c88 + DAT_00732a38)) {
          do {
            iVar3 = (unsigned int)uVar1 * 0x23 + DAT_006d4988;
            if ((*(char *)(iVar3 + 3) != '\0') && (*(unsigned char *)(iVar3 + 0x16) == uVar4)) {
              FUN_0042a1a0((unsigned int)DAT_006d9d50, (unsigned int)uVar1);
              if (DAT_006d499c < uVar6) {
                uVar6 = DAT_006d499c;
                DAT_006d497c = (char)uVar4;
                uVar5 = uVar1;
              }
            }
            uVar1 = uVar1 + 1;
          } while ((int)(unsigned int)uVar1 < (int)(DAT_00703c88 + DAT_00732a38));
        }
      }
      uVar4 = uVar4 + 1;
      uVar1 = uVar5;
    } while ((int)(unsigned int)uVar4 < (int)DAT_006d49a0);
  }
  return uVar5;
}
