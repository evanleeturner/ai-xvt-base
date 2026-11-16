/* SPDX-License-Identifier: MIT */
/* z_bop 00430a00 */
extern unsigned char DAT_009d6940;
extern unsigned char DAT_009d6941;
extern unsigned short DAT_009ec5e2;
extern char DAT_009e9680[];
extern char DAT_009ec468[];
extern unsigned int DAT_009d6960;
extern unsigned int DAT_009d6964;
extern unsigned char DAT_009ed22b;
extern unsigned char DAT_009ed22c;
extern unsigned char DAT_009ed22d;
extern short DAT_009a2000;
extern short DAT_009a2002;
extern unsigned char DAT_009a20c0[];
extern unsigned char DAT_009d9192[];
extern unsigned char DAT_009a2196[];
extern unsigned char DAT_009d8e25[];
extern unsigned char DAT_009d8e26[];
extern unsigned char DAT_009d8e27[];
extern unsigned char DAT_009d8e28[];
extern unsigned char DAT_009d8e29[];
extern unsigned char DAT_009d8e33[];
extern int DAT_009d6968[];
extern unsigned char DAT_009a8080[];
extern unsigned char DAT_009a8081[];
extern unsigned char DAT_009a8082[];
extern unsigned char DAT_009a8083[];
extern unsigned char DAT_009a8084[];
extern unsigned char DAT_009a8085[];
extern unsigned char DAT_009a8086[];
extern unsigned char DAT_009a8087[];
extern unsigned char DAT_009a808a[];
extern unsigned char DAT_009a808b[];
extern unsigned char DAT_009a808c[];
extern unsigned char DAT_009a808d[];
extern unsigned char DAT_009a808e[];
extern unsigned char DAT_009a808f[];
extern unsigned char DAT_009a8090[];
extern unsigned char DAT_009a8091[];
extern unsigned char DAT_009a8092[];
extern unsigned char DAT_009a8095[];
extern unsigned char DAT_009a80a7[];
extern unsigned char DAT_009a80a9[];
extern unsigned short DAT_00556354;
extern unsigned short DAT_00556358;
extern unsigned short DAT_009d73a8[];
extern unsigned short DAT_009d7498[];
extern unsigned short DAT_009d73aa[];
extern unsigned short DAT_009d749a[];
extern unsigned short DAT_009d73ac[];
extern unsigned short DAT_009d749c[];
extern unsigned short DAT_009d73ae[];
extern unsigned short DAT_009d749e[];
extern char DAT_009d736c[];
extern char DAT_009d738a[];
extern char DAT_009d738b[];
extern char DAT_009a2062;
extern char DAT_009a2063;
extern char DAT_009d7368;
extern char DAT_009d736b;
extern unsigned char DAT_009fd458[];
extern unsigned char DAT_009fd462[];
extern unsigned char DAT_009fd4e2[];
extern unsigned char DAT_009fd562[];
extern unsigned int DAT_009d7588[];
extern unsigned short DAT_00a08292;
extern unsigned short DAT_00a07be0;
extern unsigned short DAT_00a07be2;
extern int DAT_00a080f8;
extern int DAT_009ecc34;
extern unsigned short DAT_009d7684;
extern unsigned char DAT_00b6a5aa;   /* width unresolved */
extern unsigned char DAT_00b6a606;   /* width unresolved */
extern short DAT_009ec5ea;
extern char DAT_009d75c4[];
extern char DAT_009d7604[];
extern unsigned char DAT_009fe84a[];
extern unsigned char DAT_009fe855[];
extern unsigned char DAT_009fe871[];
extern unsigned char DAT_009fe870[];
extern unsigned char DAT_009fe840[];
extern unsigned char DAT_009fe800[];
extern unsigned char DAT_009e967c[];

unsigned int FUN_00435a10(unsigned char a, unsigned char b, unsigned char c);
char FUN_00431c10(unsigned short p1, short p2, unsigned short p3, unsigned short p4, short p5, unsigned short p6);
void FUN_00450650(int a, int b);
void FUN_00451bf0(int a, void *b);
void FUN_00430420(int a);
void *FUN_0046f720(unsigned int a);
unsigned int FUN_00431b50(unsigned char *a, int b);

