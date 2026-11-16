/* SPDX-License-Identifier: MIT */
/* z_xvt 004253a0 */
/* Prototypes */
unsigned int __cdecl FUN_004a7aa0(unsigned int a);
void __cdecl FUN_0042a230(int c, int d, int e, int f);
void __stdcall FUN_00401000(void);
void __cdecl FUN_00421df0(unsigned int a, int b);
unsigned char __stdcall FUN_00478470(void);
void __cdecl FUN_00412700(unsigned int a, int b);
void __cdecl FUN_0049f9c0(unsigned int a, unsigned int b, int c);
int __cdecl FUN_0044ad00(unsigned int a);
void __cdecl FUN_0041a750(unsigned int a, int b);
int __cdecl FUN_0042b150(int a);
void __cdecl FUN_00483410(short a, short b, int c);
unsigned long long __cdecl FUN_00413ca0(int c, unsigned int d);
void __cdecl FUN_00421c70(unsigned int a, int b);
int __cdecl FUN_00449b30(unsigned int a, unsigned int b);
int __cdecl FUN_00449cb0(unsigned int a, unsigned int b);
int __cdecl FUN_00449bf0(unsigned int a, unsigned int b);
void __cdecl FUN_00418cd0(int a, int b, int c);

/* Globals */
extern unsigned char DAT_0071b740[];
extern unsigned char DAT_0071b730[];
extern unsigned char DAT_0071b785[];
extern unsigned char DAT_0071b789[];
extern unsigned char DAT_0071b78d[];
extern unsigned char DAT_0071b791[];
extern unsigned char DAT_0071b795[];
extern unsigned char DAT_0071b799[];
extern unsigned char DAT_0052b13f[];
extern unsigned char DAT_0052b13d[];
extern unsigned char DAT_0052b141[];
extern unsigned char DAT_0052b143[];
extern unsigned char DAT_0052b145[];
extern unsigned char DAT_0052b147[];
extern unsigned char DAT_0052b149[];
extern unsigned char DAT_0052b09d[];
extern unsigned char DAT_0052686c[];
extern unsigned char DAT_00525bf7[];
extern unsigned char DAT_005248c7[];
extern unsigned char DAT_00524968[];
extern unsigned char DAT_005249c0[];
extern unsigned int DAT_0071ecf4;
extern unsigned int DAT_0071e690;
extern int DAT_006d4988;
extern int DAT_006d4980;
extern int DAT_0070ab18;
extern int DAT_0070ab14;
extern int DAT_0070ac20;
extern short DAT_0070a1d2;
extern unsigned short DAT_0071e6be;
extern int DAT_00732a38;
extern int DAT_00732a2c;
extern int DAT_00732a28;
extern int DAT_00732a24;
extern int DAT_006d4a50;
extern int DAT_006ffbec;
extern int DAT_0072f4e8;
extern int DAT_0071b71c;
extern short DAT_006da9f0;
extern short DAT_00732e40;
extern int DAT_006db720;
extern int DAT_006db704;
extern int DAT_006db708;
extern char DAT_007092c1;

