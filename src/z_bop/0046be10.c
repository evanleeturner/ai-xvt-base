/* SPDX-License-Identifier: MIT */
/* z_bop 0046be10 */
extern int DAT_009e9670;
extern int DAT_009ecc34;
extern int DAT_009a1fe8;

unsigned int __cdecl FUN_0046be10(unsigned short param_1, short param_2)
{
  unsigned char bVar1;
  int iVar2;
  int iVar3;
  unsigned short uVar5;
  unsigned int uVar4;
  int iVar6;
  unsigned short uVar7;
  unsigned char abStack_c[12];

  iVar3 = 0;
  iVar2 = *(int *)(*(int *)(*(int *)((int)&DAT_009e9670 + DAT_009ecc34 * 0x5bd) * 0x23 + 0x1f + DAT_009a1fe8) + 0xa9);
  do {
    abStack_c[*(unsigned char *)(iVar3 + 0x2a1 + iVar2)] = (unsigned char)iVar3;
    iVar3 = iVar3 + 1;
  } while (iVar3 < 10);
  uVar7 = 0xffff;
  iVar6 = 0;
  uVar5 = (unsigned short)((unsigned int)iVar3 >> 0x10);
  do {
    bVar1 = abStack_c[iVar6];
    if (*(short *)(iVar2 + 0x2ac + (unsigned int)bVar1 * 2) == 0) {
      if ((unsigned int)param_1 == (unsigned int)bVar1) {
        if (param_2 != -1) {
          do {
            uVar4 = (((uVar5 << 16) | param_1) >> 8) & 0xFFFFFF;
            if (iVar6 + 1 > 9) {
              iVar3 = 0;
              do {
                if (*(short *)(iVar2 + 0x2ac + (unsigned int)abStack_c[iVar3] * 2) != 0) {
                  return (((uVar4 << 8) | abStack_c[iVar3]) & 0xffff00ff);
                }
                iVar3 = iVar3 + 1;
              } while (iVar3 < 10);
              iVar3 = 0;
              do {
                if (*(short *)(iVar2 + 0x2ac + (unsigned int)abStack_c[iVar3] * 2) == 0) {
                  return (((uVar4 << 8) | abStack_c[iVar3]) & 0xffff00ff);
                }
                iVar3 = iVar3 + 1;
              } while (iVar3 < 10);
              return (uVar5 << 16) | param_1;
            }
            iVar3 = iVar6 + 1;
            iVar6 = iVar6 + 1;
          } while (*(short *)(iVar2 + 0x2ac + (unsigned int)abStack_c[iVar3] * 2) != 0);
          return (((uVar4 << 8) | abStack_c[iVar3]) & 0xffff00ff);
        }
        if (uVar7 != 0xffff) {
          return (uVar5 << 16) | uVar7;
        }
        iVar3 = 9;
        do {
          if (*(short *)(iVar2 + 0x2ac + (unsigned int)abStack_c[iVar3] * 2) != 0) {
            return ((((unsigned int)iVar3 >> 8) << 8) | abStack_c[iVar3]) & 0xffff00ff;
          }
          iVar3 = iVar3 - 1;
        } while (-1 < iVar3);
        return ((((unsigned int)iVar3 >> 8) << 8) | abStack_c[9]) & 0xffff00ff;
      }
      uVar7 = (unsigned short)bVar1;
    }
    iVar6 = iVar6 + 1;
    if (iVar6 > 9) {
      return (unsigned int)uVar5 << 0x10;
    }
  } while (1);
}
