/* SPDX-License-Identifier: MIT */
/* z_bop 00442bf0 */
void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);
void __cdecl FUN_00458e20(unsigned int, int);
void __cdecl FUN_00427940(short, short, short, int, int, int, int);
void __cdecl FUN_0046a8d0(int, int, int);
void __cdecl FUN_00427bd0(short, short, int);
void __cdecl FUN_00427d30(short, int, int);

extern unsigned char DAT_009e9683[];
extern unsigned char DAT_009e9670[];
extern unsigned char DAT_009e9a5b[];
extern unsigned char DAT_009e9a5f[];
extern unsigned char DAT_009e9a63[];
extern unsigned char DAT_009e9a6b[];
extern unsigned char DAT_009e9a6d[];
extern unsigned char DAT_009e9a6f[];
extern unsigned char DAT_009e9bf6[];
extern unsigned char DAT_00521f1c[];
extern unsigned char DAT_0051c659[];
extern unsigned char DAT_0051c65b[];
extern unsigned char DAT_0051c65d[];
extern unsigned char DAT_0051c65f[];
extern unsigned char DAT_00a08cbe[];
extern int DAT_009a1fe8;
extern short DAT_009a8070;
extern int DAT_009fe7f4;
extern int DAT_00a00480;
extern int DAT_00a00488;
extern int DAT_00a00490;
extern int DAT_00a004a4;
extern int DAT_00a004a8;
extern int DAT_00a004b0;
extern int DAT_00a004b8;
extern int DAT_00a004cc;
extern int DAT_00a07cd4;
extern int DAT_00a07cd8;
extern int DAT_00a07cdc;
extern short DAT_00a080f0;
extern unsigned short DAT_00a08374;

#define LD(var, addr) __builtin_memcpy(&(var), (const void *)(addr), sizeof(var))
#define ST(addr, var) __builtin_memcpy((void *)(addr), &(var), sizeof(var))

