/* SPDX-License-Identifier: MIT */
/* z_xvt 00445e10 */
#define U8(a)  (*(unsigned char *)(unsigned int)(a))
#define S8(a)  (*(signed char *)(unsigned int)(a))
#define RD16(a) ({ unsigned short _t; __builtin_memcpy(&_t, (const void *)(unsigned int)(a), 2); _t; })
#define RD32(a) ({ unsigned int _t; __builtin_memcpy(&_t, (const void *)(unsigned int)(a), 4); _t; })
#define WR16(a, v) do { unsigned short _v = (unsigned short)(v); __builtin_memcpy((void *)(unsigned int)(a), &_v, 2); } while (0)
#define WR32(a, v) do { unsigned int _v = (unsigned int)(v); __builtin_memcpy((void *)(unsigned int)(a), &_v, 4); } while (0)

extern unsigned short DAT_0071e6b4;
extern unsigned short DAT_00732a3c;
extern int DAT_006d4980;
extern int DAT_006d4988;
extern int DAT_00732e54;
extern unsigned char DAT_0070ad49[];
extern unsigned char DAT_0052b083[];
extern unsigned char DAT_0052b0b4[];
extern unsigned char DAT_0071f2dd;
extern unsigned char DAT_0071b758[];
extern unsigned char DAT_0071b757[];
extern unsigned char DAT_0071b756[];
extern unsigned char DAT_0071b741[];
extern unsigned short DAT_0071b74a;
extern unsigned char DAT_0071bce1;
extern unsigned int DAT_006db580;
extern unsigned int DAT_006d499c;
extern unsigned short DAT_0071e6be;
extern int DAT_0071ecf4;
extern unsigned short DAT_0070a1d0;
extern char DAT_007092d4;
extern int DAT_00703c88;
extern unsigned int DAT_00732a38;

unsigned int FUN_004a7aa0(unsigned int a);
unsigned long long FUN_0042a140(unsigned int a, unsigned int b);
unsigned long long FUN_00441fc0(unsigned int a, int b, unsigned int c, int d, int e);
void FUN_004a1d70(unsigned int a, int b);
short FUN_004a31a0(void);
void FUN_0040a980(int a, int b);
_Bool FUN_00444040(unsigned int a, unsigned int b);
void FUN_0041b920(unsigned int a, unsigned int b);
int FUN_0042b150(int a);
int FUN_004490f0(unsigned int a);
int FUN_00449790(unsigned int a, int b);
void FUN_0040a1a0(int a, int b, int c);
void FUN_00447240(unsigned int a, unsigned int b);
unsigned short *FUN_0042b1e0(int a, unsigned int b);
void FUN_0041aba0(unsigned short a, unsigned int b, unsigned short c);
void FUN_00413580(unsigned short a);

