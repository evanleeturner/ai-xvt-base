/* SPDX-License-Identifier: MIT */
/* z_xvt 0044f480 */
extern unsigned short DAT_006d9d5a;
extern unsigned char *DAT_006d4988;
extern unsigned int DAT_006d9d5c;
extern unsigned char DAT_006d9d78;
extern unsigned char DAT_006d9d56[4];   /* packed pointer at an unaligned address */
extern char DAT_00703ca0[];
extern int DAT_006d499c;
extern unsigned short DAT_006d9d50;
extern int DAT_006d4980;
extern unsigned short DAT_00732a3c;
extern unsigned char *DAT_00732e54;
extern int DAT_00703c88;
extern int DAT_00732a38;
extern char s_disableldr1pln_00525028[];
extern char s_capfreeldr1pln_00525060[];
extern char DAT_00525ce0[];

unsigned int FUN_0042b150(unsigned int a);
unsigned char FUN_00429a20(unsigned int a);
unsigned int FUN_0042a1a0(unsigned int a, unsigned int b);
unsigned char FUN_00444040(unsigned int a, unsigned int b);
int FUN_00429920(unsigned int a, unsigned int b, unsigned int c);
unsigned int FUN_0042af80(unsigned int a);

unsigned int __stdcall FUN_0044f480(void)
{
  char *pcVar1;
  char cVar2;
  unsigned char bVar3;
  unsigned int uVar4;
  unsigned int uVar5;
  int iVar6;
  int iVar7;
  unsigned short uVar8;
  unsigned short uVar9;
  unsigned short uVar10;
  unsigned int pcVar11;
  int iVar12;
  char *pcVar13;
  char *pcVar14;
  unsigned char *p56;
  unsigned char *pe;
  unsigned char *pobj;
  unsigned short us;
  unsigned int ti;
  unsigned int sum;

  iVar12 = (unsigned int)DAT_006d9d5a * 0x23;
  __builtin_memcpy(&iVar7, DAT_006d4988 + 0x1b + iVar12, 4);
  uVar4 = FUN_0042b150(DAT_006d9d5c);
  __builtin_memcpy(&p56, DAT_006d9d56, 4);
  DAT_006d9d78 = (unsigned char)(*(unsigned char *)(p56 + 0xb) == 0);
  pcVar1 = DAT_00703ca0 + (unsigned int)*(unsigned char *)(p56 + 0xb) * 0x55;
  iVar6 = 0xf;
  pcVar13 = pcVar1;
  pcVar14 = s_disableldr1pln_00525028;
  do {
    pcVar11 = (unsigned int)pcVar13;
    if (iVar6 == 0) break;
    iVar6 = iVar6 + -1;
    pcVar11 = (unsigned int)(pcVar13 + 1);
    DAT_006d9d78 = (unsigned char)(*pcVar13 == *pcVar14);
    pcVar13 = (char *)pcVar11;
    pcVar14 = pcVar14 + 1;
  } while (DAT_006d9d78 != 0);

  if (((*(char *)(uVar4 + 0x34) != '\f') && (*(char *)(uVar4 + 0x34) != '\x17')) && (iVar7 == -1)) {
    return uVar4 & 0xffff0000;
  }
  __builtin_memcpy(&p56, DAT_006d9d56, 4);
  __builtin_memcpy(&uVar8, p56 + 0x2b, 2);
  if ((uVar8 != 0xffff) && (uVar8 != 0xfb)) {
    pcVar11 = (unsigned int)uVar8;
    bVar3 = FUN_00429a20(pcVar11);
    if (bVar3 != 0) {
      if (iVar7 != -1) {
        uVar4 = FUN_0042a1a0((unsigned int)DAT_006d9d50, pcVar11);
        if (0x50000 < DAT_006d499c) {
          return uVar4 & 0xffff0000;
        }
      }
      __builtin_memcpy(&p56, DAT_006d9d56, 4);
      *(unsigned short *)(p56 + 0x18) = uVar8;
      __builtin_memcpy(&us, DAT_006d4988 + (int)pcVar11 * 0x23, 2);
      __builtin_memcpy(&p56, DAT_006d9d56, 4);
      *(unsigned short *)(p56 + 0x1a) = us;
      *(unsigned char *)(p56 + 0x1e) = 1;
      return ((unsigned int)DAT_006d4988 & 0xffff0000) | 1;
    }
    __builtin_memcpy(&p56, DAT_006d9d56, 4);
    us = 0xffff;
    __builtin_memcpy(p56 + 0x2b, &us, 2);
  }
  uVar8 = 0xffff;
  __builtin_memcpy(&iVar7, DAT_006d4988 + 0x1b + iVar12, 4);
  if (iVar7 == -1) {
    uVar8 = *(unsigned short *)(uVar4 + 0x18);
    if (uVar8 == 0xffff) goto LAB_0044f8ba;
  }
  else {
    uVar4 = ((pcVar11 >> 16) << 16) | (unsigned int)DAT_00732a3c;
    if ((int)(unsigned int)DAT_00732a3c < DAT_006d4980) {
      do {
        pe = DAT_006d4988 + (uVar4 & 0xffff) * 0x23;
        uVar10 = (unsigned short)uVar4;
        if (*(char *)(pe + 3) != '\0') {
          __builtin_memcpy(&pobj, pe + 0x1f, 4);
          __builtin_memcpy(&pobj, pobj + 0xa7, 4);
          __builtin_memcpy(&us, pobj + 0x1df, 2);
          if (((DAT_006d9d78 == '\0') || (us != 0)) &&
              (*(unsigned short *)(DAT_00732e54 + 0x342) != uVar10)) {
            unsigned short idx = *(unsigned short *)(pobj + 0x6a);
            __builtin_memcpy(&iVar6, (DAT_006d4988 + (unsigned int)idx * 0x23 + 0x1b), 4);
            if (iVar6 == iVar7) {
              unsigned char *q;
              __builtin_memcpy(&q, DAT_006d4988 + 0x1f + iVar12, 4);
              bVar3 = FUN_00444040(uVar4, (unsigned int)*(unsigned char *)(q + 0x87));
              if (bVar3 != 0) {
                uVar8 = uVar10;
              }
            }
          }
        }
        uVar4 = (uVar4 & 0xffff0000) | (unsigned int)(unsigned short)(uVar10 + 1);
      } while ((int)(unsigned int)(unsigned short)(uVar10 + 1) < DAT_006d4980);
    }
    if (uVar8 == 0xffff) {
      do {
        uVar4 = 0;
      } while (0 < (int)((unsigned int)DAT_00703c88 + (unsigned int)DAT_00732a38));
      goto LAB_0044f8ba;
    }
  }
  pe = DAT_006d4988 + (unsigned int)uVar8 * 0x23;
  __builtin_memcpy(&uVar4, pe + 0x1f, 4);
  if (uVar4 == 0) {
    cVar2 = *(char *)(pe + 0x16);
    uVar10 = (unsigned short)(uVar8 + 1);
    sum = (unsigned int)DAT_00703c88 + (unsigned int)DAT_00732a38;
    if ((int)sum <= (int)(unsigned int)(unsigned short)(uVar8 + 1)) {
      uVar10 = (unsigned short)DAT_00732a38;
    }
    uVar4 = (unsigned int)DAT_00732a38 & 0xffff;
    uVar8 = (unsigned short)DAT_00732a38;
    if ((int)uVar4 < (int)sum) {
      do {
        __builtin_memcpy(&us, DAT_006d4988 + (unsigned int)uVar10 * 0x23 + 0x17, 2);
        if (((DAT_006d9d78 == '\0') || (us != 0)) &&
            (*(unsigned short *)(DAT_00732e54 + 0x342) != uVar10)) {
          pe = DAT_006d4988 + (unsigned int)uVar10 * 0x23;
          if (((*(char *)(pe + 3) != '\0') && (*(char *)(pe + 0x16) == cVar2)) &&
              ((iVar7 = FUN_00429920((unsigned int)DAT_006d9d50, (unsigned int)uVar10, 1), iVar7 != 0) &&
               (uVar5 = FUN_0042af80(uVar10), (short)uVar5 != 0))) {
            __builtin_memcpy(&p56, DAT_006d9d56, 4);
            *(unsigned short *)(p56 + 0x18) = uVar10;
            __builtin_memcpy(&us, DAT_006d4988 + (unsigned int)uVar10 * 0x23, 2);
            __builtin_memcpy(&p56, DAT_006d9d56, 4);
            *(unsigned short *)(p56 + 0x1a) = us;
            *(unsigned char *)(p56 + 0x1e) = 1;
            return ((unsigned int)DAT_006d4988 & 0xffff0000) | 1;
          }
          uVar10 = (unsigned short)(uVar10 + 1);
          if ((int)((unsigned int)DAT_00703c88 + (unsigned int)DAT_00732a38) <= (int)(unsigned int)uVar10) {
            uVar10 = (unsigned short)DAT_00732a38;
          }
        }
        uVar8 = (unsigned short)(uVar8 + 1);
        if ((int)((unsigned int)DAT_00703c88 + (unsigned int)DAT_00732a38) <= (int)(unsigned int)uVar8) {
          return ((unsigned int)DAT_00703c88 + (unsigned int)DAT_00732a38) & 0xffff0000;
        }
      } while (1);
    }
  }
  else {
    __builtin_memcpy(&ti, (unsigned char *)uVar4 + 0xa7, 4);
    if (ti != 0) {
      cVar2 = *(char *)(pe + 0x16);
      uVar8 = (unsigned short)(uVar8 + *(unsigned char *)(DAT_00732e54 + 0xb0));
      if (DAT_006d4980 <= (int)(unsigned int)uVar8) {
        uVar8 = DAT_00732a3c;
      }
      uVar4 = (unsigned int)DAT_00732a3c;
      uVar10 = DAT_00732a3c;
      if ((int)uVar4 < DAT_006d4980) {
        do {
          pe = DAT_006d4988 + (unsigned int)uVar8 * 0x23;
          __builtin_memcpy(&pobj, pe + 0x1f, 4);
          __builtin_memcpy(&pobj, pobj + 0xa7, 4);
          if (pobj != 0) {
            __builtin_memcpy(&us, pobj + 0x1df, 2);
          }
          if (((pobj == 0) || ((DAT_006d9d78 != '\0') && (us == 0))) ||
              (*(unsigned short *)(DAT_00732e54 + 0x342) == uVar10)) {
            uVar9 = (unsigned short)(uVar8 + 1);
            uVar8 = (unsigned short)(uVar8 + 1);
            if (DAT_006d4980 <= (int)(unsigned int)uVar9) {
LAB_0044f89c:
              uVar8 = DAT_00732a3c;
            }
          }
          else {
            if ((*(char *)(pe + 3) != '\0') && (*(char *)(pe + 0x16) == cVar2)) {
              iVar7 = FUN_00429920((unsigned int)DAT_006d9d50, (unsigned int)uVar8, 1);
              bVar3 = (unsigned char)(iVar7 == 0);
              if (!bVar3) {
                iVar7 = 0xf;
                pcVar13 = pcVar1;
                pcVar14 = s_capfreeldr1pln_00525060;
                do {
                  if (iVar7 == 0) break;
                  iVar7 = iVar7 + -1;
                  bVar3 = (unsigned char)(*pcVar13 == *pcVar14);
                  pcVar13 = pcVar13 + 1;
                  pcVar14 = pcVar14 + 1;
                } while (bVar3);
                if (!bVar3) {
                  iVar7 = 0xf;
                  pcVar13 = pcVar1;
                  pcVar14 = s_disableldr1pln_00525028;
                  do {
                    if (iVar7 == 0) break;
                    iVar7 = iVar7 + -1;
                    bVar3 = (unsigned char)(*pcVar13 == *pcVar14);
                    pcVar13 = pcVar13 + 1;
                    pcVar14 = pcVar14 + 1;
                  } while (bVar3);
                  if (!bVar3) {
                    iVar7 = 0xd;
                    pcVar13 = pcVar1;
                    pcVar14 = DAT_00525ce0;
                    do {
                      if (iVar7 == 0) break;
                      iVar7 = iVar7 + -1;
                      bVar3 = (unsigned char)(*pcVar13 == *pcVar14);
                      pcVar13 = pcVar13 + 1;
                      pcVar14 = pcVar14 + 1;
                    } while (bVar3);
                    if (!bVar3) {
                      __builtin_memcpy(&p56, DAT_006d9d56, 4);
                      *(unsigned short *)(p56 + 0x18) = uVar8;
                      __builtin_memcpy(&us, DAT_006d4988 + (unsigned int)uVar8 * 0x23, 2);
                      __builtin_memcpy(&p56, DAT_006d9d56, 4);
                      *(unsigned short *)(p56 + 0x1a) = us;
                      *(unsigned char *)(p56 + 0x1e) = 1;
                      return ((unsigned int)DAT_006d4988 & 0xffff0000) | 1;
                    }
                  }
                }
                uVar5 = FUN_0042af80(uVar8);
                if ((short)uVar5 != 0) {
                  __builtin_memcpy(&p56, DAT_006d9d56, 4);
                  *(unsigned short *)(p56 + 0x18) = uVar8;
                  __builtin_memcpy(&us, DAT_006d4988 + (unsigned int)uVar8 * 0x23, 2);
                  __builtin_memcpy(&p56, DAT_006d9d56, 4);
                  *(unsigned short *)(p56 + 0x1a) = us;
                  *(unsigned char *)(p56 + 0x1e) = 1;
                  return ((unsigned int)DAT_006d4988 & 0xffff0000) | 1;
                }
              }
            }
            uVar8 = (unsigned short)(uVar8 + 1);
            if (DAT_006d4980 <= (int)(unsigned int)uVar8) goto LAB_0044f89c;
          }
          uVar10 = (unsigned short)(uVar10 + 1);
          uVar4 = (unsigned int)uVar10;
        } while ((int)uVar4 < DAT_006d4980);
      }
    }
  }
LAB_0044f8ba:
  return uVar4 & 0xffff0000;
}
