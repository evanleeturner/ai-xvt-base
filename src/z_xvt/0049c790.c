/* SPDX-License-Identifier: MIT */
/* z_xvt 0049c790 */
extern char DAT_007092c1;
extern char DAT_007092c0;
extern unsigned short DAT_0071e6a2;   /* DAT_0071e6a0._2_2_ */
extern unsigned short DAT_0071e6aa;   /* DAT_0071e6a8._2_2_ */
extern int DAT_007092e0;
extern unsigned int DAT_007092e4;
extern char DAT_0071b740[];
extern char DAT_0071e528[];
extern char DAT_0071b73c[];
extern unsigned char DAT_0071f2dd;
extern unsigned char DAT_0071f2db;    /* DAT_0071f2d8._3_1_ */
extern unsigned char DAT_0071f2dc;
extern short DAT_006d49a0;
extern short DAT_006d49a2;
extern unsigned char DAT_00524900[];
extern unsigned char DAT_005248f8[];
extern int DAT_007092d4;
extern unsigned char DAT_0070ad5d[];
extern unsigned char DAT_0070b252[];
extern char DAT_006d4b36[];
extern unsigned char DAT_0070aee9[];
extern unsigned char DAT_0070aee6[];
extern unsigned char DAT_0070aee7[];
extern unsigned char DAT_0070aef3[];
extern unsigned char DAT_0070aee5[];
extern unsigned char DAT_0070aee8[];
extern int DAT_007092e8[];
extern unsigned char DAT_006daa00[];
extern unsigned char DAT_006daa01[];
extern unsigned char DAT_006daa02[];
extern unsigned char DAT_006daa03[];
extern unsigned char DAT_006daa04[];
extern unsigned char DAT_006daa05[];
extern unsigned char DAT_006daa06[];
extern unsigned char DAT_006daa07[];
extern unsigned char DAT_006daa0a[];
extern unsigned char DAT_006daa0b[];
extern unsigned char DAT_006daa0c[];
extern unsigned char DAT_006daa0d[];
extern unsigned char DAT_006daa0e[];
extern unsigned char DAT_006daa0f[];
extern unsigned char DAT_006daa10[];
extern unsigned char DAT_006daa11[];
extern unsigned char DAT_006daa12[];
extern unsigned char DAT_006daa15[];
extern unsigned char DAT_006daa27[];
extern unsigned char DAT_006daa29[];
extern unsigned short DAT_00645ad8;
extern unsigned short DAT_00645adc;
extern unsigned short DAT_00709d28[];
extern unsigned short DAT_00709e18[];
extern unsigned short DAT_00709d2a[];
extern unsigned short DAT_00709e1a[];
extern unsigned short DAT_00709d2c[];
extern unsigned short DAT_00709e1c[];
extern unsigned short DAT_00709d2e[];
extern unsigned short DAT_00709e1e[];
extern char DAT_00709cec[];
extern char DAT_00709d0a[];
extern char DAT_00709d0b[];
extern char DAT_006d4a02;
extern char DAT_006d4a03;
extern int DAT_00709f08[];
extern char DAT_00709ce8;
extern char DAT_00709ceb;
extern unsigned short DAT_00732fe2;
extern short DAT_00732950;
extern short DAT_00732952;
extern short DAT_00732e48;
extern int DAT_0071ecf4;
extern char DAT_0072f512[];
extern char DAT_0072f592[];
extern char DAT_0072f612[];
extern unsigned short DAT_0070a004;
extern char DAT_00709f44[];
extern char DAT_00709f84[];
extern char DAT_007308fa[];
extern char DAT_00730905[];
extern char DAT_00730921[];
extern char DAT_00730920[];
extern char DAT_007308f0[];
extern char DAT_007308b0[];

char FUN_0049d8b0(unsigned short param_1, short param_2, unsigned short param_3,
                  unsigned short param_4, short param_5, unsigned short param_6);
