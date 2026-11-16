/* SPDX-License-Identifier: MIT */
/* z_xvt 0043fe30 */
extern int DAT_006d4988;
extern int DAT_00525550;
extern unsigned short DAT_0071eae4;
extern unsigned short DAT_0071ecf0;
extern unsigned short DAT_0071e534;
extern unsigned short DAT_0071e6be;
extern unsigned short DAT_0070a1d2;
extern unsigned short _DAT_006d4860;
extern int DAT_00732568;
extern int DAT_00732530;
extern int DAT_00732548;
extern unsigned char DAT_0071b730[];
extern unsigned char DAT_0052686c[];

void FUN_00405590(void);
unsigned int FUN_00478320(unsigned int a, unsigned int b);
unsigned int FUN_004783e0(int a, int b);
int FUN_00478290(int a, unsigned int b, int c);
void FUN_00442e60(int a, int b, unsigned int c, int d);
void FUN_00436450(int a, int b, int c);
int FUN_00412810(int a, int b, int c);
void FUN_00483180(int a, int b, int c, int d, int e, int f, int g);
/* ECX/EDX register args (param_1/param_2) only affect the unused high dword; not expressible in C */
unsigned long long FUN_00413ca0(int param_3, int param_4);

#define OFS(b, o) ((unsigned char *)((unsigned int)(b) + (unsigned int)(o)))
#define ENT(i, o) ((unsigned char *)((unsigned int)(i) * 0x23u + (unsigned int)(o) + (unsigned int)DAT_006d4988))
#define RD16(p) ({ unsigned short t16_; __builtin_memcpy(&t16_, (p), 2); t16_; })
#define RD32(p) ({ unsigned int t32_; __builtin_memcpy(&t32_, (p), 4); t32_; })
#define WR16(p, v) do { unsigned short w16_ = (unsigned short)(v); __builtin_memcpy((p), &w16_, 2); } while (0)
#define WR32(p, v) do { unsigned int w32_ = (unsigned int)(v); __builtin_memcpy((p), &w32_, 4); } while (0)
#define S16(p) ((short)RD16(p))
#define CHAIN() do { \
    iChain = (int)RD32(ENT(RD32(base), 0x1f)); \
    *OFS(iChain, 0x90) = 1; \
    iChain = (int)RD32(ENT(RD32(base), 0x1f)); \
    *OFS(iChain, 0x89) = *OFS(iChain, 0x90); \
  } while (0)

