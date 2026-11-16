/* SPDX-License-Identifier: MIT */
/* z_bop 0049d860 */
extern unsigned char DAT_00521f1c[];
extern unsigned char DAT_00521f20[];
extern char DAT_00521f1b[];
extern char DAT_00527640[];
extern unsigned char DAT_0051c633[];
extern unsigned char DAT_0051c635[];
extern unsigned char DAT_0051c637[];
extern unsigned char DAT_0051c639[];
extern unsigned char DAT_0051c63b[];
extern unsigned char DAT_0051c63d[];
extern unsigned char DAT_0051c63f[];
extern unsigned char DAT_0051c641[];
extern unsigned char DAT_0051c645[];
extern unsigned char DAT_0051c649[];
extern unsigned char DAT_0051c64d[];
extern unsigned char DAT_0051c651[];
extern unsigned char DAT_0051c655[];
extern unsigned char DAT_0051c659[];
extern unsigned char DAT_0051c65b[];
extern unsigned char DAT_0051c65d[];
extern unsigned char DAT_0051c65f[];
extern char DAT_0051c59e[];
extern char DAT_0051c5a0[];
extern char DAT_0051c5a2[];
extern char DAT_0051c5a4[];
extern char DAT_0051c5a6[];
extern char DAT_0051c5a8[];
extern char DAT_0051c5aa[];
extern char DAT_0051c5ac[];
extern char DAT_0051c5ae[];
extern unsigned char DAT_0051c5b2[];
extern unsigned char DAT_0051c5b4[];
extern unsigned char DAT_0051c5b6[];
extern char DAT_0051c5b8[];
extern char DAT_0051c5b9[];

long long __cdecl FUN_004adf10(unsigned int a);
long long __cdecl FUN_004ae0e0(unsigned int a);
long long __cdecl FUN_004ae120(unsigned int a);
long long __cdecl FUN_004ae160(unsigned int a);
long long __cdecl FUN_004ae020(unsigned int a);
long long __cdecl FUN_004ae0b0(unsigned int a);
int __cdecl FUN_004adc40(unsigned int a);
unsigned int __cdecl FUN_004af250(unsigned int a, unsigned int b);
int __cdecl FUN_004ae2a0(unsigned int a, unsigned int b);
void __cdecl FUN_004af4c0(unsigned int a, unsigned int b, unsigned int c,
                          unsigned int *d, int *e, int *f, int *g);
unsigned int __cdecl FUN_004af2d0(unsigned int a, unsigned int b, unsigned int c);

#define ST16(p, v) { unsigned short t16_ = (unsigned short)(v); __builtin_memcpy((p), &t16_, 2); }
#define ST32(p, v) { int t32_ = (int)(v); __builtin_memcpy((p), &t32_, 4); }