unsigned char FUN_004440b0(unsigned int param_1, int param_2);
void FUN_004a1d70(int param_1, int param_2);
void FUN_004a3490(int param_1, void *param_2);
void FUN_0040c290(int param_1);
int *FUN_00432cc0(int param_1);
unsigned int FUN_0049d810(char *param_1, int param_2);

void __stdcall FUN_0049c790(void)
{
  int iVar1;
  char cVar2;
  unsigned char bVar3;
  unsigned short uVar4;
  short sVar5;
  unsigned char bVar6;
  unsigned char bVar7;
  unsigned char bVar8;
  unsigned char bVar9;
  unsigned char bVar10;
  char cVar11;
  unsigned short uVar12;
  char *pcVar13;
  unsigned int uVar14;
  unsigned int uVar15;
  int *piVar16;
  unsigned int uVar17;
  unsigned char bVar18;
  int iVar19;
  int iVar20;
  int iVar21;
  unsigned int uVar22;
  unsigned short uVar23;
  unsigned short uVar24;
  unsigned short uStack_22;
  unsigned short uStack_20;
  unsigned char bStack_1e;
  unsigned short uStack_1c;
  int iStack_18;
  int iStack_10;
  unsigned short usTmp;
  unsigned short usTmp2;

  if (DAT_007092c1 == '\0') {
    if ((DAT_0071e6a2 == 0) || (DAT_007092c0 != '\0')) {
      DAT_007092e0 = 0;
      pcVar13 = DAT_0071b740;
      do {
        if (*pcVar13 != '\0') {
          DAT_007092e0 = DAT_007092e0 + 1;
        }
        pcVar13 = pcVar13 + 0x5bd;
      } while (pcVar13 < DAT_0071e528);
      if (DAT_007092e4 < (unsigned int)DAT_007092e0) {
        DAT_007092e4 = DAT_007092e0;
      }
      uVar12 = 0;
      uVar14 = (unsigned int)DAT_0071f2dd +
               ((unsigned int)DAT_0071f2db * 0x3c + (unsigned int)DAT_0071f2dc) * 0x3c;
      if (0 < DAT_006d49a0) {
        do {
          uVar22 = (unsigned int)uVar12;
          uVar24 = 0;
          iVar1 = uVar22 * 0x566;
          do {
            int iTmpI;
            __builtin_memcpy(&iTmpI, DAT_0070b252 + iVar1, 4);
            if ((((DAT_00524900[DAT_007092d4] & DAT_005248f8[DAT_0070ad5d[iVar1]]) == 0)) &&
                (iTmpI == -1)) {
              uVar17 = 0;
              do {
                uVar23 = (unsigned short)((short)uVar17 + 1);
                DAT_006d4b36[(unsigned int)uVar24 + uVar22 * 0x126 + uVar17 * 8] = 0;
                uVar17 = (unsigned int)uVar23;
              } while (uVar23 < 10);
            }
            else {
              uStack_20 = 0;
              iVar19 = iVar1 + (unsigned int)uVar24 * 0x4e;
              do {
                uVar17 = (unsigned int)uStack_20;
                if (DAT_0070aee9[iVar19 + uVar17] == '\0') {
                  DAT_006d4b36[(unsigned int)uVar24 + uVar17 * 8 + uVar22 * 0x126] = 0;
                }
                else {
                  iVar20 = uVar17 * 8 + uVar22 * 0x126;
                  if (DAT_006d4b36[(unsigned int)uVar24 + iVar20] == '\x04') {
                    bVar6 = DAT_0070aee6[iVar19];
                    if ((bVar6 == 0) || (bVar6 == 10)) {
                      DAT_006d4b36[(unsigned int)uVar24 + iVar20] = 0;
                    }
                    else {
                      cVar11 = FUN_0049d8b0((unsigned short)bVar6, 1, uVar12,
                                            (unsigned short)(unsigned char)DAT_0070aee7[iVar19], 0,
                                            uStack_20);
                      uVar15 = (unsigned int)(unsigned char)cVar11;
                      if ((short)uVar15 == 1) {
                        uVar15 = 8;
                        if (DAT_0070aef3[iVar19] != 0) {
                          uVar15 = (unsigned int)(unsigned short)(
                              ((0u - (unsigned int)((unsigned int)DAT_0070aef3[iVar19] * 5 < uVar14)) &
                               0xfffau) + 8u);
                        }
                      }
                      if (((short)uVar15 == 8) &&
                          ((DAT_0070aee5[iVar19] != '\x02') ||
                           ((DAT_0070aee7[iVar19] != '\x12') && (DAT_0070aee7[iVar19] != '\x13')))) {
                        uVar15 = 1;
                        DAT_007092e8[uVar17] =
                            (int)((unsigned int)DAT_007092e8[uVar17] +
                                  (unsigned int)((int)(char)DAT_0070aee8[iVar19] * 0xfa));
                      }
                      DAT_006d4b36[(unsigned int)uVar24 + iVar20] = (char)uVar15;
                    }
                  }
                }
                uStack_20 = (unsigned short)(uStack_20 + 1);
              } while (uStack_20 < 10);
            }
            uVar24 = (unsigned short)(uVar24 + 1);
          } while (uVar24 < 8);
          uVar12 = (unsigned short)(uVar12 + 1);
        } while ((int)(unsigned int)uVar12 < (int)DAT_006d49a0);
      }
      uStack_20 = 0;
      do {
        uStack_22 = 0;
        uVar22 = (unsigned int)uStack_20;
        iVar1 = uVar22 * 3;
        do {
          bVar10 = 0;
          bStack_1e = (uStack_22 != 1);
          iVar19 = (int)(uVar22 * 7 + (unsigned int)uStack_22);
          bVar6 = DAT_006daa00[iVar19 * 0x2a];
          bVar7 = DAT_006daa04[iVar19 * 0x2a];
          bVar9 = DAT_006daa0f[iVar19 * 0x2a];
          iVar19 = iVar19 * 0x2a;
          bVar8 = DAT_006daa0b[iVar19];
          iVar20 = (unsigned int)uStack_22 + iVar1;
          uVar12 = 0;
          do {
            uVar24 = (unsigned short)(uVar12 + 1);
            iVar21 = (unsigned int)uVar12 + iVar20 * 4;
            DAT_00709d28[iVar21] = 0;
            DAT_00709e18[iVar21] = 0;
            uVar12 = uVar24;
          } while (uVar24 < 4);
          if (((((bVar6 == 10) || (bVar6 == 0)) && ((bVar7 == 10) || (bVar7 == 0))) &&
               ((bVar8 == 10) || (bVar8 == 0))) &&
              ((bVar9 == 10) || (bVar9 == 0))) {
            cVar11 = '\0';
          }
          else {
            bVar10 = 1;
            bVar6 = (unsigned char)FUN_0049d8b0((unsigned short)bVar6,
                                 (unsigned short)DAT_006daa01[iVar19],
                                 (unsigned short)DAT_006daa02[iVar19],
                                 (unsigned short)DAT_006daa03[iVar19], 0, 10);
            uVar4 = DAT_00645ad8;
            DAT_00709d28[iVar20 * 4] = DAT_00645adc;
            DAT_00709e18[iVar20 * 4] = uVar4;
            bVar7 = (unsigned char)FUN_0049d8b0((unsigned short)bVar7,
                                 (unsigned short)DAT_006daa05[iVar19],
                                 (unsigned short)DAT_006daa06[iVar19],
                                 (unsigned short)DAT_006daa07[iVar19], 0, 10);
            uVar4 = DAT_00645ad8;
            DAT_00709d2a[iVar20 * 4] = DAT_00645adc;
            cVar11 = (char)DAT_006daa0a[iVar19];
            DAT_00709e1a[iVar20 * 4] = uVar4;
            if (cVar11 == '\x01') {
              if (((bVar7 | bVar6) & 1) == 0) {
                bVar7 = bVar7 & bVar6;
                goto LAB_0049cc37;
              }
              bVar7 = 0;
              bVar6 = 1;
            }
            else {
              if ((bVar7 & bVar6 & 1) == 0) {
                bVar7 = bVar7 | bVar6;
                goto LAB_0049cc37;
              }
              bVar7 = 0;
              bVar6 = 1;
            }
            goto LAB_0049cc37_done;
LAB_0049cc37:
            bVar7 = (unsigned char)(((0u - (unsigned int)((bVar7 & 2) == 0)) & 2U) + 2);
            bVar6 = 0;
LAB_0049cc37_done:
            bVar8 = (unsigned char)FUN_0049d8b0((unsigned short)bVar8,
                                 (unsigned short)DAT_006daa0c[iVar19],
                                 (unsigned short)DAT_006daa0d[iVar19],
                                 (unsigned short)DAT_006daa0e[iVar19], 0, 10);
            uVar4 = DAT_00645ad8;
            DAT_00709d2c[iVar20 * 4] = DAT_00645adc;
            DAT_00709e1c[iVar20 * 4] = uVar4;
            bVar9 = (unsigned char)FUN_0049d8b0((unsigned short)bVar9,
                                 (unsigned short)DAT_006daa10[iVar19],
                                 (unsigned short)DAT_006daa11[iVar19],
                                 (unsigned short)DAT_006daa12[iVar19], 0, 10);
            uVar4 = DAT_00645ad8;
            DAT_00709d2e[iVar20 * 4] = DAT_00645adc;
            cVar11 = (char)DAT_006daa15[iVar19];
            DAT_00709e1e[iVar20 * 4] = uVar4;
            if (cVar11 == '\x01') {
              if (((bVar9 | bVar8) & 1) == 0) {
                bVar9 = bVar9 & bVar8;
                goto LAB_0049cd32;
              }
              bVar18 = 0;
              bVar3 = 1;
            }
            else {
              bVar18 = 0;
              bVar3 = 1;
              if ((bVar9 & bVar8 & 1) == 0) {
                bVar9 = bVar9 | bVar8;
                goto LAB_0049cd32;
              }
            }
            goto LAB_0049cd32_done;
LAB_0049cd32:
            bVar18 = (unsigned char)(((0u - (unsigned int)((bVar9 & 2) == 0)) & 2U) + 2);
            bVar3 = 0;
LAB_0049cd32_done:
            if (DAT_006daa27[iVar19] == '\x01') {
              if ((bVar6 | bVar3) != 0) {
                cVar11 = '\x01';
              }
              else {
                cVar11 = (char)(((0u - (unsigned int)((bVar7 & bVar18 & 2) == 0)) & 2U) + 2);
              }
            }
            else if ((bVar6 & bVar3) != 0) {
              cVar11 = '\x01';
            }
            else {
              cVar11 = (char)(((0u - (unsigned int)(((bVar7 | bVar18) & 2) == 0)) & 2U) + 2);
            }
          }
          if (uStack_22 == 1) {
            if (cVar11 == '\x01') {
              bStack_1e = 1;
              cVar2 = DAT_00709cec[iVar20];
              goto joined_r0x0049ce0f;
            }
          }
          else if (cVar11 == '\x01') {
            cVar2 = DAT_00709cec[iVar20];
joined_r0x0049ce0f:
            bVar10 = 1;
            if (cVar2 != '\x01') {
              bVar10 = 1;
              DAT_007092e8[uVar22] =
                  (int)((unsigned int)DAT_007092e8[uVar22] +
                        (unsigned int)((int)(char)DAT_006daa29[iVar19] * 0xfa));
            }
          }
          else if (cVar11 == '\x02') {
            bStack_1e = 1;
          }
          else if (cVar11 == '\x04') {
            bStack_1e = 0;
          }
          sVar5 = DAT_006d49a0;
          uVar17 = 0;
          DAT_00709cec[iVar20] = cVar11;
          if (0 < sVar5) {
            do {
              uVar15 = 0;
              do {
                iVar19 = uVar15 * 0x4e + uVar17 * 0x566;
                if ((DAT_0070aee9[iVar19 + uVar22] != '\0') &&
                    ((unsigned char)DAT_0070aee5[iVar19] == uStack_22)) {
                  if (uStack_22 == 1) {
                    if (DAT_006d4b36[uVar15 + uVar22 * 8 + uVar17 * 0x126] == '\x01') {
                      bStack_1e = 1;
                      goto LAB_0049cf11;
                    }
                  }
                  else {
                    cVar11 = DAT_006d4b36[uVar15 + uVar22 * 8 + uVar17 * 0x126];
                    if (cVar11 == '\x01') {
LAB_0049cf11:
                      bVar10 = 1;
                    }
                    else if (cVar11 == '\x02') {
                      bStack_1e = 1;
                    }
                    else if ((cVar11 == '\x04') && (bStack_1e != 1)) {
                      bStack_1e = 0;
                    }
                  }
                }
                uVar12 = (unsigned short)((short)uVar15 + 1);
                uVar15 = (unsigned int)uVar12;
              } while (uVar12 < 8);
              uVar12 = (unsigned short)((short)uVar17 + 1);
              uVar17 = (unsigned int)uVar12;
            } while ((int)(unsigned int)uVar12 < (int)DAT_006d49a0);
          }
          if ((bStack_1e == 1) && (bVar10 == 0)) {
            bStack_1e = 0;
          }
          if ((((DAT_006d4a02 == '\x02') && (uStack_22 == 1)) && (bStack_1e == 1)) &&
              (DAT_00709d0a[iVar1] == '\x01')) {
            bStack_1e = 0;
          }
          if (((uStack_22 == 0) && (DAT_006d4a02 == '\x02')) && (DAT_006d4a03 != '\0')) {
            bStack_1e = 0;
          }
          if ((((DAT_00709d0a[iVar20] != 0) != bStack_1e) && (bStack_1e != 0)) &&
              ((DAT_00709d0a[iVar20] != '\x01') && (DAT_006d4a03 == '\0'))) {
            DAT_00709d0a[iVar20] = (char)bStack_1e;
            if (bStack_1e == 1) {
              iVar19 = 10;
              iStack_18 = 0;
              pcVar13 = DAT_00709d0a + uStack_22;
              do {
                if (*pcVar13 == '\x01') {
                  iStack_18 = iStack_18 + 1;
                }
                pcVar13 = pcVar13 + 3;
                iVar19 = iVar19 + -1;
              } while (iVar19 != 0);
              if (uStack_22 == 0) {
                iVar19 = 0;
                if (DAT_006d4a02 == '\x04') {
                  iVar20 = 0;
                  pcVar13 = DAT_00709d0a;
                  do {
                    bVar10 = FUN_004440b0(uVar22, iVar20);
                    if ((bVar10 != 0) && (*pcVar13 == '\x01')) {
                      iVar19 = iVar19 + 1;
                    }
                    pcVar13 = pcVar13 + 3;
                    iVar20 = iVar20 + 1;
                  } while (iVar20 < 10);
                }
                if (iVar19 == 0) {
                  DAT_007092e8[uVar22] = (int)((unsigned int)DAT_007092e8[uVar22] + 10000u);
                }
                DAT_00709f08[uVar22] = (int)uVar14;
                DAT_00709ce8 = 1;
              }
              else if (uStack_22 == 2) {
                DAT_00709ceb = 1;
                DAT_007092e8[uVar22] = (int)((unsigned int)DAT_007092e8[uVar22] + 0x9c4u);
              }
              if (uStack_22 == 0) {
                iVar19 = 0;
                pcVar13 = DAT_0071b740;
                iStack_10 = -1;
                do {
                  if (*pcVar13 != '\0') {
                    __builtin_memcpy(&usTmp, pcVar13 + -4, 2);
                    if (usTmp == uStack_20) {
                      iStack_10 = iVar19;
                    }
                  }
                  pcVar13 = pcVar13 + 0x5bd;
                  iVar19 = iVar19 + 1;
                } while (pcVar13 < DAT_0071e528);
                iVar19 = 0;
                pcVar13 = DAT_0071b740;
                do {
                  cVar11 = DAT_006d4a02;
                  if (*pcVar13 != '\0') {
                    __builtin_memcpy(&usTmp, pcVar13 + -4, 2);
                    if (usTmp == uStack_20) {
                      bVar10 = 1;
                      __builtin_memcpy(pcVar13 + 0x71, &iStack_18, 4);
                      __builtin_memcpy(&usTmp2, pcVar13 + -6, 2);
                      DAT_00732fe2 = usTmp2;
                      if ((cVar11 == '\x04') && (DAT_00709d0b[iVar1] == '\x01')) {
                        bVar10 = 0;
                      }
                      if (bVar10 != 0) {
                        FUN_004a1d70(0xc0, iVar19);
                        if ((DAT_006d4a02 == '\x02') && (1 < DAT_00732e48)) {
                          DAT_00732950 = (short)((short)iStack_18 + 0x11c);
                          FUN_004a1d70(0x127, iVar19);
                        }
                        if (iVar19 == DAT_0071ecf4) {
                          uStack_1c = 0;
                          iVar20 = uVar22 * 0x1e5;
                          do {
                            iVar21 = (unsigned int)uStack_1c * 0x40;
                            if (*(DAT_0072f512 + iVar21 + iVar20) != '\0') {
                              FUN_004a3490(0, DAT_0072f512 + iVar21 + iVar20);
                              DAT_0070a004 = 0x6c;
                              if (uStack_1c != 0) {
                                DAT_0070a004 = 0;
                              }
                              bVar6 = (unsigned char)*(DAT_0072f512 + iVar21 + iVar20);
                              if ((bVar6 < 0x31) || (0x36 < bVar6)) {
                                iVar21 = 0xc3;
                              }
                              else {
                                iVar21 = 0xce;
                              }
                              FUN_004a1d70(iVar21, iVar19);
                            }
                            uStack_1c = (unsigned short)(uStack_1c + 1);
                          } while (uStack_1c < 2);
                          if (DAT_006d4a02 == '\x04') {
                            FUN_0040c290(1);
                          }
                          else {
                            FUN_0040c290(0);
                          }
                        }
                      }
                      else {
                        if (uStack_20 == 0) {
                          iVar20 = 0xcf;
                        }
                        else {
                          iVar20 = 0xd0;
                        }
LAB_0049d286:
                        FUN_004a1d70(iVar20, iVar19);
                      }
                    }
                    else {
                      DAT_00732952 = (short)((short)iStack_18 + 0x11c);
                      if (((DAT_006d4a02 == '\x02') || (DAT_006d4a02 == '\x01')) &&
                          (iStack_10 != -1)) {
                        piVar16 = FUN_00432cc0(iStack_10);
                        FUN_004a3490(0, piVar16);
                        iVar20 = 0x128;
                        goto LAB_0049d286;
                      }
                    }
                  }
                  pcVar13 = pcVar13 + 0x5bd;
                  iVar19 = iVar19 + 1;
                } while (pcVar13 < DAT_0071e528);
              }
              else if (uStack_22 == 1) {
                iVar19 = 0;
                pcVar13 = DAT_0071b740;
                do {
                  if (*pcVar13 != '\0') {
                    __builtin_memcpy(&usTmp, pcVar13 + -4, 2);
                    if (((usTmp == uStack_20) && (DAT_00709d0a[iVar1] != '\x02')) &&
                        (iVar19 == DAT_0071ecf4)) {
                      bVar10 = 1;
                      if ((DAT_006d4a02 == '\x04') && (DAT_00709d0a[iVar1] == '\x01')) {
                        bVar10 = 0;
                      }
                      if (bVar10 != 0) {
                        __builtin_memcpy(&usTmp2, pcVar13 + -6, 2);
                        DAT_00732fe2 = usTmp2;
                        FUN_004a1d70(0xcd, iVar19);
                        uStack_1c = 0;
                        iVar20 = uVar22 * 0x1e5;
                        do {
                          iVar21 = (unsigned int)uStack_1c * 0x40;
                          if (*(DAT_0072f592 + iVar21 + iVar20) != '\0') {
                            FUN_004a3490(0, DAT_0072f592 + iVar21 + iVar20);
                            DAT_0070a004 = 0x6e;
                            if (uStack_1c != 0) {
                              DAT_0070a004 = 0;
                            }
                            bVar6 = (unsigned char)*(DAT_0072f592 + iVar21 + iVar20);
                            if ((bVar6 < 0x31) || (0x36 < bVar6)) {
                              iVar21 = 0xc3;
                            }
                            else {
                              iVar21 = 0xce;
                            }
                            FUN_004a1d70(iVar21, iVar19);
                          }
                          uStack_1c = (unsigned short)(uStack_1c + 1);
                        } while (uStack_1c < 2);
                        if (DAT_006d4a02 == '\x04') {
                          FUN_0040c290(7);
                        }
                        else {
                          FUN_0040c290(6);
                        }
                      }
                      else {
                        if (uStack_20 == 0) {
                          iVar20 = 0xd1;
                        }
                        else {
                          iVar20 = 0xd2;
                        }
                        FUN_004a1d70(iVar20, iVar19);
                      }
                    }
                  }
                  pcVar13 = pcVar13 + 0x5bd;
                  iVar19 = iVar19 + 1;
                } while (pcVar13 < DAT_0071e528);
              }
              else {
                iVar19 = 0;
                pcVar13 = DAT_0071b740;
                do {
                  if (*pcVar13 != '\0') {
                    __builtin_memcpy(&usTmp, pcVar13 + -4, 2);
                    if ((usTmp == uStack_20) && (iVar19 == DAT_0071ecf4)) {
                      uStack_1c = 0;
                      iVar20 = uVar22 * 0x1e5;
                      do {
                        iVar21 = (unsigned int)uStack_1c * 0x40;
                        if (*(DAT_0072f612 + iVar21 + iVar20) != '\0') {
                          FUN_004a3490(0, DAT_0072f612 + iVar21 + iVar20);
                          DAT_0070a004 = 0x6d;
                          if (uStack_1c != 0) {
                            DAT_0070a004 = 0;
                          }
                          __builtin_memcpy(&usTmp2, pcVar13 + -6, 2);
                          DAT_00732fe2 = usTmp2;
                          bVar6 = (unsigned char)*(DAT_0072f612 + iVar21 + iVar20);
                          if ((bVar6 < 0x31) || (0x36 < bVar6)) {
                            iVar21 = 0xc3;
                          }
                          else {
                            iVar21 = 0xce;
                          }
                          FUN_004a1d70(iVar21, iVar19);
                        }
                        uStack_1c = (unsigned short)(uStack_1c + 1);
                      } while (uStack_1c < 2);
                    }
                  }
                  pcVar13 = pcVar13 + 0x5bd;
                  iVar19 = iVar19 + 1;
                } while (pcVar13 < DAT_0071e528);
              }
            }
            else if ((bStack_1e == 1) && (uStack_22 == 0)) {
              iVar19 = 0;
              pcVar13 = DAT_0071b740;
              do {
                if (*pcVar13 != '\0') {
                  __builtin_memcpy(&usTmp, pcVar13 + -4, 2);
                  if (((usTmp == uStack_20) && (DAT_00709d0b[iVar1] != '\x01')) &&
                      (iVar19 == DAT_0071ecf4)) {
                    __builtin_memcpy(&usTmp2, pcVar13 + -6, 2);
                    DAT_00732fe2 = usTmp2;
                    FUN_004a1d70(0xcd, iVar19);
                    uStack_1c = 0;
                    iVar20 = uVar22 * 0x1e5;
                    do {
                      iVar21 = (unsigned int)uStack_1c * 0x40;
                      if (*(DAT_0072f592 + iVar21 + iVar20) != '\0') {
                        FUN_004a3490(0, DAT_0072f592 + iVar21 + iVar20);
                        DAT_0070a004 = 0x6e;
                        if (uStack_1c != 0) {
                          DAT_0070a004 = 0;
                        }
                        bVar6 = (unsigned char)*(DAT_0072f592 + iVar21 + iVar20);
                        if ((bVar6 < 0x31) || (0x36 < bVar6)) {
                          iVar21 = 0xc3;
                        }
                        else {
                          iVar21 = 0xce;
                        }
                        FUN_004a1d70(iVar21, iVar19);
                      }
                      uStack_1c = (unsigned short)(uStack_1c + 1);
                    } while (uStack_1c < 2);
                    if (DAT_006d4a02 == '\x04') {
                      iVar20 = 7;
                    }
                    else {
                      iVar20 = 6;
                    }
                    FUN_0040c290(iVar20);
                  }
                }
                pcVar13 = pcVar13 + 0x5bd;
                iVar19 = iVar19 + 1;
              } while (pcVar13 < DAT_0071e528);
            }
          }
          uStack_22 = (unsigned short)(uStack_22 + 1);
        } while (uStack_22 < 3);
        uStack_20 = (unsigned short)(uStack_20 + 1);
      } while (uStack_20 < 10);
      DAT_0071e6a2 = 0xec;
    }
    if ((short)DAT_0071e6aa < 1) {
      uVar12 = 0;
      if (0 < DAT_006d49a2) {
        do {
          uVar14 = (unsigned int)uVar12;
          if (DAT_00709f44[uVar14] == '\0') {
            iVar1 = uVar14 * 0x72;
            uVar22 = FUN_0049d810(DAT_007308fa + uVar14 * 0x72, 0);
            uVar17 = FUN_0049d810(DAT_00730905 + iVar1, 0);
            if (DAT_00730921[iVar1] == '\x01') {
              uVar24 = (unsigned short)uVar22 | (unsigned short)uVar17;
            }
            else {
              uVar24 = (unsigned short)uVar22 & (unsigned short)uVar17;
            }
            if ((uVar24 & 1) != 0) {
              DAT_00709f44[uVar14] = 1;
              cVar11 = DAT_00730920[iVar1];
              DAT_00709f84[uVar14] = cVar11;
              __builtin_memcpy(&usTmp, DAT_0071b73c + DAT_0071ecf4 * 0x5bd, 2);
              if ((cVar11 == '\0') && (DAT_007308f0[iVar1 + (unsigned int)usTmp] != '\0')) {
                FUN_004a3490(0, DAT_007308b0 + iVar1);
                if (0xf < uVar12) goto LAB_0049d7c1;
                DAT_0070a004 = (unsigned short)(uVar12 + 0x5c);
                goto LAB_0049d7ca;
              }
            }
          }
          else if (DAT_00709f84[uVar14] != '\0') {
            cVar11 = (char)(DAT_00709f84[uVar14] + -1);
            DAT_00709f84[uVar14] = cVar11;
            if (cVar11 == '\0') {
              __builtin_memcpy(&usTmp, DAT_0071b73c + DAT_0071ecf4 * 0x5bd, 2);
              if (DAT_007308f0[uVar14 * 0x72 + (unsigned int)usTmp] != '\0') {
                FUN_004a3490(0, DAT_007308b0 + uVar14 * 0x72);
                if (uVar12 < 0x10) {
                  DAT_0070a004 = (unsigned short)(uVar12 + 0x5c);
                }
                else {
LAB_0049d7c1:
                  DAT_0070a004 = 0;
                }
LAB_0049d7ca:
                FUN_004a1d70(0xce, DAT_0071ecf4);
              }
            }
          }
          uVar12 = (unsigned short)(uVar12 + 1);
        } while ((int)(unsigned int)uVar12 < (int)DAT_006d49a2);
      }
      DAT_0071e6aa = 0x49c;
    }
  }
  return;
}
