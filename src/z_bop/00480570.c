/* SPDX-License-Identifier: MIT */
/* z_bop 00480570 */
/* Callee prototypes */
void FUN_004115f0(void);
unsigned int FUN_00425b70(unsigned short a, unsigned short b);
unsigned int FUN_00425c20(unsigned int a, unsigned int b);
short FUN_00481d70(short a);
void FUN_00483a00(short a, int b, unsigned int c, int d);
void FUN_00438330(short a, int b, int c);
int FUN_0041f410(int a, int b, int c);
void FUN_00427940(int a, short b, short c, int d, short e, short f, int g);

/* Globals (types inferred from access widths) */
extern unsigned short DAT_009eca24;
extern unsigned short DAT_009ecc30;
extern unsigned short DAT_009ec474;
extern unsigned short DAT_009ec5fe;
extern unsigned short DAT_009d8112;
extern unsigned short _DAT_00999414;
extern int DAT_00523410;
extern int DAT_009a1fe8;
extern int DAT_009fe7f4;
extern int DAT_00a00490;
extern int DAT_00a004b0;
extern unsigned char DAT_00521f1c[];

/* Per-slot data, stride 0x5bd (odd => fields are packed/unaligned) */
extern unsigned char DAT_009e9670[];
extern unsigned char DAT_009e9683[];
extern unsigned char DAT_009e9684[];
extern unsigned char DAT_009e96bb[];
extern unsigned char DAT_009e96bd[];
extern unsigned char DAT_009e96bf[];
extern unsigned char DAT_009e9a5b[];
extern unsigned char DAT_009e9a5f[];
extern unsigned char DAT_009e9a63[];
extern unsigned char DAT_009e9a67[];
extern unsigned char DAT_009e9a69[];
extern unsigned char DAT_009e9a6b[];
extern unsigned char DAT_009e9a6d[];
extern unsigned char DAT_009e9a73[];
extern unsigned char DAT_009e9a75[];
extern unsigned char DAT_009e9a80[];
extern unsigned char DAT_009e9a82[];
extern unsigned char DAT_009e9a86[];

#define LDS(p) ({ short t_; __builtin_memcpy(&t_, (const void *)(unsigned int)(p), 2); t_; })
#define LDU(p) ({ unsigned short t_; __builtin_memcpy(&t_, (const void *)(unsigned int)(p), 2); t_; })
#define LDI(p) ({ int t_; __builtin_memcpy(&t_, (const void *)(unsigned int)(p), 4); t_; })
#define LDUI(p) ({ unsigned int t_; __builtin_memcpy(&t_, (const void *)(unsigned int)(p), 4); t_; })
#define LDB(p) (*(unsigned char *)(unsigned int)(p))
#define ST16(p, v) do { unsigned short s_ = (unsigned short)(v); __builtin_memcpy((void *)(unsigned int)(p), &s_, 2); } while (0)
#define ST32(p, v) do { unsigned int s_ = (unsigned int)(v); __builtin_memcpy((void *)(unsigned int)(p), &s_, 4); } while (0)
#define STB(p, v) (*(unsigned char *)(unsigned int)(p) = (unsigned char)(v))

/* entity slot address = idx * 0x23 + DAT_009a1fe8 */
#define ENT(i) ((unsigned int)(i) * 0x23u + (unsigned int)DAT_009a1fe8)
/* (a * b) >> 15 as low-32 | high-32 << 17 */
#define MULSH(a, b) ((unsigned int)((long long)(a) * (long long)(b)) >> 0xf | \
                     (unsigned int)((unsigned long long)((long long)(a) * (long long)(b)) >> 0x20) << 0x11)

