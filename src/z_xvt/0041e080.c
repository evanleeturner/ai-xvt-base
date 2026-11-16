/* SPDX-License-Identifier: MIT */
/* z_xvt 0041e080 */
/* Callee prototypes (calling convention not established by the evidence; defaults used) */
unsigned int   FUN_0049d810(unsigned char *param_1, int param_2);
unsigned short FUN_00478470(void);
void           FUN_0041eea0(int param_1);
void           FUN_004215c0(int param_1);

/* Globals (placeholder names kept). Short tables are indexed in 16-bit units
   (record stride 0x126 bytes = 0x93 shorts). */
extern unsigned short DAT_0071e6a6;
extern short          DAT_0071e6a8;
extern unsigned short DAT_0073b374;
extern short          DAT_006d49a0;
extern unsigned int   DAT_006d4980;
extern unsigned char *DAT_006d4988;
extern unsigned int   DAT_00732a38;
extern unsigned int   DAT_00732a3c;
extern unsigned int   DAT_00703c88;
extern int            DAT_00732e54;

extern unsigned char  DAT_006d4a02;
extern unsigned char  DAT_006d4a60[];
extern unsigned char  DAT_006d4a61[];
extern unsigned short DAT_006d4a62[];
extern unsigned char  DAT_006d4a64[];
extern unsigned short DAT_006d4a6a[];
extern unsigned short DAT_006d4a6c[];
extern unsigned short DAT_006d4a74[];
extern unsigned short DAT_006d4a78[];
extern unsigned short DAT_006d4a7c[];
extern unsigned short DAT_006d4a80[];
extern unsigned short DAT_006d4a88[];
extern unsigned short DAT_006d4a8a[];
extern unsigned short DAT_006d4a8c[];
extern unsigned short DAT_006d4a8e[];
extern unsigned short DAT_006d4a96[];
extern unsigned char  DAT_006d4aa2[];
extern unsigned char  DAT_006d4aa3[];
extern unsigned char  DAT_006d4aa7[];
extern unsigned char  DAT_006d4aa9[];
extern unsigned char  DAT_006d4aab[];
extern unsigned char  DAT_006d4aad[];
extern unsigned char  DAT_006d4ab1[];
extern unsigned char  DAT_006d4ab8[];

extern unsigned char  DAT_0070ad42[];
extern unsigned char  DAT_0070ad43[];
extern unsigned char  DAT_0070ad48[];
extern unsigned char  DAT_0070ad51[];
extern unsigned char  DAT_0070ad53[];
extern unsigned char  DAT_0070ad54[];
extern unsigned char  DAT_0070ad55[];
extern unsigned char  DAT_0070ad5d[];
extern unsigned char  DAT_0070ad5e[];
extern unsigned char  DAT_0070ad69[];
extern unsigned char  DAT_0070ad74[];
extern unsigned char  DAT_0070ad75[];
extern unsigned char  DAT_0070ad76[];
extern unsigned char  DAT_0070ad77[];
extern unsigned char  DAT_0070ad78[];
extern unsigned char  DAT_0070ad7c[];
extern unsigned char  DAT_0070ad86[];
extern unsigned char  DAT_0070ad8d[];
extern unsigned char  DAT_0070ad8f[];
extern unsigned char  DAT_0070b252[];   /* int read at unaligned address -> memcpy */

extern unsigned char  DAT_007092d4;
extern unsigned char  DAT_007092d7;
extern unsigned char  DAT_007092db;
extern unsigned char  DAT_00709d0a[];
extern unsigned char  DAT_00709d0b[];

extern unsigned char  DAT_005247e8[];
extern unsigned char  DAT_005248f8[];
extern unsigned char  DAT_00524900[];
extern unsigned char  DAT_0052687a[];