void FUN_0043fe30(int param_1)
{
  unsigned char *base;
  unsigned char *puVar1;
  unsigned char *psVar19;
  unsigned char *pe;
  unsigned int iVar2;
  int iVar6, iVar13, iVar14, iChain;
  unsigned short uVar8, uVar10, uVar15, uVar16;
  short sVar3, sVar9, sVar11, sVar17, sVar18;
  unsigned int uVar4, uVar5, uVar12;
  unsigned char bVar7;
  unsigned long long uVar20;
  unsigned int local_4 = 0;
  unsigned short extraout_var = 0;
  unsigned short extraout_var_00 = 0;
  unsigned short extraout_var_01 = 0;

  FUN_00405590();
  DAT_0071eae4 = (unsigned short)(DAT_0071eae4 << 1);
  iVar2 = (unsigned int)param_1 * 0x5bd;
  base = DAT_0071b730 + iVar2;

  if ((S16(base + 0x410) == 0) && (base[0x13] == 0)) {
    if (base[0x14] != 0) {
      return;
    }
    iVar6 = (int)RD32(ENT(RD32(base), 0x1f));
    iVar14 = (int)RD32(OFS(iVar6, 0xa7));
    uVar8 = RD16(OFS(iVar14, 0xbd));
    if ((*OFS(iVar14, 0x1df) & 0x40) == 0) {
      uVar8 = 0;
    }
    uVar12 = (((unsigned int)iVar6 >> 16) << 16) | uVar8;
    if (uVar8 < 0x5555) {
      uVar8 = (unsigned short)(uVar8 * 2 + 0x5555);
    } else {
      uVar12 = uVar8;
      uVar8 = (unsigned short)((short)((int)(0x5555u - uVar12) / 2) - 1);
    }
    uVar12 = (((uVar12 >> 8) << 8) | *OFS(iVar14, 0x212)) & 0xffff00ffu;
    sVar9 = (short)uVar12;
    if ((*OFS(iVar14, 0x1dd) & 1) == 0) {
      sVar9 = (short)(sVar9 * 2);
    } else {
      sVar9 = (short)(sVar9 + *OFS(iVar14, 0x20f));
    }
    sVar9 = (short)(4 - sVar9);
    if (sVar9 < 0) {
      uVar15 = (unsigned short)(sVar9 * -0xc00);
    } else {
      uVar15 = (unsigned short)(sVar9 * 0xc00);
    }
    uVar12 = FUN_00478320(((uVar12 >> 16) << 16) | RD16(OFS(iVar14, 0xa0)), uVar8);
    uVar4 = (((unsigned int)param_1 >> 16) << 16) | (unsigned short)uVar12;
    if (sVar9 < 1) {
      uVar4 = FUN_00478320(uVar4, uVar15);
      sVar17 = (short)(-(short)uVar4);
    } else {
      uVar4 = FUN_00478320(uVar4, uVar15);
      sVar17 = (short)uVar4;
    }
    uVar16 = (unsigned short)((short)uVar12 + sVar17);
    uVar5 = FUN_004783e0(uVar16 % 0x3800, 0x3800);
    uVar10 = DAT_0071ecf0;
    if (0x7fff < DAT_0071ecf0) {
      uVar10 = (unsigned short)-DAT_0071ecf0;
    }
    uVar12 = FUN_00478320(((unsigned int)extraout_var << 16) | uVar10, (unsigned short)uVar5);
    sVar17 = (short)((uVar16 / 0x3800) * uVar10 + (short)uVar12);
    if (0x7fff < DAT_0071ecf0) {
      sVar17 = (short)-sVar17;
    }
    _DAT_006d4860 = DAT_0071ecf0;
    if (0x7fff < DAT_0071ecf0) {
      _DAT_006d4860 = (unsigned short)-DAT_0071ecf0;
    }
    uVar12 = FUN_00478320(((unsigned int)extraout_var_00 << 16) | RD16(OFS(iVar14, 0x98)), uVar8);
    local_4 = (local_4 & 0xffff0000u) | (unsigned short)uVar12;
    if (sVar9 < 1) {
      uVar4 = FUN_00478320(local_4, uVar15);
      sVar9 = (short)(-(short)uVar4);
    } else {
      uVar4 = FUN_00478320(local_4, uVar15);
      sVar9 = (short)uVar4;
    }
    uVar8 = (unsigned short)((short)uVar12 + sVar9);
    uVar5 = FUN_004783e0(uVar8 % 0x1400, 0x1400);
    uVar15 = DAT_0071eae4;
    if (0x7fff < DAT_0071eae4) {
      uVar15 = (unsigned short)-DAT_0071eae4;
    }
    uVar12 = FUN_00478320(((unsigned int)extraout_var_01 << 16) | uVar15, (unsigned short)uVar5);
    sVar9 = (short)((short)uVar12 + (uVar8 / 0x1400) * uVar15);
    if (0x7fff < DAT_0071eae4) {
      sVar9 = (short)-sVar9;
    }
    if (((*OFS(iVar14, 0x1df) & 0x20) == 0) ||
        ((RD32(OFS(iVar14, 0x19)) != 0) && (S16(OFS(iVar14, 0x23e)) == 0))) {
      sVar17 = 0;
      sVar9 = 0;
    }
    uVar8 = (unsigned short)(((unsigned char)DAT_0071e534 & 0xe) == 2);
    if ((unsigned int)(int)S16(base + 0x4b) == (unsigned int)uVar8) {
      sVar11 = S16(base + 0x4d);
      sVar18 = (short)((unsigned short)sVar17 - (unsigned short)sVar11);
      if (sVar18 != 0) {
        sVar3 = sVar18;
        if (sVar17 < sVar11) {
          sVar3 = (short)-sVar18;
        }
        if (sVar3 < 8) {
          WR16(base + 0x4d, sVar11 + sVar18);
        } else {
          if (4 < DAT_0070a1d2) {
            local_4 = (unsigned int)DAT_0070a1d2;
            sVar3 = (short)((int)sVar3 / (int)local_4);
            if (sVar3 == 0) {
              sVar3 = 1;
            }
            sVar3 = (short)(sVar3 << 2);
          }
          if (sVar18 < 0) {
            WR16(base + 0x4d, sVar11 - sVar3);
          } else {
            WR16(base + 0x4d, sVar3 + sVar11);
          }
        }
      }
      sVar17 = S16(base + 0x4f);
      sVar11 = (short)((unsigned short)sVar9 - (unsigned short)sVar17);
      if (sVar11 != 0) {
        sVar18 = sVar11;
        if (sVar9 < sVar17) {
          sVar18 = (short)-sVar11;
        }
        if (sVar18 < 8) {
          WR16(base + 0x4f, sVar17 + sVar11);
        } else {
          if (4 < DAT_0070a1d2) {
            sVar18 = (short)((int)sVar18 / (int)(unsigned int)DAT_0070a1d2);
            if (sVar18 == 0) {
              sVar18 = 1;
            }
            sVar18 = (short)(sVar18 << 2);
          }
          if (sVar11 < 0) {
            WR16(base + 0x4f, sVar17 - sVar18);
          } else {
            WR16(base + 0x4f, sVar18 + sVar17);
          }
        }
      }
    } else {
      WR16(base + 0x4d, 0);
      WR16(base + 0x4f, 0);
    }
    WR16(base + 0x4b, uVar8);
    iVar6 = FUN_00478290((int)S16(base + 0x4d), (unsigned int)DAT_0071e6be, 0xec);
    sVar17 = (short)iVar6;
    iVar6 = FUN_00478290((int)S16(base + 0x4f), (unsigned int)DAT_0071e6be, 0xec);
    sVar9 = (short)iVar6;
    if (((*OFS(iVar14, 0x1df) & 0x20) == 0) ||
        ((RD32(OFS(iVar14, 0x19)) != 0) && (S16(OFS(iVar14, 0x23e)) == 0))) {
      sVar9 = 0;
      sVar17 = 0;
    }
    if (uVar8 != 0) {
      if (sVar9 != 0) {
        FUN_00442e60(sVar9, 0, RD32(base), iVar14);
        CHAIN();
      }
      if (sVar17 == 0) {
        return;
      }
      psVar19 = ENT(RD32(base), 0x14);
      WR16(psVar19, (short)RD16(psVar19) + sVar17 * -2);
      CHAIN();
      return;
    }
    if ((sVar9 == 0) && (sVar17 == 0)) {
      return;
    }
    FUN_00442e60(sVar9, -sVar17, RD32(base), iVar14);
    CHAIN();
    if (sVar17 == 0) {
      return;
    }
    psVar19 = ENT(RD32(base), 0x14);
    WR16(psVar19, (short)RD16(psVar19) - sVar17);
    return;
  }

  bVar7 = base[0x13];
  if (bVar7 != 0) {
    if ((bVar7 & 0x80) == 0) {
      uVar8 = bVar7 & 0x7f;
      if ((uVar8 != 1 && (bVar7 & 0x7f) != 0) && (DAT_00525550 == 0)) {
        if (uVar8 <= DAT_0071e6be) {
          uVar8 = (unsigned short)(DAT_0071e6be + 1);
        }
        bVar7 = (unsigned char)((char)uVar8 - (char)DAT_0071e6be);
LAB_004404a8:
        base[0x13] = bVar7;
      }
    } else {
      uVar8 = bVar7 & 0x7f;
      if (uVar8 < 0x7f) {
        if (DAT_00525550 == 0) {
          if (0x7f < (unsigned int)DAT_0071e6be + (int)(short)uVar8) {
            uVar8 = (unsigned short)(0x7f - DAT_0071e6be);
          }
          bVar7 = (unsigned char)(((char)uVar8 + (char)DAT_0071e6be) | 0x80);
          goto LAB_004404a8;
        }
      } else {
        if (RD16(base + 0x3f7) != 0xffff) {
          WR32(base + 0x3eb, RD32(ENT(RD16(base + 0x3f7), 4)));
          WR32(base + 0x3ef, RD32(ENT(RD16(base + 0x3f7), 8)));
          iVar6 = (int)RD32(ENT(RD16(base + 0x3f7), 0xc));
          WR32(base + 0x3f3, iVar6);
          WR32(base + 0x3f3, RD32(base + 0x416) + (unsigned int)iVar6);
        }
        WR16(base + 0x3f7, 0xffff);
      }
    }
    if (base[0x13] != 0) {
      uVar8 = DAT_0071ecf0;
      if ((short)DAT_0071ecf0 < 0) {
        uVar8 = (unsigned short)-DAT_0071ecf0;
      }
      uVar15 = DAT_0071eae4;
      if ((short)DAT_0071eae4 < 0) {
        uVar15 = (unsigned short)-DAT_0071eae4;
      }
      if ((short)uVar8 < 0x80) {
        DAT_0071ecf0 = 0;
      }
      if ((short)uVar15 < 0x30) {
        DAT_0071eae4 = 0;
      }
    }
  }
  iVar6 = FUN_00478290((int)(short)DAT_0071ecf0, (unsigned int)DAT_0071e6be, 0xec);
  sVar9 = (short)iVar6;
  iVar6 = FUN_00478290((int)(short)DAT_0071eae4, (unsigned int)DAT_0071e6be, 0xec);
  sVar17 = (short)iVar6;
  if ((base[0x13] & 0x80) == 0) {
    if (S16(base + 0x3f7) == -1) {
      if ((sVar17 != 0) || (sVar9 != 0)) {
        FUN_00436450(sVar17, -sVar9, param_1);
      }
    } else {
      WR16(base + 0x405, S16(base + 0x405) + sVar9);
      WR16(base + 0x403, S16(base + 0x403) + sVar17);
    }
  } else {
    WR32(base + 0x3eb, RD32(base + 0x3eb) +
        (unsigned int)(((int)RD32(base + 0x3f3) >> 0xe) + 1) * (unsigned int)(int)sVar9);
    WR32(base + 0x3ef, RD32(base + 0x3ef) +
        (unsigned int)(((int)RD32(base + 0x3f3) >> 0xe) + 1) * (unsigned int)(int)sVar17);
  }
  uVar8 = DAT_0071e534 & 0xf;
  if ((uVar8 != 1) && (uVar8 != 2)) {
    if (base[0x13] == 0) {
      WR16(base + 0x412, 0x20);
      return;
    }
    uVar8 = RD16(base + 0x412);
    if (uVar8 < 0x101) {
      WR16(base + 0x412, 0x20);
      return;
    }
    WR16(base + 0x412, (uVar8 - (uVar8 >> 3)) + -0x20);
    return;
  }
  if (1 < base[0x13]) {
    puVar1 = base + 0x412;
    if ((int)(RD32(base + 0x3f3) & 0xfffffffeu) < 0xe001) {
      WR16(puVar1, (int)RD32(base + 0x3f3) >> 1);
    } else {
      WR16(puVar1, 0x7000);
    }
    if (RD16(puVar1) < 0x800) {
      WR16(puVar1, 0x800);
    }
    goto LAB_0044075b;
  }
  if (base[0x13] == 1) {
    if (S16(base + 0x3f7) != -1) {
      puVar1 = base + 0x412;
      if ((int)(RD32(base + 0x416) & 0xfffffffeu) < 0xe001) {
        WR16(puVar1, (int)RD32(base + 0x416) >> 1);
      } else {
        WR16(puVar1, 0x7000);
      }
      if (RD16(puVar1) < 0x100) {
        WR16(puVar1, 0x100);
      }
      goto LAB_0044075b;
    }
    if (RD16(base + 0x3f9) == 0xffff) {
      puVar1 = base + 0x412;
      uVar15 = (unsigned short)(S16(base + 0x412) + 0x20);
      WR16(puVar1, uVar15);
      uVar15 = (unsigned short)(uVar15 + (uVar15 >> 3));
      WR16(puVar1, uVar15);
      if (0x4000 < uVar15) {
        WR16(puVar1, 0x4000);
      }
      goto LAB_0044075b;
    }
    pe = ENT(RD16(base + 0x3f9), 0);
    puVar1 = base + 0x412;
    iVar6 = FUN_00412810((int)(RD32(pe + 4) - RD32(base + 0x3eb)),
                         (int)(RD32(pe + 8) - RD32(base + 0x3ef)),
                         (int)(RD32(pe + 0xc) - RD32(base + 0x3f3)));
    if (iVar6 < 0x7001) {
      WR16(puVar1, iVar6);
    } else {
      WR16(puVar1, 0x7000);
    }
    if (0x3ff < RD16(puVar1)) {
      goto LAB_0044075b;
    }
  } else {
    sVar9 = S16(base + 0x412);
    WR16(base + 0x412, sVar9 + 0x20U);
    if ((unsigned short)(sVar9 + 0x20U) < 0x401) {
      goto LAB_0044075b;
    }
  }
  WR16(base + 0x412, 0x400);
LAB_0044075b:
  psVar19 = base + 0x412;
  if (uVar8 == 1) {
    if (base[0x13] == 0) {
      iVar6 = FUN_00478290((int)S16(psVar19), (unsigned int)DAT_0071e6be, 0xec);
      iVar6 = (int)(RD32(base + 0x416) - (unsigned int)(int)(short)iVar6);
      WR32(base + 0x416, iVar6);
      if (iVar6 < 0x30) {
        WR32(base + 0x416, 0x30);
        return;
      }
    } else if (S16(base + 0x3f7) == -1) {
      if (S16(base + 0x3f9) == -1) {
        sVar11 = S16(base + 0x405);
        sVar18 = S16(base + 0x403);
        sVar9 = S16(base + 0x3fd);
        sVar17 = S16(base + 0x3fb);
      } else {
        sVar9 = S16(base + 0x3fd);
        sVar11 = 0;
        sVar17 = S16(base + 0x3fb);
        sVar18 = 0;
      }
      FUN_00483180(0, sVar17, sVar9, 0, sVar18, sVar11, 0);
      iVar6 = FUN_00478290((int)S16(psVar19), (unsigned int)DAT_0071e6be, 0xec);
      uVar20 = FUN_00413ca0((int)(short)iVar6, DAT_00732568);
      uVar12 = (unsigned int)DAT_0071e6be;
      WR32(base + 0x3eb, RD32(base + 0x3eb) + (unsigned int)(int)uVar20);
      iVar6 = FUN_00478290((int)S16(psVar19), uVar12, 0xec);
      uVar20 = FUN_00413ca0((int)(short)iVar6, DAT_00732530);
      uVar12 = (unsigned int)DAT_0071e6be;
      WR32(base + 0x3ef, RD32(base + 0x3ef) + (unsigned int)(int)uVar20);
      iVar6 = FUN_00478290((int)S16(psVar19), uVar12, 0xec);
      uVar20 = FUN_00413ca0((int)(short)iVar6, DAT_00732548);
      WR32(base + 0x3f3, RD32(base + 0x3f3) + (unsigned int)(int)uVar20);
      if (RD16(base + 0x3f9) != 0xffff) {
        pe = ENT(RD16(base + 0x3f9), 0);
        iVar6 = FUN_00412810((int)(RD32(pe + 4) - RD32(base + 0x3eb)),
                             (int)(RD32(pe + 8) - RD32(base + 0x3ef)),
                             (int)(RD32(pe + 0xc) - RD32(base + 0x3f3)));
        if (iVar6 < (int)(RD16(DAT_0052686c +
                               (unsigned int)*ENT(RD16(base + 0x3f9), 3) * 0x16) + 0x200)) {
          uVar12 = (unsigned int)*ENT(RD16(base + 0x3f7), 3);
          iVar6 = (int)((unsigned int)iVar6 -
                        (unsigned int)(RD16(DAT_0052686c + uVar12 * 0x16) + 0x200));
          uVar20 = FUN_00413ca0(iVar6, DAT_00732568);
          iVar13 = (int)(RD32(base + 0x3eb) + (unsigned int)(int)uVar20);
          WR32(base + 0x3eb, iVar13);
          uVar20 = FUN_00413ca0(iVar6, DAT_00732530);
          iVar14 = (int)(RD32(base + 0x3ef) + (unsigned int)(int)uVar20);
          WR32(base + 0x3ef, iVar14);
          uVar20 = FUN_00413ca0(iVar6, DAT_00732548);
          WR32(base + 0x3f3, RD32(base + 0x3f3) + (unsigned int)(int)uVar20);
        }
      }
      if ((1 < base[0x13]) && ((int)RD32(base + 0x3f3) < 0x30)) {
        WR32(base + 0x3f3, 0x30);
      }
      if ((int)RD32(base + 0x3eb) < -0x1000000) {
        WR32(base + 0x3eb, 0xff000000u);
      }
      if (0x1000000 < (int)RD32(base + 0x3eb)) {
        WR32(base + 0x3eb, 0x1000000u);
      }
      if ((int)RD32(base + 0x3ef) < -0x1000000) {
        WR32(base + 0x3ef, 0xff000000u);
      }
      if (0x1000000 < (int)RD32(base + 0x3ef)) {
        WR32(base + 0x3ef, 0x1000000u);
      }
      if ((int)RD32(base + 0x3f3) < -0x1000000) {
        WR32(base + 0x3f3, 0xff000000u);
      }
      if (0x1000000 < (int)RD32(base + 0x3f3)) {
        WR32(base + 0x3f3, 0x1000000u);
        return;
      }
    } else {
      iVar6 = FUN_00478290((int)S16(psVar19), (unsigned int)DAT_0071e6be, 0xec);
      iVar6 = (int)(RD32(base + 0x416) - (unsigned int)(int)(short)iVar6);
      WR32(base + 0x416, iVar6);
      if (iVar6 < (int)(RD16(DAT_0052686c +
                             (unsigned int)*ENT(RD16(base + 0x3f7), 3) * 0x16) + 0x200)) {
        WR32(base + 0x416, RD16(DAT_0052686c +
                             (unsigned int)*ENT(RD16(base + 0x3f7), 3) * 0x16) + 0x200);
        return;
      }
    }
  } else if (base[0x13] == 0) {
    iVar6 = FUN_00478290((int)S16(psVar19), (unsigned int)DAT_0071e6be, 0xec);
    iVar6 = (int)((unsigned int)(int)(short)iVar6 + RD32(base + 0x416));
    WR32(base + 0x416, iVar6);
    if (0x1400 < iVar6) {
      WR32(base + 0x416, 0x1400);
      return;
    }
  } else {
    if (S16(base + 0x3f7) != -1) {
      iVar6 = FUN_00478290((int)S16(psVar19), (unsigned int)DAT_0071e6be, 0xec);
      WR32(base + 0x416, RD32(base + 0x416) + (unsigned int)(int)(short)iVar6);
      return;
    }
    if (S16(base + 0x3f9) == -1) {
      sVar11 = S16(base + 0x405);
      sVar18 = S16(base + 0x403);
      sVar9 = S16(base + 0x3fd);
      sVar17 = S16(base + 0x3fb);
    } else {
      sVar9 = S16(base + 0x3fd);
      sVar11 = 0;
      sVar17 = S16(base + 0x3fb);
      sVar18 = 0;
    }
    FUN_00483180(0, sVar17, sVar9, 0, sVar18, sVar11, 0);
    iVar6 = FUN_00478290((int)S16(psVar19), (unsigned int)DAT_0071e6be, 0xec);
    uVar20 = FUN_00413ca0((int)(short)iVar6, DAT_00732568);
    uVar12 = (unsigned int)DAT_0071e6be;
    WR32(base + 0x3eb, RD32(base + 0x3eb) - (unsigned int)(int)uVar20);
    iVar6 = FUN_00478290((int)S16(psVar19), uVar12, 0xec);
    uVar20 = FUN_00413ca0((int)(short)iVar6, DAT_00732530);
    uVar12 = (unsigned int)DAT_0071e6be;
    WR32(base + 0x3ef, RD32(base + 0x3ef) - (unsigned int)(int)uVar20);
    iVar6 = FUN_00478290((int)S16(psVar19), uVar12, 0xec);
    uVar20 = FUN_00413ca0((int)(short)iVar6, DAT_00732548);
    WR32(base + 0x3f3, RD32(base + 0x3f3) - (unsigned int)(int)uVar20);
    if ((int)RD32(base + 0x3eb) < -0x1000000) {
      WR32(base + 0x3eb, 0xff000000u);
    }
    if (0x1000000 < (int)RD32(base + 0x3eb)) {
      WR32(base + 0x3eb, 0x1000000u);
    }
    if ((int)RD32(base + 0x3ef) < -0x1000000) {
      WR32(base + 0x3ef, 0xff000000u);
    }
    if (0x1000000 < (int)RD32(base + 0x3ef)) {
      WR32(base + 0x3ef, 0x1000000u);
    }
    if ((int)RD32(base + 0x3f3) < -0x1000000) {
      WR32(base + 0x3f3, 0xff000000u);
    }
    if (0x1000000 < (int)RD32(base + 0x3f3)) {
      WR32(base + 0x3f3, 0x1000000u);
    }
  }
  return;
}
