/* SPDX-License-Identifier: MIT */
/* z_xvt 0040d2d0 */
#define R8(a)  (*(unsigned char *)(a))
#define R16(a) ((unsigned short)((unsigned int)R8(a) | ((unsigned int)R8((a) + 1) << 8)))
#define R32(a) ((int)((unsigned int)R8(a) | ((unsigned int)R8((a) + 1) << 8) | \
                      ((unsigned int)R8((a) + 2) << 16) | ((unsigned int)R8((a) + 3) << 24)))
#define W8(a, v)  (*(unsigned char *)(a) = (unsigned char)(v))
#define W16(a, v) do { unsigned char *wp_ = (unsigned char *)(a); unsigned int wv_ = (unsigned int)(v); \
                       wp_[0] = (unsigned char)wv_; wp_[1] = (unsigned char)(wv_ >> 8); } while (0)
#define W32(a, v) do { unsigned char *wp_ = (unsigned char *)(a); unsigned int wv_ = (unsigned int)(v); \
                       wp_[0] = (unsigned char)wv_; wp_[1] = (unsigned char)(wv_ >> 8); \
                       wp_[2] = (unsigned char)(wv_ >> 16); wp_[3] = (unsigned char)(wv_ >> 24); } while (0)
#define CONCAT22(h, l) (((unsigned int)(unsigned short)(h) << 16) | (unsigned short)(l))
#define ADD3(a, b, c) ((int)((unsigned int)(a) + (unsigned int)(b) + (unsigned int)(c)))

extern unsigned int _DAT_00732a3c;
extern int DAT_00732904, DAT_006d4988, DAT_006d4980, DAT_0071b71c;
extern int DAT_006d4974, DAT_006d4978, DAT_006d4970, DAT_0071e6c4, DAT_00732f38, DAT_00732fe8;
extern int DAT_0073294c, DAT_007329b0, DAT_007329b4, DAT_0070a180, DAT_0070acd8, DAT_00703ad8;
extern int DAT_006d9d38, DAT_0071ecf4, DAT_00732e48, _DAT_00732950, DAT_00732a38;
extern int DAT_0070ab18, DAT_0070ab14, DAT_0070ac20, DAT_00732f7c, DAT_00732f8c, DAT_00732f5c;
extern unsigned short DAT_00655a84, DAT_0071e6be, DAT_006db6de, DAT_0071f2d6, DAT_00732576;
extern unsigned char DAT_007092d9, DAT_007092d5, DAT_007092c1, DAT_006d4a02;
extern unsigned char DAT_0071b744[], DAT_0071bce9[], DAT_0071b75d[], DAT_0071b75f[], DAT_0071b74a[];
extern unsigned char DAT_0071b73c[], DAT_0071b7c1[], DAT_0071b7c3[], DAT_0071b776[], DAT_0071b777[], DAT_0071bcdd[];
extern unsigned char DAT_00709388[], DAT_006d4a7a[], DAT_006d4abe[], DAT_006d4aaa[], DAT_006d4ac8[];
extern unsigned char DAT_0070ad40[], DAT_0070aee5[], DAT_0070aee9[], DAT_0070ac4a[];
extern unsigned char DAT_0052686c[], DAT_0070ad8c[], DAT_0070ad8d[], DAT_0070ad8e[], DAT_0070ad8f[];
extern unsigned char DAT_0052b14b[], DAT_0052b14d[], DAT_0052b14f[], DAT_00709d0a[];
extern unsigned char DAT_00703ca0[], DAT_00525bf7[], DAT_0070ad44[], DAT_0070b215[];
extern char s_exithangarpln_00523c08[], s_outofhyperspacepln_00523bf4[], s_enterhangarpln_00523be4[];
extern char s_followhomeevadepln_00523bd0[], s_boardtogivepln_00523c18[];

void FUN_0041b530(unsigned int, unsigned int);
int FUN_004a1590(int, unsigned int, int, unsigned int, unsigned int, unsigned int);
void FUN_004a1d70(int, int);
void FUN_004a34c0(unsigned int, unsigned int, int);
void FUN_0042a230(int, unsigned int, int, short, unsigned short, short);
void FUN_0040cd90(unsigned char *, unsigned short);
void FUN_0040ed40(unsigned char *, unsigned short, unsigned int);
int FUN_0042b150(int);
unsigned int FUN_0040f7e0(unsigned short, unsigned short);
unsigned int FUN_00478470(void);
void FUN_0040a1a0(int, unsigned int, int);
unsigned int FUN_004187e0(unsigned short, unsigned short);
void FUN_00483410(short, short, int);
unsigned int FUN_00419100(int, int);
unsigned int FUN_004184f0(unsigned short);
void FUN_00483540(short, int, int);
unsigned int __cdecl FUN_004110e0(unsigned int, unsigned int, unsigned int, unsigned int);
int FUN_00413d00(int, int, int, int, int, int);
unsigned int FUN_00413010(unsigned short, unsigned short);
void FUN_0040f260(unsigned short, unsigned short);
void FUN_00413280(unsigned short, unsigned int);
void FUN_004a0ef0(unsigned short);
void FUN_00412700(unsigned int, unsigned char);
void __cdecl FUN_00410810(unsigned int, unsigned int, int);

