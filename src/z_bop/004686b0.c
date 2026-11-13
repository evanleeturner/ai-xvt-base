/* SPDX-License-Identifier: MIT */
/* z_bop 004686b0 */
extern int DAT_009a73b6;
extern int DAT_009a1fe0;
extern int DAT_009a1fe8;
extern int DAT_00a08294;
extern int DAT_00a08104;
extern unsigned char DAT_0051a44f[];
extern unsigned char DAT_0051c5aa[];
extern unsigned char DAT_0051c5ac[];

unsigned int FUN_004686b0(void)
{
  unsigned char bVar1;
  char cVar2;
  unsigned char cVar11;
  unsigned int uVar5;
  unsigned int uVar6;
  unsigned short uVar7;
  unsigned short uVar8;
  unsigned int uVar10;
  int iVar9;
  int bVar3;

  uVar5 = *(unsigned short *)(DAT_009a73b6 + 0x18);
  if ((int)uVar5 < DAT_009a1fe0) {
    uVar5 = ((DAT_009a1fe8 >> 8) << 8 | *(unsigned char *)(uVar5 * 0x23 + 2 + DAT_009a1fe8)) & 0xffff00ff;
    if (uVar5 == 3 || uVar5 == 5 || uVar5 == 4 || (uVar5 == 1 && DAT_00a08294 == 0xe)) {
      cVar11 = 2;
    } else {
      cVar11 = 1;
    }
  } else {
    cVar11 = 1;
  }
  uVar5 &= 0xffff0000;

  if (*(unsigned char *)(DAT_00a08104 + 0x232) != 0) {
    do {
      bVar1 = *(unsigned char *)((uVar5 & 0xffff) + 0x233 + DAT_00a08104);
      if (DAT_0051a44f[bVar1] == cVar11) {
LAB_00468789:
        iVar9 = *(unsigned char *)(DAT_00a08104 + 4) * 0x101 + (uVar5 & 0xffff);
        uVar10 = DAT_0051c5aa[iVar9];
        if (DAT_0051c5aa[iVar9] <= DAT_0051c5ac[iVar9]) {
          uVar6 = uVar5;
          do {
            cVar2 = *(char *)(DAT_00a08104 + 0x372 + uVar10 * 4);
            uVar7 = (unsigned short)(uVar6 >> 16);
            if (cVar2 != '\0') {
              return (uVar7 << 16) | 1;
            }
            uVar8 = (unsigned short)uVar10 + 1;
            uVar10 = uVar8;
            uVar6 = (uVar6 & 0xffffff00) | (unsigned char)cVar2;
          } while (uVar8 <= DAT_0051c5ac[iVar9]);
          uVar5 = (uVar7 << 16) | (unsigned short)uVar5;
        }
      } else {
        if (((DAT_00a08294 != 0xe) || (bVar1 != 0x98 && bVar1 != 0x99)) || (bVar3 = 1, cVar11 != 2)) {
          bVar3 = 0;
        }
        if (bVar3) goto LAB_00468789;
      }
      uVar8 = (unsigned short)uVar5 + 1;
      uVar5 = ((uVar5 >> 16) << 16) | uVar8;
    } while (uVar8 < *(unsigned char *)(DAT_00a08104 + 0x232));
  }
  return uVar5 & 0xffff0000;
}
