/* SPDX-License-Identifier: MIT */
/* z_xvt 004ceb20 */
extern unsigned int DAT_008050aa;
extern int DAT_008050ae;
extern char DAT_007810b5;
extern char DAT_007810b4;
extern unsigned int DAT_0081b5be;
extern int DAT_00815ac6;
extern int DAT_007810a6;
extern char DAT_007810b3;

unsigned int __cdecl FUN_004ceb20(unsigned int param_1, unsigned int param_2, int param_3, int param_4, unsigned int param_5)
{
  unsigned int uVar1;
  int iVar2;
  unsigned int uVar3;
  int iVar4;
  unsigned int uVar5;
  int *piVar6;
  
  uVar1 = DAT_008050aa;
  iVar4 = DAT_008050ae;
  do {
    if (iVar4 == 0) {
      return 0xffffffff;
    }
    iVar2 = uVar1 * 0x210;
    if ((&DAT_007810b5)[iVar2] != '\0') {
      uVar5 = (unsigned int)(unsigned char)(&DAT_007810b4)[iVar2];
      uVar3 = 0;
      if (DAT_0081b5be != 0) {
        piVar6 = &DAT_00815ac6;
        do {
          if (*piVar6 == (&DAT_007810a6)[uVar1 * 0x84]) break;
          piVar6 = piVar6 + 0x8e;
          uVar3 = uVar3 + 1;
        } while (uVar3 < DAT_0081b5be);
      }
      if (param_5 == uVar3) {
        if (param_3 == 0) {
          if (param_4 == 0) {
            if ((((&DAT_007810b3)[iVar2] != '\0') && ((&DAT_007810b3)[iVar2] != '\x02')) &&
               (param_2 == uVar5)) {
              return uVar1;
            }
          }
          else if (((&DAT_007810b3)[iVar2] == '\x02') && (param_2 == uVar5)) {
            return uVar1;
          }
        }
        else if (((&DAT_007810b3)[iVar2] == '\0') && (param_2 == uVar5)) {
          return uVar1;
        }
      }
    }
    uVar1 = uVar1 + 1;
    if (0x3ff < uVar1) {
      uVar1 = 0;
    }
    iVar4 = iVar4 - 1;
  } while (1);
}