void FUN_00480570(int param_1)
{
  int iVar2;
  int iVar3;
  short sVar4;
  unsigned int uVar5;
  unsigned int uVar6;
  unsigned char bVar7;
  unsigned short uVar8;
  int iVar9;
  unsigned short uVar10;
  short sVar11;
  int iVar12;
  unsigned int uVar13;
  unsigned short uVar14;
  short sVar15;
  unsigned short uVar16;
  short sVar17;
  short sVar18;
  unsigned char *psVar19;
  unsigned int local_3c;
  unsigned short uStack_a;
  unsigned short local_6;
  unsigned short *puVar1;

  FUN_004115f0();
  DAT_009eca24 = (unsigned short)(DAT_009eca24 << 1);
  iVar2 = (int)((unsigned int)param_1 * 0x5bd);
  if ((LDS(DAT_009e9a80 + iVar2) == 0) && (DAT_009e9683[iVar2] == 0)) {
    if (DAT_009e9684[iVar2] != 0) {
      return;
    }
    iVar12 = (int)(LDUI(DAT_009e9670 + iVar2) * 0x23u);
    iVar9 = LDI(LDI((unsigned int)iVar12 + 0x1fu + (unsigned int)DAT_009a1fe8) + 0xa9);
    uVar10 = LDU(iVar9 + 0xbd);
    if ((LDB(iVar9 + 0x1e5) & 0x40) == 0) {
      uVar10 = 0;
    }
    if (uVar10 < 0x5555) {
      uVar10 = (unsigned short)(uVar10 * 2 + 0x5555);
    }
    else {
      uVar10 = (unsigned short)((short)((int)(0x5555 - (unsigned int)uVar10) / 2) - 1);
    }
    sVar11 = (short)LDB(iVar9 + 0x21c);
    if ((LDB(iVar9 + 0x1e3) & 1) == 0) {
      sVar11 = (short)(sVar11 * 2);
    }
    else {
      sVar11 = (short)(sVar11 + (unsigned short)LDB(iVar9 + 0x219));
    }
    sVar11 = (short)(4 - sVar11);
    if (sVar11 < 0) {
      uVar14 = (unsigned short)(sVar11 * -0xc00);
    }
    else {
      uVar14 = (unsigned short)(sVar11 * 0xc00);
    }
    uVar13 = FUN_00425b70(LDU(iVar9 + 0xa0), uVar10);
    if (sVar11 < 1) {
      uVar5 = FUN_00425b70((unsigned short)uVar13, uVar14);
      sVar17 = (short)(-(short)uVar5);
    }
    else {
      uVar5 = FUN_00425b70((unsigned short)uVar13, uVar14);
      sVar17 = (short)uVar5;
    }
    uVar16 = (unsigned short)((short)uVar13 + sVar17);
    uVar6 = FUN_00425c20(uVar16 % 0x3800, 0x3800);
    uVar8 = DAT_009ecc30;
    if (0x7fff < DAT_009ecc30) {
      uVar8 = (unsigned short)(-DAT_009ecc30);
    }
    uVar13 = FUN_00425b70(uVar8, (unsigned short)uVar6);
    sVar17 = (short)((unsigned int)(uVar16 / 0x3800) * uVar8 + (unsigned int)(int)(short)uVar13);
    if (0x7fff < DAT_009ecc30) {
      sVar17 = (short)(-sVar17);
    }
    _DAT_00999414 = DAT_009ecc30;
    if (0x7fff < DAT_009ecc30) {
      _DAT_00999414 = (unsigned short)(-DAT_009ecc30);
    }
    uVar5 = FUN_00425b70(LDU(iVar9 + 0x98), uVar10);
    uVar13 = uVar5;
    if (sVar11 < 1) {
      uVar13 = FUN_00425b70((unsigned short)uVar13, uVar14);
      sVar11 = (short)(-(short)uVar13);
    }
    else {
      uVar13 = FUN_00425b70((unsigned short)uVar13, uVar14);
      sVar11 = (short)uVar13;
    }
    local_6 = (unsigned short)((short)uVar5 + sVar11);
    uVar6 = FUN_00425c20(local_6 % 0x1400, 0x1400);
    uVar10 = DAT_009eca24;
    if (0x7fff < DAT_009eca24) {
      uVar10 = (unsigned short)(-DAT_009eca24);
    }
    uVar13 = FUN_00425b70(uVar10, (unsigned short)uVar6);
    sVar11 = (short)((unsigned int)(local_6 / 0x1400) * uVar10 + (unsigned int)(int)(short)uVar13);
    if (0x7fff < DAT_009eca24) {
      sVar11 = (short)(-sVar11);
    }
    if (((LDB(iVar9 + 0x1e5) & 0x20) == 0) ||
        ((LDI(iVar9 + 0x19) != 0 && (LDS(iVar9 + 0x248) == 0)))) {
      sVar17 = 0;
      sVar11 = 0;
    }
    uStack_a = (unsigned short)(((unsigned char)DAT_009ec474 & 0xe) == 2);
    if ((int)LDS(DAT_009e96bb + iVar2) == (int)uStack_a) {
      sVar15 = LDS(DAT_009e96bd + iVar2);
      sVar18 = (short)(sVar17 - sVar15);
      if (sVar18 != 0) {
        sVar4 = sVar18;
        if (sVar17 < sVar15) {
          sVar4 = (short)(-sVar18);
        }
        if (sVar4 < 8) {
          ST16(DAT_009e96bd + iVar2, sVar15 + sVar18);
        }
        else {
          if (4 < DAT_009d8112) {
            local_3c = (unsigned int)DAT_009d8112;
            sVar4 = (short)((int)sVar4 / (int)local_3c);
            if (sVar4 == 0) {
              sVar4 = 1;
            }
            sVar4 = (short)(sVar4 << 2);
          }
          if (sVar18 < 0) {
            ST16(DAT_009e96bd + iVar2, sVar15 - sVar4);
          }
          else {
            ST16(DAT_009e96bd + iVar2, sVar4 + sVar15);
          }
        }
      }
      sVar17 = LDS(DAT_009e96bf + iVar2);
      sVar15 = (short)(sVar11 - sVar17);
      if (sVar15 != 0) {
        sVar18 = sVar15;
        if (sVar11 < sVar17) {
          sVar18 = (short)(-sVar15);
        }
        if (sVar18 < 8) {
          ST16(DAT_009e96bf + iVar2, sVar17 + sVar15);
        }
        else {
          if (4 < DAT_009d8112) {
            sVar18 = (short)((int)sVar18 / (int)(unsigned int)DAT_009d8112);
            if (sVar18 == 0) {
              sVar18 = 1;
            }
            sVar18 = (short)(sVar18 << 2);
          }
          if (sVar15 < 0) {
            ST16(DAT_009e96bf + iVar2, sVar17 - sVar18);
          }
          else {
            ST16(DAT_009e96bf + iVar2, sVar18 + sVar17);
          }
        }
      }
    }
    else {
      ST16(DAT_009e96bd + iVar2, 0);
      ST16(DAT_009e96bf + iVar2, 0);
    }
    ST16(DAT_009e96bb + iVar2, uStack_a);
    sVar11 = FUN_00481d70(LDS(DAT_009e96bd + iVar2));
    sVar17 = FUN_00481d70(LDS(DAT_009e96bf + iVar2));
    if (((LDB(iVar9 + 0x1e5) & 0x20) == 0) ||
        ((LDI(iVar9 + 0x19) != 0 && (LDS(iVar9 + 0x248) == 0)))) {
      sVar17 = 0;
      sVar11 = 0;
    }
    if (uStack_a != 0) {
      if (sVar17 != 0) {
        FUN_00483a00(sVar17, 0, LDUI(DAT_009e9670 + iVar2), iVar9);
        STB(LDI(ENT(LDUI(DAT_009e9670 + iVar2)) + 0x1f) + 0x92, 1);
        iVar9 = LDI(ENT(LDUI(DAT_009e9670 + iVar2)) + 0x1f);
        STB(iVar9 + 0x8b, LDB(iVar9 + 0x92));
      }
      if (sVar11 == 0) {
        return;
      }
      psVar19 = (unsigned char *)(ENT(LDUI(DAT_009e9670 + iVar2)) + 0x14);
      ST16(psVar19, LDS(psVar19) + sVar11 * -2);
      STB(LDI(ENT(LDUI(DAT_009e9670 + iVar2)) + 0x1f) + 0x92, 1);
      iVar2 = LDI(ENT(LDUI(DAT_009e9670 + iVar2)) + 0x1f);
      STB(iVar2 + 0x8b, LDB(iVar2 + 0x92));
      return;
    }
    if ((sVar17 == 0) && (sVar11 == 0)) {
      return;
    }
    FUN_00483a00(sVar17, -sVar11, LDUI(DAT_009e9670 + iVar2), iVar9);
    STB(LDI(ENT(LDUI(DAT_009e9670 + iVar2)) + 0x1f) + 0x92, 1);
    iVar9 = LDI(ENT(LDUI(DAT_009e9670 + iVar2)) + 0x1f);
    STB(iVar9 + 0x8b, LDB(iVar9 + 0x92));
    if (sVar11 == 0) {
      return;
    }
    psVar19 = (unsigned char *)(ENT(LDUI(DAT_009e9670 + iVar2)) + 0x14);
    ST16(psVar19, LDS(psVar19) - sVar11);
    return;
  }
  bVar7 = DAT_009e9683[iVar2];
  if (bVar7 != 0) {
    if ((bVar7 & 0x80) == 0) {
      uVar10 = (unsigned short)(bVar7 & 0x7f);
      if ((uVar10 != 1 && (bVar7 & 0x7f) != 0) && (DAT_00523410 == 0)) {
        if (uVar10 <= DAT_009ec5fe) {
          uVar10 = (unsigned short)(DAT_009ec5fe + 1);
        }
        bVar7 = (unsigned char)((char)uVar10 - (char)DAT_009ec5fe);
        DAT_009e9683[iVar2] = bVar7;
      }
    }
    else {
      uVar10 = (unsigned short)(bVar7 & 0x7f);
      if (uVar10 < 0x7f) {
        if (DAT_00523410 == 0) {
          if (0x7f < (unsigned int)DAT_009ec5fe + (int)(short)uVar10) {
            uVar10 = (unsigned short)(0x7f - DAT_009ec5fe);
          }
          bVar7 = (unsigned char)(((char)uVar10 + (char)DAT_009ec5fe) | 0x80);
          DAT_009e9683[iVar2] = bVar7;
        }
      }
      else {
        if (LDU(DAT_009e9a67 + iVar2) != 0xffff) {
          ST32(DAT_009e9a5b + iVar2, LDI(ENT(LDU(DAT_009e9a67 + iVar2)) + 4));
          ST32(DAT_009e9a5f + iVar2, LDI(ENT(LDU(DAT_009e9a67 + iVar2)) + 8));
          iVar9 = LDI(ENT(LDU(DAT_009e9a67 + iVar2)) + 0xc);
          ST32(DAT_009e9a63 + iVar2, iVar9);
          ST32(DAT_009e9a63 + iVar2, (unsigned int)LDI(DAT_009e9a86 + iVar2) + (unsigned int)iVar9);
        }
        ST16(DAT_009e9a67 + iVar2, 0xffff);
      }
    }
    if (DAT_009e9683[iVar2] != 0) {
      uVar10 = DAT_009ecc30;
      if ((short)DAT_009ecc30 < 0) {
        uVar10 = (unsigned short)(-DAT_009ecc30);
      }
      uVar14 = DAT_009eca24;
      if ((short)DAT_009eca24 < 0) {
        uVar14 = (unsigned short)(-DAT_009eca24);
      }
      if ((short)uVar10 < 0x80) {
        DAT_009ecc30 = 0;
      }
      if ((short)uVar14 < 0x30) {
        DAT_009eca24 = 0;
      }
    }
  }
  sVar11 = FUN_00481d70((short)DAT_009ecc30);
  sVar17 = FUN_00481d70((short)DAT_009eca24);
  if ((DAT_009e9683[iVar2] & 0x80) == 0) {
    if (LDS(DAT_009e9a67 + iVar2) == -1) {
      if ((sVar17 != 0) || (sVar11 != 0)) {
        FUN_00438330(sVar17, -sVar11, param_1);
      }
    }
    else {
      ST16(DAT_009e9a75 + iVar2, LDS(DAT_009e9a75 + iVar2) + sVar11);
      ST16(DAT_009e9a73 + iVar2, LDS(DAT_009e9a73 + iVar2) + sVar17);
    }
  }
  else {
    ST32(DAT_009e9a5b + iVar2,
         (unsigned int)LDI(DAT_009e9a5b + iVar2) +
         (unsigned int)(((unsigned int)(LDI(DAT_009e9a63 + iVar2) >> 0xe) + 1u) *
                        (unsigned int)(int)sVar11));
    ST32(DAT_009e9a5f + iVar2,
         (unsigned int)LDI(DAT_009e9a5f + iVar2) +
         (unsigned int)(((unsigned int)(LDI(DAT_009e9a63 + iVar2) >> 0xe) + 1u) *
                        (unsigned int)(int)sVar17));
  }
  uVar10 = (unsigned short)(DAT_009ec474 & 0xf);
  if ((uVar10 != 1) && (uVar10 != 2)) {
    if (DAT_009e9683[iVar2] == 0) {
      ST16(DAT_009e9a82 + iVar2, 0x20);
      return;
    }
    uVar10 = LDU(DAT_009e9a82 + iVar2);
    if (uVar10 < 0x101) {
      ST16(DAT_009e9a82 + iVar2, 0x20);
      return;
    }
    ST16(DAT_009e9a82 + iVar2, (uVar10 - (uVar10 >> 3)) + -0x20);
    return;
  }
  if (1 < DAT_009e9683[iVar2]) {
    puVar1 = (unsigned short *)(DAT_009e9a82 + iVar2);
    if ((int)(LDUI(DAT_009e9a63 + iVar2) & 0xfffffffe) < 0xe001) {
      ST16(puVar1, (unsigned short)((int)LDUI(DAT_009e9a63 + iVar2) >> 1));
    }
    else {
      ST16(puVar1, 0x7000);
    }
    if (LDU(puVar1) < 0x800) {
      ST16(puVar1, 0x800);
    }
    goto LAB_00480e5e;
  }
  if (DAT_009e9683[iVar2] == 1) {
    if (LDS(DAT_009e9a67 + iVar2) != -1) {
      puVar1 = (unsigned short *)(DAT_009e9a82 + iVar2);
      if ((int)(LDUI(DAT_009e9a86 + iVar2) & 0xfffffffe) < 0xe001) {
        ST16(puVar1, (unsigned short)((int)LDUI(DAT_009e9a86 + iVar2) >> 1));
      }
      else {
        ST16(puVar1, 0x7000);
      }
      if (LDU(puVar1) < 0x100) {
        ST16(puVar1, 0x100);
      }
      goto LAB_00480e5e;
    }
    if (LDU(DAT_009e9a69 + iVar2) == 0xffff) {
      puVar1 = (unsigned short *)(DAT_009e9a82 + iVar2);
      uVar14 = (unsigned short)(LDS(DAT_009e9a82 + iVar2) + 0x20);
      ST16(puVar1, uVar14);
      uVar14 = (unsigned short)(uVar14 + (uVar14 >> 3));
      ST16(puVar1, uVar14);
      if (0x4000 < uVar14) {
        ST16(puVar1, 0x4000);
      }
      goto LAB_00480e5e;
    }
    iVar9 = (int)(ENT(LDU(DAT_009e9a69 + iVar2)));
    puVar1 = (unsigned short *)(DAT_009e9a82 + iVar2);
    iVar9 = FUN_0041f410((int)((unsigned int)LDI(iVar9 + 4) - (unsigned int)LDI(DAT_009e9a5b + iVar2)),
                         (int)((unsigned int)LDI(iVar9 + 8) - (unsigned int)LDI(DAT_009e9a5f + iVar2)),
                         (int)((unsigned int)LDI(iVar9 + 0xc) - (unsigned int)LDI(DAT_009e9a63 + iVar2)));
    if (iVar9 < 0x7001) {
      ST16(puVar1, (unsigned short)iVar9);
    }
    else {
      ST16(puVar1, 0x7000);
    }
    if (0x3ff < LDU(puVar1)) goto LAB_00480e5e;
  }
  else {
    sVar11 = LDS(DAT_009e9a82 + iVar2);
    ST16(DAT_009e9a82 + iVar2, sVar11 + 0x20U);
    if ((unsigned short)(sVar11 + 0x20U) < 0x401) goto LAB_00480e5e;
  }
  ST16(DAT_009e9a82 + iVar2, 0x400);
LAB_00480e5e:
  psVar19 = DAT_009e9a82 + iVar2;
  if (uVar10 == 1) {
    if (DAT_009e9683[iVar2] == 0) {
      sVar11 = FUN_00481d70(LDS(psVar19));
      iVar9 = (int)((unsigned int)LDI(DAT_009e9a86 + iVar2) - (unsigned int)(int)sVar11);
      ST32(DAT_009e9a86 + iVar2, iVar9);
      if (iVar9 < 0x30) {
        ST32(DAT_009e9a86 + iVar2, 0x30);
        return;
      }
    }
    else if (LDS(DAT_009e9a67 + iVar2) == -1) {
      if (LDS(DAT_009e9a69 + iVar2) == -1) {
        sVar15 = LDS(DAT_009e9a75 + iVar2);
        sVar18 = LDS(DAT_009e9a73 + iVar2);
        sVar11 = LDS(DAT_009e9a6d + iVar2);
        sVar17 = LDS(DAT_009e9a6b + iVar2);
      }
      else {
        sVar11 = LDS(DAT_009e9a6d + iVar2);
        sVar15 = 0;
        sVar17 = LDS(DAT_009e9a6b + iVar2);
        sVar18 = 0;
      }
      FUN_00427940(0, sVar17, sVar11, 0, sVar18, sVar15, 0);
      iVar12 = DAT_00a004b0;
      sVar11 = FUN_00481d70(LDS(psVar19));
      iVar9 = DAT_009fe7f4;
      ST32(DAT_009e9a5b + iVar2,
           (unsigned int)LDI(DAT_009e9a5b + iVar2) + MULSH((int)sVar11, iVar12));
      sVar11 = FUN_00481d70(LDS(psVar19));
      iVar12 = DAT_00a00490;
      ST32(DAT_009e9a5f + iVar2,
           (unsigned int)LDI(DAT_009e9a5f + iVar2) + MULSH((int)sVar11, iVar9));
      sVar11 = FUN_00481d70(LDS(psVar19));
      iVar9 = (int)((unsigned int)LDI(DAT_009e9a63 + iVar2) + MULSH((int)sVar11, iVar12));
      ST32(DAT_009e9a63 + iVar2, iVar9);
      if (LDU(DAT_009e9a69 + iVar2) != 0xffff) {
        iVar12 = (int)ENT(LDU(DAT_009e9a69 + iVar2));
        iVar12 = FUN_0041f410((int)((unsigned int)LDI(iVar12 + 4) - (unsigned int)LDI(DAT_009e9a5b + iVar2)),
                              (int)((unsigned int)LDI(iVar12 + 8) - (unsigned int)LDI(DAT_009e9a5f + iVar2)),
                              (int)((unsigned int)LDI(iVar12 + 0xc) - (unsigned int)iVar9));
        iVar9 = DAT_009fe7f4;
        if (iVar12 < (int)((unsigned int)LDI(DAT_00521f1c +
                              (unsigned int)LDB(ENT(LDU(DAT_009e9a69 + iVar2)) + 3) * 0x1a) + 0x200u)) {
          iVar12 = (int)((unsigned int)iVar12 -
                         ((unsigned int)LDI(DAT_00521f1c +
                              (unsigned int)LDB(ENT(LDU(DAT_009e9a67 + iVar2)) + 3) * 0x1a) + 0x200u));
          ST32(DAT_009e9a5b + iVar2,
               (unsigned int)LDI(DAT_009e9a5b + iVar2) + MULSH(iVar12, DAT_00a004b0));
          iVar3 = DAT_00a00490;
          ST32(DAT_009e9a5f + iVar2,
               (unsigned int)LDI(DAT_009e9a5f + iVar2) + MULSH(iVar12, iVar9));
          ST32(DAT_009e9a63 + iVar2,
               (unsigned int)LDI(DAT_009e9a63 + iVar2) + MULSH(iVar12, iVar3));
        }
      }
      if ((1 < DAT_009e9683[iVar2]) && (LDI(DAT_009e9a63 + iVar2) < 0x30)) {
        ST32(DAT_009e9a63 + iVar2, 0x30);
      }
      if (LDI(DAT_009e9a5b + iVar2) < -0x1000000) {
        ST32(DAT_009e9a5b + iVar2, 0xff000000);
      }
      if (0x1000000 < LDI(DAT_009e9a5b + iVar2)) {
        ST32(DAT_009e9a5b + iVar2, 0x1000000);
      }
      if (LDI(DAT_009e9a5f + iVar2) < -0x1000000) {
        ST32(DAT_009e9a5f + iVar2, 0xff000000);
      }
      if (0x1000000 < LDI(DAT_009e9a5f + iVar2)) {
        ST32(DAT_009e9a5f + iVar2, 0x1000000);
      }
      if (LDI(DAT_009e9a63 + iVar2) < -0x1000000) {
        ST32(DAT_009e9a63 + iVar2, 0xff000000);
      }
      if (0x1000000 < LDI(DAT_009e9a63 + iVar2)) {
        ST32(DAT_009e9a63 + iVar2, 0x1000000);
        return;
      }
    }
    else {
      sVar11 = FUN_00481d70(LDS(psVar19));
      ST32(DAT_009e9a86 + iVar2, (unsigned int)LDI(DAT_009e9a86 + iVar2) - (unsigned int)(int)sVar11);
      if (LDI(DAT_009e9a86 + iVar2) <
          (int)((unsigned int)LDI(DAT_00521f1c +
                (unsigned int)LDB(ENT(LDU(DAT_009e9a67 + iVar2)) + 3) * 0x1a) + 0x200u)) {
        ST32(DAT_009e9a86 + iVar2,
             (unsigned int)LDI(DAT_00521f1c +
                (unsigned int)LDB(ENT(LDU(DAT_009e9a67 + iVar2)) + 3) * 0x1a) + 0x200u);
        return;
      }
    }
  }
  else if (DAT_009e9683[iVar2] == 0) {
    sVar11 = FUN_00481d70(LDS(psVar19));
    iVar9 = (int)((unsigned int)LDI(DAT_009e9a86 + iVar2) + (unsigned int)(int)sVar11);
    ST32(DAT_009e9a86 + iVar2, iVar9);
    if (0x1400 < iVar9) {
      ST32(DAT_009e9a86 + iVar2, 0x1400);
      return;
    }
  }
  else {
    if (LDS(DAT_009e9a67 + iVar2) != -1) {
      sVar11 = FUN_00481d70(LDS(psVar19));
      ST32(DAT_009e9a86 + iVar2, (unsigned int)LDI(DAT_009e9a86 + iVar2) + (unsigned int)(int)sVar11);
      return;
    }
    if (LDS(DAT_009e9a69 + iVar2) == -1) {
      sVar15 = LDS(DAT_009e9a75 + iVar2);
      sVar18 = LDS(DAT_009e9a73 + iVar2);
      sVar11 = LDS(DAT_009e9a6d + iVar2);
      sVar17 = LDS(DAT_009e9a6b + iVar2);
    }
    else {
      sVar11 = LDS(DAT_009e9a6d + iVar2);
      sVar15 = 0;
      sVar17 = LDS(DAT_009e9a6b + iVar2);
      sVar18 = 0;
    }
    FUN_00427940(0, sVar17, sVar11, 0, sVar18, sVar15, 0);
    iVar12 = DAT_00a004b0;
    sVar11 = FUN_00481d70(LDS(psVar19));
    iVar9 = DAT_009fe7f4;
    ST32(DAT_009e9a5b + iVar2,
         (unsigned int)LDI(DAT_009e9a5b + iVar2) - MULSH((int)sVar11, iVar12));
    sVar11 = FUN_00481d70(LDS(psVar19));
    iVar12 = DAT_00a00490;
    ST32(DAT_009e9a5f + iVar2,
         (unsigned int)LDI(DAT_009e9a5f + iVar2) - MULSH((int)sVar11, iVar9));
    sVar11 = FUN_00481d70(LDS(psVar19));
    ST32(DAT_009e9a63 + iVar2,
         (unsigned int)LDI(DAT_009e9a63 + iVar2) - MULSH((int)sVar11, iVar12));
    if (LDI(DAT_009e9a5b + iVar2) < -0x1000000) {
      ST32(DAT_009e9a5b + iVar2, 0xff000000);
    }
    if (0x1000000 < LDI(DAT_009e9a5b + iVar2)) {
      ST32(DAT_009e9a5b + iVar2, 0x1000000);
    }
    if (LDI(DAT_009e9a5f + iVar2) < -0x1000000) {
      ST32(DAT_009e9a5f + iVar2, 0xff000000);
    }
    if (0x1000000 < LDI(DAT_009e9a5f + iVar2)) {
      ST32(DAT_009e9a5f + iVar2, 0x1000000);
    }
    if (LDI(DAT_009e9a63 + iVar2) < -0x1000000) {
      ST32(DAT_009e9a63 + iVar2, 0xff000000);
    }
    if (0x1000000 < LDI(DAT_009e9a63 + iVar2)) {
      ST32(DAT_009e9a63 + iVar2, 0x1000000);
    }
  }
  return;
}
