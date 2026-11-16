/* SPDX-License-Identifier: MIT */
/* z_xvt 0042a3f0 */
/* Prototypes for every callee. */
unsigned char  __cdecl FUN_0049f010(unsigned int param_1, unsigned short param_2, unsigned short param_3);
int            __cdecl FUN_0042b150(int param_1);
void           __cdecl FUN_00421c70(unsigned int param_1, int param_2);
unsigned short __cdecl FUN_0049ec30(unsigned short param_1, unsigned short param_2, unsigned short param_3);

extern unsigned short DAT_00732a3c;
extern int            DAT_006d4980;
extern int            DAT_006d4988;
extern int            DAT_00732e54;
extern unsigned short DAT_006d9d50;
extern int            DAT_006d9d56;   /* unaligned (0x6d9d56): read via memcpy */
extern unsigned int   DAT_006d499c;
extern int            DAT_00732a24;
extern int            DAT_00732a28;
extern int            DAT_00732a2c;
extern int            DAT_00732a38;
extern int            DAT_00703c88;
extern char           DAT_00703ca0[];
extern unsigned char  DAT_0052687a[];
extern char           DAT_00524948[];
extern char           s_stationaryldrpln_00524934[];
extern char           s_stationaryflwpln_00524920[];
extern char           s_boardtocapturepln_00524f90[];
extern char           s_boardtodestroypln_00524f7c[];
extern char           s_boardtogivepln_00523c18[];
extern char           s_boardtotakepln_00524f6c[];
extern char           s_boardtoexchangepln_00524f58[];
extern char           s_boardtopickuppln_00524f44[];
extern char           s_boardtocontactpln_00524f30[];
extern char           s_boardtorepairpln_00524f1c[];

