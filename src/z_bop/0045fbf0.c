/* SPDX-License-Identifier: MIT */
/* z_bop 0045fbf0 */
/* Callee prototypes: calling conventions are not supplied by the evidence, so
   these are declared without an explicit convention (unverified). */
void FUN_004035d0(unsigned int, short, short, short);
void FUN_00403540(unsigned int, unsigned int);
unsigned int FUN_0041f410(int, int, int);
void FUN_00458e20(unsigned int, int);
int FUN_004a5490(unsigned short, unsigned short);
unsigned char FUN_00403360(unsigned int, int);
unsigned int FUN_00407090(unsigned int, int);

extern unsigned short DAT_009a73b0;
extern unsigned char DAT_009a73d8;
extern unsigned int DAT_009a1fe8;
extern unsigned int DAT_00a08104;
extern unsigned char DAT_009d8c84[];
extern unsigned char DAT_009d9155[];
extern int DAT_00a08144;
extern int DAT_00a08140;
extern int DAT_00a08148;
extern int DAT_009a73c4;
extern int DAT_009a73c8;
extern int DAT_009a73cc;
extern unsigned char DAT_0051c5b2[];
extern int DAT_009d8a58;
extern int DAT_009d8a54;
extern int DAT_009d8b60;
extern int DAT_009ec604;
extern int DAT_00a081e8;
extern int DAT_00a08298;
extern unsigned int DAT_009a1ff8;
extern unsigned char DAT_00521f1c[];
extern int DAT_009a1fd4;
extern int DAT_009a1fd8;
extern int DAT_009a1fd0;
extern int DAT_00a07cdc;
extern int DAT_00a07cd8;
extern int DAT_00a07cd4;
extern unsigned char DAT_009a73b6[4]; /* packed 4-byte pointer at unaligned address */
extern char DAT_009d1320[];
extern char DAT_00521690[];
extern char s_disableldr1pln_0051a310[];
extern unsigned short DAT_00a07cec;
extern int DAT_009a1fe0;
extern unsigned short DAT_009a73d0;
extern int DAT_005243c8[];
extern unsigned short DAT_009e9640;
extern int DAT_00a00854;
extern unsigned short DAT_009a73c0;

#define BYTE_AT(a) (*(unsigned char *)(unsigned int)(a))
#define RD32(a, out) __builtin_memcpy(&(out), (const void *)(unsigned int)(a), 4)
#define RD16(a, out) __builtin_memcpy(&(out), (const void *)(unsigned int)(a), 2)
#define WR16(a, v) do { unsigned short wr_t = (unsigned short)(v); __builtin_memcpy((void *)(unsigned int)(a), &wr_t, 2); } while (0)
#define ADD16(a, v) do { unsigned short ad_t; __builtin_memcpy(&ad_t, (const void *)(unsigned int)(a), 2); ad_t = (unsigned short)(ad_t + (v)); __builtin_memcpy((void *)(unsigned int)(a), &ad_t, 2); } while (0)
#define LOAD_G6(out) __builtin_memcpy(&(out), DAT_009a73b6, 4)

