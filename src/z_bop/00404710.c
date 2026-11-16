/* SPDX-License-Identifier: MIT */
/* z_bop 00404710 */
#define U8(a)  (*(unsigned char *)(unsigned int)(a))
#define S8(a)  (*(signed char *)(unsigned int)(a))
#define R16(a) ({ unsigned short _r16; __builtin_memcpy(&_r16, (const void *)(unsigned int)(a), 2); _r16; })
#define R32(a) ({ unsigned int _r32; __builtin_memcpy(&_r32, (const void *)(unsigned int)(a), 4); _r32; })
#define W16(a, v) ({ unsigned short _w16 = (unsigned short)(v); __builtin_memcpy((void *)(unsigned int)(a), &_w16, 2); (void)0; })
#define W32(a, v) ({ unsigned int _w32 = (unsigned int)(v); __builtin_memcpy((void *)(unsigned int)(a), &_w32, 4); (void)0; })

extern unsigned short DAT_009ec5f4;
extern unsigned short DAT_009ec5fe;
extern unsigned short DAT_00a07cec;
extern unsigned int   DAT_00a07ce8;
extern int            DAT_009a1fe0;
extern unsigned int   DAT_009a1fe8;
extern unsigned int   DAT_009a1ff8;
extern unsigned int   DAT_009a8c00;
extern int            DAT_009d1308;
extern unsigned char  DAT_009d6954;
extern unsigned short DAT_009d8110;
extern int            DAT_009ecc34;
extern unsigned char  DAT_009ed22d;
extern unsigned int   DAT_00a08104;
extern unsigned char  DAT_009d8c88[];
extern unsigned char  DAT_009d8c89[];
extern unsigned char  DAT_009fd458[];
extern unsigned char  DAT_0051c5aa[];
extern int            DAT_00a00870[];
extern int            DAT_00a00874[];
extern unsigned char  DAT_009e9681[];
extern unsigned char  DAT_009e968a[];
extern unsigned char  DAT_009e9696[];
extern unsigned char  DAT_009e9697[];
extern unsigned char  DAT_009e9698[];
extern unsigned char  DAT_009e9699[];
extern unsigned char  DAT_009e9c21[];

unsigned int FUN_004269e0(unsigned int param_1);
void FUN_004034e0(unsigned int param_1, unsigned int param_2);
void FUN_004836e0(unsigned int param_1, unsigned int param_2, unsigned int param_3);
unsigned long long FUN_00482640(int param_1, int param_2, int param_3, int param_4, int param_5);
void FUN_00450650(int param_1, int param_2);
short FUN_00451930(void);
void FUN_0042edc0(int param_1, int param_2);
void FUN_004a8cc0(unsigned int param_1, unsigned int param_2);
int FUN_00405820(unsigned int param_1);
int FUN_004adc40(unsigned int param_1);
int FUN_004ae2a0(unsigned int param_1, int param_2);
void FUN_00405790(unsigned int param_1, unsigned int param_2, int param_3);
void FUN_0042e4d0(int param_1, int param_2, int param_3);
void FUN_00405ac0(unsigned int param_1, unsigned int param_2);
void FUN_004a7900(unsigned int param_1, unsigned short param_2, unsigned short param_3);
void FUN_00446c60(unsigned short param_1);

