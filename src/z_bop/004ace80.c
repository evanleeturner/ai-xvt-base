/* SPDX-License-Identifier: MIT */
/* z_bop 004ace80 */
extern int DAT_009a1fe0;
extern int DAT_009a1fe8;
extern unsigned short DAT_009a7394;
extern short DAT_009d8112;
extern int DAT_009e9658;
extern int DAT_009ec5d0;
extern unsigned short DAT_009ec5fe;
extern unsigned short DAT_00a07cec;
extern int DAT_00a08104;
extern unsigned char DAT_0051c58b;

unsigned int FUN_00425b70(unsigned int, unsigned short);
void FUN_004adb50(unsigned int);
void FUN_004ad880(unsigned int, unsigned int, int, unsigned short);
void FUN_004ada90(unsigned int, unsigned int);
void FUN_004ad9e0(unsigned int, int);

#define RD16(a) ({ unsigned short t_; __builtin_memcpy(&t_, (const void *)(a), 2); t_; })
#define RD32(a) ({ unsigned int t_; __builtin_memcpy(&t_, (const void *)(a), 4); t_; })
#define WR16(a, x) ({ unsigned short t_ = (unsigned short)(x); __builtin_memcpy((void *)(a), &t_, 2); })
#define B8(a) (*(unsigned char *)(a))