void __stdcall FUN_0041e080(void)
{
  unsigned short uVar1;
  int iVar2;
  char cVar3;
  unsigned short uVar4;
  unsigned int uVar5;
  unsigned int uVar6;
  unsigned char *pcVar7;
  unsigned short sVar8;
  unsigned int uVar9;
  unsigned char bVar10;
  int iVar11;
  int iVar12;
  unsigned char *puVar13;
  int bVar14;
  int iVar15;
  unsigned char *pbVar16;

  if (DAT_0071e6a6 == 0) {
    DAT_0071e6a6 = 0xec;
    DAT_0073b374 = 0;
    if (0 < DAT_006d49a0) {
LAB_0041e0b5:
      uVar5 = (unsigned int)DAT_0073b374;
      if ((DAT_006d4a60[uVar5 * 0x126] != 0) || (DAT_006d4a64[uVar5 * 0x126] != 0)) {
        iVar11 = (int)(uVar5 * 0x566);
        __builtin_memcpy(&iVar15, &DAT_0070b252[iVar11], 4);
        if (((DAT_00524900[DAT_007092d4] & DAT_005248f8[DAT_0070ad5d[iVar11]]) == 0)
            || ((DAT_006d4a61[uVar5 * 0x126] == 0 || (DAT_006d4a6a[uVar5 * 0x93] == 0)) ||
                (iVar15 != -1))) goto LAB_0041e6df;
        if (DAT_0070ad54[iVar11] != 0) {
          iVar2 = (int)((unsigned int)DAT_0070ad48[iVar11] * 3);
          if (((DAT_00709d0a[(unsigned int)DAT_0070ad48[iVar11] * 3] == 1) &&
               ((DAT_00709d0b[iVar2] == 1) || (DAT_00709d0b[iVar2] == 2))) ||
              ((DAT_00709d0a[(unsigned int)DAT_0070ad48[iVar11] * 3] == 2) &&
               (DAT_00709d0b[iVar2] == 1))) goto LAB_0041e6df;
        }
        bVar14 = 1;
        if ((DAT_0052687a[(unsigned int)DAT_005247e8[DAT_0070ad42[iVar11]] * 0x16] & 0x80) == 0) {
          if (DAT_00732a3c < DAT_006d4980) {
            puVar13 = DAT_006d4988 + (DAT_00732a3c * 0x23 + 0x1f);
            uVar6 = DAT_00732a3c;
            do {
              if ((puVar13[-0x1c] != 0)) {
                __builtin_memcpy(&pbVar16, puVar13, 4);
                if ((*pbVar16 == 0) && (puVar13[-9] == uVar5)) {
                  __builtin_memcpy(&DAT_00732e54, pbVar16 + 0xa7, 4);
                  if ((*((unsigned char *)DAT_00732e54 + 7) != 3) &&
                      (*((unsigned char *)DAT_00732e54 + 7) != 4)) goto LAB_0041e40a;
                }
              }
              puVar13 = puVar13 + 0x23;
              uVar6 = uVar6 + 1;
            } while (uVar6 < DAT_006d4980);
          }
        }
        else if (DAT_00732a38 < DAT_00703c88 + DAT_00732a38) {
          pcVar7 = DAT_006d4988 + (DAT_00732a38 * 0x23 + 3);
          uVar6 = DAT_00732a38;
          do {
            if ((*pcVar7 != 0) && (pcVar7[0x13] == uVar5)) goto LAB_0041e40a;
            pcVar7 = pcVar7 + 0x23;
            uVar6 = uVar6 + 1;
          } while (uVar6 < DAT_00703c88 + DAT_00732a38);
        }
        goto LAB_0041e40f;
      }
      iVar11 = (int)(uVar5 * 0x566);
      __builtin_memcpy(&iVar15, &DAT_0070b252[iVar11], 4);
      if ((((DAT_005248f8[DAT_0070ad5d[iVar11]] & DAT_00524900[DAT_007092d4]) != 0) ||
           (iVar15 != -1)) && (DAT_006d4a6a[uVar5 * 0x93] != 0)) {
        uVar5 = FUN_0049d810(&DAT_0070ad5e[iVar11], 1);
        uVar6 = FUN_0049d810(&DAT_0070ad69[(unsigned int)DAT_0073b374 * 0x566], 1);
        uVar9 = (unsigned int)DAT_0073b374;
        iVar11 = (int)(uVar9 * 0x566);
        if (DAT_0070ad74[uVar9 * 0x566] == 1) {
          bVar10 = (unsigned char)uVar5 | (unsigned char)uVar6;
        }
        else {
          bVar10 = (unsigned char)uVar5 & (unsigned char)uVar6;
        }
        __builtin_memcpy(&iVar15, &DAT_0070b252[iVar11], 4);
        if (((bVar10 & 1) != 0) && ((iVar15 != -1) || (DAT_0070ad55[iVar11] == 0))) {
          bVar10 = DAT_0070ad86[iVar11];
          sVar8 = (unsigned short)((unsigned short)DAT_0070ad76[iVar11] * 0x3c +
                                   (unsigned short)DAT_0070ad77[iVar11]);
          DAT_006d4a62[uVar9 * 0x93] = sVar8;
          uVar1 = (unsigned short)((unsigned short)DAT_0070ad75[iVar11] * 0x3c +
                                   (unsigned short)bVar10);
          if (DAT_007092d7 == 0) {
            DAT_006d4a62[uVar9 * 0x93] = (unsigned short)((uVar1 >> 1) + sVar8);
          }
          else if (uVar1 != 0) {
            uVar4 = FUN_00478470();
            DAT_006d4a62[(unsigned int)DAT_0073b374 * 0x93] =
                (unsigned short)(DAT_006d4a62[(unsigned int)DAT_0073b374 * 0x93] +
                                 (unsigned short)(uVar4 - (uVar4 / uVar1) * uVar1));
          }
          DAT_006d4a64[(unsigned int)DAT_0073b374 * 0x126] = 1;
        }
      }
      goto LAB_0041e6df;
    }
  }
LAB_0041e6fd:
  if (DAT_0071e6a8 == 0) {
    DAT_0071e6a8 = 0xec;
    DAT_0073b374 = 0;
    if (0 < DAT_006d49a0) {
      do {
        uVar5 = (unsigned int)DAT_0073b374;
        iVar11 = (int)(uVar5 * 0x126);
        if ((DAT_006d4a60[uVar5 * 0x126] == 0) && (DAT_006d4a64[iVar11] == 1)) {
          if (DAT_006d4a62[uVar5 * 0x93] == 0) {
            iVar2 = (int)(uVar5 * 0x566);
            bVar10 = DAT_0052687a[(unsigned int)DAT_005247e8[DAT_0070ad42[iVar2]] * 0x16];
            if ((bVar10 & 0x80) == 0) {
              uVar5 = 0;
              if (DAT_00732a3c < DAT_006d4980) {
                pcVar7 = DAT_006d4988 + (DAT_00732a3c * 0x23 + 3);
                iVar12 = (int)(DAT_006d4980 - DAT_00732a3c);
                do {
                  if (*pcVar7 == 0) {
                    uVar5 = uVar5 + 1;
                  }
                  pcVar7 = pcVar7 + 0x23;
                  iVar12 = iVar12 + -1;
                } while (iVar12 != 0);
              }
              bVar14 = (unsigned int)DAT_0070ad43[iVar2] <= uVar5;
            }
            else {
              bVar14 = 1;
            }
            if (bVar14) {
              DAT_006d4a60[iVar11] = 1;
              if ((bVar10 & 0x80) == 0) {
                DAT_006d4a61[iVar11] = DAT_0070ad51[iVar2];
                FUN_0041eea0(0xffff);
              }
              else {
                if (DAT_006d4a02 == 2) {
                  DAT_006d4a61[iVar11] = DAT_0070ad51[iVar2];
                }
                else {
                  DAT_006d4a61[iVar11] = 0;
                }
                FUN_004215c0(0xffff);
              }
            }
          }
          else {
            DAT_006d4a62[uVar5 * 0x93] = (unsigned short)(DAT_006d4a62[uVar5 * 0x93] - 1);
          }
        }
        DAT_0073b374 = (unsigned short)(DAT_0073b374 + 1);
      } while ((int)(unsigned int)DAT_0073b374 < (int)DAT_006d49a0);
    }
  }
  return;
LAB_0041e40a:
  bVar14 = 0;
LAB_0041e40f:
  if (bVar14) {
    bVar14 = 0;
    __builtin_memcpy(&iVar15, &DAT_0070b252[iVar11], 4);
    if ((iVar15 == -1) || (DAT_007092db != 2)) {
      if (((DAT_0070ad78[iVar11] != 0) || (DAT_0070ad7c[iVar11] != 0)) &&
          (uVar5 = FUN_0049d810(&DAT_0070ad78[iVar11], 0), (uVar5 & 1) != 0)) {
        bVar14 = 1;
      }
      iVar11 = (int)((unsigned int)DAT_0073b374 * 0x566);
      cVar3 = (char)DAT_0070ad53[iVar11];
      if ((cVar3 == 1) && (DAT_006d4a8a[(unsigned int)DAT_0073b374 * 0x93] != 0)) {
        bVar14 = 1;
      }
      if ((cVar3 == 2) && (DAT_00709d0a[(unsigned int)DAT_0070ad48[iVar11] * 3] == 1)) {
        bVar14 = 1;
      }
      if ((cVar3 == 3) &&
          ((DAT_00709d0a[(unsigned int)DAT_0070ad48[iVar11] * 3] == 2 ||
            (DAT_00709d0b[(unsigned int)DAT_0070ad48[iVar11] * 3] == 1)))) {
        bVar14 = 1;
      }
    }
    if (bVar14) {
      uVar5 = (unsigned int)DAT_0073b374;
      iVar11 = (int)(uVar5 * 0x126);
      sVar8 = (unsigned short)(DAT_006d4a6a[uVar5 * 0x93] - DAT_006d4a6c[uVar5 * 0x93]);
      DAT_006d4a6c[uVar5 * 0x93] = (unsigned short)(DAT_006d4a6c[uVar5 * 0x93] + sVar8);
      cVar3 = (char)(DAT_006d4aa2[iVar11] - DAT_006d4aa3[iVar11]);
      DAT_006d4a96[uVar5 * 0x93] = (unsigned short)(DAT_006d4a96[uVar5 * 0x93] + sVar8);
      DAT_006d4ab8[iVar11] = (unsigned char)(DAT_006d4ab8[iVar11] + cVar3);
      DAT_006d4a7c[uVar5 * 0x93] = (unsigned short)(DAT_006d4a7c[uVar5 * 0x93] + sVar8);
      DAT_006d4aab[iVar11] = (unsigned char)(DAT_006d4aab[iVar11] + cVar3);
      DAT_006d4a88[uVar5 * 0x93] = (unsigned short)(DAT_006d4a88[uVar5 * 0x93] + sVar8);
      DAT_006d4ab1[iVar11] = (unsigned char)(DAT_006d4ab1[iVar11] + cVar3);
      DAT_006d4a78[uVar5 * 0x93] = (unsigned short)(DAT_006d4a78[uVar5 * 0x93] + sVar8);
      DAT_006d4aa9[iVar11] = (unsigned char)(DAT_006d4aa9[iVar11] + cVar3);
      DAT_006d4a74[uVar5 * 0x93] = (unsigned short)(DAT_006d4a74[uVar5 * 0x93] + sVar8);
      DAT_006d4aa7[iVar11] = (unsigned char)(DAT_006d4aa7[iVar11] + cVar3);
      DAT_006d4a80[uVar5 * 0x93] = (unsigned short)(DAT_006d4a80[uVar5 * 0x93] + sVar8);
      DAT_006d4aad[iVar11] = (unsigned char)(DAT_006d4aad[iVar11] + cVar3);
      DAT_006d4a61[iVar11] = 0;
      if ((DAT_0070ad8d[uVar5 * 0x566] == 0) && (DAT_0070ad8f[uVar5 * 0x566] == 0)) {
        DAT_006d4a8c[uVar5 * 0x93] = (unsigned short)(DAT_006d4a8c[uVar5 * 0x93] + sVar8);
      }
      else {
        DAT_006d4a8e[uVar5 * 0x93] = (unsigned short)(DAT_006d4a8e[uVar5 * 0x93] + sVar8);
      }
    }
    else {
      if ((DAT_0052687a[(unsigned int)DAT_005247e8[DAT_0070ad42[(unsigned int)DAT_0073b374 * 0x566]] * 0x16] &
           0x80) == 0) {
        uVar5 = 0;
        if (DAT_00732a3c < DAT_006d4980) {
          iVar11 = (int)(DAT_006d4980 - DAT_00732a3c);
          pcVar7 = DAT_006d4988 + (DAT_00732a3c * 0x23 + 3);
          do {
            if (*pcVar7 == 0) {
              uVar5 = uVar5 + 1;
            }
            pcVar7 = pcVar7 + 0x23;
            iVar11 = iVar11 + -1;
          } while (iVar11 != 0);
        }
        bVar14 = (unsigned int)DAT_0070ad43[(unsigned int)DAT_0073b374 * 0x566] <= uVar5;
      }
      else {
        bVar14 = 1;
      }
      if (bVar14) {
        if ((DAT_0052687a[(unsigned int)DAT_005247e8[DAT_0070ad42[(unsigned int)DAT_0073b374 * 0x566]] * 0x16] &
             0x80) == 0) {
          FUN_0041eea0(0xffff);
        }
        else {
          FUN_004215c0(0xffff);
        }
        uVar5 = (unsigned int)DAT_0073b374;
        if ((DAT_006d4a61[uVar5 * 0x126] != 0) &&
            ((DAT_0070ad54[uVar5 * 0x566] == 0) || (DAT_007092db != 2))) {
          DAT_006d4a61[uVar5 * 0x126] = (unsigned char)(DAT_006d4a61[uVar5 * 0x126] - 1);
        }
      }
    }
  }
LAB_0041e6df:
  DAT_0073b374 = (unsigned short)(DAT_0073b374 + 1);
  if ((int)DAT_006d49a0 <= (int)(unsigned int)DAT_0073b374) goto LAB_0041e6fd;
  goto LAB_0041e0b5;
}