void __cdecl FUN_00404710(int param_1)
{
  signed char cVar2;
  unsigned char bVar3;
  unsigned short uVar4;
  int iVar5;
  unsigned int uVar6;
  unsigned int uVar7;
  int iVar8;
  short sVar10;
  int iVar11;
  int iVar12 = 0;
  char *pcVar13;
  unsigned int uVar14;
  short sVar15;
  unsigned int uVar16;
  unsigned short uVar18;
  int iVar19;
  _Bool bVar20;
  _Bool bVar21;
  unsigned long long uVar22;
  unsigned short local_e;
  unsigned short bStack_c = 0;

  bVar21 = (DAT_009ec5f4 == 0);
  if (bVar21) {
    DAT_009ec5f4 = 0xec;
  }
  uVar16 = (unsigned int)DAT_00a07cec;
  if ((int)uVar16 < DAT_009a1fe0) {
    do {
      iVar5 = uVar16 * 0x23 + DAT_009a1fe8;
      if ((U8(iVar5 + 3) != 0) && (pcVar13 = (char *)R32(iVar5 + 0x1f), U8(pcVar13) == 0)) {
        iVar5 = R32(pcVar13 + 0xa9);
        uVar18 = 0;
        do {
          uVar4 = uVar18 + 1;
          W32(iVar5 + 0x15 + (unsigned int)uVar18 * 4, 0);
          uVar18 = uVar4;
        } while (uVar4 < 5);
      }
      uVar16 = (unsigned int)(unsigned short)((short)uVar16 + 1);
    } while ((int)uVar16 < DAT_009a1fe0);
  }

  local_e = DAT_00a07cec;
  if ((int)(unsigned int)local_e < DAT_009a1fe0) {
    do {
      uVar6 = (unsigned int)local_e;
      iVar5 = uVar6 * 0x23;
      iVar11 = DAT_009a1fe8 + iVar5;
      uVar16 = U8(iVar11 + 3);
      if ((U8(iVar11 + 3) != 0) && (pcVar13 = (char *)R32(iVar11 + 0x1f), U8(pcVar13) == 0)) {
        if ((int)R32(iVar11 + 0x1b) == -1) {
          if (!bVar21) goto LAB_004054a5;
          uVar6 = R32(pcVar13 + 0xa9);
          DAT_00a08104 = uVar6;
          if (U8(iVar11 + 2) == 0) {
            U8(uVar6 + 0x219) = 2;
            U8(DAT_00a08104 + 0x21c) = 2;
            if ((U8(DAT_00a08104 + 0x1e3) & 1) != 0) {
              uVar16 = (unsigned int)U8(DAT_009a1fe8 + 0x16 + iVar5);
              uVar18 = (unsigned short)DAT_009d8c89[uVar16 * 0x566];
              uVar16 = (unsigned int)uVar18;
              if (U8(uVar6 + 0x5e) == ' ') {
                if (uVar18 == 5) {
                  U8(DAT_00a08104 + 0x219) = 0;
                }
                else if ((uVar18 == 4) || (uVar18 == 3)) {
                  U8(DAT_00a08104 + 0x219) = 1;
                }
                else {
                  U8(DAT_00a08104 + 0x219) = 2;
                }
                U8(DAT_00a08104 + 0x21c) = 3;
              }
              iVar11 = FUN_00405820((unsigned int)local_e);
              if ((int)R32(DAT_00a08104 + 0x211) < iVar11) {
                if (U8(DAT_00a08104 + 0x21c) == 2) {
                  U8(DAT_00a08104 + 0x21c) = 4;
                }
                if ((int)R32(DAT_00a08104 + 0x211) < 1) {
                  if (U8(DAT_00a08104 + 0x219) == 2) {
                    U8(DAT_00a08104 + 0x219) = 4;
                  }
                  if (uVar18 < 2) {
                    bStack_c = 0xfa;
                  }
                  else {
                    uVar18 = (unsigned short)((((unsigned short)-(unsigned short)(uVar18 < 3)) & 0xdae4) + 10000);
                    bStack_c = uVar18;
                  }
                }
                else {
                  if (1 < uVar18) {
                    if (uVar18 == 5) {
                      uVar4 = 1;
                    }
                    else {
                      uVar4 = 3;
                      if (uVar18 != 4) {
                        uVar4 = (unsigned short)((((unsigned short)-(unsigned short)(uVar18 == 3)) & 0xfff8) + 0xf);
                      }
                    }
                    uVar18 = (unsigned short)(((unsigned short)-(unsigned short)((DAT_009ed22d & uVar4) == uVar4)) & 100);
                    bStack_c = uVar18;
                  }
                  else {
                    bStack_c = 0;
                  }
                }
                sVar10 = 0;
                uVar16 = 0;
                if (U8(DAT_00a08104 + 0x21d) != 0) {
                  do {
                    cVar2 = S8(DAT_00a08104 + 0x371 + uVar16 * 4);
                    if (0 < cVar2) {
                      sVar10 = (short)(sVar10 + cVar2);
                    }
                    uVar18 = (unsigned short)((short)uVar16 + 1);
                    uVar16 = (unsigned int)uVar18;
                  } while (uVar18 < U8(DAT_00a08104 + 0x21d));
                }
                uVar6 = 0;
                uVar16 = ((unsigned int)(-(unsigned int)(U8(DAT_009a1fe8 + 3 + iVar5) == 0xc)) & 0x1c) + 4;
                for (uVar18 = 0; (sVar10 != 0 && (uVar18 < bStack_c)); uVar18 = uVar18 + 1) {
                  pcVar13 = (char *)(DAT_00a08104 + 0x371 + uVar6 * 4);
                  cVar2 = (signed char)*pcVar13;
                  if (0 < cVar2) {
                    sVar10 = (short)(sVar10 - 1);
                    *pcVar13 = (char)(cVar2 - 1);
                    W32(DAT_00a08104 + 0x211, R32(DAT_00a08104 + 0x211) + uVar16);
                    if (iVar11 <= (int)R32(DAT_00a08104 + 0x211)) {
                      sVar10 = 0;
                    }
                  }
                  uVar4 = (unsigned short)((short)uVar6 + 1);
                  uVar6 = (unsigned int)uVar4;
                  if (U8(DAT_00a08104 + 0x21d) <= uVar4) {
                    uVar6 = 0;
                  }
                }
              }
            }
            pcVar13 = (char *)(DAT_00a08104 + 0x21c);
            if (U8(pcVar13) == 2) {
              sVar10 = 0;
              uVar18 = 0;
              uVar16 = uVar16 & 0xffff0000;
              if (U8(DAT_00a08104 + 0x21d) != 0) {
                do {
                  iVar11 = DAT_00a08104 + (uVar16 & 0xffff) * 4;
                  if (U8(iVar11 + 0x370) != 0) {
                    sVar10 = (short)(sVar10 + S8(iVar11 + 0x371));
                    uVar18 = uVar18 + 1;
                  }
                  uVar4 = (unsigned short)((short)uVar16 + 1);
                  uVar16 = (uVar16 & 0xffff0000) | uVar4;
                } while (uVar4 < U8(DAT_00a08104 + 0x21d));
              }
              if (uVar18 != 0) {
                sVar10 = (short)((int)sVar10 / (int)(unsigned int)uVar18);
                if (sVar10 < 0x20) {
                  sVar10 = 0x14;
                  *pcVar13 = 4;
                  goto LAB_00405184;
                }
                *pcVar13 = (char)((sVar10 < 0x60) + 2);
              }
            }
            sVar10 = 0x14;
          }
          else {
            U8(uVar6 + 0x219) = 2;
            sVar10 = 0;
            U8(DAT_00a08104 + 0x21c) = 2;
            if ((DAT_009d6954 == 2) && (U8(DAT_009a1fe8 + iVar5 + 2) == 4)) {
              if (R32(DAT_00a08104 + 0xc3) != 0) {
                bVar3 = U8(DAT_009a1fe8 + iVar5 + 3);
                if ((((bVar3 == 0x33) || (bVar3 == 0x34)) || (bVar3 == 0x35)) || (bVar3 == 0x36)) {
                  sVar10 = 0;
                  if (bVar3 < 0x49) {
                    iVar11 = DAT_00a00870[(unsigned int)bVar3 * 0x65];
                  }
                  else {
                    iVar11 = FUN_004adc40((unsigned int)bVar3);
                  }
                  iVar19 = 0;
                  if (0 < iVar11) {
                    do {
                      bVar3 = U8(DAT_009a1fe8 + 3 + iVar5);
                      uVar6 = (unsigned int)bVar3;
                      if (bVar3 < 0x49) {
                        if (iVar19 < 0) {
                          iVar8 = 0;
                        }
                        else {
                          iVar8 = iVar19;
                          if (DAT_00a00870[uVar6 * 0x65] <= iVar19) {
                            iVar8 = DAT_00a00870[uVar6 * 0x65] + -1;
                          }
                          iVar8 = DAT_00a00874[uVar6 * 0x65 + iVar8];
                        }
                      }
                      else {
                        iVar8 = FUN_004ae2a0(uVar6, iVar19);
                      }
                      if ((iVar8 == 8) && (U8(DAT_00a08104 + 0x33c + iVar19) != 0)) {
                        sVar10 = (short)(sVar10 + 1);
                      }
                      iVar19 = iVar19 + 1;
                    } while (iVar19 < iVar11);
                  }
                }
                else {
                  sVar10 = 1;
                }
              }
              U8(DAT_00a08104 + 0x219) = 3;
            }
            sVar10 = (short)(sVar10 * 5);
          }
        }
        else {
          DAT_00a08104 = R32(pcVar13 + 0xa9);
          iVar19 = (int)R32(iVar11 + 0x1b);
          iVar8 = (int)((unsigned int)iVar19 * 0x5bd);
          if (DAT_009e9698[iVar8] != 0) {
            uVar7 = FUN_004269e0((unsigned int)U8(iVar11 + 3));
            uVar18 = R16(&DAT_009e968a[iVar8]);
            if ((uVar18 == 0xffff) ||
                ((unsigned short)((unsigned short)U8(DAT_00a08104 +
                                    (unsigned int)DAT_0051c5aa[(unsigned int)DAT_009e9697[iVar8] + (uVar7 & 0xffff) * 0x101] * 4 + 0x376) +
                                  (unsigned short)U8(DAT_00a08104 + 0x372 +
                                    (unsigned int)DAT_0051c5aa[(unsigned int)DAT_009e9697[iVar8] + (uVar7 & 0xffff) * 0x101] * 4)) == 0)) {
              DAT_009e9696[iVar8] = 0;
              W16(DAT_00a08104 + 0x23b, 0);
            }
            else {
              uVar7 = (unsigned int)uVar18;
              iVar11 = (int)(uVar7 * 0x23);
              cVar2 = (signed char)U8(DAT_009a1fe8 + 2 + iVar11);
              if ((cVar2 == 4) || (cVar2 == 5)) {
                FUN_004836e0(uVar6, (unsigned int)uVar18, (unsigned int)R16(&DAT_009e9699[iVar8]));
              }
              else {
                FUN_004034e0(uVar6, uVar7);
              }
              uVar14 = 0x18dad;
              if (((int)uVar7 < DAT_009a1fe0) &&
                  (((cVar2 = (signed char)U8(DAT_009a1fe8 + 2 + iVar11), iVar12 = iVar11, cVar2 == 4)
                    || (cVar2 == 5)) || (cVar2 == 3))) {
                uVar14 = 0x3ba6c;
              }
              if ((DAT_009a8c00 < uVar14) &&
                  (uVar22 = FUN_00482640(iVar12, (int)uVar14, (int)uVar18, 0, iVar19), (short)uVar22 != 0)) {
                W16(DAT_00a08104 + 0x23b, R16(DAT_00a08104 + 0x23b) + DAT_009ec5fe);
                if ((int)uVar7 < DAT_009a1fe0) {
                  iVar11 = (int)R32(R32(DAT_009a1fe8 + 0x1f + iVar11) + 0xa9);
                  cVar2 = (signed char)U8(iVar11 + 0x246);
                  if ((cVar2 == 1) && (R16(iVar11 + 0x248) != 0)) {
                    W16(DAT_00a08104 + 0x23b, R16(DAT_00a08104 + 0x23b) - (DAT_009ec5fe >> 1));
                  }
                }
                uVar7 = FUN_004269e0(0xc);
                uVar14 = FUN_004269e0((unsigned int)U8(DAT_009a1fe8 + 3 + iVar5));
                if ((int)(short)R16(DAT_00a08104 + 0x23b) <
                    (int)(unsigned int)(unsigned short)((((unsigned short)-(unsigned short)((short)uVar14 == (short)uVar7)) & 0xfe9e) + 0x2c4)) {
                  DAT_009e9696[iVar8] = 1;
                }
                else {
                  DAT_009e9696[iVar8] = 2;
                }
              }
              else {
                if (0 < (short)R16(DAT_00a08104 + 0x23b)) {
                  uVar4 = (unsigned short)((R16(DAT_00a08104 + 0x23b) - (DAT_009ec5fe >> 1)) - DAT_009ec5fe);
                  W16(DAT_00a08104 + 0x23b, uVar4);
                  if ((short)R16(DAT_00a08104 + 0x23b) < 0) {
                    W16(DAT_00a08104 + 0x23b, 0);
                  }
                }
                DAT_009e9696[iVar8] = 0;
              }
            }
          }
          uVar18 = 0xffff;
          if (((((U8(DAT_00a08104 + 0x1e6) & 1) == 0) ||
                (U8(DAT_00a08104 + 0x241) == 0)) ||
               (U8(DAT_00a08104 + 0x23d) == 0)) || (DAT_009e9681[iVar8] != 0)) {
            if ((iVar19 == DAT_009ecc34) &&
                (DAT_009d8110 = 0xffff, (U8(DAT_00a08104 + 0x1e4) & 1) != 0)) {
              iVar11 = 0;
              goto LAB_00404c6c;
            }
          }
          else {
            if (R32(&DAT_009e9c21[iVar8]) == 0) {
              W32(&DAT_009e9c21[iVar8], 0x3b);
              sVar10 = (short)(R16(DAT_00a08104 + 0x23f) + -0x7d);
              if (sVar10 < 0) {
                sVar10 = 0;
              }
              W16(DAT_00a08104 + 0x23f, sVar10);
              if ((sVar10 == 0) && (U8(DAT_00a08104 + 0x241) != 0)) {
                U8(DAT_00a08104 + 0x241) = 0;
                W16(DAT_00a08104 + 0x242, 0);
                if (iVar19 == DAT_009ecc34) {
                  FUN_00450650((int)U8(DAT_00a08104 + 0x23d) + 0xf2, DAT_009ecc34);
                }
              }
            }
            uVar4 = R16(&DAT_009e968a[iVar8]);
            if ((uVar4 != 0xffff) && ((int)(unsigned int)uVar4 < DAT_009a1fe0)) {
              iVar11 = (int)R32((unsigned int)uVar4 * 0x23 + 0x1f + DAT_009a1fe8);
              iVar8 = (int)R32(iVar11 + 0xa9);
              cVar2 = (signed char)U8(iVar8 + 7);
              if ((cVar2 == 0) &&
                  ((uVar22 = FUN_00482640(iVar11, iVar8, (int)uVar4, 0, iVar19), (short)uVar22 != 0) &&
                   (DAT_009a1ff8 < 0x20000))) {
                uVar18 = uVar4;
              }
            }
            if (uVar18 != 0xffff) {
              iVar11 = (int)R32(R32((unsigned int)uVar18 * 0x23 + 0x1f + DAT_009a1fe8) + 0xa9);
              bVar3 = U8(DAT_00a08104 + 0x23d);
              if ((bVar3 == 1) || (bVar3 == 2)) {
                if ((U8(iVar11 + 0x246) == 1) && (R16(iVar11 + 0x248) != 0)) {
                  if ((iVar19 == DAT_009ecc34) && (sVar10 = FUN_00451930(), sVar10 != 0x100)) {
                    FUN_00450650(0x100, iVar19);
                  }
                  uVar18 = 0xffff;
                }
                else {
                  W32(iVar11 + 0x15 + (unsigned int)bVar3 * 4,
                      R32(iVar11 + 0x15 + (unsigned int)bVar3 * 4) + (unsigned int)R16(DAT_00a08104 + 0x242));
                }
              }
            }
            if (iVar19 == DAT_009ecc34) {
              iVar11 = 1;
              DAT_009d8110 = uVar18;
LAB_00404c6c:
              FUN_0042edc0(iVar11, iVar19);
            }
          }
          sVar10 = 0x14;
          uVar18 = DAT_00a07cec;
          if ((int)(unsigned int)DAT_00a07cec < DAT_009a1fe0) {
            do {
              iVar11 = (int)((unsigned int)uVar18 * 0x23 + DAT_009a1fe8);
              cVar2 = (signed char)U8(iVar11 + 3);
              if ((cVar2 != 0) &&
                  ((U8(iVar11 + 2) == 4 ||
                    ((U8(iVar11 + 2) == 5 && ((cVar2 == 'E' || (cVar2 == '[')))))))) {
                uVar7 = (unsigned int)U8(DAT_009a1fe8 + 0x16 + iVar5);
                uVar14 = (unsigned int)U8(R32(iVar11 + 0x1f) + 0x89);
                if ((unsigned char)DAT_009d8c88[uVar7 * 0x566] == uVar14) {
                  bVar20 = 0;
                }
                else {
                  bVar20 = DAT_009fd458[(unsigned int)DAT_009d8c88[uVar7 * 0x566] + uVar14 * 0x1e5] == 0;
                }
                if (bVar20) {
                  FUN_004a8cc0(uVar6, (unsigned int)uVar18);
                }
              }
              uVar18 = uVar18 + 1;
            } while ((int)(unsigned int)uVar18 < DAT_009a1fe0);
          }
        }
LAB_00405184:
        if (bVar21) {
          DAT_00a08104 = R32(R32(iVar5 + DAT_009a1fe8 + 0x1f) + 0xa9);
          cVar2 = (signed char)U8(iVar5 + DAT_009a1fe8 + 3);
          if (cVar2 == 2) {
            sVar10 = (short)((unsigned int)(int)sVar10 * 2);
          }
          if ((((U8(DAT_00a08104 + 0x1e5) & 1) != 0) && (sVar10 != 0)) &&
              (sVar10 = (short)((unsigned int)((int)U8(DAT_00a08104 + 0x219) - 2) * (unsigned int)(int)sVar10), sVar10 != 0)) {
            if (U8(DAT_00a08104 + 0x21a) == 0) {
              uVar6 = 0;
            }
            else {
              if (U8(DAT_00a08104 + 0x21a) != 2) {
                sVar10 = (short)((int)sVar10 / 2);
                FUN_00405790((unsigned int)local_e, 0, (int)sVar10);
              }
              uVar6 = 1;
            }
            FUN_00405790((unsigned int)local_e, uVar6, (int)sVar10);
          }
          if (((U8(DAT_00a08104 + 0x1e5) & 0x10) != 0) &&
              (uVar6 = 0, U8(DAT_00a08104 + 0x21d) != 0)) {
            do {
              iVar19 = (int)(uVar6 * 4);
              iVar11 = (int)(DAT_00a08104 + uVar6 * 4);
              if ((U8(iVar11 + 0x370) != 0) && (U8(iVar11 + 0x370) != 2)) {
                sVar15 = (short)U8(DAT_00a08104 + 0x21c);
                sVar10 = (short)(sVar15 + -2);
                if (R16(DAT_00a08104 + 0xbf) == 0) {
                  sVar10 = (short)(sVar15 + -6);
                }
                cVar2 = (signed char)U8(DAT_009a1fe8 + 3 + iVar5);
                if ((cVar2 == 5) || (cVar2 == 7)) {
                  sVar10 = (short)((unsigned int)(int)sVar10 * 3);
                }
                else {
                  sVar10 = (short)((unsigned int)(int)sVar10 * 2);
                }
                U8(iVar11 + 0x371) = (unsigned char)(U8(iVar11 + 0x371) + (char)sVar10);
                if ((sVar10 < 0) && (S8(iVar19 + 0x371 + DAT_00a08104) < 0)) {
                  U8(iVar19 + 0x371 + DAT_00a08104) = 0;
                }
                if ((0 < sVar10) && (S8(iVar19 + 0x371 + DAT_00a08104) < 0)) {
                  U8(iVar19 + 0x371 + DAT_00a08104) = 0x7f;
                }
              }
              uVar6 = (unsigned int)(unsigned short)((short)uVar6 + 1);
            } while (uVar6 < U8(DAT_00a08104 + 0x21d));
          }
          if (R16(DAT_00a08104 + 0xbf) == 0) {
            bVar20 = 0;
            uVar6 = 0;
            if (U8(DAT_00a08104 + 0x21d) != 0) {
              do {
                if (0 < S8(DAT_00a08104 + 0x371 + uVar6 * 4)) {
                  bVar20 = 1;
                }
                uVar18 = (unsigned short)((short)uVar6 + 1);
                uVar6 = (unsigned int)uVar18;
              } while (uVar18 < U8(DAT_00a08104 + 0x21d));
            }
            if (!bVar20) {
              W16(DAT_00a08104 + 0xbf, 0xffff);
              FUN_00450650(0x11d, DAT_009ecc34);
              FUN_0042e4d0(0x5e, -1, DAT_009ecc34);
            }
          }
          if ((U8(DAT_00a08104 + 0x1e6) & 1) != 0) {
            sVar10 = (short)((unsigned int)((int)U8(DAT_00a08104 + 0x23e) - 2) * 0x7d + (unsigned int)R16(DAT_00a08104 + 0x23f));
            if (sVar10 < 0) {
              sVar10 = 0;
            }
            if (9999 < sVar10) {
              sVar10 = 9999;
            }
            W16(DAT_00a08104 + 0x23f, sVar10);
            if ((sVar10 == 0) && (U8(DAT_00a08104 + 0x241) != 0)) {
              U8(DAT_00a08104 + 0x241) = 0;
              W16(DAT_00a08104 + 0x242, 0);
              if ((int)R32(DAT_009a1fe8 + 0x1b + iVar5) == DAT_009ecc34) {
                FUN_00450650((int)U8(DAT_00a08104 + 0x23d) + 0xf2, DAT_009ecc34);
              }
            }
          }
          if ((((R16(DAT_00a08104 + 0x248) != 0) &&
                (W16(DAT_00a08104 + 0x248, R16(DAT_00a08104 + 0x248) - 1),
                 U8(DAT_00a08104 + 0x246) == 1)) &&
               (R16(DAT_00a08104 + 0x248) == 0)) &&
              (iVar5 = (int)R32(DAT_009a1fe8 + 0x1b + iVar5), iVar5 != -1)) {
            FUN_00450650(0x170, iVar5);
          }
        }
      }
LAB_004054a5:
      local_e = local_e + 1;
    } while ((int)(unsigned int)local_e < DAT_009a1fe0);
  }

  local_e = DAT_00a07cec;
  if ((int)(unsigned int)local_e < DAT_009a1fe0) {
    do {
      iVar5 = (int)((unsigned int)local_e * 0x23);
      iVar11 = (int)(DAT_009a1fe8 + iVar5);
      if (((U8(iVar11 + 3) != 0) &&
           (pcVar13 = (char *)R32(iVar11 + 0x1f), U8(pcVar13) == 0)) &&
          (DAT_00a08104 = R32(pcVar13 + 0xa9), R16(DAT_00a08104 + 0x1e7) == 0)) {
        if ((R32(DAT_00a08104 + 0x1d) == 0) &&
            (uVar16 = 0, U8(DAT_00a08104 + 0x21b) != 0)) {
          do {
            sVar10 = (short)R16(DAT_00a08104 + 0x226 + uVar16 * 2);
            if (sVar10 != 0) {
              sVar10 = (short)(sVar10 - DAT_009ec5fe);
              if (sVar10 < 0) {
                sVar10 = 0;
              }
              W16(DAT_00a08104 + 0x226 + uVar16 * 2, sVar10);
            }
            if (((int)R32(DAT_009a1fe8 + 0x1b + iVar5) == -1) && (sVar10 < (short)DAT_009ec5fe) &&
                (U8(uVar16 + 0x220 + DAT_00a08104) != 0)) {
              if (((U8(DAT_00a08104 + 0x1e5) & 0x10) != 0) &&
                  (U8(DAT_00a08104 + 7) == 0)) {
                FUN_00405ac0((unsigned int)local_e, uVar16);
              }
              pcVar13 = (char *)(uVar16 + 0x222 + DAT_00a08104);
              *pcVar13 = (char)(*pcVar13 - 1);
              W16(DAT_00a08104 + 0x226 + uVar16 * 2, R16(DAT_00a08104 + 0x226 + uVar16 * 2) + DAT_009ec5fe * 2);
              W32(DAT_00a08104 + 0x22a + uVar16 * 4,
                  R32(DAT_00a08104 + 0x22a + uVar16 * 4) + (unsigned int)DAT_009ec5fe * 2);
              if (U8(uVar16 + DAT_00a08104 + 0x222) == 0) {
                U8(uVar16 + DAT_00a08104 + 0x220) = 0;
              }
            }
            uVar16 = (unsigned int)(unsigned short)((short)uVar16 + 1);
          } while (uVar16 < U8(DAT_00a08104 + 0x21b));
        }
        uVar16 = 0;
        if (U8(DAT_00a08104 + 0x21d) != 0) {
          do {
            if ((U8(DAT_00a08104 + 0x370 + uVar16 * 4) == 2) &&
                (uVar18 = R16(DAT_00a08104 + uVar16 * 4 + 0x3b2), uVar18 != 0xffff)) {
              FUN_004a7900((unsigned int)local_e, (unsigned short)uVar16, uVar18);
            }
            uVar16 = (unsigned int)(unsigned short)((unsigned short)uVar16 + 1);
          } while (uVar16 < U8(DAT_00a08104 + 0x21d));
        }
        uVar16 = 0;
        if (U8(DAT_00a08104 + 0x232) != 0) {
          do {
            pcVar13 = (char *)(DAT_00a08104 + 0x237 + uVar16 * 2);
            sVar10 = (short)R16(pcVar13);
            if (sVar10 != 0) {
              sVar10 = (short)(sVar10 - DAT_009ec5fe);
              if (sVar10 < 0) {
                sVar10 = 0;
              }
              W16(pcVar13, sVar10);
            }
            uVar16 = (unsigned int)(unsigned short)((short)uVar16 + 1);
          } while (uVar16 < U8(DAT_00a08104 + 0x232));
        }
      }
      local_e = local_e + 1;
    } while ((int)(unsigned int)local_e < DAT_009a1fe0);
  }

  uVar16 = DAT_00a07ce8 & 0xffff;
  if ((int)uVar16 < (int)(DAT_009d1308 + DAT_00a07ce8)) {
    do {
      iVar5 = (int)(uVar16 * 0x23 + DAT_009a1fe8);
      if ((U8(iVar5 + 3) != 0) && (U8(iVar5 + 2) == 8)) {
        FUN_00446c60((unsigned short)uVar16);
      }
      uVar18 = (unsigned short)uVar16 + 1;
      uVar16 = (unsigned int)uVar18;
    } while ((int)(unsigned int)uVar18 < (int)(DAT_009d1308 + DAT_00a07ce8));
  }
  return;
}