void __cdecl FUN_00442bf0(unsigned int param_1, short param_2, int param_3)
{
  int iVar2;
  short sVar3;
  unsigned int uVar4;
  int iVar5;
  signed char sVar6;
  unsigned char bVar7;
  unsigned short uVar8;
  short sVar9;
  unsigned int uVar10;
  unsigned short uVar11;
  unsigned int uVar12;
  short sVar13;
  int iVar14;
  int iVar15;
  int iVar16;
  int iVar17;
  int iStack_8;
  short sStack_8;
  int tI;
  int pS;
  short tS1;
  short tS2;
  short tS3;
  unsigned char tB;
  unsigned short tU;
  long long prod;
  unsigned int outv;

  FUN_00458e20(param_1 & 0xffff, 0);
  iVar2 = (int)((unsigned int)param_3 * 0x5bdu);
  if (DAT_009e9683[iVar2] == 0) {
    LD(tI, DAT_009e9670 + iVar2);
    iVar17 = (int)((unsigned int)tI * 0x23u + (unsigned int)DAT_009a1fe8);
    LD(iVar15, iVar17 + 4);
    LD(iVar14, iVar17 + 8);
    LD(iVar17, iVar17 + 0xc);
  }
  else {
    LD(iVar15, DAT_009e9a5b + iVar2);
    LD(iVar14, DAT_009e9a5f + iVar2);
    LD(iVar17, DAT_009e9a63 + iVar2);
  }
  iVar15 = (int)(((unsigned int)DAT_00a07cdc - (unsigned int)iVar15) * 2u);
  iVar14 = (int)(((unsigned int)DAT_00a07cd8 - (unsigned int)iVar14) * 2u);
  uVar10 = (unsigned int)(iVar14 >> 0x10);
  iVar17 = (int)(((unsigned int)DAT_00a07cd4 - (unsigned int)iVar17) * 2u);
  uVar4 = (unsigned int)(iVar15 >> 0x10);
  uVar12 = (unsigned int)(iVar17 >> 0x10);
  if ((uVar4 & 0x8000) != 0) {
    uVar4 = (unsigned int)(unsigned short)-(short)((unsigned int)iVar15 >> 0x10);
  }
  if ((uVar10 & 0x8000) != 0) {
    uVar10 = (unsigned int)(unsigned short)-(short)((unsigned int)iVar14 >> 0x10);
  }
  if ((uVar12 & 0x8000) != 0) {
    uVar12 = (unsigned int)(unsigned short)-(short)((unsigned int)iVar17 >> 0x10);
  }
  do {
    do {
      iStack_8 = iVar17;
      iVar5 = iVar15;
      iVar16 = iVar14;
      uVar4 = uVar4 >> 1 & 0x7fff;
      iVar15 = iVar5 >> 1;
      iVar14 = iVar16 >> 1;
      iVar17 = iStack_8 >> 1;
      uVar8 = (unsigned short)uVar10 >> 1;
      uVar10 = (unsigned int)uVar8;
      uVar11 = (unsigned short)uVar12 >> 1;
      uVar12 = (unsigned int)uVar11;
    } while ((short)uVar4 != 0);
  } while ((uVar8 != 0) || (uVar11 != 0));
  sVar13 = (short)(iVar16 >> 2);
  sVar9 = (short)(iVar5 >> 2);
  sStack_8 = (short)(iStack_8 >> 2);
  if (DAT_009e9683[iVar2] == 0) {
    LD(tI, DAT_009e9670 + iVar2);
    iVar17 = (int)((unsigned int)tI * 0x23u + (unsigned int)DAT_009a1fe8);
    LD(pS, iVar17 + 0x1f);
    LD(tB, pS + 0x92);
    if (tB != 0) {
      LD(tS1, iVar17 + 0x12);
      LD(tS2, iVar17 + 0x10);
      FUN_00427bd0(tS1, tS2, iVar17);
      LD(tI, DAT_009e9670 + iVar2);
      iVar17 = (int)((unsigned int)tI * 0x23u + (unsigned int)DAT_009a1fe8);
      LD(tS1, iVar17 + 0x14);
      FUN_00427d30(tS1, 0, iVar17);
    }
    LD(tI, DAT_009e9670 + iVar2);
    iVar17 = (int)((unsigned int)tI * 0x23u);
    LD(pS, iVar17 + 0x1f + DAT_009a1fe8);
    iVar15 = (int)sStack_8;
    iVar16 = (int)sVar9;
    iVar14 = (int)sVar13;
    LD(tS1, pS + 0x99);
    LD(tS2, pS + 0x9b);
    LD(tS3, pS + 0x9d);
    iVar17 = (int)((unsigned int)iVar16 * (unsigned int)(int)tS1 +
                   (unsigned int)iVar14 * (unsigned int)(int)tS2 +
                   (unsigned int)iVar15 * (unsigned int)(int)tS3);
    if (0x3fffffff < iVar17) {
      iVar17 = 0x3fffffff;
    }
    if (iVar17 < -0x3fffffff) {
      iVar17 = -0x3fff0000;
    }
    LD(tS1, pS + 0x93);
    LD(tS2, pS + 0x95);
    LD(tS3, pS + 0x97);
    iVar5 = (int)((unsigned int)iVar16 * (unsigned int)(int)tS1 +
                  (unsigned int)iVar14 * (unsigned int)(int)tS2 +
                  (unsigned int)iVar15 * (unsigned int)(int)tS3);
    if (0x3fffffff < iVar5) {
      iVar5 = 0x3fffffff;
    }
    if (iVar5 < -0x3fffffff) {
      iVar5 = -0x3fff0000;
    }
    LD(tS1, pS + 0x9f);
    LD(tS2, pS + 0xa1);
    LD(tS3, pS + 0xa3);
    iVar14 = (int)((unsigned int)iVar16 * (unsigned int)(int)tS1 +
                   (unsigned int)iVar14 * (unsigned int)(int)tS2 +
                   (unsigned int)iVar15 * (unsigned int)(int)tS3);
    if (0x3fffffff < iVar14) {
      iVar14 = 0x3fffffff;
    }
    if (iVar14 < -0x3fffffff) {
      iVar14 = -0x3fff0000;
    }
    FUN_0046a8d0(iVar17 >> 0xf, iVar5 >> 0xf, iVar14 >> 0xf);
    LD(tI, DAT_009e9670 + iVar2);
    iVar17 = (int)((unsigned int)tI * 0x23u + (unsigned int)DAT_009a1fe8);
    LD(sVar13, iVar17 + 0x10);
    LD(sVar9, iVar17 + 0x12);
    LD(sVar3, iVar17 + 0x14);
  }
  else {
    LD(tS1, DAT_009e9a6f + iVar2);
    LD(tS2, DAT_009e9a6b + iVar2);
    LD(tS3, DAT_009e9a6d + iVar2);
    FUN_00427940(tS1, tS2, tS3, 0, 0, 0, 0);
    iVar14 = (int)sStack_8;
    iVar15 = (int)sVar13;
    iVar16 = (int)sVar9;
    iVar17 = (int)((unsigned int)iVar16 * (unsigned int)DAT_00a004a8 +
                   (unsigned int)iVar15 * (unsigned int)DAT_00a004cc +
                   (unsigned int)iVar14 * (unsigned int)DAT_00a00488);
    if (0x3fffffff < iVar17) {
      iVar17 = 0x3fffffff;
    }
    if (iVar17 < -0x3fffffff) {
      iVar17 = -0x3fff0000;
    }
    iVar5 = (int)((unsigned int)iVar16 * (unsigned int)DAT_00a004b0 +
                  (unsigned int)iVar15 * (unsigned int)DAT_009fe7f4 +
                  (unsigned int)iVar14 * (unsigned int)DAT_00a00490);
    if (0x3fffffff < iVar5) {
      iVar5 = 0x3fffffff;
    }
    if (iVar5 < -0x3fffffff) {
      iVar5 = -0x3fff0000;
    }
    iVar14 = (int)((unsigned int)iVar16 * (unsigned int)DAT_00a004a4 +
                   (unsigned int)iVar15 * (unsigned int)DAT_00a004b8 +
                   (unsigned int)iVar14 * (unsigned int)DAT_00a00480);
    if (0x3fffffff < iVar14) {
      iVar14 = 0x3fffffff;
    }
    if (iVar14 < -0x3fffffff) {
      iVar14 = -0x3fff0000;
    }
    FUN_0046a8d0(iVar17 >> 0xf, iVar5 >> 0xf, iVar14 >> 0xf);
    LD(sVar13, DAT_009e9a6d + iVar2);
    LD(sVar9, DAT_009e9a6b + iVar2);
    LD(sVar3, DAT_009e9a6f + iVar2);
  }
  FUN_00427940(sVar3, sVar9, sVar13, 0, 0x4000 - DAT_00a080f0, DAT_009a8070, 0);
  iVar14 = DAT_009fe7f4;
  iVar15 = (int)((param_1 & 0xffff) * 0x23u + (unsigned int)DAT_009a1fe8);
  LD(iVar17, iVar15 + 0x1f);
  if (iVar17 == 0 || (LD(iVar17, iVar17 + 0xa9), iVar17 == 0)) {
    LD(tB, iVar15 + 3);
    LD(iVar17, DAT_00521f1c + (unsigned int)tB * 0x1a);
  }
  else {
    LD(tB, iVar17 + 4);
    iVar17 = (int)((unsigned int)tB * 0x101u);
    LD(sVar13, DAT_0051c65b + iVar17);
    LD(sVar9, DAT_0051c65f + iVar17);
    LD(tS1, DAT_0051c65d + iVar17);
    if ((sVar9 < sVar13) || (tS1 < sVar13)) {
      if ((sVar13 < sVar9) || (tS1 < sVar9)) {
        iVar15 = (int)sVar13;
        iVar16 = (int)sVar9;
      }
      else {
        iVar15 = (int)sVar13;
        iVar16 = (int)tS1;
      }
    }
    else {
      iVar16 = (int)tS1;
      iVar15 = (int)sVar9;
    }
    tB = (&DAT_0051c659[0])[iVar17];
    iVar17 = (int)((((unsigned int)iVar15 + (unsigned int)iVar16) >> 1) << (tB & 0x1f));
  }
  if (param_2 == 0) {
    uVar4 = 0x3c;
    LD(tS1, DAT_009e9bf6 + iVar2);
    if (tS1 != 0x13) {
      uVar4 = (unsigned int)(unsigned short)((-(unsigned short)(tS1 == 0x112) & 0xffd4) + 0x90);
    }
  }
  else {
    LD(tU, DAT_00a08cbe + (unsigned int)DAT_00a08374 * 0xc);
    uVar4 = (unsigned int)tU;
  }
  LD(sVar13, DAT_009e9bf6 + iVar2);
  if ((sVar13 == 0x101) || (sVar6 = 8, sVar13 == 0x112)) {
    sVar6 = 9;
  }
  sVar9 = 0;
  uVar12 = (unsigned int)iVar17 << sVar6;
  if (uVar4 == 0) {
    RaiseException(0xC0000094UL, 0, 0, 0);
  }
  for (uVar4 = uVar12 / uVar4; 0x3fff < uVar4; uVar4 = uVar4 >> 1) {
    sVar9 = sVar9 + 1;
  }
  if (sVar13 == 0x101) {
    uVar4 = (unsigned int)(unsigned short)((unsigned short)uVar4 + ((unsigned short)uVar4 >> 2));
  }
  uVar4 = uVar4 & 0xffff;
  prod = (long long)(int)uVar4 * (long long)DAT_00a004b0;
  outv = ((unsigned int)prod >> 0xf) | ((unsigned int)((unsigned long long)prod >> 0x20) << 0x11);
  ST(DAT_009e9a5b + iVar2, outv);
  iVar17 = DAT_00a00490;
  prod = (long long)(int)uVar4 * (long long)iVar14;
  outv = ((unsigned int)prod >> 0xf) | ((unsigned int)((unsigned long long)prod >> 0x20) << 0x11);
  ST(DAT_009e9a5f + iVar2, outv);
  prod = (long long)(int)uVar4 * (long long)iVar17;
  outv = ((unsigned int)prod >> 0xf) | ((unsigned int)((unsigned long long)prod >> 0x20) << 0x11);
  ST(DAT_009e9a63 + iVar2, outv);
  if (sVar9 != 0) {
    bVar7 = (unsigned char)sVar9;
    LD(tI, DAT_009e9a5b + iVar2);
    tI = (int)((unsigned int)tI << (bVar7 & 0x1f));
    ST(DAT_009e9a5b + iVar2, tI);
    LD(tI, DAT_009e9a5f + iVar2);
    tI = (int)((unsigned int)tI << (bVar7 & 0x1f));
    ST(DAT_009e9a5f + iVar2, tI);
    LD(tI, DAT_009e9a63 + iVar2);
    tI = (int)((unsigned int)tI << (bVar7 & 0x1f));
    ST(DAT_009e9a63 + iVar2, tI);
  }
  LD(tI, DAT_009e9a5b + iVar2);
  tI = (int)((unsigned int)DAT_00a07cdc - (unsigned int)tI);
  ST(DAT_009e9a5b + iVar2, tI);
  LD(tI, DAT_009e9a5f + iVar2);
  tI = (int)((unsigned int)DAT_00a07cd8 - (unsigned int)tI);
  ST(DAT_009e9a5f + iVar2, tI);
  LD(tI, DAT_009e9a63 + iVar2);
  tI = (int)((unsigned int)DAT_00a07cd4 - (unsigned int)tI);
  ST(DAT_009e9a63 + iVar2, tI);
  return;
}