void __stdcall FUN_0040d2d0(void)
{
  char cVar2;
  char cVar3;
  unsigned char bVar4;
  int iVar5;
  short sVar6;
  unsigned short uVar7;
  unsigned int uVar8;
  int iVar9, iVar10, iVar11;
  char *pcVar12;
  unsigned int uVar13;
  unsigned char *pbVar14;
  int iVar15, iVar16;
  unsigned int uVar17;
  unsigned int puVar18;
  unsigned short uVar19;
  int iVar20;
  char *pcVar21;
  unsigned short uVar22;
  int iVar23, iVar24;
  unsigned int uVar25;
  unsigned int unaff_EBX = 0;
  unsigned short uVar26, uVar27;
  int iVar28, iVar29;
  unsigned int puVar30;
  unsigned int puVar31;
  int bVar32 = 0;
  int bCond;
  unsigned char uVar33;
  unsigned int in_stack_ffffffc4 = 0;
  short sVar35;
  unsigned short uStack_38, local_36, uStack_34, local_32;
  unsigned char bStack_30;
  char local_1c;
  unsigned int local_18;
  unsigned int local_10 = 0;

  local_32 = (unsigned short)_DAT_00732a3c;
  if ((int)local_32 < DAT_00732904) {
    do {
      uVar8 = local_32;
      iVar5 = uVar8 * 0x23;
      iVar9 = DAT_006d4988 + iVar5;
      if ((R8(iVar9 + 3) != 0) && (R8(iVar9 + 2) != 0x0d)) {
        iVar9 = R32(iVar9 + 0x1b);
        if (iVar9 != -1) {
          iVar15 = (int)((unsigned int)iVar9 * 0x5bd);
          if (DAT_0071b744[iVar15] == 2) goto LAB_0040ed02;
          if (R32((int)DAT_0071bce9 + iVar15) < DAT_0071b71c) {
            uVar26 = (unsigned short)_DAT_00732a3c;
            iVar10 = (int)((unsigned int)DAT_0071b71c + 0x1d);
            W16((int)DAT_0071b75d + iVar15, 0xffff);
            W16((int)DAT_0071b75f + iVar15, 0);
            iVar23 = DAT_006d4980;
            W32((int)DAT_0071bce9 + iVar15, iVar10);
            if ((int)(_DAT_00732a3c & 0xffff) < iVar23) {
              do {
                iVar23 = (unsigned int)uVar26 * 0x23 + DAT_006d4988;
                if (((R8(iVar23 + 3) != 0) &&
                     ((iVar10 = R32(R32(iVar23 + 0x1f) + 0xa7)) != 0)) &&
                    (R16(iVar10 + 0x1df) != 0) &&
                    ((2 < R8(iVar23 + 2)) && (R8(iVar23 + 2) < 5))) {
                  FUN_0041b530(uVar8, (unsigned int)uVar26);
                }
                uVar26 = uVar26 + 1;
              } while ((int)(unsigned int)uVar26 < DAT_006d4980);
            }
          }
          iVar23 = DAT_006d4988 + iVar5;
          DAT_006d4974 = R32(iVar23 + 4);
          DAT_006d4978 = R32(iVar23 + 8);
          DAT_006d4970 = R32(iVar23 + 0xc);
          DAT_0071e6c4 = R32(R32(iVar23 + 0x1f) + 6);
          DAT_00732f38 = R32(R32(iVar23 + 0x1f) + 10);
          DAT_00732fe8 = R32(R32(iVar23 + 0x1f) + 0xe);
          local_10 = (local_10 & 0xffff0000u) | R16((int)DAT_0071b74a + iVar15);
          uVar25 = R16((int)DAT_0071b74a + iVar15);
          unaff_EBX = _DAT_00732a3c;
          if (((int)_DAT_00732a3c <= (int)uVar25) && ((int)uVar25 < DAT_006d4980)) {
            iVar10 = uVar25 * 0x23;
            iVar20 = DAT_006d4988 + iVar10;
            unaff_EBX = ((_DAT_00732a3c >> 8) << 8) | R8(iVar20 + 3);
            if ((R8(iVar20 + 3) != 0) && (R8(iVar20 + 2) != 0x0d)) {
              DAT_0073294c = R32(iVar20 + 4);
              DAT_007329b0 = R32(iVar20 + 8);
              DAT_007329b4 = R32(iVar20 + 0xc);
              DAT_0070a180 = R32(R32(iVar20 + 0x1f) + 6);
              DAT_0070acd8 = R32(R32(iVar20 + 0x1f) + 10);
              DAT_00703ad8 = R32(R32(iVar20 + 0x1f) + 0xe);
              iVar24 = (int)((unsigned int)DAT_006d4970 - (unsigned int)DAT_007329b4);
              iVar28 = (int)((unsigned int)DAT_006d4978 - (unsigned int)DAT_007329b0);
              iVar11 = (int)((unsigned int)DAT_006d4974 - (unsigned int)DAT_0073294c);
              if (iVar11 < 0) iVar11 = (int)(0u - (unsigned int)iVar11);
              if (iVar28 < 0) iVar28 = (int)(0u - (unsigned int)iVar28);
              if (iVar24 < 0) iVar24 = (int)(0u - (unsigned int)iVar24);
              if ((iVar28 < iVar11) && (iVar24 < iVar11)) {
                DAT_006d9d38 = ADD3(iVar11, iVar28 >> 2, iVar24 >> 2);
              } else if ((iVar11 < iVar28) && (iVar24 < iVar28)) {
                DAT_006d9d38 = ADD3(iVar11 >> 2, iVar24 >> 2, iVar28);
              } else {
                DAT_006d9d38 = ADD3(iVar11 >> 2, iVar28 >> 2, iVar24);
              }
              unaff_EBX = R32(R32(iVar20 + 0x1f) + 0xa7);
              if ((R8(unaff_EBX + 7) != 3) && (R8(unaff_EBX + 7) != 4) &&
                  (R8(R8(R32(iVar23 + 0x1f) + 0x87) + 0x1f0 + unaff_EBX) == 0)) {
                uVar26 = R16((int)DAT_0052686c + (unsigned int)R8(iVar20 + 3) * 0x16);
                if (3000 < uVar26) uVar26 = uVar26 >> 1;
                if (DAT_006d9d38 < (int)((unsigned int)uVar26 * 4)) {
                  uVar26 = 0;
                  uVar19 = 0;
                  do {
                    uVar22 = (unsigned short)R8(uVar19 + 0x1f0 + unaff_EBX);
                    if (uVar26 < uVar22) uVar26 = uVar22;
                    uVar19 = uVar19 + 1;
                  } while (uVar19 < 10);
                  local_1c = (char)uVar26 + '\x01';
                  W8(R16((int)DAT_0071b73c + iVar15) + 0x1f0 + unaff_EBX, local_1c);
                  uVar19 = R16((int)DAT_0071b73c + iVar15);
                  pbVar14 = (unsigned char *)(DAT_006d4988 + 0x16 + iVar10);
                  DAT_00709388[(unsigned int)*pbVar14 + (unsigned int)uVar19 * 0x30] =
                      DAT_00709388[(unsigned int)*pbVar14 + (unsigned int)uVar19 * 0x30] + 1;
                  uVar25 = *pbVar14;
                  W16((int)DAT_0071b7c1 + iVar15, R16((int)DAT_0071b7c1 + iVar15) + 1);
                  W16((int)DAT_006d4a7a + uVar25 * 0x126, R16((int)DAT_006d4a7a + uVar25 * 0x126) + 1);
                  iVar23 = uVar25 * 0x126;
                  DAT_006d4abe[(unsigned int)uVar19 + iVar23] =
                      DAT_006d4abe[(unsigned int)uVar19 + iVar23] + 1;
                  cVar2 = (char)R8(unaff_EBX + 0xb0);
                  iVar20 = uVar25 * 0x566;
                  cVar3 = DAT_0070ad40[iVar20];
                  if (cVar3 == cVar2) {
                    DAT_006d4aaa[iVar23] = 1;
                    W16((int)DAT_0071b7c3 + iVar15, R16((int)DAT_0071b7c3 + iVar15) + 1);
                    DAT_006d4ac8[iVar23 + (unsigned int)R16((int)DAT_0071b73c + iVar15)] =
                        DAT_006d4ac8[iVar23 + (unsigned int)R16((int)DAT_0071b73c + iVar15)] + 1;
                  }
                  local_18 = (unsigned int)(cVar3 == cVar2);
                  iVar11 = FUN_004a1590(5, (unsigned int)*pbVar14, iVar9, uVar26, local_18,
                                        (unsigned int)R16((int)DAT_0071b73c + iVar15));
                  FUN_004a1590(5,
                               (((unsigned int)DAT_006d4988 & 0xffffff00u) | R8(DAT_006d4988 + 0x16 + iVar10)) & 0xffff00ffu,
                               -1, uVar26, local_18,
                               (unsigned int)R16((int)DAT_0071b73c + iVar15));
                  pcVar21 = (char *)DAT_0070aee5 + iVar20;
                  bVar32 = 0;
                  pcVar12 = (char *)DAT_0070aee9 + iVar20 + (unsigned int)R16((int)DAT_0071b73c + iVar15);
                  iVar20 = 8;
                  do {
                    if (((*pcVar12 != '\0') &&
                         ((pcVar21[1] == '\x05' ||
                           ((pcVar21[2] == '\x06' &&
                             ((local_18 != 0 || (DAT_006d4aaa[iVar23] == 0)))))))) &&
                        ((*pcVar21 == '\0' || (*pcVar21 == '\x02')))) {
                      bVar32 = 1;
                    }
                    pcVar21 = pcVar21 + 0x4e;
                    pcVar12 = pcVar12 + 0x4e;
                    iVar20 = iVar20 + -1;
                  } while (iVar20 != 0);
                  if ((bVar32) && (iVar9 == DAT_0071ecf4)) {
                    if (local_18 == 0) iVar23 = 0x15b; else iVar23 = 0x15c;
                    FUN_004a1d70(iVar23, DAT_0071ecf4);
                    W16((int)DAT_0070ac4a + DAT_0071ecf4 * 0x10, 0x3b0);
                  }
                  DAT_00655a84 = 0xfffd;
                  if ((((bVar32) || (R8(DAT_006d4988 + iVar10 + 2) != 0)) ||
                       ((DAT_007092d9 == 0) && (R32(DAT_006d4988 + iVar10 + 0x1b) != -1))) &&
                      (((iVar9 == DAT_0071ecf4 &&
                         (FUN_004a34c0(local_10, unaff_EBX, 0x8d), iVar11 != 0)) &&
                        (1 < DAT_00732e48)) && (DAT_006d4a02 == 2))) {
                    _DAT_00732950 = uVar26 + 0x11c;
                    FUN_004a1d70(0x12a, DAT_0071ecf4);
                  }
                }
              }
            }
          }
          if (0x2c < R16(R32(DAT_006d4988 + 0x1f + iVar5) + 0x81)) {
            local_36 = 0;
            do {
              if (local_36 == 0) {
                uVar25 = R8(DAT_006d4988 + 0x16 + iVar5);
                unaff_EBX = uVar25 * 0x159;
                if (DAT_0070ad8d[uVar25 * 0x566] != 0) {
                  cVar2 = DAT_0070ad8c[uVar25 * 0x566];
LAB_0040d92f:
                  uVar26 = (unsigned short)_DAT_00732a3c;
                  if ((int)(_DAT_00732a3c & 0xffff) < DAT_006d4980) {
                    do {
                      unaff_EBX = (unsigned int)uVar26 * 0x23;
                      iVar23 = DAT_006d4988 + (int)unaff_EBX;
                      if (((R8(iVar23 + 3) != 0) && (R8(iVar23 + 2) != 0x0d)) &&
                          (R8(iVar23 + 0x16) == (unsigned char)cVar2)) {
                        DAT_0073294c = R32(iVar23 + 4);
                        DAT_007329b0 = R32(iVar23 + 8);
                        DAT_007329b4 = R32(iVar23 + 0xc);
                        DAT_0070a180 = R32(R32(iVar23 + 0x1f) + 6);
                        DAT_0070acd8 = R32(R32(iVar23 + 0x1f) + 10);
                        DAT_00703ad8 = R32(R32(iVar23 + 0x1f) + 0xe);
                        iVar10 = R32(R32(iVar23 + 0x1f) + 0xa7);
                        if (R8(iVar10 + 7) == 0) {
                          iVar10 = (unsigned int)R8(iVar10 + 4) * 0xef;
                          FUN_0042a230(iVar23, (unsigned int)R16((int)DAT_0052b14d + iVar10), iVar23,
                                       (short)R16((int)DAT_0052b14b + iVar10),
                                       R16((int)DAT_0052b14d + iVar10),
                                       (short)R16((int)DAT_0052b14f + iVar10));
                          DAT_0073294c = (int)((unsigned int)DAT_0073294c + (unsigned int)DAT_0070ab18);
                          DAT_007329b0 = (int)((unsigned int)DAT_007329b0 + (unsigned int)DAT_0070ab14);
                          DAT_007329b4 = (int)((unsigned int)DAT_007329b4 + (unsigned int)DAT_0070ac20);
                          if (R8(DAT_006d4988 + 2 + (int)unaff_EBX) == 4) {
                            uVar25 = ((0u - (unsigned int)(DAT_00709d0a[(unsigned int)R16((int)DAT_0071b73c + iVar15) * 3] == 1)) & 0x2000) + 0x2000;
                          } else {
                            uVar25 = 0x2000;
                          }
                          unaff_EBX = (unsigned int)DAT_007329b4 - (unsigned int)DAT_006d4970;
                          iVar23 = (int)((unsigned int)DAT_007329b0 - (unsigned int)DAT_006d4978);
                          iVar10 = (int)((unsigned int)DAT_0073294c - (unsigned int)DAT_006d4974);
                          if (iVar10 < 0) iVar10 = (int)(0u - (unsigned int)iVar10);
                          if (iVar23 < 0) iVar23 = (int)(0u - (unsigned int)iVar23);
                          if ((int)unaff_EBX < 0) unaff_EBX = 0u - unaff_EBX;
                          puVar31 = (unsigned int)((int)unaff_EBX >> 2);
                          if ((iVar23 < iVar10) && ((int)unaff_EBX < iVar10)) {
                            uVar13 = (unsigned int)ADD3((int)puVar31, iVar10, iVar23 >> 2);
                            unaff_EBX = puVar31;
                          } else if ((iVar10 < iVar23) && ((int)unaff_EBX < iVar23)) {
                            uVar13 = (unsigned int)ADD3((int)puVar31, iVar23, iVar10 >> 2);
                            unaff_EBX = puVar31;
                          } else {
                            uVar13 = (unsigned int)ADD3(iVar10 >> 2, iVar23 >> 2, (int)unaff_EBX);
                          }
                          if ((uVar13 < uVar25) && (DAT_0071b776[iVar15] == 0)) {
                            if (iVar9 == DAT_0071ecf4) FUN_004a1d70(0xd5, DAT_0071ecf4);
                            DAT_0071b776[iVar15] = 2;
                            W16((int)DAT_0071b777 + iVar15, uVar26);
                            W32((int)DAT_0071bcdd + iVar15, 0x760);
                          }
                        }
                      }
                      uVar26 = uVar26 + 1;
                    } while ((int)(unsigned int)uVar26 < DAT_006d4980);
                  }
                }
              } else {
                uVar25 = R8(DAT_006d4988 + 0x16 + iVar5);
                unaff_EBX = uVar25 * 0x159;
                if (DAT_0070ad8f[uVar25 * 0x566] != 0) {
                  cVar2 = DAT_0070ad8e[uVar25 * 0x566];
                  goto LAB_0040d92f;
                }
              }
              local_36 = local_36 + 1;
            } while (local_36 < 2);
          }
        }
        iVar9 = R32(DAT_006d4988 + 0x1f + iVar5);
        if (iVar9 != 0) {
          pbVar14 = (unsigned char *)(iVar9 + 0x12);
          iVar15 = (int)((unsigned int)R32(iVar9 + 0x73) - (unsigned int)DAT_0071e6be);
          W32(iVar9 + 0x73, iVar15);
          if (iVar15 < 1) {
            W32(iVar9 + 0x73, 0x7fff);
            FUN_0040cd90(pbVar14, local_32);
          }
          local_36 = 0;
          if (*pbVar14 != 0) {
            do {
              uVar13 = local_36;
              uVar26 = R16(iVar9 + 0x53 + uVar13 * 2);
              unaff_EBX = (unaff_EBX & 0xffff0000u) | uVar26;
              uVar25 = uVar26;
              iVar15 = uVar25 * 0x23;
              iVar23 = iVar15 + DAT_006d4988;
              if ((R8(iVar23 + 3) == 0) || (R8(iVar23 + 2) == 0x0d)) {
                uVar25 = 0;
                bStack_30 = *pbVar14;
                if (bStack_30 != 0) {
                  puVar18 = iVar9 + 0x53;
                  do {
                    if (R16(puVar18) == uVar26) break;
                    puVar18 = puVar18 + 2;
                    uVar25 = uVar25 + 1;
                  } while ((int)uVar25 < (int)(unsigned int)bStack_30);
                }
                if (uVar25 != bStack_30) {
                  iVar15 = uVar25 + 1;
                  if (iVar15 < (int)(unsigned int)bStack_30) {
                    puVar30 = iVar9 + 0x51 + iVar15 * 2;
                    puVar31 = iVar9 + 0xf + iVar15 * 4;
                    do {
                      W16(puVar30, R16(puVar30 + 2));
                      puVar30 = puVar30 + 2;
                      iVar15 = iVar15 + 1;
                      W32(puVar31, R32(puVar31 + 4));
                      puVar31 = puVar31 + 4;
                    } while (iVar15 < (int)(unsigned int)*pbVar14);
                  }
LAB_0040ecc9:
                  *pbVar14 = *pbVar14 - 1;
                }
LAB_0040eccf:
                local_36 = local_36 - 1;
              }
              else {
                bCond = ((R32(iVar23 + 0x1b) == -1) ||
                         (((cVar2 = (char)R8(DAT_006d4988 + 2 + iVar5), cVar2 != '\x04') &&
                           (cVar2 != '\x05')) || (2 < R16(R32(iVar23 + 0x1f) + 0x81))));
                if (bCond) {
                  iVar23 = (int)((unsigned int)R32(iVar9 + 0x13 + uVar13 * 4) - (unsigned int)DAT_0071e6be);
                  W32(iVar9 + 0x13 + uVar13 * 4, iVar23);
                  bCond = (iVar23 < 1);
                }
                if (bCond) {
                iVar23 = DAT_006d4988 + iVar5;
                if (R32(iVar23 + 0x1b) == -1) {
                  bVar4 = R8(iVar23 + 2);
                  if (bVar4 < 8) {
                    switch (bVar4) {
                    default:
                      DAT_006d4974 = R32(iVar23 + 4);
                      DAT_006d4978 = R32(iVar23 + 8);
                      DAT_006d4970 = R32(iVar23 + 0xc);
                      DAT_0071e6c4 = R32(R32(iVar23 + 0x1f) + 6);
                      DAT_00732f38 = R32(R32(iVar23 + 0x1f) + 10);
                      DAT_00732fe8 = R32(R32(iVar23 + 0x1f) + 0xe);
                      if ((DAT_00732a38 <= (int)uVar25) &&
                          (uVar25 = FUN_00413010(local_32, uVar26), (short)uVar25 != 0)) {
                        if (DAT_007092d5 == 0) {
                          FUN_0040f260(local_32, uVar26);
                        } else {
                          FUN_004110e0((unsigned int)local_32, 0xffff, unaff_EBX, 0);
                        }
                      }
                      break;
                    case 3:
                    case 4:
                    case 5:
                      iVar23 = R32(R32(iVar23 + 0x1f) + 0xa7);
                      iVar10 = FUN_0042b150(iVar23);
                      if (R8(iVar10 + 0x34) == 0x1e) {
                        local_36 = 0x10;
                      } else if ((short)R16(iVar23 + 0x68) == -1) {
                        if (R16(iVar23 + 0x66) == uVar26) {
                          FUN_0040ed40(pbVar14, local_32, unaff_EBX);
                        } else if ((R8(iVar10 + 0x34) == 0x12) && (R16(iVar10 + 0x18) == uVar26)) {
                          FUN_0040ed40(pbVar14, local_32, unaff_EBX);
                        } else {
                          local_10 = R32(R32(DAT_006d4988 + 0x1f + iVar15) + 0xa7);
                          iVar23 = FUN_0042b150(local_10);
                          iVar10 = 0xe;
                          pcVar12 = (char *)((int)DAT_00703ca0 + (unsigned int)R8(iVar23 + 10) * 0x55);
                          pcVar21 = s_exithangarpln_00523c08;
                          do {
                            if (iVar10 == 0) break;
                            iVar10 = iVar10 + -1;
                            bVar32 = *pcVar12 == *pcVar21;
                            pcVar12 = pcVar12 + 1;
                            pcVar21 = pcVar21 + 1;
                          } while (bVar32);
                          if (!bVar32) {
                            iVar10 = 0x13;
                            pcVar12 = (char *)((int)DAT_00703ca0 + (unsigned int)R8(iVar23 + 10) * 0x55);
                            pcVar21 = s_outofhyperspacepln_00523bf4;
                            do {
                              if (iVar10 == 0) break;
                              iVar10 = iVar10 + -1;
                              bVar32 = *pcVar12 == *pcVar21;
                              pcVar12 = pcVar12 + 1;
                              pcVar21 = pcVar21 + 1;
                            } while (bVar32);
                            if (!bVar32) {
                              iVar10 = 0xf;
                              pcVar12 = (char *)((int)DAT_00703ca0 + (unsigned int)R8(iVar23 + 10) * 0x55);
                              pcVar21 = s_enterhangarpln_00523be4;
                              do {
                                if (iVar10 == 0) break;
                                iVar10 = iVar10 + -1;
                                bVar32 = *pcVar12 == *pcVar21;
                                pcVar12 = pcVar12 + 1;
                                pcVar21 = pcVar21 + 1;
                              } while (bVar32);
                              if (!bVar32) {
                                iVar10 = 0x13;
                                pcVar12 = (char *)((int)DAT_00703ca0 + (unsigned int)R8(iVar23 + 10) * 0x55);
                                pcVar21 = s_followhomeevadepln_00523bd0;
                                do {
                                  if (iVar10 == 0) break;
                                  iVar10 = iVar10 + -1;
                                  bVar32 = *pcVar12 == *pcVar21;
                                  pcVar12 = pcVar12 + 1;
                                  pcVar21 = pcVar21 + 1;
                                } while (bVar32);
                                if (bVar32) {
                                  iVar10 = FUN_0042b150(R32(R32((unsigned int)R8(local_10 + 5) * 0x23 + 0x1f + DAT_006d4988) + 0xa7));
                                  iVar20 = 0xe;
                                  pcVar12 = (char *)((int)DAT_00703ca0 + (unsigned int)R8(iVar10 + 10) * 0x55);
                                  pcVar21 = s_exithangarpln_00523c08;
                                  do {
                                    if (iVar20 == 0) break;
                                    iVar20 = iVar20 + -1;
                                    bVar32 = *pcVar12 == *pcVar21;
                                    pcVar12 = pcVar12 + 1;
                                    pcVar21 = pcVar21 + 1;
                                  } while (bVar32);
                                  if (!bVar32) {
                                    iVar20 = 0xf;
                                    pcVar12 = (char *)((int)DAT_00703ca0 + (unsigned int)R8(iVar10 + 10) * 0x55);
                                    pcVar21 = s_enterhangarpln_00523be4;
                                    do {
                                      if (iVar20 == 0) break;
                                      iVar20 = iVar20 + -1;
                                      bVar32 = *pcVar12 == *pcVar21;
                                      pcVar12 = pcVar12 + 1;
                                      pcVar21 = pcVar21 + 1;
                                    } while (bVar32);
                                    if (!bVar32) goto LAB_0040e829;
                                  }
                                  FUN_0040ed40(pbVar14, local_32, unaff_EBX);
                                } else {
LAB_0040e829:
                                  if ((((short)R16(local_10 + 0x68) == -1) &&
                                       (cVar2 = (char)R8(iVar23 + 0x34), cVar2 != '\x12')) &&
                                      (cVar2 != '\x15') && (cVar2 != '\x1e')) {
                                    iVar23 = DAT_006d4988 + iVar5;
                                    DAT_0073294c = R32(iVar23 + 4);
                                    DAT_007329b0 = R32(iVar23 + 8);
                                    DAT_007329b4 = R32(iVar23 + 0xc);
                                    DAT_0070a180 = R32(R32(iVar23 + 0x1f) + 6);
                                    DAT_0070acd8 = R32(R32(iVar23 + 0x1f) + 10);
                                    iVar10 = iVar15 + DAT_006d4988;
                                    DAT_00703ad8 = R32(R32(iVar23 + 0x1f) + 0xe);
                                    DAT_006d4974 = R32(iVar10 + 4);
                                    DAT_006d4978 = R32(iVar10 + 8);
                                    DAT_006d4970 = R32(iVar10 + 0xc);
                                    DAT_0071e6c4 = R32(R32(iVar10 + 0x1f) + 6);
                                    DAT_00732f38 = R32(R32(iVar10 + 0x1f) + 10);
                                    DAT_00732fe8 = R32(R32(iVar10 + 0x1f) + 0xe);
                                    uVar25 = FUN_0040f7e0(uVar26, local_32);
                                    if ((short)uVar25 != 0) {
                                      FUN_004110e0((unsigned int)local_32, uVar25, unaff_EBX, 0);
                                      FUN_004110e0(unaff_EBX, 0xffff, (unsigned int)local_32, 0);
                                    }
                                    break;
                                  }
                                  FUN_0040ed40(pbVar14, local_32, unaff_EBX);
                                }
                                goto LAB_0040ecd4;
                              }
                            }
                          }
                          FUN_0040ed40(pbVar14, local_32, unaff_EBX);
                        }
                      } else {
                        local_36 = 0x10;
                      }
                      goto LAB_0040ecd4;
                    case 6:
                    case 7:
                      if ((R32((unsigned int)R16(R32(iVar23 + 0x1f) + 0x83) * 0x23 + 0x1b + DAT_006d4988) != -1) &&
                          ((int)uVar25 < DAT_006d4980)) {
                        iVar23 = FUN_0042b150(R32(R32(DAT_006d4988 + 0x1f + iVar15) + 0xa7));
                        iVar10 = 0xf;
                        pcVar12 = (char *)((int)DAT_00703ca0 + (unsigned int)R8(iVar23 + 0xb) * 0x55);
                        pcVar21 = s_boardtogivepln_00523c18;
                        do {
                          if (iVar10 == 0) break;
                          iVar10 = iVar10 + -1;
                          bVar32 = *pcVar12 == *pcVar21;
                          pcVar12 = pcVar12 + 1;
                          pcVar21 = pcVar21 + 1;
                        } while (bVar32);
                        if (((bVar32) && (R8(iVar23 + 0x34) == 0x12)) &&
                            ((R8(iVar23 + 0x35) == 2) && (R16(iVar23 + 0x18) == local_32))) {
                          FUN_0040ed40(pbVar14, local_32, unaff_EBX);
                          goto LAB_0040ecd4;
                        }
                      }
                      iVar23 = DAT_006d4988 + iVar5;
                      DAT_006d4974 = R32(iVar23 + 4);
                      DAT_006d4978 = R32(iVar23 + 8);
                      DAT_006d4970 = R32(iVar23 + 0xc);
                      DAT_0071e6c4 = R32(R32(iVar23 + 0x1f) + 6);
                      DAT_00732f38 = R32(R32(iVar23 + 0x1f) + 10);
                      iVar10 = iVar15 + DAT_006d4988;
                      DAT_00732fe8 = R32(R32(iVar23 + 0x1f) + 0xe);
                      if (R32(iVar10 + 0x1f) == 0) {
                        if ((DAT_007092c1 == 0) &&
                            (uVar25 = FUN_00413010(local_32, uVar26), (short)uVar25 != 0)) {
                          FUN_00413280(local_32, unaff_EBX);
                          FUN_004a0ef0(local_32);
                        }
                      } else {
                        DAT_0073294c = R32(iVar10 + 4);
                        DAT_007329b0 = R32(iVar10 + 8);
                        DAT_007329b4 = R32(iVar10 + 0xc);
                        DAT_0070a180 = R32(R32(iVar10 + 0x1f) + 6);
                        uVar17 = R32(R32(iVar10 + 0x1f) + 10);
                        DAT_00703ad8 = R32(R32(iVar10 + 0x1f) + 0xe);
                        DAT_0070acd8 = uVar17;
                        uVar13 = FUN_0040f7e0(local_32, uVar26);
                        if ((short)uVar13 != 0) {
                          if ((int)uVar25 < DAT_006d4980) {
                            FUN_004a0ef0(local_32);
                            FUN_00410810((unsigned int)local_32, unaff_EBX,
                                         (int)CONCAT22((unsigned short)(uVar17 >> 0x10), (short)uVar13));
                          } else {
                            if (DAT_00525bf7[R8(DAT_006d4988 + iVar5 + 3)] == 0) {
                              FUN_004a0ef0(local_32);
                              cVar2 = (char)R8(DAT_006d4988 + 3 + iVar5);
                              if ((cVar2 == (char)-0x73) || (cVar2 == (char)-0x72)) {
                                uVar33 = 0x84;
                              } else {
                                uVar33 = 0x83;
                              }
                            } else {
                              puVar31 = unaff_EBX;
                              if (DAT_00525bf7[R8(DAT_006d4988 + iVar15 + 3)] == 0) {
LAB_0040eb16:
                                FUN_004a0ef0((unsigned short)puVar31);
                              } else {
                                if (R16(R32(R32(DAT_006d4988 + iVar5 + 0x1f) + 0xa3) + 4) == uVar26) {
                                  puVar31 = (unsigned int)local_32;
                                  goto LAB_0040eb16;
                                }
                                if (R16(R32(R32(DAT_006d4988 + iVar15 + 0x1f) + 0xa3) + 4) == local_32)
                                  goto LAB_0040eb16;
                              }
                              uVar33 = 0x81;
                            }
                            FUN_00412700(uVar8, uVar33);
                            FUN_00412700(uVar25, 0x81);
                          }
                        }
                      }
                    }
                  }
LAB_0040ebc5:
                  if ((R8(DAT_006d4988 + iVar5 + 3) == 0) || (R8(DAT_006d4988 + iVar5 + 2) == 0x0d)) {
                    W32(iVar9 + 0x73, 0);
                    *pbVar14 = 0;
                    break;
                  }
                  if ((R8(DAT_006d4988 + iVar15 + 3) == 0) || (R8(DAT_006d4988 + iVar15 + 2) == 0x0d)) {
                    uVar25 = 0;
                    bStack_30 = *pbVar14;
                    if (bStack_30 != 0) {
                      puVar18 = iVar9 + 0x53;
                      do {
                        if (R16(puVar18) == uVar26) break;
                        puVar18 = puVar18 + 2;
                        uVar25 = uVar25 + 1;
                      } while ((int)uVar25 < (int)(unsigned int)bStack_30);
                    }
                    if (uVar25 != bStack_30) {
                      iVar15 = uVar25 + 1;
                      if (iVar15 < (int)(unsigned int)bStack_30) {
                        puVar30 = iVar9 + 0x51 + iVar15 * 2;
                        puVar31 = iVar9 + 0xf + iVar15 * 4;
                        do {
                          W16(puVar30, R16(puVar30 + 2));
                          puVar30 = puVar30 + 2;
                          iVar15 = iVar15 + 1;
                          W32(puVar31, R32(puVar31 + 4));
                          puVar31 = puVar31 + 4;
                        } while (iVar15 < (int)(unsigned int)*pbVar14);
                      }
                      goto LAB_0040ecc9;
                    }
                    goto LAB_0040eccf;
                  }
                } else {
                  if ((2 < R16(R32(iVar23 + 0x1f) + 0x81)) ||
                      ((cVar2 = (char)R8(DAT_006d4988 + 2 + iVar15), cVar2 != '\x04') &&
                       (cVar2 != '\x05'))) {
                    iVar10 = R32(R32(iVar23 + 0x1f) + 0xa7);
                    if (R16(iVar10 + 0x70) != uVar26) {
                      DAT_006d4974 = R32(iVar23 + 4);
                      DAT_006d4978 = R32(iVar23 + 8);
                      DAT_006d4970 = R32(iVar23 + 0xc);
                      DAT_0071e6c4 = R32(R32(iVar23 + 0x1f) + 6);
                      DAT_00732f38 = R32(R32(iVar23 + 0x1f) + 10);
                      DAT_00732fe8 = R32(R32(iVar23 + 0x1f) + 0xe);
                      iVar23 = R32(DAT_006d4988 + 0x1f + iVar15);
                      if (iVar23 == 0) {
                        if (((DAT_007092c1 == 0) &&
                             (uVar25 = FUN_00413010(local_32, uVar26), (short)uVar25 != 0)) &&
                            (FUN_0040f260(local_32, uVar26), DAT_007092d5 != 0)) {
                          iVar23 = (unsigned int)R8(DAT_006d4988 + 0x16 + iVar15) * 0x566;
                          if ((((DAT_0070ad44[iVar23] != 0x14) && (DAT_0070b215[iVar23] != 0x14)) &&
                               ((uVar25 = R8(DAT_006d4988 + 0x16 + iVar5),
                                 DAT_0070ad44[uVar25 * 0x566] != 0x14) &&
                                (DAT_0070b215[uVar25 * 0x566] != 0x14)))) {
                            FUN_00413280(local_32, unaff_EBX);
                          }
                          FUN_004110e0((unsigned int)local_32, 0xffff, unaff_EBX, 0);
                        }
                      } else {
                        local_10 = R32(iVar23 + 0xa7);
                        iVar23 = FUN_0042b150(local_10);
                        iVar20 = 0xf;
                        pcVar12 = (char *)((int)DAT_00703ca0 + (unsigned int)R8(iVar23 + 0xb) * 0x55);
                        pcVar21 = s_boardtogivepln_00523c18;
                        do {
                          if (iVar20 == 0) break;
                          iVar20 = iVar20 + -1;
                          bVar32 = *pcVar12 == *pcVar21;
                          pcVar12 = pcVar12 + 1;
                          pcVar21 = pcVar21 + 1;
                        } while (bVar32);
                        if ((bVar32) && (R16(iVar23 + 0x18) == local_32)) {
                          FUN_0040ed40(pbVar14, local_32, unaff_EBX);
                          goto LAB_0040ecd4;
                        }
                        iVar23 = iVar15 + DAT_006d4988;
                        DAT_0073294c = R32(iVar23 + 4);
                        DAT_007329b0 = R32(iVar23 + 8);
                        DAT_007329b4 = R32(iVar23 + 0xc);
                        DAT_0070a180 = R32(R32(iVar23 + 0x1f) + 6);
                        DAT_0070acd8 = R32(R32(iVar23 + 0x1f) + 10);
                        DAT_00703ad8 = R32(R32(iVar23 + 0x1f) + 0xe);
                        uVar13 = FUN_0040f7e0(local_32, uVar26);
                        sVar6 = (short)uVar13;
                        in_stack_ffffffc4 = CONCAT22((short)(in_stack_ffffffc4 >> 0x10), sVar6);
                        if (sVar6 != 0) {
                          if (DAT_007092c1 == 0) {
                            cVar2 = (char)R8(DAT_006d4988 + 2 + iVar15);
                            if (((cVar2 == '\0') || (cVar2 == '\x01')) || (cVar2 == '\x02')) {
                              W16(iVar10 + 0x70, uVar26);
                              sVar35 = (short)R16(R32(DAT_006d4988 + 0x1f + iVar5) + 0x79);
                              uVar19 = (unsigned short)(R16(DAT_006d4988 + iVar15 + 0x10) -
                                                        R16(DAT_006d4988 + iVar5 + 0x10));
                              if (0x7fff < uVar19) uVar19 = (unsigned short)(0u - uVar19);
                              if (0x4000 < uVar19) uVar19 = (unsigned short)(0x8000 - uVar19);
                              uVar22 = R16(R32(DAT_006d4988 + iVar15 + 0x1f) + 0x79);
                              if (uVar19 < 0x4000) {
                                uVar13 = FUN_004187e0(uVar22, uVar19);
                                in_stack_ffffffc4 = CONCAT22((unsigned short)(sVar35 - (short)uVar13), sVar6);
                              } else {
                                uVar13 = FUN_004187e0(uVar22, uVar19);
                                in_stack_ffffffc4 = CONCAT22((unsigned short)(sVar35 + (short)uVar13), sVar6);
                              }
                              if ((int)in_stack_ffffffc4 < 0) {
                                in_stack_ffffffc4 = CONCAT22(-(short)(in_stack_ffffffc4 >> 0x10),
                                                             (short)in_stack_ffffffc4);
                              }
                              iVar29 = DAT_006d4988 + iVar5;
                              iVar20 = iVar15 + DAT_006d4988;
                              local_18 = (unsigned int)R32(iVar29 + 4) - (unsigned int)R32(iVar20 + 4);
                              iVar23 = (int)((unsigned int)R32(iVar29 + 8) - (unsigned int)R32(iVar20 + 8));
                              iVar20 = (int)((unsigned int)R32(iVar29 + 0xc) - (unsigned int)R32(iVar20 + 0xc));
                              iVar20 = (int)((unsigned int)iVar20 * (unsigned int)iVar20);
                              iVar11 = (int)((unsigned int)iVar23 * (unsigned int)iVar23);
                              iVar24 = (int)(local_18 * local_18);
                              iVar28 = (int)((unsigned int)iVar11 + (unsigned int)iVar20 + (unsigned int)iVar24);
                              if (iVar28 < 0x33) {
                                uStack_34 = 0;
                                uStack_38 = 100;
                              } else {
                                iVar16 = (int)(short)(in_stack_ffffffc4 >> 0x10);
                                local_18 = (unsigned int)iVar16 * local_18 * 1000u;
                                uStack_34 = (unsigned short)((int)local_18 / iVar28);
                                iVar23 = (int)((unsigned int)iVar23 * (unsigned int)iVar16 * 1000u);
                                uStack_38 = (unsigned short)(iVar23 / iVar28);
                              }
                              if (R8(R32(iVar29 + 0x1f) + 0x89) != 0) {
                                FUN_00483410((short)R16(iVar29 + 0x12), (short)R16(iVar29 + 0x10), iVar29);
                              }
                              iVar29 = R32(DAT_006d4988 + 0x1f + iVar5);
                              iVar28 = DAT_006d4988 + iVar5;
                              uVar19 = R16(iVar29 + 0x8a);
                              uStack_34 = (unsigned short)(uStack_34 + uVar19);
                              if (((uStack_34 ^ uVar19) & 0x8000) != 0) uStack_34 = uVar19;
                              uVar19 = R16(iVar29 + 0x8c);
                              uStack_38 = (unsigned short)(uStack_38 + uVar19);
                              if (((uStack_38 ^ uVar19) & 0x8000) != 0) uStack_38 = uVar19;
                              uVar19 = R16(iVar28 + 0x10);
                              uVar17 = FUN_00419100((int)(short)uStack_34, (int)(short)uStack_38);
                              W16(iVar28 + 0x10, (short)uVar17);
                              uVar7 = (unsigned short)FUN_004184f0(uStack_34);
                              W16(DAT_006d4988 + 0x12 + iVar15, uVar7);
                              uVar27 = (unsigned short)((short)(in_stack_ffffffc4 >> 0x10) * 100);
                              W16(local_10 + 0xd, R16(DAT_006d4988 + 0x12 + iVar15));
                              uVar22 = uVar27;
                              if (0x7fff < uVar27) uVar22 = 0x7fff;
                              if (uVar19 < R16(iVar15 + DAT_006d4988 + 0x10)) uVar22 = (unsigned short)(0u - uVar22);
                              W16(R32(iVar15 + DAT_006d4988 + 0x1f) + 0x77, uVar22);
                              iVar28 = DAT_006d4988 + iVar15;
                              FUN_00483410((short)R16(iVar28 + 0x12), (short)R16(iVar28 + 0x10), iVar28);
                              FUN_00483540((short)R16(DAT_006d4988 + iVar15 + 0x14), 0, DAT_006d4988 + iVar15);
                              W16(R32(R32(DAT_006d4988 + 0x1f + iVar15) + 0xa7) + 0x70, local_32);
                              iVar11 = (int)((unsigned int)iVar20 + (unsigned int)iVar24 + (unsigned int)iVar11);
                              if (iVar11 < 0x33) {
                                uStack_34 = 0;
                                uStack_38 = 100;
                              } else {
                                sVar6 = (short)(in_stack_ffffffc4 >> 0x10);
                                uStack_34 = (unsigned short)((int)((unsigned int)(int)sVar6 * local_18 * 1000u) / iVar11);
                                uStack_38 = (unsigned short)((int)((unsigned int)iVar23 * (unsigned int)(int)sVar6 * 1000u) / iVar11);
                              }
                              iVar23 = iVar15 + DAT_006d4988;
                              if (R8(R32(iVar23 + 0x1f) + 0x89) != 0) {
                                FUN_00483410((short)R16(iVar23 + 0x12), (short)R16(iVar23 + 0x10), iVar23);
                              }
                              iVar23 = iVar15 + DAT_006d4988;
                              uVar19 = R16(R32(iVar23 + 0x1f) + 0x8a);
                              uStack_34 = (unsigned short)(uStack_34 + uVar19);
                              if (((uStack_34 ^ uVar19) & 0x8000) != 0) uStack_34 = uVar19;
                              uVar19 = R16(R32(iVar23 + 0x1f) + 0x8c);
                              uStack_38 = (unsigned short)(uStack_38 + uVar19);
                              if (((uStack_38 ^ uVar19) & 0x8000) != 0) uStack_38 = uVar19;
                              uVar19 = R16(iVar23 + 0x10);
                              uVar17 = FUN_00419100((int)(short)uStack_34, (int)(short)uStack_38);
                              W16(iVar23 + 0x10, (short)uVar17);
                              uVar7 = (unsigned short)FUN_004184f0(uStack_34);
                              W16(DAT_006d4988 + 0x12 + iVar5, uVar7);
                              W16(iVar10 + 0xd, R16(DAT_006d4988 + 0x12 + iVar5));
                              W8(R32(DAT_006d4988 + 0x1f + iVar15) + 0x90, 1);
                              W8(R32(DAT_006d4988 + 0x1f + iVar15) + 0x89, 1);
                              if (0x7fff < uVar27) uVar27 = 0x7fff;
                              if (uVar19 < R16(DAT_006d4988 + iVar5 + 0x10)) uVar27 = (unsigned short)(0u - uVar27);
                              W16(R32(DAT_006d4988 + iVar5 + 0x1f) + 0x77, uVar27);
                              iVar23 = DAT_006d4988 + iVar5;
                              FUN_00483410((short)R16(iVar23 + 0x12), (short)R16(iVar23 + 0x10), iVar23);
                              FUN_00483540((short)R16(DAT_006d4988 + iVar5 + 0x14), 0, DAT_006d4988 + iVar5);
                              FUN_004a1d70(0xdb, R32(DAT_006d4988 + 0x1b + iVar5));
                              FUN_0040a1a0(0x1f, uVar8, R32(DAT_006d4988 + 0x1b + iVar5));
                              FUN_0040a1a0(0x20, uVar25, R32(DAT_006d4988 + 0x1b + iVar5));
                            }
                            if (((DAT_007092d5 != 0) ||
                                 (cVar2 = (char)R8(DAT_006d4988 + 2 + iVar15), cVar2 == '\x04')) ||
                                ((cVar2 == '\x03' || (cVar2 == '\x05')))) {
                              FUN_004110e0(unaff_EBX, in_stack_ffffffc4, (unsigned int)local_32, 0);
                              iVar23 = DAT_006d4988 + iVar5;
                              if (R8(R32(DAT_006d4988 + 0x1f + iVar5) + 0x90) != 0) {
                                FUN_00483410((short)R16(iVar23 + 0x12), (short)R16(iVar23 + 0x10), iVar23);
                                FUN_00483540((short)R16(DAT_006d4988 + iVar5 + 0x14), 0, DAT_006d4988 + iVar5);
                              }
                              iVar23 = R32(DAT_006d4988 + 0x1f + iVar5);
                              iVar23 = FUN_00413d00((int)(short)R16(iVar23 + 0x91),
                                                    (int)(short)R16(iVar23 + 0x93),
                                                    (int)(short)R16(iVar23 + 0x95),
                                                    (int)(short)((short)DAT_0073294c - (short)DAT_0070a180),
                                                    (int)(short)((short)DAT_007329b0 - (short)DAT_0070acd8),
                                                    (int)(short)((short)DAT_007329b4 - (short)DAT_00703ad8));
                              FUN_004110e0((unsigned int)local_32, 0xffff, unaff_EBX,
                                           (unsigned int)((short)iVar23 < 0));
                            }
                          } else {
                            W16(R32(DAT_006d4988 + 0x1f + iVar5) + 0x79, 0);
                            W16(iVar10 + 0xbd, 0);
                            W32(DAT_006d4988 + 4 + iVar5, DAT_00732f7c);
                            W32(R32(DAT_006d4988 + iVar5 + 0x1f) + 6, R32(DAT_006d4988 + iVar5 + 4));
                            W32(DAT_006d4988 + 8 + iVar5, DAT_00732f8c);
                            W32(R32(DAT_006d4988 + iVar5 + 0x1f) + 10, R32(DAT_006d4988 + iVar5 + 8));
                            W32(DAT_006d4988 + 0xc + iVar5, DAT_00732f5c);
                            W32(R32(DAT_006d4988 + iVar5 + 0x1f) + 0xe, R32(DAT_006d4988 + iVar5 + 0xc));
                            W16(DAT_006d4988 + 0x14 + iVar5, DAT_006db6de);
                            W16(DAT_006d4988 + 0x12 + iVar5, DAT_0071f2d6);
                            W16(iVar10 + 0xd, R16(DAT_006d4988 + 0x12 + iVar5));
                            W16(DAT_006d4988 + 0x10 + iVar5, DAT_00732576);
                            W8(R32(DAT_006d4988 + 0x1f + iVar5) + 0x89, 1);
                            iVar23 = R32(DAT_006d4988 + 0x1f + iVar5);
                            W8(iVar23 + 0x90, R8(iVar23 + 0x89));
                            uVar25 = FUN_00478470();
                            iVar23 = R32(DAT_006d4988 + 0x1b + iVar5);
                            if ((uVar25 & 0x8000) == 0) {
                              FUN_0040a1a0(0x20, uVar8, iVar23);
                            } else {
                              FUN_0040a1a0(0x1f, uVar8, iVar23);
                            }
                          }
                        }
                      }
                      goto LAB_0040ebc5;
                    }
                    FUN_0040ed40(pbVar14, local_32, unaff_EBX);
                  }
                }
                }
              }
LAB_0040ecd4:
              local_36 = local_36 + 1;
            } while (local_36 < *pbVar14);
          }
        }
      }
LAB_0040ed02:
      local_32 = local_32 + 1;
    } while ((int)local_32 < DAT_00732904);
  }
  return;
}