unsigned int __cdecl
FUN_0042a3f0(unsigned short param_1, unsigned short param_2, short param_3,
             unsigned short param_4, unsigned short param_5)
{
  unsigned char bVar1;
  int bVar2;
  int bVar14;
  unsigned short uVar3;
  unsigned short uVar9;
  unsigned int uVar7;
  int iVar4;
  int iVar5;
  int iVar6;
  int iVar13;
  int iSum;
  int iDx;
  int iDy;
  int iDz;
  short sVar8;
  short local_10;
  short s1df;
  unsigned short local_12;
  unsigned short local_e;
  unsigned short uStack_c;
  unsigned int local_4;
  const char *pcVar10;
  const char *pcVar11;
  const char *pcVar12;
  int tmp;
  int tmpb;

  local_e = DAT_00732a3c;
  local_4 = 0xffffffff;
  local_12 = 0xffff;
  iSum = 0;
  if ((int)(unsigned int)DAT_00732a3c < DAT_006d4980) {
    do {
      iVar13 = (int)((unsigned int)local_e * 0x23u);
      uStack_c = (unsigned short)iVar13;
      if (*(char *)((unsigned char *)DAT_006d4988 + 3 + iVar13) != '\0') {
        uVar9 = (unsigned short)FUN_0049f010(((unsigned int)uStack_c << 16) | (unsigned int)local_e,
                                             param_1, param_2);
        uVar3 = (unsigned short)FUN_0049f010(((unsigned int)uStack_c << 16) | (unsigned int)local_e,
                                             param_4, param_5);
        if (param_3 == 1) {
          uVar9 = uVar9 | uVar3;
        } else {
          uVar9 = uVar9 & uVar3;
        }
        if (uVar9 != 0) {
          bVar2 = 0;
          __builtin_memcpy(&tmp, (unsigned char *)DAT_006d4988 + 0x1f + iVar13, 4);
          __builtin_memcpy(&iVar5, (unsigned char *)tmp + 0xa7, 4);
          iVar4 = FUN_0042b150(iVar5);
          bVar14 = 0;
          iVar6 = 8;
          pcVar11 = DAT_00703ca0 + (unsigned int)*(unsigned char *)(iVar4 + 0xb) * 0x55;
          pcVar10 = pcVar11;
          pcVar12 = DAT_00524948;
          do {
            if (iVar6 == 0) break;
            iVar6 = iVar6 - 1;
            bVar14 = *pcVar10 == *pcVar12;
            pcVar10 = pcVar10 + 1;
            pcVar12 = pcVar12 + 1;
          } while (bVar14);
          if (bVar14) {
LAB_0042a59a:
            bVar2 = 1;
          } else {
            iVar6 = 0x11;
            bVar14 = 0;
            pcVar10 = pcVar11;
            pcVar12 = s_stationaryldrpln_00524934;
            do {
              if (iVar6 == 0) break;
              iVar6 = iVar6 - 1;
              bVar14 = *pcVar10 == *pcVar12;
              pcVar10 = pcVar10 + 1;
              pcVar12 = pcVar12 + 1;
            } while (bVar14);
            if (bVar14) goto LAB_0042a59a;
            iVar6 = 0x11;
            bVar14 = 0;
            pcVar10 = s_stationaryflwpln_00524920;
            do {
              if (iVar6 == 0) break;
              iVar6 = iVar6 - 1;
              bVar14 = *pcVar11 == *pcVar10;
              pcVar11 = pcVar11 + 1;
              pcVar10 = pcVar10 + 1;
            } while (bVar14);
            if (bVar14 || *(char *)((unsigned char *)DAT_006d4988 + 2 + iVar13) == '\x05')
              goto LAB_0042a59a;
            iVar6 = 0x12;
            bVar14 = 0;
            __builtin_memcpy(&tmpb, &DAT_006d9d56, 4);
            pcVar11 = DAT_00703ca0 + (unsigned int)*(unsigned char *)(tmpb + 0xb) * 0x55;
            pcVar10 = s_boardtocapturepln_00524f90;
            do {
              if (iVar6 == 0) break;
              iVar6 = iVar6 - 1;
              bVar14 = *pcVar11 == *pcVar10;
              pcVar11 = pcVar11 + 1;
              pcVar10 = pcVar10 + 1;
            } while (bVar14);
            if (bVar14) {
LAB_0042a590:
              __builtin_memcpy(&s1df, (unsigned char *)iVar5 + 0x1df, 2);
              if (s1df == 0) goto LAB_0042a59a;
            } else {
              iVar6 = 0x12;
              bVar14 = 0;
              __builtin_memcpy(&tmpb, &DAT_006d9d56, 4);
              pcVar11 = DAT_00703ca0 + (unsigned int)*(unsigned char *)(tmpb + 0xb) * 0x55;
              pcVar10 = s_boardtodestroypln_00524f7c;
              do {
                if (iVar6 == 0) break;
                iVar6 = iVar6 - 1;
                bVar14 = *pcVar11 == *pcVar10;
                pcVar11 = pcVar11 + 1;
                pcVar10 = pcVar10 + 1;
              } while (bVar14);
              if (bVar14) goto LAB_0042a590;
              __builtin_memcpy(&s1df, (unsigned char *)iVar5 + 0x1df, 2);
              if (s1df == 0 || *(char *)(iVar4 + 0x34) == '\x13' ||
                  *(char *)(iVar4 + 0x34) == '\x19')
                goto LAB_0042a59a;
            }
          }
          if (bVar2) {
            local_10 = 0;
            uVar9 = DAT_00732a3c;
            if ((int)(unsigned int)DAT_00732a3c < DAT_006d4980) {
              do {
                iVar5 = (int)((unsigned int)uVar9 * 0x23u + (unsigned int)DAT_006d4988);
                if (*(char *)(iVar5 + 3) != '\0' && uVar9 != DAT_006d9d50) {
                  __builtin_memcpy(&tmp, (unsigned char *)iVar5 + 0x1f, 4);
                  __builtin_memcpy(&iVar5, (unsigned char *)tmp + 0xa7, 4);
                  iVar4 = FUN_0042b150(iVar5);
                  bVar2 = 0;
                  pcVar11 = DAT_00703ca0 + (unsigned int)*(unsigned char *)(iVar4 + 0xb) * 0x55;
                  iVar6 = 0xf;
                  pcVar10 = pcVar11;
                  pcVar12 = s_boardtogivepln_00523c18;
                  do {
                    if (iVar6 == 0) break;
                    iVar6 = iVar6 - 1;
                    bVar2 = *pcVar10 == *pcVar12;
                    pcVar10 = pcVar10 + 1;
                    pcVar12 = pcVar12 + 1;
                  } while (bVar2);
                  if (bVar2) goto LAB_0042a6a0;
                  iVar6 = 0xf;
                  pcVar10 = pcVar11;
                  pcVar12 = s_boardtotakepln_00524f6c;
                  do {
                    if (iVar6 == 0) break;
                    iVar6 = iVar6 - 1;
                    bVar2 = *pcVar10 == *pcVar12;
                    pcVar10 = pcVar10 + 1;
                    pcVar12 = pcVar12 + 1;
                  } while (bVar2);
                  if (bVar2) goto LAB_0042a6a0;
                  iVar6 = 0x13;
                  pcVar10 = pcVar11;
                  pcVar12 = s_boardtoexchangepln_00524f58;
                  do {
                    if (iVar6 == 0) break;
                    iVar6 = iVar6 - 1;
                    bVar2 = *pcVar10 == *pcVar12;
                    pcVar10 = pcVar10 + 1;
                    pcVar12 = pcVar12 + 1;
                  } while (bVar2);
                  if (bVar2) goto LAB_0042a6a0;
                  iVar6 = 0x12;
                  pcVar10 = pcVar11;
                  pcVar12 = s_boardtocapturepln_00524f90;
                  do {
                    if (iVar6 == 0) break;
                    iVar6 = iVar6 - 1;
                    bVar2 = *pcVar10 == *pcVar12;
                    pcVar10 = pcVar10 + 1;
                    pcVar12 = pcVar12 + 1;
                  } while (bVar2);
                  if (bVar2) goto LAB_0042a6a0;
                  iVar6 = 0x12;
                  pcVar10 = pcVar11;
                  pcVar12 = s_boardtodestroypln_00524f7c;
                  do {
                    if (iVar6 == 0) break;
                    iVar6 = iVar6 - 1;
                    bVar2 = *pcVar10 == *pcVar12;
                    pcVar10 = pcVar10 + 1;
                    pcVar12 = pcVar12 + 1;
                  } while (bVar2);
                  if (bVar2) goto LAB_0042a6a0;
                  iVar6 = 0x11;
                  pcVar10 = pcVar11;
                  pcVar12 = s_boardtopickuppln_00524f44;
                  do {
                    if (iVar6 == 0) break;
                    iVar6 = iVar6 - 1;
                    bVar2 = *pcVar10 == *pcVar12;
                    pcVar10 = pcVar10 + 1;
                    pcVar12 = pcVar12 + 1;
                  } while (bVar2);
                  if (bVar2) goto LAB_0042a6a0;
                  iVar6 = 0x12;
                  pcVar10 = pcVar11;
                  pcVar12 = s_boardtocontactpln_00524f30;
                  do {
                    if (iVar6 == 0) break;
                    iVar6 = iVar6 - 1;
                    bVar2 = *pcVar10 == *pcVar12;
                    pcVar10 = pcVar10 + 1;
                    pcVar12 = pcVar12 + 1;
                  } while (bVar2);
                  if (bVar2) goto LAB_0042a6a0;
                  iVar6 = 0x11;
                  pcVar10 = s_boardtorepairpln_00524f1c;
                  do {
                    if (iVar6 == 0) break;
                    iVar6 = iVar6 - 1;
                    bVar2 = *pcVar11 == *pcVar10;
                    pcVar11 = pcVar11 + 1;
                    pcVar10 = pcVar10 + 1;
                  } while (bVar2);
                  if (!bVar2) goto LAB_0042a6c3;
LAB_0042a6a0:
                  if (*(unsigned short *)(iVar4 + 0x18) == local_e ||
                      *(unsigned short *)(iVar5 + 0x66) == local_e) {
                    local_10 = (short)(local_10 + 1);
                  }
                }
LAB_0042a6c3:
                uVar9 = (unsigned short)(uVar9 + 1);
              } while ((int)(unsigned int)uVar9 < DAT_006d4980);
            }
            uVar7 = 0;
            if (*(unsigned char *)(DAT_00732e54 + 0x7d) != 0) {
              do {
                if (*(short *)(DAT_00732e54 + 0x7e + uVar7 * 2) ==
                    *(short *)((unsigned char *)DAT_006d4988 + iVar13)) {
                  local_10 = (short)(local_10 + 1);
                }
                uVar7 = (unsigned int)(unsigned short)((short)uVar7 + 1);
              } while (uVar7 < *(unsigned char *)(DAT_00732e54 + 0x7d));
            }
            if (local_10 == 0) {
              FUN_00421c70((unsigned int)DAT_006d9d50, 0);
              iDx = DAT_00732a2c;
              iDy = DAT_00732a28;
              iDz = DAT_00732a24;
              FUN_00421c70((unsigned int)local_e, 0);
              iDx = (int)((unsigned int)iDx - (unsigned int)DAT_00732a2c);
              iDy = (int)((unsigned int)iDy - (unsigned int)DAT_00732a28);
              iDz = (int)((unsigned int)iDz - (unsigned int)DAT_00732a24);
              if (iDx < 0) iDx = (int)(0u - (unsigned int)iDx);
              if (iDy < 0) iDy = (int)(0u - (unsigned int)iDy);
              if (iDz < 0) iDz = (int)(0u - (unsigned int)iDz);
              if (iDy < iDx) {
                DAT_006d499c = (unsigned int)(iDy >> 1) + (unsigned int)iDx;
              } else {
                DAT_006d499c = (unsigned int)(iDx >> 1) + (unsigned int)iDy;
              }
              if (iDz < (int)DAT_006d499c) {
                iDz = iDz >> 1;
              } else {
                DAT_006d499c = (unsigned int)((int)DAT_006d499c >> 1);
              }
              DAT_006d499c = DAT_006d499c + (unsigned int)iDz;
              if (DAT_006d499c < local_4) {
                local_12 = local_e;
                local_4 = DAT_006d499c;
              }
            }
          }
        }
      }
      local_e = (unsigned short)(local_e + 1);
    } while ((int)(unsigned int)local_e < DAT_006d4980);
  }
  local_e = (unsigned short)DAT_00732a38;
  iSum = (int)((unsigned int)DAT_00703c88 + (unsigned int)DAT_00732a38);
  if ((int)((unsigned int)DAT_00732a38 & 0xffff) < iSum) {
    do {
      iVar13 = (int)((((unsigned int)local_e * 8u) - (unsigned int)local_e) * 5u);
      bVar1 = *(unsigned char *)((unsigned char *)DAT_006d4988 + iVar13 + 3);
      if (bVar1 != 0 && (DAT_0052687a[(unsigned int)bVar1 * 0x16] & 2) != 0) {
        uVar9 = FUN_0049ec30((unsigned short)*(unsigned char *)((unsigned char *)DAT_006d4988 + iVar13 + 0x16),
                             param_1, param_2);
        uVar3 = FUN_0049ec30((unsigned short)*(unsigned char *)((unsigned char *)DAT_006d4988 + 0x16 + iVar13),
                             param_4, param_5);
        if (param_3 == 1) {
          uVar9 = uVar9 | uVar3;
        } else {
          uVar9 = uVar9 & uVar3;
        }
        if (uVar9 != 0) {
          sVar8 = 0;
          uVar9 = DAT_00732a3c;
          if ((int)(unsigned int)DAT_00732a3c < DAT_006d4980) {
            do {
              iVar5 = (int)((unsigned int)uVar9 * 0x23u + (unsigned int)DAT_006d4988);
              if (*(char *)(iVar5 + 3) != '\0' && uVar9 != DAT_006d9d50) {
                __builtin_memcpy(&tmp, (unsigned char *)iVar5 + 0x1f, 4);
                __builtin_memcpy(&tmpb, (unsigned char *)tmp + 0xa7, 4);
                iVar5 = FUN_0042b150(tmpb);
                bVar2 = 0;
                pcVar11 = DAT_00703ca0 + (unsigned int)*(unsigned char *)(iVar5 + 0xb) * 0x55;
                iVar4 = 0xf;
                pcVar10 = pcVar11;
                pcVar12 = s_boardtogivepln_00523c18;
                do {
                  if (iVar4 == 0) break;
                  iVar4 = iVar4 - 1;
                  bVar2 = *pcVar10 == *pcVar12;
                  pcVar10 = pcVar10 + 1;
                  pcVar12 = pcVar12 + 1;
                } while (bVar2);
                if (bVar2) goto LAB_0042a9a0;
                iVar4 = 0xf;
                pcVar10 = pcVar11;
                pcVar12 = s_boardtotakepln_00524f6c;
                do {
                  if (iVar4 == 0) break;
                  iVar4 = iVar4 - 1;
                  bVar2 = *pcVar10 == *pcVar12;
                  pcVar10 = pcVar10 + 1;
                  pcVar12 = pcVar12 + 1;
                } while (bVar2);
                if (bVar2) goto LAB_0042a9a0;
                iVar4 = 0x13;
                pcVar10 = pcVar11;
                pcVar12 = s_boardtoexchangepln_00524f58;
                do {
                  if (iVar4 == 0) break;
                  iVar4 = iVar4 - 1;
                  bVar2 = *pcVar10 == *pcVar12;
                  pcVar10 = pcVar10 + 1;
                  pcVar12 = pcVar12 + 1;
                } while (bVar2);
                if (bVar2) goto LAB_0042a9a0;
                iVar4 = 0x12;
                pcVar10 = pcVar11;
                pcVar12 = s_boardtocapturepln_00524f90;
                do {
                  if (iVar4 == 0) break;
                  iVar4 = iVar4 - 1;
                  bVar2 = *pcVar10 == *pcVar12;
                  pcVar10 = pcVar10 + 1;
                  pcVar12 = pcVar12 + 1;
                } while (bVar2);
                if (bVar2) goto LAB_0042a9a0;
                iVar4 = 0x12;
                pcVar10 = pcVar11;
                pcVar12 = s_boardtodestroypln_00524f7c;
                do {
                  if (iVar4 == 0) break;
                  iVar4 = iVar4 - 1;
                  bVar2 = *pcVar10 == *pcVar12;
                  pcVar10 = pcVar10 + 1;
                  pcVar12 = pcVar12 + 1;
                } while (bVar2);
                if (bVar2) goto LAB_0042a9a0;
                iVar4 = 0x12;
                pcVar10 = pcVar11;
                pcVar12 = s_boardtocontactpln_00524f30;
                do {
                  if (iVar4 == 0) break;
                  iVar4 = iVar4 - 1;
                  bVar2 = *pcVar10 == *pcVar12;
                  pcVar10 = pcVar10 + 1;
                  pcVar12 = pcVar12 + 1;
                } while (bVar2);
                if (bVar2) goto LAB_0042a9a0;
                iVar4 = 0x11;
                pcVar10 = s_boardtorepairpln_00524f1c;
                do {
                  if (iVar4 == 0) break;
                  iVar4 = iVar4 - 1;
                  bVar2 = *pcVar11 == *pcVar10;
                  pcVar11 = pcVar11 + 1;
                  pcVar10 = pcVar10 + 1;
                } while (bVar2);
                if (!bVar2) goto LAB_0042a9c6;
LAB_0042a9a0:
                if (*(unsigned short *)(iVar5 + 0x18) == local_e) {
                  sVar8 = (short)(sVar8 + 1);
                }
              }
LAB_0042a9c6:
              uVar9 = (unsigned short)(uVar9 + 1);
            } while ((int)(unsigned int)uVar9 < DAT_006d4980);
          }
          uVar7 = 0;
          if (*(unsigned char *)(DAT_00732e54 + 0x7d) != 0) {
            do {
              if (*(short *)(DAT_00732e54 + 0x7e + uVar7 * 2) ==
                  *(short *)((unsigned char *)DAT_006d4988 + iVar13)) {
                sVar8 = (short)(sVar8 + 1);
              }
              uVar7 = (unsigned int)(unsigned short)((short)uVar7 + 1);
            } while (uVar7 < *(unsigned char *)(DAT_00732e54 + 0x7d));
          }
          if (sVar8 == 0) {
            FUN_00421c70((unsigned int)DAT_006d9d50, 0);
            iDx = DAT_00732a2c;
            iDy = DAT_00732a28;
            iDz = DAT_00732a24;
            FUN_00421c70((unsigned int)local_e, 0);
            iDx = (int)((unsigned int)iDx - (unsigned int)DAT_00732a2c);
            iDy = (int)((unsigned int)iDy - (unsigned int)DAT_00732a28);
            iDz = (int)((unsigned int)iDz - (unsigned int)DAT_00732a24);
            if (iDx < 0) iDx = (int)(0u - (unsigned int)iDx);
            if (iDy < 0) iDy = (int)(0u - (unsigned int)iDy);
            if (iDz < 0) iDz = (int)(0u - (unsigned int)iDz);
            if (iDy < iDx) {
              DAT_006d499c = (unsigned int)(iDy >> 1) + (unsigned int)iDx;
            } else {
              DAT_006d499c = (unsigned int)(iDx >> 1) + (unsigned int)iDy;
            }
            if (iDz < (int)DAT_006d499c) {
              iDz = iDz >> 1;
            } else {
              DAT_006d499c = (unsigned int)((int)DAT_006d499c >> 1);
            }
            DAT_006d499c = DAT_006d499c + (unsigned int)iDz;
            if (DAT_006d499c < local_4) {
              local_12 = local_e;
              local_4 = DAT_006d499c;
            }
          }
        }
      }
      local_e = (unsigned short)(local_e + 1);
      iSum = (int)((unsigned int)DAT_00703c88 + (unsigned int)DAT_00732a38);
    } while ((int)(unsigned int)local_e < iSum);
  }
  return (((unsigned int)iSum >> 16) << 16) | (unsigned int)local_12;
}