void __stdcall FUN_004ace80(void)
{
  char cVar2;
  short sVar3;
  short sVar4;
  short sVar5;
  short sVar6;
  int iVar7;
  int iVar8;
  int bVar9;
  short sVar10;
  unsigned short uVar11;
  unsigned int uVar12;
  unsigned short uVar13;
  int iVar14;
  unsigned int unaff_EBX = 0;
  unsigned short uVar15;
  unsigned short uVar16;
  short sVar17;
  unsigned short local_14;
  unsigned short uStack_12;

  uVar11 = DAT_009ec5fe;
  sVar10 = DAT_009d8112;
  local_14 = DAT_00a07cec;
  bVar9 = 0;
  if ((int)(unsigned int)DAT_00a07cec < DAT_009a1fe0) {
    do {
      if (DAT_009ec5d0 != -1) {
        if (bVar9) {
          DAT_009d8112 = sVar10;
          DAT_009ec5fe = uVar11;
          return;
        }
        bVar9 = 1;
        local_14 = (unsigned short)DAT_009ec5d0;
      }
      iVar8 = (int)(((unsigned int)local_14 * 8u - (unsigned int)local_14) * 5u);
      iVar14 = DAT_009a1fe8 + iVar8;
      DAT_009d8112 = sVar10;
      DAT_009ec5fe = uVar11;
      if ((RD32(iVar14 + 0x1f) == 0) ||
          (iVar7 = (int)RD32(RD32(iVar14 + 0x1f) + 2), iVar7 == 0)) {
LAB_004acf74:
        if ((B8(iVar14 + 3) != 0) && (B8(RD32(iVar14 + 0x1f)) == 0)) {
          uStack_12 = 0;
          iVar7 = (int)RD32(RD32(iVar14 + 0x1f) + 0xa9);
          sVar3 = (short)RD16(iVar14 + 0x12);
          sVar4 = (short)RD16(iVar14 + 0x10);
          sVar5 = (short)RD16(iVar14 + 0x14);
          unaff_EBX = (unaff_EBX & 0xffff0000u) | (unsigned short)sVar5;
          DAT_009a7394 = (unsigned short)B8(iVar7 + 4);
          if (RD32(iVar14 + 0x1b) == 0xffffffffu) {
            if (RD16(iVar7 + 0x1e5) != 0) {
              uStack_12 = RD16(iVar7 + 0xbd);
            }
          }
          else if ((B8(iVar7 + 0x1e5) & 0x40) != 0) {
            uStack_12 = RD16(iVar7 + 0xbd);
          }
          DAT_00a08104 = iVar7;
          if (((RD32(iVar14 + 0x1b) == 0xffffffffu) && (RD16(iVar7 + 0x1e5) != 0)) &&
              (RD32(iVar7 + 0x19) == 0)) {
            if ((B8(iVar7 + 0xa4) != 0) && (B8(iVar7 + 0xa4) < 4)) {
              uVar16 = (unsigned short)(RD16(iVar7 + 0x5a) - RD16(iVar14 + 0x14));
              unaff_EBX = 0;
              uVar12 = FUN_00425b70((unsigned int)((int)((unsigned int)RD16(iVar7 + 0xa0) *
                                                         (unsigned int)DAT_009ec5fe) / 0xec),
                                    RD16(iVar7 + 0xa2));
              uVar12 = FUN_00425b70(uVar12, RD16(DAT_00a08104 + 0xa5));
              uVar13 = (unsigned short)((short)uVar12 * 2);
              if (B8(DAT_00a08104 + 0xa4) == 3) {
                if (RD16(iVar7 + 0x5a) < 0x8000) {
                  WR16(DAT_009a1fe8 + 0x14 + iVar8, RD16(DAT_009a1fe8 + 0x14 + iVar8) + uVar13);
                }
                else {
LAB_004ad120:
                  WR16(DAT_009a1fe8 + 0x14 + iVar8,
                       RD16(DAT_009a1fe8 + 0x14 + iVar8) + (short)uVar12 * -2);
                }
              }
              else if (uVar16 < 0x8000) {
                if (uVar13 < uVar16) {
                  WR16(DAT_009a1fe8 + 0x14 + iVar8, RD16(DAT_009a1fe8 + 0x14 + iVar8) + uVar13);
                }
                else {
                  WR16(DAT_009a1fe8 + 0x14 + iVar8, RD16(iVar7 + 0x5a));
                  B8(DAT_00a08104 + 0xa4) = 4;
                }
              }
              else {
                if (uVar13 < (unsigned short)-uVar16) goto LAB_004ad120;
                WR16(DAT_009a1fe8 + 0x14 + iVar8, RD16(iVar7 + 0x5a));
                B8(DAT_00a08104 + 0xa4) = 4;
              }
            }
            if (B8(DAT_00a08104 + 0x9c) != 0) {
              uVar13 = (unsigned short)(RD16(iVar7 + 0x58) - RD16(DAT_00a08104 + 0xd));
              if (0x7fff < uVar13) {
                uVar13 = (unsigned short)-uVar13;
              }
              uVar12 = FUN_00425b70((unsigned int)((int)((unsigned int)RD16(DAT_00a08104 + 0x98) *
                                                         (unsigned int)DAT_009ec5fe) / 0xec),
                                    RD16(DAT_00a08104 + 0x9a));
              uVar12 = FUN_00425b70(uVar12, RD16(DAT_00a08104 + 0x9e));
              uVar16 = (unsigned short)uVar12;
              unaff_EBX = uVar12 & 0xffff;
              FUN_00425b70(unaff_EBX, 0x8000);
              if (B8(DAT_00a08104 + 0x9c) == 1) {
                if ((uVar16 < uVar13) || (B8(DAT_00a08104 + 0x9d) != 0)) {
                  WR16(DAT_00a08104 + 0xd, RD16(DAT_00a08104 + 0xd) - uVar16);
                  if (0xdfff < RD16(DAT_00a08104 + 0xd)) {
                    WR16(DAT_00a08104 + 0xd, -RD16(DAT_00a08104 + 0xd));
                    WR16(DAT_009a1fe8 + 0x10 + iVar8, RD16(DAT_009a1fe8 + 0x10 + iVar8) + -0x8000);
                    WR16(DAT_009a1fe8 + 0x14 + iVar8, RD16(DAT_009a1fe8 + 0x14 + iVar8) + -0x8000);
                    B8(DAT_00a08104 + 0x9d) = 0;
                    B8(DAT_00a08104 + 0x9c) = 2;
                  }
                }
                else {
                  WR16(DAT_00a08104 + 0xd, RD16(iVar7 + 0x58));
                  B8(DAT_00a08104 + 0x9c) = 3;
                }
              }
              else if (B8(DAT_00a08104 + 0x9c) == 2) {
                if ((uVar16 < uVar13) || (B8(DAT_00a08104 + 0x9d) != 0)) {
                  WR16(DAT_00a08104 + 0xd, RD16(DAT_00a08104 + 0xd) + uVar16);
                  uVar13 = RD16(DAT_00a08104 + 0xd);
                  if (0x7fff < uVar13) {
                    WR16(DAT_00a08104 + 0xd, -uVar13);
                    WR16(DAT_009a1fe8 + 0x10 + iVar8, RD16(DAT_009a1fe8 + 0x10 + iVar8) + -0x8000);
                    WR16(DAT_009a1fe8 + 0x14 + iVar8, RD16(DAT_009a1fe8 + 0x14 + iVar8) + -0x8000);
                    B8(DAT_00a08104 + 0x9d) = 0;
                    B8(DAT_00a08104 + 0x9c) = 1;
                  }
                }
                else {
                  WR16(DAT_00a08104 + 0xd, RD16(iVar7 + 0x58));
                  B8(DAT_00a08104 + 0x9c) = 3;
                }
              }
            }
            if (((B8(DAT_00a08104 + 7) != 2) && (B8(DAT_00a08104 + 0xab) != 0)) &&
                (uVar13 = (unsigned short)(RD16(iVar7 + 0x5c) - RD16(DAT_009a1fe8 + 0x10 + iVar8)),
                 uVar13 != 0)) {
              unaff_EBX = 0;
              uVar12 = FUN_00425b70((unsigned int)((int)((unsigned int)RD16(DAT_00a08104 + 0xa7) *
                                                         (unsigned int)DAT_009ec5fe) / 0xec),
                                    RD16(DAT_00a08104 + 0xa9));
              uVar12 = FUN_00425b70(uVar12, RD16(DAT_00a08104 + 0xac));
              uVar16 = (unsigned short)uVar12;
              if (uVar13 < 0x8000) {
                if (uVar16 < uVar13) {
                  WR16(DAT_009a1fe8 + 0x10 + iVar8, RD16(DAT_009a1fe8 + 0x10 + iVar8) + uVar16);
                }
                else {
LAB_004ad385:
                  WR16(DAT_009a1fe8 + 0x10 + iVar8, RD16(iVar7 + 0x5c));
                  uVar12 = uVar12 & 0xffff0000u;
                  B8(DAT_00a08104 + 0xab) = 3;
                }
              }
              else {
                if ((unsigned short)-uVar13 <= uVar16) goto LAB_004ad385;
                WR16(DAT_009a1fe8 + 0x10 + iVar8, RD16(DAT_009a1fe8 + 0x10 + iVar8) - uVar16);
              }
              if ((B8(DAT_00a08104 + 0xa4) == 0) || (B8(DAT_00a08104 + 0xa4) == 4)) {
                uVar12 = FUN_00425b70(uVar12,
                                      RD16((unsigned int)&DAT_0051c58b +
                                           (unsigned int)DAT_009a7394 * 0x101u));
                if (uVar13 < 0x8000) {
                  WR16(DAT_009a1fe8 + 0x14 + iVar8,
                       RD16(DAT_009a1fe8 + 0x14 + iVar8) - (short)uVar12);
                }
                else {
                  WR16(DAT_009a1fe8 + 0x14 + iVar8,
                       RD16(DAT_009a1fe8 + 0x14 + iVar8) + (short)uVar12);
                }
              }
            }
          }
          if (((RD32(DAT_009a1fe8 + iVar8 + 0x1b) == 0xffffffffu) &&
               (B8(DAT_00a08104 + 0x96) == 1)) &&
              ((RD16(DAT_00a08104 + 0x1e5) != 0 &&
                ((unaff_EBX = RD32(DAT_00a08104 + 0x19), unaff_EBX == 0 &&
                  ((int)RD32(iVar7 + 0x51) <= (int)RD32(DAT_009a1fe8 + iVar8 + 0xc))))))) {
            B8(DAT_00a08104 + 0x96) = 0;
            WR16(DAT_00a08104 + 0xd, 0x4000);
          }
          if ((((RD32(DAT_009a1fe8 + 0x1b + iVar8) == 0xffffffffu) &&
                (B8(DAT_00a08104 + 0x97) == 1)) &&
               (RD16(DAT_00a08104 + 0x1e5) != 0)) && (RD32(DAT_00a08104 + 0x19) == 0)) {
            FUN_004adb50((unsigned int)local_14);
          }
          switch (B8(DAT_00a08104 + 7)) {
          case 0:
            WR16(DAT_009a1fe8 + 0x12 + iVar8, RD16(DAT_00a08104 + 0xd));
            uVar13 = RD16(DAT_00a08104 + 0xc1);
            if ((uVar13 == 0) || (uStack_12 != 0xffff)) {
              sVar6 = (short)RD16(DAT_00a08104 + 0x92);
              uVar15 = (unsigned short)(unaff_EBX >> 0x10);
              sVar17 = (short)(((6 - (unsigned short)B8(DAT_00a08104 + 0x219)) -
                                (unsigned short)B8(DAT_00a08104 + 0x23e)) -
                               (unsigned short)B8(DAT_00a08104 + 0x21c));
              cVar2 = (char)B8(DAT_009a1fe8 + 3 + iVar8);
              if (cVar2 == '\a') {
                uVar13 = 0x1000;
              }
              else if ((cVar2 == '\x05') && (0 < sVar17)) {
                uVar13 = 0x3000;
              }
              else {
                uVar13 = 0x2000;
              }
              uVar12 = FUN_00425b70(((unsigned int)uVar15 << 16) | (unsigned short)sVar6, uVar13);
              unaff_EBX = ((unsigned int)uVar15 << 16) |
                          (unsigned short)(sVar6 + (short)uVar12 * sVar17);
              uVar12 = FUN_00425b70(unaff_EBX, uStack_12);
              uVar13 = (unsigned short)uVar12;
              if (RD16(DAT_00a08104 + 0xbf) == 0) {
                uVar13 = (unsigned short)(uVar13 * 2);
              }
            }
            uVar16 = RD16(RD32(DAT_009a1fe8 + 0x1f + iVar8) + 0x79);
            if (uVar13 < uVar16) {
              uVar12 = (unsigned int)uVar16 - (unsigned int)uVar13;
              if (uVar12 < 200) {
                FUN_004ad880((unsigned int)local_14, (unsigned int)uVar13, 1, uStack_12);
              }
              else {
                FUN_004ada90((unsigned int)local_14, uVar12 / 3);
              }
            }
            else {
              FUN_004ad880((unsigned int)local_14, (unsigned int)uVar13, 1, uStack_12);
            }
            break;
          case 1:
          case 3:
          case 4:
            uVar13 = RD16(RD32(DAT_009a1fe8 + 0x1f + iVar8) + 0x79);
            if (RD16(DAT_00a08104 + 0x92) < uVar13) {
              unaff_EBX = 0;
              FUN_004ada90((unsigned int)local_14,
                           (unsigned int)((int)((unsigned int)uVar13 -
                                                (unsigned int)RD16(DAT_00a08104 + 0x92)) / 3));
            }
            /* fallthrough */
          case 6:
            B8(DAT_00a08104 + 0x96) = 0;
            B8(DAT_00a08104 + 0x97) = 0;
            B8(DAT_00a08104 + 0xa4) = 0;
            B8(DAT_00a08104 + 0x9c) = 0;
            break;
          case 2:
            if (RD16(RD32(DAT_009a1fe8 + 0x1f + iVar8) + 0x79) != 0) {
              FUN_004ada90((unsigned int)local_14, 0x14);
            }
            break;
          case 5:
            if (RD16(DAT_00a08104 + 0x1e5) == 0) {
              if (RD16(RD32(DAT_009a1fe8 + 0x1f + iVar8) + 0x79) != 0) {
                FUN_004ada90((unsigned int)local_14, 0x14);
              }
              if (RD16(RD32(DAT_009a1fe8 + 0x1f + iVar8) + 0x79) == 0) {
                B8(DAT_00a08104 + 7) = 0;
              }
            }
            else if (RD16(iVar7 + 100) == 0) {
              if (RD32(iVar7 + 0x60) == 0) {
                FUN_004ad9e0((unsigned int)local_14, 500);
              }
              else {
                FUN_004ad9e0((unsigned int)local_14, 200);
              }
            }
            else {
              FUN_004ad9e0((unsigned int)local_14, 0x32);
            }
          }
          WR16(DAT_00a08104 + 0xf, RD16(DAT_009a1fe8 + 0x10 + iVar8));
          iVar14 = DAT_009a1fe8 + iVar8;
          if (((short)RD16(iVar14 + 0x12) != sVar3) || ((short)RD16(iVar14 + 0x10) != sVar4) ||
              ((short)RD16(iVar14 + 0x14) != sVar5)) {
            B8(RD32(iVar14 + 0x1f) + 0x8b) = 1;
            B8(RD32(DAT_009a1fe8 + 0x1f + iVar8) + 0x92) = 1;
          }
          if (RD16(DAT_00a08104 + 0x66) != 0xffff) {
            iVar14 = (int)((unsigned int)RD16(DAT_00a08104 + 0x66) * 0x23u);
            if (RD32(DAT_009a1fe8 + iVar14 + 0x1f) != 0) {
              WR16(DAT_009a1fe8 + iVar14 + 0x12, RD16(DAT_009a1fe8 + 0x12 + iVar8));
              WR16(DAT_009a1fe8 + 0x10 + iVar14, RD16(DAT_009a1fe8 + 0x10 + iVar8));
              WR16(DAT_009a1fe8 + 0x14 + iVar14, RD16(DAT_009a1fe8 + 0x14 + iVar8));
              unaff_EBX = RD32(RD32(DAT_009a1fe8 + 0x1f + iVar14) + 0xa9);
              WR16(unaff_EBX + 0xd, RD16(DAT_00a08104 + 0xd));
              B8(RD32(DAT_009a1fe8 + 0x1f + iVar14) + 0x8b) = 1;
              B8(RD32(DAT_009a1fe8 + 0x1f + iVar14) + 0x92) = 1;
            }
          }
        }
      }
      else {
        DAT_009ec5fe = (unsigned short)(uVar11 + ((short)DAT_009e9658 - (short)iVar7));
        if (DAT_009ec5fe != 0) {
          DAT_009d8112 = (short)(0xecu / (unsigned int)DAT_009ec5fe);
          if (DAT_009d8112 == 0) {
            DAT_009d8112 = 1;
          }
          goto LAB_004acf74;
        }
      }
      local_14 = (unsigned short)(local_14 + 1);
    } while ((int)(unsigned int)local_14 < DAT_009a1fe0);
  }
  DAT_009d8112 = sVar10;
  DAT_009ec5fe = uVar11;
  return;
}