void __stdcall FUN_00430a00(void)
{
  int iVar1;
  unsigned char bVar2;
  char cVar3;
  unsigned char bVar5;
  unsigned char bVar6;
  unsigned char bVar7;
  char cVar8;
  unsigned short uVar9;
  unsigned short uVar10;
  char *pcVar11;
  unsigned int uVar12;
  unsigned int uVar13;
  int iVar14;
  unsigned int uVar15;
  int iVar16;
  int iVar17;
  unsigned int uVar18;
  unsigned short uVar19;
  unsigned int uVar20;
  unsigned int puVar21;
  int iVar22;
  unsigned char bVar23;
  char *pcVar24;
  unsigned char bVar25;
  unsigned char bStack_24;
  unsigned short uStack_22;
  unsigned short uStack_20;
  unsigned char bStack_1e;
  short sStack_1c;
  unsigned int uStack_18;
  unsigned int puStack_10;
  unsigned int uStack_c;
  int iTmp;
  unsigned int uIdx;
  unsigned short usTmp;
  unsigned short usTmp2;
  int iCond;

  if (DAT_009d6941 == 0) {
    if ((DAT_009ec5e2 == 0) || ((char)DAT_009d6940 != '\0')) {
      DAT_009d6960 = 0;
      pcVar11 = DAT_009e9680;
      do {
        if (*pcVar11 != '\0') {
          DAT_009d6960 = DAT_009d6960 + 1;
        }
        pcVar11 = pcVar11 + 0x5bd;
      } while (pcVar11 < DAT_009ec468);
      if (DAT_009d6964 < DAT_009d6960) {
        DAT_009d6964 = DAT_009d6960;
      }
      uVar12 = FUN_00435a10(DAT_009ed22b, DAT_009ed22c, DAT_009ed22d);
      uVar10 = 0;
      if (0 < DAT_009a2000) {
        do {
          uStack_c = 0;
          iVar1 = ((unsigned int)uVar10 + (unsigned int)uVar10 * 0x92) * 2;
          do {
            iCond = 0;
            if (*(char *)(&DAT_009a20c0[iVar1 + 3]) == '\0') {
              uIdx = (unsigned int)uVar10 * 0x500 + (unsigned int)uVar10 * 0x66;
              __builtin_memcpy(&iTmp, &DAT_009d9192[uIdx], 4);
              if (iTmp == -1) {
                iCond = 1;
              }
            }
            if (iCond) {
              uVar19 = 0;
              do {
                uVar9 = uVar19 + 1;
                DAT_009a2196[uStack_c + iVar1 + (unsigned int)uVar19 * 8] = 0;
                uVar19 = uVar9;
              } while (uVar9 < 10);
            }
            else {
              uVar15 = 0;
              iVar14 = uStack_c * 0x4e + (unsigned int)uVar10 * 0x500 + (unsigned int)uVar10 * 0x66;
              do {
                iVar16 = iVar1 + uVar15 * 8;
                if (DAT_009d8e29[iVar14 + uVar15] == '\0') {
                  DAT_009a2196[uStack_c + iVar16] = 0;
                }
                else if (DAT_009a2196[uStack_c + iVar16] == '\x04') {
                  bVar5 = DAT_009d8e26[iVar14];
                  if ((bVar5 == 0) || (bVar5 == 10)) {
                    DAT_009a2196[uStack_c + iVar16] = 0;
                  }
                  else {
                    cVar8 = FUN_00431c10((unsigned short)bVar5, 1, uVar10,
                                         (unsigned short)(unsigned char)DAT_009d8e27[iVar14], 0, (unsigned short)uVar15);
                    uVar13 = (unsigned int)(unsigned char)cVar8;
                    if ((short)uVar13 == 1) {
                      uVar13 = 8;
                      if (DAT_009d8e33[iVar14] != 0) {
                        uVar13 = (unsigned int)(unsigned short)((-(unsigned short)((unsigned int)(unsigned char)DAT_009d8e33[iVar14] * 5 < uVar12) & 0xfffa) + 8);
                      }
                    }
                    if (((short)uVar13 == 8) &&
                        ((DAT_009d8e25[iVar14] != '\x02') ||
                         ((DAT_009d8e27[iVar14] != '\x12') && (DAT_009d8e27[iVar14] != '\x13')))) {
                      uVar13 = 1;
                      DAT_009d6968[uVar15] = (int)((unsigned int)DAT_009d6968[uVar15] + (unsigned int)((int)(char)DAT_009d8e28[iVar14] * 0xfa));
                    }
                    DAT_009a2196[uStack_c + iVar16] = (char)uVar13;
                  }
                }
                uVar19 = (unsigned short)uVar15 + 1;
                uVar15 = (unsigned int)uVar19;
              } while (uVar19 < 10);
            }
            uVar19 = (short)uStack_c + 1;
            uStack_c = (unsigned int)uVar19;
          } while (uVar19 < 8);
          uVar10 = uVar10 + 1;
        } while ((int)(unsigned int)uVar10 < (int)DAT_009a2000);
      }
      uStack_22 = 0;
      do {
        uStack_20 = 0;
        uVar15 = (unsigned int)uStack_22;
        iVar1 = uVar15 * 3;
        do {
          bStack_1e = uStack_20 != 1;
          uVar13 = (unsigned int)uStack_20;
          iVar14 = uVar15 * 7 + uVar13;
          bVar5 = DAT_009a8080[iVar14 * 0x2a];
          bVar7 = DAT_009a808b[iVar14 * 0x2a];
          bVar2 = DAT_009a808f[iVar14 * 0x2a];
          iVar14 = iVar14 * 0x2a;
          bVar23 = DAT_009a8084[iVar14];
          bVar25 = 0;
          iVar16 = iVar1 + uVar13;
          uVar10 = 0;
          do {
            uVar19 = uVar10 + 1;
            iVar17 = (unsigned int)uVar10 + iVar16 * 4;
            DAT_009d73a8[iVar17] = 0;
            DAT_009d7498[iVar17] = 0;
            uVar10 = uVar19;
          } while (uVar19 < 4);
          if (((((bVar5 == 10) || (bVar5 == 0)) && ((bVar23 == 10) || (bVar23 == 0))) &&
               ((bVar7 == 10) || (bVar7 == 0))) && ((bVar2 == 10) || (bVar2 == 0))) {
            cVar8 = '\0';
          }
          else {
            uVar10 = 10;
            bVar25 = 1;
            if ((bVar23 == 0x27) && (DAT_009a8085[iVar14] == '\f')) {
              uVar10 = (unsigned short)(unsigned char)DAT_009a8086[iVar14];
            }
            bVar5 = FUN_00431c10((unsigned short)bVar5, (unsigned short)(unsigned char)DAT_009a8081[iVar14],
                                 (unsigned short)(unsigned char)DAT_009a8082[iVar14],
                                 (unsigned short)(unsigned char)DAT_009a8083[iVar14], 0, uVar10);
            usTmp = DAT_00556354;
            DAT_009d73a8[iVar16 * 4] = DAT_00556358;
            DAT_009d7498[iVar16 * 4] = usTmp;
            if (bVar23 == 0x27) {
              if ((bVar5 & 1) == 0) goto LAB_00430ed7;
              bVar23 = 1;
            }
            else {
              bVar6 = FUN_00431c10((unsigned short)bVar23, (unsigned short)(unsigned char)DAT_009a8085[iVar14],
                                   (unsigned short)(unsigned char)DAT_009a8086[iVar14],
                                   (unsigned short)(unsigned char)DAT_009a8087[iVar14], 0, uVar10);
              usTmp = DAT_00556354;
              DAT_009d73aa[iVar16 * 4] = DAT_00556358;
              DAT_009d749a[iVar16 * 4] = usTmp;
              if (DAT_009a808a[iVar14] == '\x01') {
                if (((bVar6 | bVar5) & 1) == 0) {
                  bVar5 = bVar6 & bVar5;
LAB_00430ed7:
                  bVar23 = (-((bVar5 & 2) == 0) & 2U) + 2;
                }
                else {
                  bVar23 = 1;
                }
              }
              else {
                bVar23 = 1;
                if ((bVar6 & bVar5 & 1) == 0) {
                  bVar5 = bVar6 | bVar5;
                  goto LAB_00430ed7;
                }
              }
            }
            uVar10 = 10;
            if ((bVar2 == 0x27) && (DAT_009a8090[iVar14] == '\f')) {
              uVar10 = (unsigned short)(unsigned char)DAT_009a8091[iVar14];
            }
            bVar5 = FUN_00431c10((unsigned short)bVar7, (unsigned short)(unsigned char)DAT_009a808c[iVar14],
                                 (unsigned short)(unsigned char)DAT_009a808d[iVar14],
                                 (unsigned short)(unsigned char)DAT_009a808e[iVar14], 0, uVar10);
            usTmp = DAT_00556354;
            DAT_009d73ac[iVar16 * 4] = DAT_00556358;
            DAT_009d749c[iVar16 * 4] = usTmp;
            if (bVar2 == 0x27) {
              if ((bVar5 & 1) == 0) {
                bVar23 = (-((bVar5 & 2) == 0) & 2U) + 2;
              }
              else {
                bVar23 = 1;
              }
            }
            else {
              bVar7 = FUN_00431c10((unsigned short)bVar2, (unsigned short)(unsigned char)DAT_009a8090[iVar14],
                                   (unsigned short)(unsigned char)DAT_009a8091[iVar14],
                                   (unsigned short)(unsigned char)DAT_009a8092[iVar14], 0, uVar10);
              usTmp = DAT_00556354;
              DAT_009d73ae[iVar16 * 4] = DAT_00556358;
              DAT_009d749e[iVar16 * 4] = usTmp;
              if (DAT_009a8095[iVar14] == '\x01') {
                if (((bVar7 | bVar5) & 1) == 0) {
                  bVar7 = bVar7 & bVar5;
LAB_00431023:
                  bStack_24 = (-((bVar7 & 2) == 0) & 2U) + 2;
                }
                else {
                  bStack_24 = 1;
                }
              }
              else {
                if ((bVar7 & bVar5 & 1) == 0) {
                  bVar7 = bVar7 | bVar5;
                  goto LAB_00431023;
                }
                bStack_24 = 1;
              }
            }
            if (DAT_009a80a7[iVar14] == '\x01') {
              if (((bStack_24 | bVar23) & 1) == 0) {
                cVar8 = (-((bStack_24 & bVar23 & 2) == 0) & 2U) + 2;
              }
              else {
                cVar8 = '\x01';
              }
            }
            else if ((bStack_24 & bVar23 & 1) == 0) {
              cVar8 = (-(((bStack_24 | bVar23) & 2) == 0) & 2U) + 2;
            }
            else {
              cVar8 = '\x01';
            }
          }
          if (uStack_20 == 1) {
            if (cVar8 == '\x01') {
              bStack_1e = 1;
              cVar3 = DAT_009d736c[iVar16];
              goto joined_r0x00431111;
            }
          }
          else if (cVar8 == '\x01') {
            cVar3 = DAT_009d736c[iVar16];
joined_r0x00431111:
            bVar25 = 1;
            if (cVar3 != '\x01') {
              bVar25 = 1;
              DAT_009d6968[uVar15] = (int)((unsigned int)DAT_009d6968[uVar15] + (unsigned int)((int)(char)DAT_009a80a9[iVar14] * 0xfa));
            }
          }
          else if (cVar8 == '\x02') {
            bStack_1e = 1;
          }
          else if (cVar8 == '\x04') {
            bStack_1e = 0;
          }
          DAT_009d736c[iVar16] = cVar8;
          uVar10 = 0;
          if (0 < DAT_009a2000) {
            do {
              uVar20 = 0;
              uVar18 = (unsigned int)uVar10;
              do {
                iVar14 = uVar20 * 0x4e + uVar18 * 0x566;
                if ((DAT_009d8e29[iVar14 + uVar15] != '\0') &&
                    ((unsigned char)DAT_009d8e25[iVar14] == uVar13)) {
                  if (uStack_20 == 1) {
                    if (DAT_009a2196[uVar20 + uVar15 * 8 + uVar18 * 0x126] == '\x01') {
                      bStack_1e = 1;
                      goto LAB_0043121f;
                    }
                  }
                  else {
                    cVar8 = DAT_009a2196[uVar20 + uVar15 * 8 + uVar18 * 0x126];
                    if (cVar8 == '\x01') {
LAB_0043121f:
                      bVar25 = 1;
                    }
                    else if (cVar8 == '\x02') {
                      bStack_1e = 1;
                    }
                    else if ((cVar8 == '\x04') && (bStack_1e != 1)) {
                      bStack_1e = 0;
                    }
                  }
                }
                uVar19 = (short)uVar20 + 1;
                uVar20 = (unsigned int)uVar19;
              } while (uVar19 < 8);
              uVar10 = uVar10 + 1;
            } while ((int)(unsigned int)uVar10 < (int)DAT_009a2000);
          }
          if ((bStack_1e == 1) && (!bVar25)) {
            bStack_1e = 0;
          }
          if ((((DAT_009a2062 == '\x02') && (uStack_20 == 1)) && (bStack_1e == 1)) &&
              (DAT_009d738a[iVar1] == '\x01')) {
            bStack_1e = 0;
          }
          if (((uStack_20 == 0) && (DAT_009a2062 == '\x02')) && (DAT_009a2063 != '\0')) {
            bStack_1e = 0;
          }
          if ((((DAT_009d738a[iVar16] != 0) != bStack_1e) && (bStack_1e != 0)) &&
              ((DAT_009d738a[iVar16] != '\x01') && (DAT_009a2063 == '\0'))) {
            DAT_009d738a[iVar16] = bStack_1e;
            if (bStack_1e == 1) {
              iVar16 = 10;
              iVar14 = 0;
              pcVar24 = &DAT_009d738a[uVar13];
              pcVar11 = pcVar24;
              do {
                if (*pcVar11 == '\x01') {
                  iVar14 = iVar14 + 1;
                }
                sStack_1c = (short)iVar14;
                pcVar11 = pcVar11 + 3;
                iVar16 = iVar16 + -1;
              } while (iVar16 != 0);
              if (uStack_20 == 0) {
                iVar16 = 0;
                if (DAT_009a2062 == '\x04') {
                  uVar13 = 0;
                  iVar17 = 0;
                  do {
                    bVar25 = 0;
                    if (uVar15 != uVar13) {
                      bVar25 = DAT_009fd458[uVar15 + iVar17] == '\0';
                    }
                    if ((bVar25) && (*pcVar24 == '\x01')) {
                      iVar16 = iVar16 + 1;
                    }
                    pcVar24 = pcVar24 + 3;
                    iVar17 = iVar17 + 0x1e5;
                    uVar13 = uVar13 + 1;
                  } while (iVar17 < 0x12f2);
                }
                if (iVar16 == 0) {
                  DAT_009d6968[uVar15] = (int)((unsigned int)DAT_009d6968[uVar15] + 10000u);
                }
                DAT_009d7588[uVar15] = uVar12;
                DAT_009d7368 = 1;
              }
              else if (uStack_20 == 2) {
                DAT_009d736b = 1;
                DAT_009d6968[uVar15] = (int)((unsigned int)DAT_009d6968[uVar15] + 0x9c4u);
              }
              if (uStack_20 == 0) {
                puVar21 = 0;
                pcVar11 = DAT_009e9680;
                puStack_10 = 0xffffffff;
                do {
                  if (*pcVar11 != '\0') {
                    __builtin_memcpy(&usTmp, pcVar11 - 4, 2);
                    if (usTmp == uStack_22) {
                      puStack_10 = puVar21;
                    }
                  }
                  pcVar11 = pcVar11 + 0x5bd;
                  puVar21 = puVar21 + 1;
                } while (pcVar11 < DAT_009ec468);
                iVar16 = 0;
                pcVar11 = DAT_009e9680;
                do {
                  cVar8 = DAT_009a2062;
                  if (*pcVar11 != '\0') {
                    __builtin_memcpy(&usTmp, pcVar11 - 4, 2);
                    if (usTmp == uStack_22) {
                      __builtin_memcpy(pcVar11 + 0x71, &iVar14, 4);
                      __builtin_memcpy(&usTmp, pcVar11 - 6, 2);
                      DAT_00a08292 = usTmp;
                      bVar25 = 1;
                      if ((cVar8 == '\x04') && (DAT_009d738b[iVar1] == '\x01')) {
                        bVar25 = 0;
                      }
                      if (((DAT_00b6a5aa == 0) && (DAT_00b6a606 == 1)) &&
                          (DAT_009d738b[iVar1] == '\x01')) {
                        bVar25 = 0;
                      }
                      if (bVar25) {
                        FUN_00450650(0xc1, iVar16);
                        if ((DAT_009a2062 == '\x02') && (1 < DAT_00a080f8)) {
                          DAT_00a07be0 = (unsigned short)(sStack_1c + 0x126);
                          FUN_00450650(0x131, iVar16);
                        }
                        if (iVar16 == DAT_009ecc34) {
                          uStack_18 = 0;
                          iVar17 = uVar15 * 0x1e5;
                          do {
                            iVar22 = uStack_18 * 0x40;
                            if (*(char *)(&DAT_009fd462[iVar17 + iVar22]) != '\0') {
                              FUN_00451bf0(0, &DAT_009fd462[iVar17 + iVar22]);
                              DAT_009d7684 = 0x6f;
                              if ((short)uStack_18 != 0) {
                                DAT_009d7684 = 0;
                              }
                              bVar5 = DAT_009fd462[iVar17 + iVar22];
                              if ((bVar5 < 0x31) || (0x36 < bVar5)) {
                                iVar22 = 0xc4;
                              }
                              else {
                                iVar22 = 0xcf;
                              }
                              FUN_00450650(iVar22, iVar16);
                            }
                            uVar10 = (short)uStack_18 + 1;
                            uStack_18 = (unsigned int)uVar10;
                          } while (uVar10 < 2);
                          if (DAT_009a2062 == '\x04') {
                            FUN_00430420(1);
                          }
                          else {
                            FUN_00430420(0);
                          }
                        }
                      }
                      else {
                        if (uStack_22 == 0) {
                          iVar17 = 0xd0;
                        }
                        else {
                          iVar17 = 0xd1;
                        }
LAB_004315db:
                        FUN_00450650(iVar17, iVar16);
                      }
                    }
                    else {
                      DAT_00a07be2 = (unsigned short)(sStack_1c + 0x126);
                      if (((DAT_009a2062 == '\x02') || (DAT_009a2062 == '\x01')) &&
                          (puStack_10 != 0xffffffff)) {
                        FUN_00451bf0(0, FUN_0046f720(puStack_10));
                        iVar17 = 0x132;
                        goto LAB_004315db;
                      }
                    }
                  }
                  pcVar11 = pcVar11 + 0x5bd;
                  iVar16 = iVar16 + 1;
                } while (pcVar11 < DAT_009ec468);
              }
              else if (uStack_20 == 1) {
                iVar14 = 0;
                pcVar11 = DAT_009e9680;
                do {
                  iCond = 0;
                  if (*pcVar11 != '\0') {
                    __builtin_memcpy(&usTmp, pcVar11 - 4, 2);
                    if ((usTmp == uStack_22) &&
                        ((DAT_009d738a[iVar1] != '\x02') && (iVar14 == DAT_009ecc34))) {
                      iCond = 1;
                    }
                  }
                  if (iCond) {
                    bVar25 = 1;
                    if ((DAT_009a2062 == '\x04') && (DAT_009d738a[iVar1] == '\x01')) {
                      bVar25 = 0;
                    }
                    if (bVar25) {
                      __builtin_memcpy(&usTmp, pcVar11 - 6, 2);
                      DAT_00a08292 = usTmp;
                      FUN_00450650(0xce, iVar14);
                      uStack_18 = 0;
                      iVar16 = uVar15 * 0x1e5;
                      do {
                        iVar17 = uStack_18 * 0x40;
                        if (*(char *)(&DAT_009fd4e2[iVar16 + iVar17]) != '\0') {
                          FUN_00451bf0(0, &DAT_009fd4e2[iVar16 + iVar17]);
                          DAT_009d7684 = 0x71;
                          if ((short)uStack_18 != 0) {
                            DAT_009d7684 = 0;
                          }
                          bVar5 = DAT_009fd4e2[iVar16 + iVar17];
                          if ((bVar5 < 0x31) || (0x36 < bVar5)) {
                            iVar17 = 0xc4;
                          }
                          else {
                            iVar17 = 0xcf;
                          }
                          FUN_00450650(iVar17, iVar14);
                        }
                        uVar10 = (short)uStack_18 + 1;
                        uStack_18 = (unsigned int)uVar10;
                      } while (uVar10 < 2);
                      if (DAT_009a2062 == '\x04') {
                        FUN_00430420(7);
                      }
                      else {
                        FUN_00430420(6);
                      }
                    }
                    else {
                      if (uStack_22 == 0) {
                        iVar16 = 0xd2;
                      }
                      else {
                        iVar16 = 0xd3;
                      }
                      FUN_00450650(iVar16, iVar14);
                    }
                  }
                  pcVar11 = pcVar11 + 0x5bd;
                  iVar14 = iVar14 + 1;
                } while (pcVar11 < DAT_009ec468);
              }
              else {
                iVar14 = 0;
                pcVar11 = DAT_009e9680;
                do {
                  iCond = 0;
                  if (*pcVar11 != '\0') {
                    __builtin_memcpy(&usTmp, pcVar11 - 4, 2);
                    if ((usTmp == uStack_22) && (iVar14 == DAT_009ecc34)) {
                      iCond = 1;
                    }
                  }
                  if (iCond) {
                    uStack_18 = 0;
                    iVar16 = uVar15 * 0x1e5;
                    do {
                      iVar17 = uStack_18 * 0x40;
                      if (*(char *)(&DAT_009fd562[iVar16 + iVar17]) != '\0') {
                        FUN_00451bf0(0, &DAT_009fd562[iVar16 + iVar17]);
                        DAT_009d7684 = 0x70;
                        if ((short)uStack_18 != 0) {
                          DAT_009d7684 = 0;
                        }
                        __builtin_memcpy(&usTmp, pcVar11 - 6, 2);
                        DAT_00a08292 = usTmp;
                        bVar5 = DAT_009fd562[iVar16 + iVar17];
                        if ((bVar5 < 0x31) || (0x36 < bVar5)) {
                          iVar17 = 0xc4;
                        }
                        else {
                          iVar17 = 0xcf;
                        }
                        FUN_00450650(iVar17, iVar14);
                      }
                      uVar10 = (short)uStack_18 + 1;
                      uStack_18 = (unsigned int)uVar10;
                    } while (uVar10 < 2);
                  }
                  pcVar11 = pcVar11 + 0x5bd;
                  iVar14 = iVar14 + 1;
                } while (pcVar11 < DAT_009ec468);
              }
            }
            else if ((bStack_1e == 1) && (uStack_20 == 0)) {
              iVar14 = 0;
              pcVar11 = DAT_009e9680;
              do {
                iCond = 0;
                if (*pcVar11 != '\0') {
                  __builtin_memcpy(&usTmp, pcVar11 - 4, 2);
                  if ((usTmp == uStack_22) &&
                      (DAT_009d738b[iVar1] != '\x01') && (iVar14 == DAT_009ecc34)) {
                    iCond = 1;
                  }
                }
                if (iCond) {
                  __builtin_memcpy(&usTmp, pcVar11 - 6, 2);
                  DAT_00a08292 = usTmp;
                  FUN_00450650(0xce, iVar14);
                  uStack_18 = 0;
                  iVar16 = uVar15 * 0x1e5;
                  do {
                    iVar17 = uStack_18 * 0x40;
                    if (*(char *)(&DAT_009fd4e2[iVar16 + iVar17]) != '\0') {
                      FUN_00451bf0(0, &DAT_009fd4e2[iVar16 + iVar17]);
                      DAT_009d7684 = 0x71;
                      if ((short)uStack_18 != 0) {
                        DAT_009d7684 = 0;
                      }
                      bVar5 = DAT_009fd4e2[iVar16 + iVar17];
                      if ((bVar5 < 0x31) || (0x36 < bVar5)) {
                        iVar17 = 0xc4;
                      }
                      else {
                        iVar17 = 0xcf;
                      }
                      FUN_00450650(iVar17, iVar14);
                    }
                    uVar10 = (short)uStack_18 + 1;
                    uStack_18 = (unsigned int)uVar10;
                  } while (uVar10 < 2);
                  if (DAT_009a2062 == '\x04') {
                    iVar16 = 7;
                  }
                  else {
                    iVar16 = 6;
                  }
                  FUN_00430420(iVar16);
                }
                pcVar11 = pcVar11 + 0x5bd;
                iVar14 = iVar14 + 1;
              } while (pcVar11 < DAT_009ec468);
            }
          }
          uStack_20 = uStack_20 + 1;
        } while (uStack_20 < 3);
        uStack_22 = uStack_22 + 1;
      } while (uStack_22 < 10);
      DAT_009ec5e2 = 0xec;
    }
    if (DAT_009ec5ea < 1) {
      uVar10 = 0;
      if (0 < DAT_009a2002) {
        do {
          uVar12 = (unsigned int)uVar10;
          if (DAT_009d75c4[uVar12] == '\0') {
            iVar1 = uVar12 * 0x72;
            uVar15 = FUN_00431b50(&DAT_009fe84a[uVar12 * 0x72], 0);
            uVar13 = FUN_00431b50(&DAT_009fe855[iVar1], 0);
            if (DAT_009fe871[iVar1] == '\x01') {
              uVar19 = (unsigned short)uVar15 | (unsigned short)uVar13;
            }
            else {
              uVar19 = (unsigned short)uVar15 & (unsigned short)uVar13;
            }
            if ((uVar19 & 1) != 0) {
              DAT_009d75c4[uVar12] = 1;
              cVar8 = DAT_009fe870[iVar1];
              DAT_009d7604[uVar12] = cVar8;
              if (cVar8 == '\0') {
                __builtin_memcpy(&usTmp2, &DAT_009e967c[DAT_009ecc34 * 0x5bd], 2);
                if (DAT_009fe840[iVar1 + (unsigned int)usTmp2] != '\0') {
                  FUN_00451bf0(0, &DAT_009fe800[iVar1]);
                  if (0xf < uVar10) goto LAB_00431b09;
                  DAT_009d7684 = uVar10 + 0x5f;
                  goto LAB_00431b12;
                }
              }
            }
          }
          else {
            if (DAT_009d7604[uVar12] != '\0') {
              cVar8 = DAT_009d7604[uVar12] + -1;
              DAT_009d7604[uVar12] = cVar8;
              if (cVar8 == '\0') {
                __builtin_memcpy(&usTmp2, &DAT_009e967c[DAT_009ecc34 * 0x5bd], 2);
                if (DAT_009fe840[uVar12 * 0x72 + (unsigned int)usTmp2] != '\0') {
                  FUN_00451bf0(0, &DAT_009fe800[uVar12 * 0x72]);
                  if (uVar10 < 0x10) {
                    DAT_009d7684 = uVar10 + 0x5f;
                  }
                  else {
LAB_00431b09:
                    DAT_009d7684 = 0;
                  }
LAB_00431b12:
                  FUN_00450650(0xcf, DAT_009ecc34);
                }
              }
            }
          }
          uVar10 = uVar10 + 1;
        } while ((int)(unsigned int)uVar10 < (int)DAT_009a2002);
      }
      DAT_009ec5ea = 0x49c;
    }
  }
  return;
}