unsigned int __stdcall FUN_0045fbf0(void)
{
  signed char cVar1;
  unsigned int uVar2;
  unsigned int iVar3;
  unsigned int puVar4;
  unsigned int iVar5;
  unsigned int uVar6;
  unsigned int iVar7;
  unsigned int puVar8;
  unsigned short uVar9;
  unsigned short uVar11;
  char *pcVar10;
  char *pcVar12;
  int bVar13;
  int bVar14;
  unsigned short local_14;
  unsigned short local_12;
  unsigned int local_4;
  unsigned int t1;
  unsigned int t2;
  unsigned int t3;
  unsigned int g6;
  unsigned short s1;
  unsigned short s2;
  short sh1;
  short sh2;
  short sh3;
  int i1;
  int i2;
  int i3;

  iVar3 = (unsigned int)DAT_009a73b0 * 0x23 + DAT_009a1fe8;
  DAT_009a73d8 = 0;
  bVar14 = BYTE_AT(iVar3 + 3) == '6';
  uVar6 = (unsigned int)BYTE_AT(iVar3 + 0x16);
  if ((DAT_009d8c84[uVar6 * 0x566] != 0x0e) && (DAT_009d9155[uVar6 * 0x566] != 0x0e) &&
      (local_12 = 0, BYTE_AT(DAT_00a08104 + 0x21d) != 0)) {
    do {
      uVar6 = (unsigned int)local_12;
      iVar3 = DAT_00a08104 + uVar6 * 4;
      puVar4 = iVar3 + 0x3b2;
      RD16(iVar3 + 0x3b4, sh1);
      if ((BYTE_AT(iVar3 + 0x370) == 2) && (sh1 < 1)) {
        WR16(puVar4, 0xffff);
        BYTE_AT(uVar6 * 4 + 0x372 + DAT_00a08104) = 0;
        RD16(DAT_00a08104 + 0x1e5, sh2);
        RD16(DAT_00a08104 + 0x1e7, sh3);
        if ((BYTE_AT(DAT_00a08104 + 7) != 3) && ((sh2 != 0) && (sh3 == 0))) {
          DAT_00a08144 = DAT_009a73c4;
          DAT_00a08140 = DAT_009a73c8;
          DAT_00a08148 = DAT_009a73cc;
          if (!bVar14) {
            iVar7 = (unsigned int)BYTE_AT(DAT_00a08104 + 4) * 0x101;
            iVar5 = iVar7 + uVar6 * 8;
            RD16((unsigned int)DAT_0051c5b2 + iVar5, sh1);
            RD16((unsigned int)DAT_0051c5b2 + 2 + iVar5, sh2);
            RD16((unsigned int)DAT_0051c5b2 + 4 + uVar6 * 8 + iVar7, sh3);
            FUN_004035d0((unsigned int)DAT_009a73b0 * 0x23 + DAT_009a1fe8, sh1, sh2, sh3);
            if (BYTE_AT((unsigned int)DAT_009a73b0 * 0x23 + 3 + DAT_009a1fe8) == '5') {
              DAT_009d8a58 = (int)((unsigned int)DAT_009d8a58 * 2);
              DAT_009d8a54 = (int)((unsigned int)DAT_009d8a54 * 2);
              DAT_009d8b60 = (int)((unsigned int)DAT_009d8b60 * 2);
            }
            DAT_009ec604 = (int)((unsigned int)DAT_00a08144 + (unsigned int)DAT_009d8a58);
            DAT_00a08140 = (int)((unsigned int)DAT_00a08140 + (unsigned int)DAT_009d8a54);
            DAT_00a08148 = (int)((unsigned int)DAT_00a08148 + (unsigned int)DAT_009d8b60);
            DAT_00a08144 = DAT_009ec604;
            DAT_00a081e8 = DAT_00a08140;
            DAT_00a08298 = DAT_00a08148;
          }
          RD16(DAT_00a08104 + 0x6a, uVar9);
          if (uVar9 == 0xffff) {
            iVar5 = 0;
          }
          else {
            iVar7 = (unsigned int)uVar9 * 0x23;
            RD32(DAT_009a1fe8 + iVar7 + 0x1f, iVar5);
            if (iVar5 == 0) {
              bVar13 = BYTE_AT(DAT_009a1fe8 + iVar7 + 3) == 0;
LAB_0045ff51:
              iVar5 = 1 - (unsigned int)bVar13;
            }
            else if (BYTE_AT(DAT_009a1fe8 + iVar7 + 3) == 0) {
              iVar5 = 0;
            }
            else {
              RD32(iVar5 + 0xa9, iVar5);
              if (iVar5 == 0) {
                iVar5 = 1;
              }
              else if ((BYTE_AT(iVar5 + 0x5e) == 0x15) && (BYTE_AT(iVar5 + 0x5f) != 0)) {
                iVar5 = 0;
              }
              else {
                cVar1 = (signed char)BYTE_AT(iVar5 + 7);
                if ((cVar1 == 1) || ((cVar1 == 3 || (cVar1 == 4)))) {
                  iVar5 = 0;
                }
                else if ((BYTE_AT(iVar5 + 0x23d) == 3) && (BYTE_AT(iVar5 + 0x241) != 0)) {
                  FUN_00403540((unsigned int)uVar9, (unsigned int)DAT_009a73b0);
                  cVar1 = (signed char)BYTE_AT((unsigned int)DAT_009a73b0 * 0x23 + 2 + DAT_009a1fe8);
                  if ((cVar1 == 0) || ((cVar1 == 1 || (cVar1 == 2)))) {
                    if ((int)DAT_009a1ff8 < 0x8001) goto LAB_0045ff15;
                    iVar5 = 0;
                  }
                  else {
                    if ((int)DAT_009a1ff8 < 0x10001) goto LAB_0045ff15;
                    iVar5 = 0;
                  }
                }
                else {
LAB_0045ff15:
                  RD32(DAT_009a1fe8 + iVar7 + 0x1b, i1);
                  if (i1 == -1) {
                    RD32(DAT_009a1fe8 + iVar7 + 0x1f, t1);
                    RD32(t1 + 0xa9, t2);
                    RD32(t2 + 0xcb, t3);
                    RD32(iVar5 + 0xc3, t1);
                    bVar13 = t3 < t1;
                    goto LAB_0045ff51;
                  }
                  iVar5 = 1;
                }
              }
            }
          }
          if (iVar5 == 0) {
            WR16(DAT_00a08104 + 0x6a, 0xffff);
          }
          else {
            iVar5 = (unsigned int)uVar9 * 0x23 + DAT_009a1fe8;
            RD32(iVar5 + 4, i1);
            RD32(iVar5 + 8, i2);
            RD32(iVar5 + 0xc, i3);
            DAT_009a1ff8 = FUN_0041f410((int)((unsigned int)i1 - (unsigned int)DAT_00a08144),
                                        (int)((unsigned int)i2 - (unsigned int)DAT_00a08140),
                                        (int)((unsigned int)i3 - (unsigned int)DAT_00a08148));
            uVar6 = 0x10000;
            if (bVar14) {
              RD32((unsigned int)DAT_00521f1c +
                   (unsigned int)BYTE_AT((unsigned int)DAT_009a73b0 * 0x23 + 3 + DAT_009a1fe8) * 0x1a, t1);
              uVar6 = t1 + 0x10000;
            }
            if (DAT_009a1ff8 < uVar6) {
              bVar13 = 1;
              if (!bVar14) {
                FUN_00458e20((unsigned int)uVar9, 0);
                DAT_009a1fd4 = DAT_00a07cdc;
                DAT_009a1fd8 = DAT_00a07cd8;
                DAT_009a1fd0 = DAT_00a07cd4;
                iVar5 = (unsigned int)FUN_004a5490(DAT_009a73b0, DAT_009a73b0);
                bVar13 = iVar5 == 0;
              }
              bVar13 = !bVar13;
              if (!bVar13) {
                iVar5 = 10;
                LOAD_G6(g6);
                pcVar10 = (char *)((unsigned int)DAT_009d1320 + (unsigned int)BYTE_AT(g6 + 0xb) * 0x55);
                pcVar12 = DAT_00521690;
                do {
                  if (iVar5 == 0) break;
                  iVar5 = iVar5 - 1;
                  bVar13 = *pcVar10 == *pcVar12;
                  pcVar10 = pcVar10 + 1;
                  pcVar12 = pcVar12 + 1;
                } while (bVar13);
                LOAD_G6(g6);
                RD16(g6 + 0x18, s1);
                if ((!bVar13) || (bVar13 = (s1 == uVar9), !bVar13)) {
                  iVar5 = 0xf;
                  LOAD_G6(g6);
                  pcVar10 = (char *)((unsigned int)DAT_009d1320 + (unsigned int)BYTE_AT(g6 + 0xb) * 0x55);
                  pcVar12 = s_disableldr1pln_0051a310;
                  do {
                    if (iVar5 == 0) break;
                    iVar5 = iVar5 - 1;
                    bVar13 = *pcVar10 == *pcVar12;
                    pcVar10 = pcVar10 + 1;
                    pcVar12 = pcVar12 + 1;
                  } while (bVar13);
                  LOAD_G6(g6);
                  RD16(g6 + 0x18, s1);
                  if ((bVar13) && (uVar9 == s1)) {
                    LOAD_G6(g6);
                    RD16(g6 + 0x18, s2);
                    RD32((unsigned int)s2 * 0x23 + 0x1f + DAT_009a1fe8, t1);
                    RD32(t1 + 0xa9, t2);
                    RD32(t2 + 0x211, t3);
                    if (t3 == 0) goto LAB_00460159;
                    WR16(puVar4, uVar9);
                    ADD16(iVar3 + 0x3b4, 0x1d8);
                  }
                  else {
                    if (bVar14) {
                      iVar7 = 0;
                      iVar5 = 0;
                      if (BYTE_AT(DAT_00a08104 + 0x21d) != 0) {
                        puVar8 = DAT_00a08104 + 0x3b2;
                        do {
                          RD16(puVar8, s2);
                          if ((s2 == uVar9) && (iVar7 = iVar7 + 1, iVar7 == 2))
                            goto LAB_00460481;
                          puVar8 = puVar8 + 4;
                          iVar5 = iVar5 + 1;
                        } while ((int)iVar5 < (int)(unsigned int)BYTE_AT(DAT_00a08104 + 0x21d));
                      }
                      if (iVar7 == 2) goto LAB_00460481;
                    }
                    WR16(puVar4, uVar9);
                    ADD16(iVar3 + 0x3b4, 0x1d8);
                  }
                  goto LAB_00460481;
                }
              }
            }
          }
LAB_00460159:
          local_4 = 0x10000;
          local_14 = 0xffff;
          if (bVar14) {
            RD32((unsigned int)DAT_00521f1c +
                 (unsigned int)BYTE_AT((unsigned int)DAT_009a73b0 * 0x23 + 3 + DAT_009a1fe8) * 0x1a, t1);
            local_4 = t1 + 0x10000;
          }
          uVar9 = DAT_00a07cec;
          if ((int)(unsigned int)DAT_00a07cec < DAT_009a1fe0) {
            do {
              uVar6 = (unsigned int)uVar9;
              iVar5 = DAT_009a1fe8 + uVar6 * 0x23;
              if (BYTE_AT(iVar5 + 3) != 0) {
                RD32(iVar5 + 0x1f, t1);
                RD32(t1 + 0xa9, iVar5);
                RD16(iVar5 + 0x42, s1);
                if (s1 == DAT_009a73b0) {
                  cVar1 = (signed char)BYTE_AT(iVar5 + 0x5e);
                  if ((cVar1 == 0x0c) || (cVar1 == 0x17)) {
                    if (uVar9 == 0xffff) {
                      iVar5 = 0;
                    }
                    else {
                      iVar5 = DAT_009a1fe8 + uVar6 * 0x23;
                      RD32(iVar5 + 0x1f, i1);
                      if (i1 == 0) {
                        bVar13 = BYTE_AT(iVar5 + 3) == 0;
LAB_00460347:
                        iVar5 = 1 - (unsigned int)bVar13;
                      }
                      else if (BYTE_AT(iVar5 + 3) == 0) {
                        iVar5 = 0;
                      }
                      else {
                        RD32(iVar5 + 0x1f, t1);
                        RD32(t1 + 0xa9, iVar5);
                        if (iVar5 == 0) {
                          iVar5 = 1;
                        }
                        else if ((BYTE_AT(iVar5 + 0x5e) == 0x15) && (BYTE_AT(iVar5 + 0x5f) != 0)) {
                          iVar5 = 0;
                        }
                        else {
                          cVar1 = (signed char)BYTE_AT(iVar5 + 7);
                          if ((cVar1 == 1) || ((cVar1 == 3 || (cVar1 == 4)))) {
                            iVar5 = 0;
                          }
                          else if ((BYTE_AT(iVar5 + 0x23d) == 3) && (BYTE_AT(iVar5 + 0x241) != 0)) {
                            FUN_00403540(uVar6, (unsigned int)DAT_009a73b0);
                            cVar1 = (signed char)BYTE_AT((unsigned int)DAT_009a73b0 * 0x23 + 2 + DAT_009a1fe8);
                            if ((cVar1 == 0) || ((cVar1 == 1 || (cVar1 == 2)))) {
                              if ((int)DAT_009a1ff8 < 0x8001) goto LAB_0046030b;
                              iVar5 = 0;
                            }
                            else {
                              if ((int)DAT_009a1ff8 < 0x10001) goto LAB_0046030b;
                              iVar5 = 0;
                            }
                          }
                          else {
LAB_0046030b:
                            iVar7 = DAT_009a1fe8 + uVar6 * 0x23;
                            RD32(iVar7 + 0x1b, i1);
                            if (i1 == -1) {
                              RD32(iVar7 + 0x1f, t1);
                              RD32(t1 + 0xa9, t2);
                              RD32(t2 + 0xcb, t3);
                              RD32(iVar5 + 0xc3, t1);
                              bVar13 = t3 < t1;
                              goto LAB_00460347;
                            }
                            iVar5 = 1;
                          }
                        }
                      }
                    }
                    if (iVar5 != 0) {
                      iVar5 = uVar6 * 0x23 + DAT_009a1fe8;
                      RD32(iVar5 + 4, i1);
                      RD32(iVar5 + 8, i2);
                      RD32(iVar5 + 0xc, i3);
                      DAT_009a1ff8 = FUN_0041f410((int)((unsigned int)i1 - (unsigned int)DAT_00a08144),
                                                  (int)((unsigned int)i2 - (unsigned int)DAT_00a08140),
                                                  (int)((unsigned int)i3 - (unsigned int)DAT_00a08148));
                      if (bVar14) {
                        iVar5 = 0;
                        if (BYTE_AT(DAT_00a08104 + 0x21d) != 0) {
                          puVar8 = DAT_00a08104 + 0x3b2;
                          do {
                            RD16(puVar8, s2);
                            if (s2 == uVar9) {
                              DAT_009a1ff8 = DAT_009a1ff8 + 0x8000;
                            }
                            puVar8 = puVar8 + 4;
                            iVar5 = iVar5 + 1;
                          } while ((int)iVar5 < (int)(unsigned int)BYTE_AT(DAT_00a08104 + 0x21d));
                        }
                      }
                      if (DAT_009a1ff8 < local_4) {
                        bVar13 = 1;
                        if (!bVar14) {
                          FUN_00458e20(uVar6, 0);
                          DAT_009a1fd4 = DAT_00a07cdc;
                          DAT_009a1fd8 = DAT_00a07cd8;
                          DAT_009a1fd0 = DAT_00a07cd4;
                          iVar5 = (unsigned int)FUN_004a5490(DAT_009a73b0, DAT_009a73b0);
                          bVar13 = iVar5 == 0;
                        }
                        if (bVar13) {
                          local_4 = DAT_009a1ff8;
                          local_14 = uVar9;
                        }
                      }
                    }
                  }
                }
              }
              uVar9 = (unsigned short)(uVar9 + 1);
            } while ((int)(unsigned int)uVar9 < DAT_009a1fe0);
          }
          if (local_14 != 0xffff) {
            WR16(puVar4, local_14);
            RD16(iVar3 + 0x3b4, sh1);
            if (sh1 < 1) {
              ADD16(iVar3 + 0x3b4, 0x1d8);
            }
          }
        }
      }
LAB_00460481:
      local_12 = (unsigned short)(local_12 + 1);
    } while (local_12 < BYTE_AT(DAT_00a08104 + 0x21d));
  }
  uVar6 = DAT_009a1fe8;
  if (((BYTE_AT((unsigned int)DAT_009a73b0 * 0x23 + 2 + DAT_009a1fe8) == 0) &&
       (uVar6 = DAT_00a08104, BYTE_AT(DAT_00a08104 + 0x246) != 0)) &&
      (BYTE_AT(DAT_00a08104 + 0x247) != 0)) {
    iVar3 = (unsigned int)DAT_005243c8[(unsigned int)DAT_009a73d0];
    uVar6 = (unsigned int)(unsigned short)DAT_009e9640;
    uVar9 = (unsigned short)DAT_009e9640;
    if ((int)uVar6 < DAT_00a00854) {
      do {
        iVar5 = (unsigned int)uVar9 * 0x23 + DAT_009a1fe8;
        cVar1 = (signed char)BYTE_AT(iVar5 + 3);
        if (cVar1 != 0) {
          RD32(iVar5 + 0x1f, t1);
          RD32(t1 + 0xa5, iVar5);
          if (BYTE_AT(iVar5 + 1) != 0) {
            RD16(iVar5 + 4, s1);
            if (s1 == DAT_009a73b0) {
              iVar5 = iVar3;
              if ((cVar1 == -0x70) || (cVar1 == -0x6b)) {
                iVar5 = iVar3 * 3;
              }
              bVar14 = (int)(unsigned int)FUN_00403360((unsigned int)uVar9, (int)iVar5);
              if (bVar14 == 1) {
                uVar6 = DAT_00a08104;
                if ((BYTE_AT(DAT_00a08104 + 0x1e5) & 2) == 0) break;
                cVar1 = (signed char)BYTE_AT(DAT_00a08104 + 0x246);
                uVar6 = ((DAT_00a08104 >> 8) << 8) | (unsigned int)(unsigned char)cVar1;
                if (cVar1 == 1) {
                  RD16(DAT_00a08104 + 0x248, s1);
                  uVar6 = (DAT_00a08104 & 0xffff0000) | (unsigned int)s1;
                  if (s1 == 0) {
                    WR16(DAT_00a08104 + 0x248, 10);
                    uVar2 = DAT_00a08104;
                    uVar6 = (unsigned int)DAT_009a73c0 * 0x566;
                    if ((DAT_009d8c84[uVar6] != 0x15) && (DAT_009d9155[uVar6] != 0x15)) {
                      BYTE_AT(DAT_00a08104 + 0x247) = (unsigned char)(BYTE_AT(DAT_00a08104 + 0x247) - 1);
                      return uVar2 & 0xffff0000;
                    }
                  }
                  break;
                }
                if (cVar1 != 2) break;
                bVar14 = 0;
                uVar6 = (unsigned int)(unsigned short)DAT_009e9640;
                uVar11 = (unsigned short)DAT_009e9640;
                if ((int)uVar6 < DAT_00a00854) goto LAB_0046064f;
                goto LAB_004606a1;
              }
            }
          }
        }
        uVar9 = (unsigned short)(uVar9 + 1);
        if (DAT_00a00854 <= (int)(unsigned int)uVar9) {
          return 0;
        }
      } while (1);
    }
  }
  goto LAB_004606ca;
  for (;;) {
    uVar6 = (unsigned int)(unsigned short)(uVar11 + 1);
    uVar11 = (unsigned short)(uVar11 + 1);
    if (DAT_00a00854 <= (int)uVar6) break;
LAB_0046064f:
    iVar3 = (unsigned int)uVar11 * 0x23 + DAT_009a1fe8;
    if (BYTE_AT(iVar3 + 3) == 0x9b) {
      RD32(iVar3 + 0x1f, t1);
      RD32(t1 + 0xa5, uVar6);
      if (BYTE_AT(uVar6 + 1) != 0) {
        RD16(uVar6 + 4, s1);
        if (s1 == uVar9) {
          bVar14 = 1;
          goto LAB_004606a1;
        }
      }
    }
  }
  bVar14 = 0;
LAB_004606a1:
  if (!bVar14) {
    uVar6 = DAT_00a08104;
    RD16(DAT_00a08104 + 0x24a, s1);
    if (s1 == 0) {
      uVar6 = FUN_00407090((unsigned int)DAT_009a73b0, 0x9b);
    }
  }
LAB_004606ca:
  return uVar6 & 0xffff0000;
}