void __stdcall FUN_00445e10(void)
{
  char cVar1;
  unsigned char bVar2;
  char cVar3;
  int pcVar4;
  int iVar5;
  _Bool bVar6;
  unsigned short uVar7;
  int iVar8;
  unsigned int uVar9;
  int iVar10;
  int psVar11;
  unsigned int uVar12;
  int puVar13;
  short sVar14;
  short sVar15;
  int iVar16;
  unsigned int uVar17;
  int extraout_EDX;
  unsigned short uVar18;
  unsigned short uVar19;
  unsigned int unaff_EDI = 0;
  unsigned short uVar20;
  unsigned short uVar21;
  _Bool bVar22;
  unsigned long long uVar23;
  unsigned long long uVar24;
  unsigned short local_a;
  unsigned short uStack_8;

  bVar22 = DAT_0071e6b4 == 0;
  if (bVar22) {
    DAT_0071e6b4 = 0xec;
  }
  uVar12 = (unsigned int)DAT_00732a3c;
  if ((int)uVar12 < DAT_006d4980) {
    do {
      iVar8 = (int)(uVar12 * 0x23u) + DAT_006d4988;
      if ((U8(iVar8 + 3) != 0) && (pcVar4 = (int)RD32(iVar8 + 0x1f), U8(pcVar4) == 0)) {
        iVar8 = (int)RD32(pcVar4 + 0xa7);
        uVar18 = 0;
        do {
          uVar7 = (unsigned short)(uVar18 + 1);
          unaff_EDI = (unsigned int)uVar18;
          WR32(iVar8 + 0x15 + (int)unaff_EDI * 4, 0);
          uVar18 = uVar7;
        } while (uVar7 < 5);
      }
      uVar12 = (unsigned int)(unsigned short)((short)uVar12 + 1);
    } while ((int)uVar12 < DAT_006d4980);
  }
  local_a = DAT_00732a3c;
  if ((int)(unsigned int)DAT_00732a3c < DAT_006d4980) {
    do {
      uVar12 = (unsigned int)local_a;
      iVar8 = (int)(uVar12 * 0x23u);
      iVar16 = DAT_006d4988 + iVar8;
      uStack_8 = (unsigned short)iVar8;
      if ((U8(iVar16 + 3) != 0) && (pcVar4 = (int)RD32(iVar16 + 0x1f), U8(pcVar4) == 0)) {
        uVar18 = (unsigned short)(unaff_EDI >> 0x10);
        if ((int)RD32(iVar16 + 0x1b) == -1) {
          if (!bVar22) goto LAB_00446c37;
          DAT_00732e54 = (int)RD32(pcVar4 + 0xa7);
          if (U8(iVar16 + 2) == 0) {
            iVar16 = FUN_0042b150(DAT_00732e54);
            U8(DAT_00732e54 + 0x20f) = 2;
            U8(DAT_00732e54 + 0x212) = 2;
            if ((U8(DAT_00732e54 + 0x1dd) & 1) != 0) {
              bVar2 = DAT_0070ad49[(unsigned int)U8(DAT_006d4988 + 0x16 + iVar8) * 0x566u];
              if (S8(iVar16 + 0x34) == ' ') {
                if (bVar2 == 5) {
                  U8(DAT_00732e54 + 0x20f) = 0;
                }
                else if ((bVar2 == 4) || (bVar2 == 3)) {
                  U8(DAT_00732e54 + 0x20f) = 1;
                }
                else {
                  U8(DAT_00732e54 + 0x20f) = 2;
                }
                U8(DAT_00732e54 + 0x212) = 3;
              }
              sVar14 = (short)RD16((unsigned int)&DAT_0052b083[0] +
                       (unsigned int)U8(RD32(RD32(DAT_006d4988 + 0x1f + iVar8) + 0xa7) + 4) * 0xefu);
              psVar11 = DAT_00732e54 + 0x20b;
              if ((short)RD16(psVar11) < (short)(sVar14 * 2)) {
                if (U8(DAT_00732e54 + 0x212) == 2) {
                  U8(DAT_00732e54 + 0x212) = 4;
                }
                if ((short)RD16(psVar11) < 1) {
                  if (U8(DAT_00732e54 + 0x20f) == 2) {
                    U8(DAT_00732e54 + 0x20f) = 4;
                  }
                  if (bVar2 < 2) {
                    uVar7 = 0xfa;
                  }
                  else {
                    uVar7 = (unsigned short)((unsigned short)(-(unsigned short)(bVar2 < 3) & 0xdae4) + 10000);
                  }
                }
                else if (bVar2 < 2) {
                  uVar7 = 0;
                }
                else {
                  if (bVar2 == 5) {
                    uVar7 = 1;
                  }
                  else {
                    uVar7 = 3;
                    if (bVar2 != 4) {
                      uVar7 = (unsigned short)((unsigned short)(-(unsigned short)(bVar2 == 3) & 0xfff8) + 0xf);
                    }
                  }
                  uVar7 = (unsigned short)(-(unsigned short)((DAT_0071f2dd & uVar7) == uVar7) & 100);
                }
                sVar15 = 0;
                uVar12 = (unsigned int)uVar18 << 0x10;
                if (U8(DAT_00732e54 + 0x213) != 0) {
                  do {
                    cVar1 = S8(DAT_00732e54 + 0x345 + (int)((uVar12 & 0xffff) * 4u));
                    if ('\0' < cVar1) {
                      sVar15 = (short)(sVar15 + cVar1);
                    }
                    uVar18 = (unsigned short)((short)uVar12 + 1);
                    uVar12 = ((uVar12 >> 0x10) << 0x10) | (unsigned int)uVar18;
                  } while (uVar18 < U8(DAT_00732e54 + 0x213));
                }
                cVar1 = S8(DAT_006d4988 + 3 + iVar8);
                unaff_EDI = uVar12 & 0xffff0000u;
                for (uVar18 = 0; (sVar15 != 0 && (uVar18 < uVar7)); uVar18 = (unsigned short)(uVar18 + 1)) {
                  pcVar4 = DAT_00732e54 + 0x345 + (int)((unaff_EDI & 0xffff) * 4u);
                  cVar3 = S8(pcVar4);
                  if ('\0' < cVar3) {
                    sVar15 = (short)(sVar15 - 1);
                    S8(pcVar4) = (signed char)(cVar3 - 1);
                    WR16(DAT_00732e54 + 0x20b, (unsigned short)(RD16(DAT_00732e54 + 0x20b) +
                          (unsigned short)((unsigned short)(-(unsigned short)(cVar1 == '\f') & 0x1c) + 4)));
                    if ((short)(sVar14 * 2) <= (short)RD16(DAT_00732e54 + 0x20b)) {
                      sVar15 = 0;
                    }
                  }
                  uVar21 = (unsigned short)(unaff_EDI >> 0x10);
                  uVar19 = (unsigned short)((short)unaff_EDI + 1);
                  unaff_EDI = ((unsigned int)uVar21 << 0x10) | (unsigned int)uVar19;
                  if (U8(DAT_00732e54 + 0x213) <= uVar19) {
                    unaff_EDI = (unsigned int)uVar21 << 0x10;
                  }
                }
              }
            }
            if (U8(DAT_00732e54 + 0x212) == 2) {
              sVar14 = 0;
              uVar18 = 0;
              unaff_EDI = unaff_EDI & 0xffff0000u;
              if (U8(DAT_00732e54 + 0x213) != 0) {
                do {
                  if (S8(DAT_00732e54 + 0x344 + (int)((unaff_EDI & 0xffff) * 4u)) != 0) {
                    sVar14 = (short)(sVar14 + S8(DAT_00732e54 + (int)((unaff_EDI & 0xffff) * 4u) + 0x345));
                    uVar18 = (unsigned short)(uVar18 + 1);
                  }
                  uVar7 = (unsigned short)((short)unaff_EDI + 1);
                  unaff_EDI = (((unaff_EDI >> 0x10) & 0xffff) << 0x10) | (unsigned int)uVar7;
                } while (uVar7 < U8(DAT_00732e54 + 0x213));
              }
              if (uVar18 != 0) {
                sVar14 = (short)((int)sVar14 / (int)(unsigned int)uVar18);
                if (sVar14 < 0x20) {
                  sVar14 = 0x14;
                  U8(DAT_00732e54 + 0x212) = 4;
                  goto LAB_004467df;
                }
                U8(DAT_00732e54 + 0x212) = (unsigned char)((sVar14 < 0x60) + 2);
              }
            }
            sVar14 = 0x14;
          }
          else {
            U8(DAT_00732e54 + 0x20f) = 2;
            sVar14 = 0;
            U8(DAT_00732e54 + 0x212) = 2;
            if ((DAT_007092d4 == 2) && (S8(DAT_006d4988 + iVar8 + 2) == 4)) {
              if (RD16(DAT_00732e54 + 0xc3) != 0) {
                bVar2 = U8(DAT_006d4988 + iVar8 + 3);
                if ((((bVar2 == 0x33) || (bVar2 == 0x34)) || (bVar2 == 0x35)) || (bVar2 == 0x36)) {
                  sVar14 = 0;
                  unaff_EDI = 0;
                  iVar16 = FUN_004490f0((unsigned int)bVar2);
                  if (0 < iVar16) {
                    do {
                      iVar10 = FUN_00449790((unsigned int)U8(DAT_006d4988 + 3 + iVar8), (int)unaff_EDI);
                      if ((iVar10 == 8) && (U8(DAT_00732e54 + 0x31a + (int)unaff_EDI) != 0)) {
                        sVar14 = (short)(sVar14 + 1);
                      }
                      unaff_EDI = unaff_EDI + 1;
                    } while ((int)unaff_EDI < iVar16);
                  }
                }
                else {
                  sVar14 = 1;
                }
              }
              U8(DAT_00732e54 + 0x20f) = 3;
            }
            sVar14 = (short)(sVar14 * 5);
          }
        }
        else {
          DAT_00732e54 = (int)RD32(pcVar4 + 0xa7);
          iVar10 = (int)RD32(iVar16 + 0x1b);
          iVar5 = (int)((unsigned int)iVar10 * 0x5bdu);
          if (DAT_0071b758[iVar5] != 0) {
            uVar9 = FUN_004a7aa0(((((((unsigned int)((unsigned int)iVar10 * 0x16fu) >> 8) & 0xffffffu) << 8) |
                                    (unsigned int)U8(iVar16 + 3))) & 0xffff00ffu);
            uVar7 = (unsigned short)RD16((int)&DAT_0071b74a + iVar5);
            unaff_EDI = ((unsigned int)uVar18 << 0x10) | (unsigned int)uVar7;
            if ((uVar7 == 0xffff) ||
               ((unsigned short)((unsigned short)U8(DAT_00732e54 +
                           (unsigned int)DAT_0052b0b4[(unsigned int)DAT_0071b757[iVar5] +
                                                      (uVar9 & 0xffff) * 0xefu] * 4 + 0x346) +
                (unsigned short)U8(DAT_00732e54 + 0x34a +
                           (unsigned int)DAT_0052b0b4[(unsigned int)DAT_0071b757[iVar5] +
                                                      (uVar9 & 0xffff) * 0xefu] * 4)) == 0)) {
              DAT_0071b756[iVar5] = 0;
              WR16(DAT_00732e54 + 0x231, 0);
            }
            else {
              uVar9 = (unsigned int)uVar7;
              uVar24 = FUN_0042a140(uVar12, uVar9);
              extraout_EDX = (int)(unsigned int)(uVar24 >> 32);
              uVar17 = 0x18dad;
              iVar16 = extraout_EDX;
              if ((int)uVar9 < DAT_006d4980) {
                iVar16 = (int)(uVar9 * 0x23u);
                cVar1 = S8(iVar16 + 2 + DAT_006d4988);
                if (((cVar1 == '\x04') || (cVar1 == '\x05')) || (cVar1 == '\x03')) {
                  uVar17 = 0x3ba6c;
                }
              }
              if ((DAT_006db580 < uVar17) &&
                 (uVar23 = FUN_00441fc0(uVar17, iVar16, unaff_EDI, 0, iVar10), (short)uVar23 != 0)) {
                WR16(DAT_00732e54 + 0x231, (unsigned short)(RD16(DAT_00732e54 + 0x231) + DAT_0071e6be));
                if ((((int)uVar9 < DAT_006d4980) &&
                    (iVar16 = (int)RD32(RD32(uVar9 * 0x23u + 0x1f + DAT_006d4988) + 0xa7),
                    S8(iVar16 + 0x23c) == '\x01')) && (RD16(iVar16 + 0x23e) != 0)) {
                  WR16(DAT_00732e54 + 0x231, (unsigned short)(RD16(DAT_00732e54 + 0x231) - (DAT_0071e6be >> 1)));
                }
                uVar9 = FUN_004a7aa0(0xc);
                uVar17 = FUN_004a7aa0(((((((unsigned int)DAT_006d4988 >> 8) & 0xffffffu) << 8) |
                                        (unsigned int)U8(DAT_006d4988 + 3 + iVar8))) & 0xffff00ffu);
                if ((int)(short)RD16(DAT_00732e54 + 0x231) <
                    (int)(unsigned int)(unsigned short)((unsigned short)(-(unsigned short)((short)uVar17 == (short)uVar9) & 0xfe9e) + 0x2c4)) {
                  DAT_0071b756[iVar5] = 1;
                }
                else {
                  DAT_0071b756[iVar5] = 2;
                }
              }
              else {
                if (0 < (short)RD16(DAT_00732e54 + 0x231)) {
                  WR16(DAT_00732e54 + 0x231, (unsigned short)((RD16(DAT_00732e54 + 0x231) - (DAT_0071e6be >> 1)) - DAT_0071e6be));
                  if ((short)RD16(DAT_00732e54 + 0x231) < 0) {
                    WR16(DAT_00732e54 + 0x231, 0);
                  }
                }
                DAT_0071b756[iVar5] = 0;
              }
            }
          }
          uVar20 = (unsigned short)(unaff_EDI >> 0x10);
          uVar18 = 0xffff;
          if (((((U8(DAT_00732e54 + 0x1e0) & 1) == 0) ||
               (U8(DAT_00732e54 + 0x237) == 0)) ||
              (U8(DAT_00732e54 + 0x233) == 0)) || (DAT_0071b741[iVar5] != 0)) {
            if ((DAT_0071ecf4 == iVar10) &&
               (DAT_0070a1d0 = 0xffff, (U8(DAT_00732e54 + 0x1de) & 1) != 0)) {
              iVar16 = 0;
              goto LAB_0044633f;
            }
          }
          else {
            if (RD32((int)&DAT_0071bce1 + iVar5) == 0) {
              WR32((int)&DAT_0071bce1 + iVar5, 0x3b);
              sVar14 = (short)((short)RD16(DAT_00732e54 + 0x235) + -0x7d);
              if (sVar14 < 0) {
                sVar14 = 0;
              }
              WR16(DAT_00732e54 + 0x235, sVar14);
              if ((sVar14 == 0) && (U8(DAT_00732e54 + 0x237) != 0)) {
                U8(DAT_00732e54 + 0x237) = 0;
                WR16(DAT_00732e54 + 0x238, 0);
                if (DAT_0071ecf4 == iVar10) {
                  FUN_004a1d70((unsigned int)U8(DAT_00732e54 + 0x233) + 0xf1, DAT_0071ecf4);
                }
              }
            }
            uVar7 = (unsigned short)RD16((int)&DAT_0071b74a + iVar5);
            uVar9 = ((unsigned int)uVar20 << 0x10) | (unsigned int)uVar7;
            if ((uVar7 != 0xffff) && ((int)(unsigned int)uVar7 < DAT_006d4980)) {
              iVar16 = (int)RD32((unsigned int)uVar7 * 0x23u + 0x1f + DAT_006d4988);
              iVar5 = (int)RD32(iVar16 + 0xa7);
              if ((U8(iVar5 + 7) == 0) &&
                 ((uVar23 = FUN_00441fc0((unsigned int)iVar16, iVar5, uVar9, 0, iVar10), (short)uVar23 != 0 &&
                  (DAT_006d499c < 0x20000)))) {
                uVar18 = uVar7;
              }
            }
            if (uVar18 != 0xffff) {
              iVar16 = (int)RD32(RD32((unsigned int)uVar18 * 0x23u + 0x1f + DAT_006d4988) + 0xa7);
              bVar2 = U8(DAT_00732e54 + 0x233);
              if ((bVar2 == 1) || (bVar2 == 2)) {
                if ((S8(iVar16 + 0x23c) == '\x01') && (RD16(iVar16 + 0x23e) != 0)) {
                  if ((DAT_0071ecf4 == iVar10) && (sVar14 = FUN_004a31a0(), sVar14 != 0xff)) {
                    FUN_004a1d70(0xff, iVar10);
                  }
                  uVar18 = 0xffff;
                }
                else {
                  uVar9 = (unsigned int)RD16(DAT_00732e54 + 0x238);
                  WR32(iVar16 + 0x15 + (unsigned int)bVar2 * 4,
                       RD32(iVar16 + 0x15 + (unsigned int)bVar2 * 4) + uVar9);
                }
              }
            }
            uVar20 = (unsigned short)(uVar9 >> 0x10);
            if (DAT_0071ecf4 == iVar10) {
              iVar16 = 1;
              DAT_0070a1d0 = uVar18;
LAB_0044633f:
              FUN_0040a980(iVar16, iVar10);
            }
          }
          sVar14 = 0x14;
          unaff_EDI = ((unsigned int)uVar20 << 0x10) | (unsigned int)DAT_00732a3c;
          if ((int)(unsigned int)DAT_00732a3c < DAT_006d4980) {
            do {
              iVar16 = (int)((unaff_EDI & 0xffff) * 0x23u) + DAT_006d4988;
              if (((U8(iVar16 + 3) != 0) && (S8(iVar16 + 2) == '\x04')) &&
                 ((bVar6 = FUN_00444040(((unsigned int)uStack_8 << 16) | (unsigned int)local_a,
                                       (unsigned int)U8(RD32(iVar16 + 0x1f) + 0x87))) != 0)) {
                FUN_0041b920(uVar12, unaff_EDI & 0xffff);
              }
              uVar18 = (unsigned short)((short)unaff_EDI + 1);
              unaff_EDI = (((unaff_EDI >> 0x10) & 0xffff) << 0x10) | (unsigned int)uVar18;
            } while ((int)(unsigned int)uVar18 < DAT_006d4980);
          }
        }
LAB_004467df:
        if (bVar22) {
          DAT_00732e54 = (int)RD32(RD32(DAT_006d4988 + iVar8 + 0x1f) + 0xa7);
          if (S8(DAT_006d4988 + iVar8 + 3) == '\x02') {
            sVar14 = (short)(sVar14 * 2);
          }
          if ((((U8(DAT_00732e54 + 0x1df) & 1) != 0) && (sVar14 != 0)) &&
             (sVar14 = (short)((U8(DAT_00732e54 + 0x20f) - 2) * sVar14), sVar14 != 0)) {
            if (U8(DAT_00732e54 + 0x210) == 0) {
              sVar15 = (short)((short)RD16((unsigned int)&DAT_0052b083[0] + (unsigned int)U8(DAT_00732e54 + 4) * 0xefu) * 2);
              WR16(DAT_00732e54 + 0x20b, (unsigned short)((short)RD16(DAT_00732e54 + 0x20b) + sVar14));
              if ((short)RD16(DAT_00732e54 + 0x20b) < 0) {
                WR16(DAT_00732e54 + 0x20b, 0);
              }
              psVar11 = DAT_00732e54 + 0x20b;
              if (sVar15 < (short)RD16(psVar11)) goto LAB_004469ba;
            }
            else if (U8(DAT_00732e54 + 0x210) == 2) {
              sVar15 = (short)((short)RD16((unsigned int)&DAT_0052b083[0] + (unsigned int)U8(DAT_00732e54 + 4) * 0xefu) * 2);
              WR16(DAT_00732e54 + 0x20d, (unsigned short)((short)RD16(DAT_00732e54 + 0x20d) + sVar14));
              if ((short)RD16(DAT_00732e54 + 0x20d) < 0) {
                WR16(DAT_00732e54 + 0x20d, 0);
              }
              psVar11 = DAT_00732e54 + 0x20d;
              if (sVar15 < (short)RD16(psVar11)) {
LAB_004469ba:
                WR16(psVar11, sVar15);
              }
            }
            else {
              sVar15 = (short)RD16((unsigned int)&DAT_0052b083[0] + (unsigned int)U8(DAT_00732e54 + 4) * 0xefu);
              WR16(DAT_00732e54 + 0x20b, (unsigned short)((short)RD16(DAT_00732e54 + 0x20b) + sVar14 / 2));
              if ((short)RD16(DAT_00732e54 + 0x20b) < 0) {
                WR16(DAT_00732e54 + 0x20b, 0);
              }
              if ((short)(sVar15 * 2) < (short)RD16(DAT_00732e54 + 0x20b)) {
                WR16(DAT_00732e54 + 0x20b, sVar15 * 2);
              }
              sVar15 = (short)((short)RD16((unsigned int)&DAT_0052b083[0] + (unsigned int)U8(DAT_00732e54 + 4) * 0xefu) * 2);
              WR16(DAT_00732e54 + 0x20d, (unsigned short)((short)RD16(DAT_00732e54 + 0x20d) + sVar14 / 2));
              if ((short)RD16(DAT_00732e54 + 0x20d) < 0) {
                WR16(DAT_00732e54 + 0x20d, 0);
              }
              psVar11 = DAT_00732e54 + 0x20d;
              if (sVar15 < (short)RD16(psVar11)) goto LAB_004469ba;
            }
          }
          if (((U8(DAT_00732e54 + 0x1df) & 0x10) != 0) &&
             (uVar12 = 0, U8(DAT_00732e54 + 0x213) != 0)) {
            do {
              iVar16 = (int)(uVar12 * 4u);
              cVar1 = S8(DAT_00732e54 + 0x344 + iVar16);
              if ((cVar1 != '\0') && (cVar1 != '\x02')) {
                sVar14 = (short)(U8(DAT_00732e54 + 0x212) - 2);
                if (RD16(DAT_00732e54 + 0xbf) == 0) {
                  sVar14 = (short)(U8(DAT_00732e54 + 0x212) - 6);
                }
                cVar1 = S8(DAT_006d4988 + 3 + iVar8);
                if ((cVar1 == '\x05') || (cVar1 == '\a')) {
                  sVar14 = (short)(sVar14 * 3);
                }
                else {
                  sVar14 = (short)(sVar14 * 2);
                }
                S8(DAT_00732e54 + iVar16 + 0x345) = (signed char)(S8(DAT_00732e54 + iVar16 + 0x345) + (char)sVar14);
                if ((sVar14 < 0) && (pcVar4 = DAT_00732e54 + 0x345 + iVar16, S8(pcVar4) < '\0')) {
                  S8(pcVar4) = '\0';
                }
                if ((0 < sVar14) && (S8(DAT_00732e54 + 0x345 + iVar16) < '\0')) {
                  U8(DAT_00732e54 + 0x345 + iVar16) = 0x7f;
                }
              }
              uVar12 = (unsigned int)(unsigned short)((short)uVar12 + 1);
            } while (uVar12 < U8(DAT_00732e54 + 0x213));
          }
          unaff_EDI = (unsigned int)(DAT_00732e54 + 0xbf);
          if (RD16(unaff_EDI) == 0) {
            bVar6 = 0;
            uVar12 = 0;
            if (U8(DAT_00732e54 + 0x213) != 0) {
              do {
                if ('\0' < S8(DAT_00732e54 + 0x345 + (int)(uVar12 * 4u))) {
                  bVar6 = 1;
                }
                uVar12 = (unsigned int)(unsigned short)((short)uVar12 + 1);
              } while (uVar12 < U8(DAT_00732e54 + 0x213));
            }
            if (!bVar6) {
              WR16(unaff_EDI, -1);
              FUN_004a1d70(0x113, DAT_0071ecf4);
              FUN_0040a1a0(0x5b, 0xffffffff, DAT_0071ecf4);
            }
          }
          if ((U8(DAT_00732e54 + 0x1e0) & 1) != 0) {
            sVar14 = (short)((U8(DAT_00732e54 + 0x234) - 2) * 0x7d + (short)RD16(DAT_00732e54 + 0x235));
            if (sVar14 < 0) {
              sVar14 = 0;
            }
            if (9999 < sVar14) {
              sVar14 = 9999;
            }
            WR16(DAT_00732e54 + 0x235, sVar14);
            if ((sVar14 == 0) && (U8(DAT_00732e54 + 0x237) != 0)) {
              U8(DAT_00732e54 + 0x237) = 0;
              WR16(DAT_00732e54 + 0x238, 0);
              if ((int)RD32(DAT_006d4988 + 0x1b + iVar8) == DAT_0071ecf4) {
                FUN_004a1d70((unsigned int)U8(DAT_00732e54 + 0x233) + 0xf1, DAT_0071ecf4);
              }
            }
          }
          if (RD16(DAT_00732e54 + 0x23e) != 0) {
            WR16(DAT_00732e54 + 0x23e, (unsigned short)(RD16(DAT_00732e54 + 0x23e) - 1));
            if ((S8(DAT_00732e54 + 0x23c) == '\x01') && (RD16(DAT_00732e54 + 0x23e) == 0)) {
              iVar8 = (int)RD32(DAT_006d4988 + 0x1b + iVar8);
              if (iVar8 != -1) {
                FUN_004a1d70(0x166, iVar8);
              }
            }
          }
        }
      }
LAB_00446c37:
      local_a = (unsigned short)(local_a + 1);
    } while ((int)(unsigned int)local_a < DAT_006d4980);
  }
  local_a = DAT_00732a3c;
  if ((int)(unsigned int)DAT_00732a3c < DAT_006d4980) {
    do {
      iVar8 = (int)((unsigned int)local_a * 0x23u);
      iVar16 = DAT_006d4988 + iVar8;
      if (((U8(iVar16 + 3) != 0) && (pcVar4 = (int)RD32(iVar16 + 0x1f), U8(pcVar4) == 0))
         && (DAT_00732e54 = (int)RD32(pcVar4 + 0xa7), RD16(DAT_00732e54 + 0x1e1) == 0)) {
        if ((RD32(DAT_00732e54 + 0x1d) == 0) &&
           (uVar12 = 0, U8(DAT_00732e54 + 0x211) != 0)) {
          do {
            sVar14 = (short)RD16(DAT_00732e54 + 0x21c + (int)(uVar12 * 2u));
            if (sVar14 != 0) {
              sVar14 = (short)(sVar14 - DAT_0071e6be);
              if (sVar14 < 0) {
                sVar14 = 0;
              }
              WR16(DAT_00732e54 + 0x21c + (int)(uVar12 * 2u), sVar14);
            }
            if ((((int)RD32(DAT_006d4988 + 0x1b + iVar8) == -1) && (sVar14 < (short)DAT_0071e6be)) &&
               (U8(DAT_00732e54 + 0x216 + (int)uVar12) != 0)) {
              if (((U8(DAT_00732e54 + 0x1df) & 0x10) != 0) &&
                 (U8(DAT_00732e54 + 7) == 0)) {
                FUN_00447240((unsigned int)local_a, uVar12);
              }
              pcVar4 = DAT_00732e54 + 0x218 + (int)uVar12;
              S8(pcVar4) = (signed char)(S8(pcVar4) - 1);
              psVar11 = DAT_00732e54 + 0x21c + (int)(uVar12 * 2u);
              WR16(psVar11, (unsigned short)(RD16(psVar11) + DAT_0071e6be * 2));
              WR32(DAT_00732e54 + 0x220 + (int)(uVar12 * 4u),
                   RD32(DAT_00732e54 + 0x220 + (int)(uVar12 * 4u)) + (unsigned int)DAT_0071e6be * 2u);
              if (U8(DAT_00732e54 + (int)uVar12 + 0x218) == 0) {
                U8(DAT_00732e54 + (int)uVar12 + 0x216) = 0;
              }
            }
            uVar12 = (unsigned int)(unsigned short)((short)uVar12 + 1);
          } while (uVar12 < U8(DAT_00732e54 + 0x211));
        }
        uVar12 = 0;
        if (U8(DAT_00732e54 + 0x213) != 0) {
          do {
            if (S8(DAT_00732e54 + 0x344 + (int)(uVar12 * 4u)) == '\x02') {
              puVar13 = (int)FUN_0042b1e0(DAT_00732e54, uVar12);
              if (*(unsigned short *)puVar13 != 0xffff) {
                FUN_0041aba0(local_a, uVar12, *(unsigned short *)puVar13);
              }
            }
            uVar12 = (unsigned int)(unsigned short)((short)uVar12 + 1);
          } while (uVar12 < U8(DAT_00732e54 + 0x213));
        }
        uVar12 = 0;
        if (U8(DAT_00732e54 + 0x228) != 0) {
          do {
            psVar11 = DAT_00732e54 + 0x22d + (int)(uVar12 * 2u);
            sVar14 = (short)RD16(psVar11);
            if (sVar14 != 0) {
              sVar14 = (short)(sVar14 - DAT_0071e6be);
              if (sVar14 < 0) {
                sVar14 = 0;
              }
              WR16(psVar11, sVar14);
            }
            uVar12 = (unsigned int)(unsigned short)((short)uVar12 + 1);
          } while (uVar12 < U8(DAT_00732e54 + 0x228));
        }
      }
      local_a = (unsigned short)(local_a + 1);
    } while ((int)(unsigned int)local_a < DAT_006d4980);
  }
  uVar12 = DAT_00732a38 & 0xffff;
  if ((int)(DAT_00732a38 & 0xffff) < (int)((unsigned int)DAT_00703c88 + DAT_00732a38)) {
    do {
      iVar8 = (int)(uVar12 * 0x23u) + DAT_006d4988;
      if ((U8(iVar8 + 3) != 0) && (S8(iVar8 + 2) == '\b')) {
        FUN_00413580((unsigned short)uVar12);
      }
      uVar18 = (unsigned short)((unsigned short)uVar12 + 1);
      uVar12 = (unsigned int)uVar18;
    } while ((int)(unsigned int)uVar18 < (int)((unsigned int)DAT_00703c88 + DAT_00732a38));
  }
  return;
}