void __cdecl FUN_0049d860(unsigned char param_1, unsigned char param_2)
{
  int iVar1;
  int iVar2;
  int bVar3;
  char cVar4;
  unsigned short uVar5;
  unsigned short uVar6;
  unsigned int uVar7;
  int iVar8;
  unsigned int uVar9;
  unsigned short uVar10;
  unsigned short uVar11;
  short sVar12;
  unsigned char bVar13;
  unsigned char bVar14;
  unsigned int uVar15;
  unsigned int uVar16;
  long long lVar17;
  unsigned short uStack_32;
  unsigned char uStack_2e;
  unsigned short uStack_2c;
  int local_28;
  int local_24;
  int local_20;
  unsigned int local_1c;
  int local_18;
  unsigned int local_14;
  unsigned int local_10;
  int local_c;
  unsigned int local_8;
  int local_4;
  short tmps;

  uStack_2c = (unsigned short)param_2;
  local_20 = (int)((unsigned int)param_2 * 0x1a);
  lVar17 = FUN_004adf10((unsigned int)param_2);
  ST32(&DAT_00521f1c[local_20], (int)lVar17);
  ST32(&DAT_00521f20[local_20], (int)lVar17 >> 1);
  if (param_1 != 0xff) {
    lVar17 = FUN_004ae0e0((unsigned int)uStack_2c);
    uVar16 = (unsigned int)lVar17;
    lVar17 = FUN_004ae120((unsigned int)uStack_2c);
    uVar15 = (unsigned int)lVar17;
    lVar17 = FUN_004ae160((unsigned int)uStack_2c);
    sVar12 = 0;
    for (uVar7 = (unsigned int)lVar17; ((0x280 < uVar16 || (0x280 < uVar15)) || (0x280 < uVar7));
         uVar7 = uVar7 >> 1) {
      uVar16 = uVar16 >> 1;
      uVar15 = uVar15 >> 1;
      sVar12 = (short)(sVar12 + 1);
    }
    iVar2 = (int)((unsigned int)param_1 * 0x101);
    ST16(&DAT_0051c659[iVar2], sVar12);
    ST16(&DAT_0051c65b[iVar2], (short)uVar16);
    ST16(&DAT_0051c65f[iVar2], (short)uVar15);
    ST16(&DAT_0051c65d[iVar2], (short)uVar7);
    __builtin_memcpy(&tmps, &DAT_0051c63d[iVar2], 2);
    if (tmps == 0) {
      lVar17 = FUN_004ae020((unsigned int)uStack_2c);
      ST16(&DAT_0051c63d[iVar2], (short)lVar17);
      lVar17 = FUN_004ae020((unsigned int)uStack_2c);
      ST16(&DAT_0051c63f[iVar2], (short)lVar17);
    }
    __builtin_memcpy(&tmps, &DAT_0051c639[iVar2], 2);
    if (tmps == 0) {
      lVar17 = FUN_004ae0b0((unsigned int)uStack_2c);
      ST16(&DAT_0051c639[iVar2], (short)lVar17);
      lVar17 = FUN_004ae0b0((unsigned int)uStack_2c);
      ST16(&DAT_0051c63b[iVar2], (short)lVar17);
    }
    local_c = FUN_004adc40((unsigned int)uStack_2c);
    sVar12 = 0;
    if (0 < local_c) {
      do {
        local_10 = (unsigned int)(unsigned short)sVar12;
        local_1c = FUN_004af250((unsigned int)uStack_2c, local_10);
        if (local_1c != 0) {
          iVar8 = FUN_004ae2a0((unsigned int)uStack_2c, local_10);
          uStack_32 = 0;
          if (0 < (int)local_1c) {
            do {
              bVar3 = 0;
              FUN_004af4c0((unsigned int)uStack_2c, local_10, (unsigned int)uStack_32, &local_14,
                           &local_18, &local_28, &local_24);
              uVar5 = (unsigned short)local_24;
              uVar10 = (unsigned short)local_28;
              switch (local_14) {
              case 0x19:
                ST32(&DAT_0051c641[iVar2], local_18);
                ST32(&DAT_0051c645[iVar2], local_24);
                ST32(&DAT_0051c649[iVar2], local_28);
                bVar3 = 1;
                break;
              case 0x1a:
                ST32(&DAT_0051c64d[iVar2], local_18);
                ST32(&DAT_0051c651[iVar2], local_24);
                ST32(&DAT_0051c655[iVar2], local_28);
                bVar3 = 1;
                break;
              case 0x1b:
                ST16(&DAT_0051c63b[iVar2], uVar5);
                ST16(&DAT_0051c637[iVar2], uVar10);
                bVar3 = 1;
                break;
              case 0x1c:
                ST16(&DAT_0051c639[iVar2], uVar5);
                ST16(&DAT_0051c637[iVar2], uVar10);
                bVar3 = 1;
                break;
              case 0x1d:
                ST16(&DAT_0051c63f[iVar2], uVar5);
                ST16(&DAT_0051c637[iVar2], uVar10);
                bVar3 = 1;
                break;
              case 0x1e:
                ST16(&DAT_0051c63d[iVar2], uVar5);
                ST16(&DAT_0051c637[iVar2], uVar10);
                bVar3 = 1;
                break;
              case 0x1f:
                ST16(&DAT_0051c635[iVar2], uVar5);
                ST16(&DAT_0051c633[iVar2], uVar10);
                bVar3 = 1;
                break;
              default:
                break;
              }
              if (!bVar3) {
                uVar16 = 0;
                cVar4 = (char)local_14 + -0x78;
                do {
                  uVar11 = (unsigned short)uVar16;
                  if (DAT_0051c59e[iVar2 + uVar16] == cVar4) break;
                  uVar11 = (unsigned short)(uVar11 + 1);
                  uVar16 = (unsigned int)uVar11;
                } while (uVar11 < 2);
                if (1 < uVar11) {
                  uVar11 = 0;
                  do {
                    if (DAT_0051c5a8[iVar2 + (unsigned int)uVar11] == cVar4) break;
                    uVar11 = (unsigned short)(uVar11 + 1);
                  } while (uVar11 < 2);
                  if (1 < uVar11) {
                    if (DAT_00527640[local_14] == '\x01') {
                      uVar16 = 0;
                      do {
                        if (DAT_0051c59e[iVar2 + uVar16] == '\0') break;
                        uVar11 = (unsigned short)((short)uVar16 + 1);
                        uVar16 = (unsigned int)uVar11;
                      } while (uVar11 < 2);
                      if ((unsigned short)uVar16 < 2) {
                        DAT_0051c59e[iVar2 + uVar16] = cVar4;
                        iVar1 = (int)(uVar16 + iVar2);
                        if ((((iVar8 == 4) || (iVar8 == 0x15)) ||
                             ((iVar8 == 5 ||
                               ((cVar4 = DAT_00521f1b[local_20], cVar4 == '\x03' ||
                                 (cVar4 == '\x05')))))) || (cVar4 == '\x04')) {
                          DAT_0051c5a6[iVar1] = 2;
                        }
                        else if ((local_14 == 5) || (local_14 == 0x10)) {
                          DAT_0051c5a6[iVar1] = 1;
                        }
                        else {
                          DAT_0051c5a6[iVar1] = 0;
                        }
                      }
                    }
                    else if (DAT_00527640[local_14] == '\x02') {
                      uVar16 = 0;
                      do {
                        if (DAT_0051c5a8[iVar2 + uVar16] == '\0') break;
                        uVar11 = (unsigned short)((short)uVar16 + 1);
                        uVar16 = (unsigned int)uVar11;
                      } while (uVar11 < 2);
                      if ((unsigned short)uVar16 < 2) {
                        DAT_0051c5a8[iVar2 + uVar16] = cVar4;
                      }
                    }
                  }
                }
              }
              uStack_32 = (unsigned short)(uStack_32 + 1);
            } while ((int)(unsigned int)uStack_32 < (int)local_1c);
          }
        }
        sVar12 = (short)(sVar12 + 1);
      } while ((int)(unsigned int)(unsigned short)sVar12 < local_c);
    }
    uVar11 = 0;
    bVar13 = 0;
    do {
      bVar14 = bVar13;
      if (bVar13 == 0x10) {
        iVar8 = (int)((unsigned int)uVar11 + iVar2);
        DAT_0051c59e[iVar8] = 0;
        DAT_0051c5a6[iVar8] = 0;
      }
      else {
        local_4 = (int)((unsigned int)uVar11 + iVar2);
        sVar12 = 0;
        cVar4 = DAT_0051c59e[local_4];
        if (0 < local_c) {
          do {
            uStack_2e = (unsigned char)sVar12;
            local_8 = (unsigned int)(unsigned short)sVar12;
            local_1c = FUN_004af250((unsigned int)uStack_2c, local_8);
            if (local_1c != 0) {
              uVar6 = 0xff;
              local_20 = FUN_004ae2a0((unsigned int)uStack_2c, local_8);
              uStack_32 = 0;
              if (0 < (int)local_1c) {
                local_10 = (unsigned int)(unsigned char)(cVar4 + 0x78);
                do {
                  FUN_004af4c0((unsigned int)uStack_2c, local_8, (unsigned int)uStack_32,
                               &local_14, &local_18, &local_28, &local_24);
                  if (local_10 == local_14) {
                    if (uVar6 == 0xff) {
                      if (param_2 == 0x35) {
                        local_18 = local_18 >> 1;
                        local_28 = local_28 >> 1;
                        local_24 = local_24 >> 1;
                      }
                      ST16(&DAT_0051c5b2[(unsigned int)bVar14 * 8 + iVar2], (short)local_18);
                      iVar8 = (int)(iVar2 + (unsigned int)bVar14 * 8);
                      ST16(&DAT_0051c5b6[iVar8], (short)local_28);
                      ST16(&DAT_0051c5b4[iVar8], (short)local_24);
                      DAT_0051c5b9[iVar8] = (char)uStack_2e;
                      DAT_0051c5b8[iVar8] = (char)0xff;
                      if ((local_20 == 4) || (local_20 == 0x15)) {
                        uVar6 = (unsigned short)bVar14;
                      }
                      bVar14 = (unsigned char)(bVar14 + 1);
                      if (bVar14 == 0x10) break;
                    }
                    else {
                      uVar9 = FUN_004af2d0((unsigned int)uStack_2c, local_8,
                                           (unsigned int)uStack_32);
                      DAT_0051c5b8[(unsigned int)uVar6 * 8 + iVar2] = (char)uVar9;
                      uVar6 = 0xff;
                    }
                  }
                  uStack_32 = (unsigned short)(uStack_32 + 1);
                } while ((int)(unsigned int)uStack_32 < (int)local_1c);
              }
              if (bVar14 == 0x10) break;
            }
            sVar12 = (short)(sVar12 + 1);
          } while ((int)(unsigned int)(unsigned short)sVar12 < local_c);
        }
        if (bVar13 != bVar14) {
          DAT_0051c5a0[local_4] = (char)bVar13;
          DAT_0051c5a2[local_4] = (char)(bVar14 - 1);
          DAT_0051c5a4[local_4] = (char)(bVar14 - bVar13);
        }
      }
      bVar13 = bVar14;
      uVar11 = (unsigned short)(uVar11 + 1);
    } while (uVar11 < 2);
    uVar11 = 0;
    do {
      bVar14 = bVar13;
      if (bVar13 == 0x10) {
        DAT_0051c5a8[iVar2 + (unsigned int)uVar11] = 0;
      }
      else {
        local_20 = (int)((unsigned int)uVar11 + iVar2);
        sVar12 = 0;
        cVar4 = DAT_0051c5a8[local_20];
        if (0 < local_c) {
          do {
            uStack_2e = (unsigned char)sVar12;
            uVar16 = (unsigned int)(unsigned short)sVar12;
            local_1c = FUN_004af250((unsigned int)uStack_2c, uVar16);
            if (local_1c != 0) {
              uVar15 = 0;
              FUN_004ae2a0((unsigned int)uStack_2c, uVar16);
              if (0 < (int)local_1c) {
                local_10 = (unsigned int)(unsigned char)(cVar4 + 0x78);
                do {
                  FUN_004af4c0((unsigned int)uStack_2c, uVar16, uVar15, &local_14, &local_18,
                               &local_28, &local_24);
                  if (local_10 == local_14) {
                    if (param_2 == 0x35) {
                      local_18 = local_18 >> 1;
                      local_28 = local_28 >> 1;
                      local_24 = local_24 >> 1;
                    }
                    uVar7 = (unsigned int)bVar14;
                    bVar14 = (unsigned char)(bVar14 + 1);
                    ST16(&DAT_0051c5b2[uVar7 * 8 + iVar2], (short)local_18);
                    iVar8 = (int)(iVar2 + uVar7 * 8);
                    ST16(&DAT_0051c5b6[iVar8], (short)local_28);
                    ST16(&DAT_0051c5b4[iVar8], (short)local_24);
                    DAT_0051c5b9[iVar8] = (char)uStack_2e;
                    if (bVar14 == 0x10) goto LAB_0049dffc;
                  }
                  uVar15 = (unsigned int)(unsigned short)((short)uVar15 + 1);
                } while ((int)uVar15 < (int)local_1c);
              }
              if (bVar14 == 0x10) break;
            }
            sVar12 = (short)(sVar12 + 1);
          } while ((int)(unsigned int)(unsigned short)sVar12 < local_c);
        }
LAB_0049dffc:
        if (bVar13 != bVar14) {
          DAT_0051c5aa[local_20] = (char)bVar13;
          DAT_0051c5ac[local_20] = (char)(bVar14 - 1);
          DAT_0051c5ae[local_20] = (char)(bVar14 - bVar13);
        }
      }
      bVar13 = bVar14;
      uVar11 = (unsigned short)(uVar11 + 1);
    } while (uVar11 < 2);
  }
  return;
}