void __stdcall FUN_004253a0(void)
{
  unsigned char *pE, *pI, *pO, *pP, *pQ, *pS, *pT, *pq;
  int iVar2, iVar13, iVar14, iVar17, iVar18, iVar20;
  unsigned char uVar3, bVar4, bVar5, bVar10;
  unsigned short uVar6, uVar21, uVar28, uStack_1e, u16, us16;
  short sVar9, sVar12, sVar24, sVar26, sStack_1c, s16, s16b;
  char cVar11;
  int bVar7, bVar8;
  unsigned int uVar15, uVar16, uVar19, uVar22, u32;
  int s32;
  unsigned long long uVar29;
  unsigned int extraout_EDX = 0, extraout_EDX_00 = 0, extraout_EDX_01 = 0, extraout_EDX_02 = 0,
               extraout_EDX_03 = 0, extraout_EDX_04 = 0, extraout_EDX_06 = 0,
               extraout_EDX_07 = 0;

  sStack_1c = 0;
  uVar28 = 0;
  do {
    uVar16 = (unsigned int)uVar28;
    iVar2 = (int)(uVar16 * 0x5bdu);
    if ((((((DAT_0071b740[iVar2] != 0) && (DAT_0071ecf4 != uVar16)) &&
         (__builtin_memcpy(&uVar16, DAT_0071b730 + iVar2, 4), uVar16 != 0xffffffffu)) &&
        (iVar17 = (int)(uVar16 * 0x23u) + DAT_006d4988, *(char *)(iVar17 + 2) == 0)) &&
       ((DAT_0071e690 == 0xffffffffu || (DAT_0071e690 == uVar16))))) {
      uVar16 = FUN_004a7aa0((((uVar16 >> 8) << 8) | (unsigned int)*(unsigned char *)(iVar17 + 3)) & 0xffff00ffu);
      iVar17 = (int)((uVar16 & 0xffff) * 0xefu);
      __builtin_memcpy(&u32, DAT_0071b730 + iVar2, 4);
      iVar18 = (int)(u32 * 0x23u) + DAT_006d4988;
      __builtin_memcpy(&s16, DAT_0052b13f + iVar17, 2);
      __builtin_memcpy(&s16b, DAT_0052b13d + iVar17, 2);
      FUN_0042a230(iVar18, 0, s16, s16b);
      __builtin_memcpy(&u32, DAT_0071b785 + iVar2, 4);
      __builtin_memcpy(DAT_0071b791 + iVar2, &u32, 4);
      uVar22 = (unsigned int)DAT_0070ab18;
      __builtin_memcpy(&u32, DAT_0071b789 + iVar2, 4);
      __builtin_memcpy(DAT_0071b795 + iVar2, &u32, 4);
      uVar16 = (unsigned int)DAT_0070ab14;
      __builtin_memcpy(&u32, DAT_0071b78d + iVar2, 4);
      __builtin_memcpy(DAT_0071b799 + iVar2, &u32, 4);
      uVar19 = (unsigned int)DAT_0070ac20;
      __builtin_memcpy(DAT_0071b785 + iVar2, &uVar22, 4);
      __builtin_memcpy(DAT_0071b789 + iVar2, &uVar16, 4);
      __builtin_memcpy(DAT_0071b78d + iVar2, &uVar19, 4);
    }
    uVar28 = (unsigned short)(uVar28 + 1);
  } while (uVar28 < 8);

  uVar22 = DAT_0071ecf4 & 0xffff;
  iVar2 = (int)(uVar22 * 0x5bdu);
  __builtin_memcpy(&uVar16, DAT_0071b730 + iVar2, 4);
  uVar22 = uVar22 * 5;
  if (((uVar16 != 0xffffffffu) &&
      (iVar17 = (int)(uVar16 * 0x23u) + DAT_006d4988, uVar22 = (unsigned int)DAT_006d4988,
       *(char *)(iVar17 + 2) == 0)) &&
     ((DAT_0071e690 == 0xffffffffu || (uVar22 = DAT_0071e690, DAT_0071e690 == uVar16)))) {
    uVar16 = FUN_004a7aa0((((uVar16 >> 8) << 8) | (unsigned int)*(unsigned char *)(iVar17 + 3)) & 0xffff00ffu);
    iVar17 = (int)((uVar16 & 0xffff) * 0xefu);
    __builtin_memcpy(&u32, DAT_0071b730 + iVar2, 4);
    iVar18 = (int)(u32 * 0x23u) + DAT_006d4988;
    __builtin_memcpy(&s16, DAT_0052b13f + iVar17, 2);
    __builtin_memcpy(&s16b, DAT_0052b13d + iVar17, 2);
    FUN_0042a230(iVar18, 0, s16, s16b);
    __builtin_memcpy(&u32, DAT_0071b785 + iVar2, 4);
    __builtin_memcpy(DAT_0071b791 + iVar2, &u32, 4);
    uVar19 = (unsigned int)DAT_0070ab18;
    __builtin_memcpy(&u32, DAT_0071b789 + iVar2, 4);
    __builtin_memcpy(DAT_0071b795 + iVar2, &u32, 4);
    uVar16 = (unsigned int)DAT_0070ab14;
    __builtin_memcpy(&u32, DAT_0071b78d + iVar2, 4);
    __builtin_memcpy(DAT_0071b799 + iVar2, &u32, 4);
    uVar22 = (unsigned int)DAT_0070ac20;
    __builtin_memcpy(DAT_0071b785 + iVar2, &uVar19, 4);
    __builtin_memcpy(DAT_0071b789 + iVar2, &uVar16, 4);
    __builtin_memcpy(DAT_0071b78d + iVar2, &uVar22, 4);
  }

  uVar28 = DAT_0071e6be;
  sVar9 = DAT_0070a1d2;
  bVar8 = 0;
  uStack_1e = 0;
  if (0 < DAT_00732a38) {
    do {
      if (DAT_0071e690 != 0xffffffffu) {
        if (bVar8) {
          DAT_0070a1d2 = sVar9;
          DAT_0071e6be = uVar28;
          return;
        }
        bVar8 = 1;
        uStack_1e = (unsigned short)DAT_0071e690;
      }
      uVar16 = (unsigned int)uStack_1e;
      iVar17 = (int)(uVar16 * 0x23u);
      pE = (unsigned char *)(DAT_006d4988 + iVar17);
      DAT_0070a1d2 = sVar9;
      DAT_0071e6be = uVar28;
      __builtin_memcpy(&pI, pE + 0x1f, 4);
      if ((pI != 0) && (__builtin_memcpy(&iVar18, pI + 2, 4), iVar18 != 0)) {
        DAT_0071e6be = (unsigned short)(uVar28 + (short)((short)DAT_0071b71c - (short)iVar18));
        if (DAT_0071e6be != 0) {
          uVar19 = (unsigned int)DAT_0071e6be;
          uVar22 = 0xecu % uVar19;
          DAT_0070a1d2 = (short)(0xecu / uVar19);
          if (DAT_0070a1d2 == 0) {
            DAT_0070a1d2 = 1;
          }
          __builtin_memcpy(&s32, pI + 2, 4);
          s32 = (int)((unsigned int)s32 + uVar19);
          __builtin_memcpy(pI + 2, &s32, 4);
          goto LAB_004256ff;
        }
        if ((DAT_0071e690 == 0xffffffffu) &&
            (__builtin_memcpy(&u32, pE + 0x1b, 4), u32 != 0xffffffffu)) {
          if ((DAT_007092c1 != 0) && (DAT_0071ecf4 == u32)) {
            FUN_00401000();
          }
          __builtin_memcpy(&s32, pE + 4, 4);
          __builtin_memcpy(pI + 6, &s32, 4);
          __builtin_memcpy(&uVar22, pE + 8, 4);
          __builtin_memcpy(pI + 10, &uVar22, 4);
          __builtin_memcpy(&s32, pE + 0xc, 4);
          __builtin_memcpy(pI + 0xe, &s32, 4);
        }
        goto switchD_004259f9_caseD_5;
      }
LAB_004256ff:
      if (pE[3] == 0) goto switchD_004259f9_caseD_5;
      uVar3 = pE[2];
      __builtin_memcpy(&uVar6, pI + 0x7f, 2);
      if (uVar6 == 0) goto LAB_0042587d;
      uVar21 = (unsigned short)(uVar6 - DAT_0071e6be);
      if (uVar6 < uVar21) {
        uVar21 = 0;
      }
      uVar22 = (uVar22 & 0xffff0000u) | (unsigned int)uVar21;
      __builtin_memcpy(pI + 0x7f, &uVar21, 2);
      if (uVar21 != 0) goto LAB_0042587d;
      switch (uVar3) {
      case 0:
        FUN_00421df0(((unsigned int)(unsigned short)sStack_1c << 16) | (unsigned int)uStack_1e, 1);
        bVar10 = FUN_00478470();
        FUN_00412700(uVar16, (bVar10 & 1) + 0x7f);
        FUN_0049f9c0(uStack_1e, (unsigned short)*(unsigned char *)(DAT_006d4988 + 0x16 + iVar17), 2);
        uVar22 = extraout_EDX;
        break;
      case 1:
      case 2:
      case 3:
      case 4:
      case 5:
        iVar18 = FUN_0044ad00((unsigned int)pE[3]);
        if (iVar18 == 0) {
          FUN_0041a750(((unsigned int)(unsigned short)sStack_1c << 16) | (unsigned int)uStack_1e, 1);
          FUN_00412700(uVar16, 0x82);
          __builtin_memcpy(&pq, (unsigned char *)(DAT_006d4988 + iVar17) + 0x1f, 4);
          __builtin_memcpy(&u16, DAT_0052686c + (unsigned int)pE[3] * 0x16u, 2);
          pq[1] = (unsigned char)(u16 >> 9);
        }
        else {
          FUN_00412700(uVar16, 0x82);
        }
        FUN_0049f9c0(uStack_1e, (unsigned short)*(unsigned char *)(DAT_006d4988 + 0x16 + iVar17), 2);
        uVar22 = extraout_EDX_00;
        break;
      case 6:
      case 7:
        if (DAT_00525bf7[pE[3]] != 0) {
          cVar11 = -0x7f;
          goto LAB_00425875;
        }
        pE[3] = 0;
        goto switchD_004259f9_caseD_5;
      default:
        pE[3] = 0;
        goto switchD_004259f9_caseD_5;
      case 0xb:
        bVar10 = FUN_00478470();
        cVar11 = (char)((bVar10 & 1) + 0x7f);
LAB_00425875:
        FUN_00412700(uVar16, cVar11);
        uVar22 = extraout_EDX_01;
      }
LAB_0042587d:
      if (DAT_0071e690 == 0xffffffffu) {
        if ((DAT_007092c1 != 0) &&
            (__builtin_memcpy(&u32, (unsigned char *)(DAT_006d4988 + iVar17) + 0x1b, 4),
             u32 == DAT_0071ecf4)) {
          FUN_00401000();
        }
        __builtin_memcpy(&pI, pE + 0x1f, 4);
        __builtin_memcpy(&s32, pE + 4, 4);
        __builtin_memcpy(pI + 6, &s32, 4);
        __builtin_memcpy(&uVar22, pE + 8, 4);
        __builtin_memcpy(pI + 10, &uVar22, 4);
        __builtin_memcpy(&s32, pE + 0xc, 4);
        __builtin_memcpy(pI + 0xe, &s32, 4);
      }
      __builtin_memcpy(&pI, pE + 0x1f, 4);
      __builtin_memcpy(&sVar24, pI + 0x77, 2);
      pS = pI + 0x77;
      if (sVar24 != 0) {
        if (((int)uVar16 < DAT_006d4980) &&
           (__builtin_memcpy(&pq, (unsigned char *)(DAT_006d4988 + iVar17) + 0x1f, 4),
            __builtin_memcpy(&pO, pq + 0xa7, 4),
            __builtin_memcpy(&s16, pO + 0x70, 2),
            s16 != -1)) {
          if (sVar24 < 0) {
            sStack_1c = 0xec;
            s16 = (short)((((unsigned int)DAT_0071e6be << 0xc) / 0xecu)) + sVar24;
            __builtin_memcpy(pS, &s16, 2);
            __builtin_memcpy(&pI, pE + 0x1f, 4);
            pS = pI + 0x77;
            __builtin_memcpy(&s16, pS, 2);
            if (-1 < s16) {
LAB_0042595c:
              s16 = 0;
              __builtin_memcpy(pS, &s16, 2);
              __builtin_memcpy(&pI, pE + 0x1f, 4);
              __builtin_memcpy(&sVar24, pI + 0x77, 2);
              us16 = 0xffff;
              __builtin_memcpy(pO + 0x70, &us16, 2);
            }
          }
          else {
            sStack_1c = -0xec;
            s16 = (short)((int)((unsigned int)DAT_0071e6be << 0xc) / -0xec) + sVar24;
            __builtin_memcpy(pS, &s16, 2);
            __builtin_memcpy(&pI, pE + 0x1f, 4);
            pS = pI + 0x77;
            __builtin_memcpy(&s16, pS, 2);
            if (s16 < 1) goto LAB_0042595c;
          }
        }
        iVar13 = (int)((unsigned int)(int)sVar24 * (unsigned int)DAT_0071e6be);
        uVar22 = (unsigned int)(iVar13 % 0xec);
        __builtin_memcpy(&s16, pE + 0x14, 2);
        s16 = (short)(s16 + (short)(iVar13 / 0xec) * 4);
        __builtin_memcpy(pE + 0x14, &s16, 2);
        __builtin_memcpy(&pI, pE + 0x1f, 4);
        pI[0x90] = 1;
      }
      uVar19 = 0;
      __builtin_memcpy(&pI, pE + 0x1f, 4);
      __builtin_memcpy(&us16, pI + 0x79, 2);
      if (us16 != 0) {
        iVar13 = (int)((unsigned int)((int)((unsigned int)us16 * 0x1234u + 0x80u) >> 8)
                       * (unsigned int)DAT_0071e6be);
        uVar22 = (unsigned int)(iVar13 % 0xec);
        uVar19 = (unsigned int)(iVar13 / 0xec) & 0xffff;
      }
      switch (uVar3) {
      case 0:
      case 1:
      case 2:
      case 3:
      case 4:
        __builtin_memcpy(&pO, pI + 0xa7, 4);
        iVar13 = FUN_0042b150((int)pO);
        sStack_1c = (short)iVar13;
        uVar15 = extraout_EDX_02;
        __builtin_memcpy(&pI, pE + 0x1f, 4);
        if (pI[0x89] != 0) {
          __builtin_memcpy(&s16, pE + 0x12, 2);
          __builtin_memcpy(&s16b, pE + 0x10, 2);
          FUN_00483410(s16, s16b, (int)pE);
          uVar15 = extraout_EDX_03;
        }
        __builtin_memcpy(&pI, pE + 0x1f, 4);
        __builtin_memcpy(&s16, pI + 0x8a, 2);
        iVar20 = (int)s16;
        uVar29 = FUN_00413ca0(iVar20, uVar19);
        DAT_006d4a50 = (int)uVar29;
        __builtin_memcpy(&pI, pE + 0x1f, 4);
        __builtin_memcpy(&s16, pI + 0x8c, 2);
        iVar20 = (int)s16;
        uVar29 = FUN_00413ca0(iVar20, uVar19);
        DAT_006ffbec = (int)uVar29;
        __builtin_memcpy(&pI, pE + 0x1f, 4);
        __builtin_memcpy(&s16, pI + 0x8e, 2);
        iVar20 = (int)s16;
        uVar29 = FUN_00413ca0(iVar20, uVar19);
        iVar20 = (int)(uVar29 >> 0x20);
        DAT_0072f4e8 = (int)uVar29;
        __builtin_memcpy(&s16, pO + 0x1df, 2);
        if (s16 != 0) {
          if (*((char *)iVar13 + 0x34) == 0x12) {
            uVar16 = 0xfa;
          }
          else if (*((char *)iVar13 + 0x34) == 0x1e) {
            uVar16 = 0x2ee;
          }
          else {
            iVar20 = (int)((unsigned int)pO[4] * 3);
            __builtin_memcpy(&us16, DAT_0052b09d + (unsigned int)pO[4] * 0xefu, 2);
            uVar16 = (unsigned int)us16;
          }
          __builtin_memcpy(&iVar13, pO + 0xb1, 4);
          sVar24 = (short)uVar16;
          if (iVar13 != 0) {
            if ((0u - (unsigned int)iVar13) == uVar16 || (int)(0u - uVar16) < iVar13) {
              sVar12 = sVar24;
              if (iVar13 <= (int)uVar16) {
                sVar12 = (short)iVar13;
              }
            }
            else {
              sVar12 = (short)(-sVar24);
            }
            sStack_1c = 0xec;
            iVar14 = (int)((unsigned int)(int)sVar12 * (unsigned int)DAT_0071e6be) / 0xec;
            iVar20 = (int)((unsigned int)(int)sVar12 * (unsigned int)DAT_0071e6be) % 0xec;
            if (iVar14 == 0) {
              iVar14 = iVar13;
            }
            s32 = (int)((unsigned int)iVar13 - (unsigned int)iVar14);
            __builtin_memcpy(pO + 0xb1, &s32, 4);
            DAT_006d4a50 = (int)((unsigned int)DAT_006d4a50 + (unsigned int)iVar14);
          }
          __builtin_memcpy(&iVar13, pO + 0xb5, 4);
          if (iVar13 != 0) {
            if ((0u - (unsigned int)iVar13) == uVar16 || (int)(0u - uVar16) < iVar13) {
              sVar12 = sVar24;
              if (iVar13 <= (int)uVar16) {
                sVar12 = (short)iVar13;
              }
            }
            else {
              sVar12 = (short)(-sVar24);
            }
            sStack_1c = 0xec;
            iVar14 = (int)((unsigned int)(int)sVar12 * (unsigned int)DAT_0071e6be) / 0xec;
            iVar20 = (int)((unsigned int)(int)sVar12 * (unsigned int)DAT_0071e6be) % 0xec;
            if (iVar14 == 0) {
              iVar14 = iVar13;
            }
            s32 = (int)((unsigned int)iVar13 - (unsigned int)iVar14);
            __builtin_memcpy(pO + 0xb5, &s32, 4);
            DAT_006ffbec = (int)((unsigned int)DAT_006ffbec + (unsigned int)iVar14);
          }
          uVar29 = ((unsigned long long)(unsigned int)iVar20 << 32) | (unsigned int)DAT_0072f4e8;
          __builtin_memcpy(&uVar22, pO + 0xb9, 4);
          if (uVar22 != 0) {
            if ((0u - uVar22) == uVar16 || (int)(0u - uVar16) < (int)uVar22) {
              if ((int)uVar22 <= (int)uVar16) {
                uVar16 = uVar22 & 0xffff;
              }
            }
            else {
              uVar16 = (unsigned int)(unsigned short)(-sVar24);
            }
            iVar13 = (int)((unsigned int)(int)(short)uVar16 * (unsigned int)DAT_0071e6be);
            uVar16 = (unsigned int)(iVar13 / 0xec);
            if (uVar16 == 0) {
              uVar16 = uVar22;
            }
            u32 = uVar22 - uVar16;
            __builtin_memcpy(pO + 0xb9, &u32, 4);
            DAT_0072f4e8 = (int)((unsigned int)DAT_0072f4e8 + uVar16);
            uVar29 = ((unsigned long long)(unsigned int)(iVar13 % 0xec) << 32) | (unsigned int)DAT_0072f4e8;
          }
        }
        uVar22 = (unsigned int)(uVar29 >> 0x20);
        DAT_0072f4e8 = (int)uVar29;
        __builtin_memcpy(&s32, pE + 4, 4);
        iVar13 = (int)((unsigned int)s32 + (unsigned int)DAT_006d4a50);
        __builtin_memcpy(pE + 4, &iVar13, 4);
        if (iVar13 < -0x1000000) {
          u32 = 0xff000000u;
          __builtin_memcpy(pE + 4, &u32, 4);
        }
        __builtin_memcpy(&s32, pE + 4, 4);
        if (0x1000000 < s32) {
          s32 = 0x1000000;
          __builtin_memcpy(pE + 4, &s32, 4);
        }
        __builtin_memcpy(&s32, pE + 8, 4);
        iVar13 = (int)((unsigned int)s32 + (unsigned int)DAT_006ffbec);
        __builtin_memcpy(pE + 8, &iVar13, 4);
        if (iVar13 < -0x1000000) {
          u32 = 0xff000000u;
          __builtin_memcpy(pE + 8, &u32, 4);
        }
        __builtin_memcpy(&s32, pE + 8, 4);
        if (0x1000000 < s32) {
          s32 = 0x1000000;
          __builtin_memcpy(pE + 8, &s32, 4);
        }
        __builtin_memcpy(&s32, pE + 0xc, 4);
        iVar13 = (int)((unsigned int)s32 + (unsigned int)DAT_0072f4e8);
        __builtin_memcpy(pE + 0xc, &iVar13, 4);
        if (iVar13 < -0x1000000) {
          u32 = 0xff000000u;
          __builtin_memcpy(pE + 0xc, &u32, 4);
        }
        __builtin_memcpy(&s32, pE + 0xc, 4);
        if (0x1000000 < s32) {
          s32 = 0x1000000;
          __builtin_memcpy(pE + 0xc, &s32, 4);
        }
        __builtin_memcpy(&us16, pO + 0x66, 2);
        if (us16 != 0xffff) {
          iVar20 = (int)((unsigned int)us16 * 0x23u);
          pT = (unsigned char *)(DAT_006d4988 + iVar20);
          __builtin_memcpy(&pQ, pT + 0x1f, 4);
          if (pQ != 0) {
            __builtin_memcpy(&pq, pQ + 0xa7, 4);
            iVar13 = (int)((unsigned int)pq[4] * 0xefu);
            if (pT[2] < 3) {
              __builtin_memcpy(&s16, DAT_0052b147 + (unsigned int)pO[4] * 0xefu, 2);
              __builtin_memcpy(&s16b, DAT_0052b143 + iVar13, 2);
              uVar16 = (unsigned int)(unsigned short)(s16 - s16b);
            }
            else if (*(unsigned char *)(DAT_006d4988 + 2 + iVar17) < 3) {
              __builtin_memcpy(&s16, DAT_0052b149 + (unsigned int)pO[4] * 0xefu, 2);
              __builtin_memcpy(&s16b, DAT_0052b143 + iVar13, 2);
              uVar16 = (unsigned int)(unsigned short)(s16 - s16b);
            }
            else {
              __builtin_memcpy(&s16, DAT_0052b149 + (unsigned int)pO[4] * 0xefu, 2);
              __builtin_memcpy(&s16b, DAT_0052b145 + iVar13, 2);
              uVar16 = (unsigned int)(unsigned short)(s16 - s16b);
            }
            __builtin_memcpy(&u16, DAT_0052b141 + iVar13, 2);
            FUN_0042a230((int)pE, 0, (short)uVar16, u16);
            pT = (unsigned char *)(DAT_006d4988 + iVar20);
            __builtin_memcpy(&pQ, pT + 0x1f, 4);
            __builtin_memcpy(&s32, pT + 4, 4);
            __builtin_memcpy(pQ + 6, &s32, 4);
            __builtin_memcpy(&pQ, pT + 0x1f, 4);
            __builtin_memcpy(&s32, pT + 8, 4);
            __builtin_memcpy(pQ + 10, &s32, 4);
            __builtin_memcpy(&uVar22, pT + 0x1f, 4);
            __builtin_memcpy(&s32, pT + 0xc, 4);
            __builtin_memcpy((unsigned char *)uVar22 + 0xe, &s32, 4);
            __builtin_memcpy(&s32, pE + 4, 4);
            u32 = (unsigned int)s32 + (unsigned int)DAT_0070ab18;
            __builtin_memcpy(pT + 4, &u32, 4);
            __builtin_memcpy(&s32, pE + 8, 4);
            u32 = (unsigned int)s32 + (unsigned int)DAT_0070ab14;
            __builtin_memcpy(pT + 8, &u32, 4);
            __builtin_memcpy(&s32, pE + 0xc, 4);
            u32 = (unsigned int)s32 + (unsigned int)DAT_0070ac20;
            __builtin_memcpy(pT + 0xc, &u32, 4);
          }
        }
        break;
      case 6:
      case 7:
        __builtin_memcpy(&pP, pI + 0xa3, 4);
        bVar10 = pP[1];
        iVar18 = (int)pP;
        if ((bVar10 != 0) && (__builtin_memcpy(&us16, pP + 4, 2), us16 != 0xffff)) {
          uVar22 = (unsigned int)us16;
          if (((int)uVar22 < DAT_00732a38) &&
              (*(char *)(DAT_006d4988 + 3 + (int)(uVar22 * 0x23u)) == 0)) {
            FUN_00412700(uVar16, 0x81);
            uVar22 = extraout_EDX_04;
            break;
          }
          __builtin_memcpy(&uVar6, pP + 2, 2);
          bVar7 = 0;
          bVar4 = DAT_005248c7[pE[3]];
          if ((int)uVar22 < DAT_006d4980) {
            __builtin_memcpy(&pq, (unsigned char *)(DAT_006d4988 + (int)(uVar22 * 0x23u)) + 0x1f, 4);
            __builtin_memcpy(&pQ, pq + 0xa7, 4);
            cVar11 = *(char *)(pQ + 0x233);
            iVar18 = (int)((((unsigned int)DAT_006d4988 >> 8) << 8) | (unsigned int)(unsigned char)cVar11);
            if ((cVar11 == 3) &&
               (cVar11 = *(char *)(pQ + 0x237),
                iVar18 = (int)((((unsigned int)DAT_006d4988 >> 8) << 8) | (unsigned int)(unsigned char)cVar11),
                cVar11 != 0)) {
              bVar7 = 1;
            }
          }
          if (bVar7) goto LAB_00426241;
          FUN_00421c70(uVar22, 0);
          if ((int)uVar22 < DAT_006d4980) {
            iVar18 = (int)(uVar22 * 0x23u);
            iVar13 = DAT_006d4988 + iVar18;
            __builtin_memcpy(&pq, (unsigned char *)iVar13 + 0x1f, 4);
            if (*pq != 0) goto LAB_00426014;
            bVar5 = *(unsigned char *)(iVar13 + 3);
            if (bVar5 == 0x35) {
              if (uVar6 == 0xffff) {
                DAT_0070ab18 = FUN_00449b30(0x35, 0);
                DAT_0070ab14 = FUN_00449cb0((unsigned int)*(unsigned char *)(DAT_006d4988 + 3 + iVar18), 0);
                uVar22 = 0;
              }
              else {
                uVar22 = (unsigned int)uVar6;
                DAT_0070ab18 = FUN_00449b30(0x35, uVar22);
                DAT_0070ab14 = FUN_00449cb0((unsigned int)*(unsigned char *)(DAT_006d4988 + 3 + iVar18), uVar22);
              }
              iVar13 = FUN_00449bf0((unsigned int)*(unsigned char *)(DAT_006d4988 + 3 + iVar18), uVar22);
              DAT_0070ab14 = DAT_0070ab14 >> 1;
              DAT_0070ab18 = DAT_0070ab18 >> 1;
              DAT_0070ac20 = (int)(0u - (unsigned int)iVar13) >> 1;
              FUN_0042a230(DAT_006d4988 + iVar18, (short)DAT_0070ab18,
                           (short)DAT_0070ab14, (short)DAT_0070ac20);
              DAT_0070ab18 = (int)((unsigned int)DAT_0070ab18 * 2u);
              DAT_0070ab14 = (int)((unsigned int)DAT_0070ab14 * 2u);
              DAT_0070ac20 = (int)((unsigned int)DAT_0070ac20 * 2u);
            }
            else {
              if (uVar6 == 0xffff) {
                uVar15 = (unsigned int)FUN_00449bf0((unsigned int)bVar5, 0);
                sVar24 = (short)(-(short)uVar15);
                uVar15 = (unsigned int)FUN_00449cb0((unsigned int)bVar5, 0);
                sVar12 = (short)uVar15;
                uVar22 = 0;
              }
              else {
                uVar22 = (unsigned int)uVar6;
                uVar15 = (unsigned int)FUN_00449bf0((unsigned int)bVar5, uVar22);
                sVar24 = (short)(-(short)uVar15);
                uVar15 = (unsigned int)FUN_00449cb0((unsigned int)bVar5, uVar22);
                sVar12 = (short)uVar15;
              }
              uVar15 = (unsigned int)FUN_00449b30((unsigned int)bVar5, uVar22);
              FUN_0042a230(iVar13, (short)uVar15, sVar12, sVar24);
            }
            DAT_0070ab18 = (int)((unsigned int)DAT_0070ab18 + (unsigned int)DAT_00732a2c);
            DAT_0070ab14 = (int)((unsigned int)DAT_0070ab14 + (unsigned int)DAT_00732a28);
            DAT_0070ac20 = (int)((unsigned int)DAT_0070ac20 + (unsigned int)DAT_00732a24);
          }
          else {
LAB_00426014:
            DAT_0070ab18 = DAT_00732a2c;
            DAT_0070ab14 = DAT_00732a28;
            DAT_0070ac20 = DAT_00732a24;
          }
          FUN_00421c70(uVar16, 0);
          DAT_0070ab18 = (int)((unsigned int)DAT_0070ab18 - (unsigned int)DAT_00732a2c);
          DAT_0070ab14 = (int)((unsigned int)DAT_0070ab14 - (unsigned int)DAT_00732a28);
          DAT_0070ac20 = (int)((unsigned int)DAT_0070ac20 - (unsigned int)DAT_00732a24);
          FUN_00418cd0(DAT_0070ab18, DAT_0070ab14, DAT_0070ac20);
          __builtin_memcpy(&s16, pE + 0x10, 2);
          sVar26 = (short)(DAT_006da9f0 - s16);
          iVar18 = (int)((unsigned int)(unsigned short)((unsigned short)bVar10 + (unsigned short)bVar4) * 2u);
          __builtin_memcpy(&u16, DAT_00524968 + iVar18, 2);
          uVar16 = (unsigned int)((int)((unsigned int)u16 * (unsigned int)DAT_0071e6be) / 0xec);
          sVar12 = (short)uVar16;
          sVar24 = sVar26;
          if (sVar26 < 0) {
            sVar24 = (short)(-sVar26);
          }
          if ((int)(uVar16 & 0xffff) < (int)sVar24) {
            if (sVar26 < 0) {
              sVar12 = (short)(-sVar12);
            }
            __builtin_memcpy(&s16, pE + 0x10, 2);
            s16 = (short)(s16 + sVar12);
            __builtin_memcpy(pE + 0x10, &s16, 2);
            __builtin_memcpy(&pI, pE + 0x1f, 4);
            pS = pI + 0x79;
            __builtin_memcpy(&us16, pS, 2);
            if (200 < us16) {
              __builtin_memcpy(&u16, DAT_005249c0 + iVar18, 2);
              us16 = (unsigned short)(us16 +
                     (short)((int)((unsigned int)u16 * (unsigned int)DAT_0071e6be) / -0xec));
              __builtin_memcpy(pS, &us16, 2);
              __builtin_memcpy(&pI, pE + 0x1f, 4);
              pS = pI + 0x79;
              __builtin_memcpy(&us16, pS, 2);
              if (us16 < 200) {
                us16 = 200;
                __builtin_memcpy(pS, &us16, 2);
              }
            }
          }
          else {
            __builtin_memcpy(pE + 0x10, &DAT_006da9f0, 2);
            __builtin_memcpy(&pI, pE + 0x1f, 4);
            pS = pI + 0x79;
            __builtin_memcpy(&us16, pI + 0x79, 2);
            __builtin_memcpy(&u16, pP + 6, 2);
            if (us16 < u16) {
              __builtin_memcpy(&s16, pS, 2);
              __builtin_memcpy(&u16, DAT_005249c0 + iVar18, 2);
              s16 = (short)(s16 +
                    (short)((int)((unsigned int)u16 * (unsigned int)DAT_0071e6be) / 0xec));
              __builtin_memcpy(pS, &s16, 2);
            }
          }
          __builtin_memcpy(&sStack_1c, pE + 0x12, 2);
          sVar26 = (short)(DAT_00732e40 - sStack_1c);
          __builtin_memcpy(&u16, DAT_00524968 + iVar18, 2);
          uVar16 = (unsigned int)((int)((unsigned int)u16 * (unsigned int)DAT_0071e6be) / 0xec);
          sVar12 = (short)uVar16;
          sVar24 = sVar26;
          if (sVar26 < 0) {
            sVar24 = (short)(-sVar26);
          }
          if ((int)(uVar16 & 0xffff) < (int)sVar24) {
            if (sVar26 < 0) {
              sVar12 = (short)(-sVar12);
            }
            s16 = (short)(sStack_1c + sVar12);
            __builtin_memcpy(pE + 0x12, &s16, 2);
          }
          else {
            __builtin_memcpy(pE + 0x12, &DAT_00732e40, 2);
          }
          __builtin_memcpy(&pI, pE + 0x1f, 4);
          pI[0x90] = 1;
          pI[0x89] = pI[0x90];
          __builtin_memcpy(&s16, pE + 0x12, 2);
          __builtin_memcpy(&s16b, pE + 0x10, 2);
          FUN_00483410(s16, s16b, (int)pE);
          s16 = (short)DAT_006db720;
          __builtin_memcpy(&pI, pE + 0x1f, 4);
          __builtin_memcpy(pI + 0x8a, &s16, 2);
          s16 = (short)DAT_006db704;
          __builtin_memcpy(&pI, pE + 0x1f, 4);
          __builtin_memcpy(pI + 0x8c, &s16, 2);
          iVar18 = DAT_006db708;
          s16 = (short)DAT_006db708;
          __builtin_memcpy(&pI, pE + 0x1f, 4);
          __builtin_memcpy(pI + 0x8e, &s16, 2);
        }
LAB_00426241:
        __builtin_memcpy(&pI, pE + 0x1f, 4);
        if (pI[0x89] != 0) {
          __builtin_memcpy(&s16, pE + 0x12, 2);
          __builtin_memcpy(&s16b, pE + 0x10, 2);
          FUN_00483410(s16, s16b, (int)pE);
          iVar18 = (int)extraout_EDX_06;
        }
        (void)iVar18;
        __builtin_memcpy(&pI, pE + 0x1f, 4);
        __builtin_memcpy(&s16, pI + 0x8a, 2);
        iVar17 = (int)s16;
        uVar29 = FUN_00413ca0(iVar17, uVar19);
        DAT_006d4a50 = (int)uVar29;
        __builtin_memcpy(&pI, pE + 0x1f, 4);
        __builtin_memcpy(&s16, pI + 0x8c, 2);
        iVar17 = (int)s16;
        uVar29 = FUN_00413ca0(iVar17, uVar19);
        DAT_006ffbec = (int)uVar29;
        __builtin_memcpy(&pI, pE + 0x1f, 4);
        __builtin_memcpy(&s16, pI + 0x8e, 2);
        iVar17 = (int)s16;
        uVar29 = FUN_00413ca0(iVar17, uVar19);
        uVar22 = (unsigned int)(uVar29 >> 0x20);
        DAT_0072f4e8 = (int)uVar29;
        __builtin_memcpy(&s32, pE + 4, 4);
        iVar17 = (int)((unsigned int)s32 + (unsigned int)DAT_006d4a50);
        __builtin_memcpy(pE + 4, &iVar17, 4);
        if (iVar17 < -0x1000000) {
          u32 = 0xff000000u;
          __builtin_memcpy(pE + 4, &u32, 4);
        }
        __builtin_memcpy(&s32, pE + 4, 4);
        if (0x1000000 < s32) {
          s32 = 0x1000000;
          __builtin_memcpy(pE + 4, &s32, 4);
        }
        __builtin_memcpy(&s32, pE + 8, 4);
        iVar17 = (int)((unsigned int)s32 + (unsigned int)DAT_006ffbec);
        __builtin_memcpy(pE + 8, &iVar17, 4);
        if (iVar17 < -0x1000000) {
          u32 = 0xff000000u;
          __builtin_memcpy(pE + 8, &u32, 4);
        }
        __builtin_memcpy(&s32, pE + 8, 4);
        if (0x1000000 < s32) {
          s32 = 0x1000000;
          __builtin_memcpy(pE + 8, &s32, 4);
        }
        __builtin_memcpy(&s32, pE + 0xc, 4);
        iVar17 = (int)((unsigned int)s32 + (unsigned int)DAT_0072f4e8);
        __builtin_memcpy(pE + 0xc, &iVar17, 4);
        if (iVar17 < -0x1000000) {
          u32 = 0xff000000u;
          __builtin_memcpy(pE + 0xc, &u32, 4);
        }
        __builtin_memcpy(&iVar17, pE + 0xc, 4);
        goto joined_r0x00426427;
      case 0xb:
      case 0xd:
        if (pI[0x89] != 0) {
          __builtin_memcpy(&s16, pE + 0x12, 2);
          __builtin_memcpy(&s16b, pE + 0x10, 2);
          FUN_00483410(s16, s16b, (int)pE);
          uVar22 = extraout_EDX_07;
        }
        __builtin_memcpy(&pI, pE + 0x1f, 4);
        __builtin_memcpy(&s16, pI + 0x8a, 2);
        iVar17 = (int)s16;
        uVar29 = FUN_00413ca0(iVar17, uVar19);
        DAT_006d4a50 = (int)uVar29;
        __builtin_memcpy(&pI, pE + 0x1f, 4);
        __builtin_memcpy(&s16, pI + 0x8c, 2);
        iVar17 = (int)s16;
        uVar29 = FUN_00413ca0(iVar17, uVar19);
        DAT_006ffbec = (int)uVar29;
        __builtin_memcpy(&pI, pE + 0x1f, 4);
        __builtin_memcpy(&s16, pI + 0x8e, 2);
        iVar17 = (int)s16;
        uVar29 = FUN_00413ca0(iVar17, uVar19);
        uVar22 = (unsigned int)(uVar29 >> 0x20);
        DAT_0072f4e8 = (int)uVar29;
        __builtin_memcpy(&s32, pE + 4, 4);
        iVar17 = (int)((unsigned int)s32 + (unsigned int)DAT_006d4a50);
        __builtin_memcpy(pE + 4, &iVar17, 4);
        if (iVar17 < -0x1000000) {
          u32 = 0xff000000u;
          __builtin_memcpy(pE + 4, &u32, 4);
        }
        __builtin_memcpy(&s32, pE + 4, 4);
        if (0x1000000 < s32) {
          s32 = 0x1000000;
          __builtin_memcpy(pE + 4, &s32, 4);
        }
        __builtin_memcpy(&s32, pE + 8, 4);
        iVar17 = (int)((unsigned int)s32 + (unsigned int)DAT_006ffbec);
        __builtin_memcpy(pE + 8, &iVar17, 4);
        if (iVar17 < -0x1000000) {
          u32 = 0xff000000u;
          __builtin_memcpy(pE + 8, &u32, 4);
        }
        __builtin_memcpy(&s32, pE + 8, 4);
        if (0x1000000 < s32) {
          s32 = 0x1000000;
          __builtin_memcpy(pE + 8, &s32, 4);
        }
        __builtin_memcpy(&s32, pE + 0xc, 4);
        iVar17 = (int)((unsigned int)s32 + (unsigned int)DAT_0072f4e8);
        __builtin_memcpy(pE + 0xc, &iVar17, 4);
        if (iVar17 < -0x1000000) {
          u32 = 0xff000000u;
          __builtin_memcpy(pE + 0xc, &u32, 4);
        }
        __builtin_memcpy(&iVar17, pE + 0xc, 4);
joined_r0x00426427:
        if (0x1000000 < iVar17) {
          s32 = 0x1000000;
          __builtin_memcpy(pE + 0xc, &s32, 4);
        }
      }
switchD_004259f9_caseD_5:
      uStack_1e = (unsigned short)(uStack_1e + 1);
    } while ((int)(unsigned int)uStack_1e < DAT_00732a38);
  }
  (void)uVar22;
  DAT_0070a1d2 = sVar9;
  DAT_0071e6be = uVar28;
  return;
}
