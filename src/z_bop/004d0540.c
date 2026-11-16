/* SPDX-License-Identifier: MIT */
/* z_bop 004d0540 */
extern unsigned char DAT_00acf56d[];
extern unsigned char DAT_00acf571[];   /* record table, stride 0x210 */
extern unsigned char DAT_00b53575[];
extern unsigned char DAT_00b53579[];
extern unsigned char DAT_00b63d81[];   /* 0x210-byte copy buffer */
extern unsigned char DAT_00b63d91[];
extern unsigned char DAT_00b63f95[];   /* per-entry table, stride 0x238 */
extern unsigned char DAT_00b63f9d[];
extern unsigned char DAT_00b69a89[];

DWORD __stdcall GetTickCount(void);
int __cdecl _sprintf(char *, const char *, ...);
void __cdecl FUN_004ce130(void);
void __cdecl FUN_004d1ea0(void);
unsigned int __cdecl FUN_004d23b0(unsigned int);
int __cdecl FUN_004d2170(unsigned int);
unsigned int __cdecl FUN_004d2080(unsigned int, unsigned int, unsigned int, unsigned int, unsigned int);
void __cdecl FUN_004cf830(unsigned int, unsigned int *, int);

#define REC(i) (DAT_00acf571 + (unsigned int)(i) * 0x210u)
#define PLR(i) (DAT_00b63f95 + (unsigned int)(i) * 0x238u)
/* unaligned little-endian dword access, byte-wise (no alignment assumptions) */
#define RD32(p) ((unsigned int)(p)[0] | ((unsigned int)(p)[1] << 8) | ((unsigned int)(p)[2] << 16) | ((unsigned int)(p)[3] << 24))
#define WR32(p, v) (wtmp = (v), (p)[0] = (unsigned char)wtmp, (p)[1] = (unsigned char)(wtmp >> 8), (p)[2] = (unsigned char)(wtmp >> 16), (p)[3] = (unsigned char)(wtmp >> 24), (void)0)

unsigned char * __cdecl FUN_004d0540(unsigned int *param_1, unsigned int *param_2)
{
  unsigned char bVar1;
  char cVar2;
  unsigned int uVar3;
  int bVar4;
  unsigned int uVar5;
  int iVar6;
  unsigned int uVar7;
  unsigned int uVar8;
  DWORD DVar9;
  unsigned int uVar10;
  unsigned int uVar11;
  int iVar12;
  int iVar13;
  unsigned char *pbVar14;
  unsigned char *pbVar15;
  unsigned int uVar16;
  unsigned int uVar17;
  unsigned char *puVar18;
  unsigned char *puVar20;
  unsigned int wtmp;
  unsigned int local_3f0;
  int local_3d8;
  unsigned int local_3d4;
  unsigned char local_3c8[120];
  unsigned char local_350[40];
  unsigned char local_328[40];
  char local_300[256];
  unsigned int local_200[3]; /* local_200 / local_1fc / local_1f8, contiguous */

  FUN_004ce130();
  FUN_004d1ea0();
  if (RD32(DAT_00b53579) == 0) {
    return (unsigned char *)0;
  }
  pbVar14 = local_350;
  for (iVar13 = 10; iVar13 != 0; iVar13 = iVar13 + -1) {
    pbVar14[0] = 0;
    pbVar14[1] = 0;
    pbVar14[2] = 0;
    pbVar14[3] = 0;
    pbVar14 = pbVar14 + 4;
  }
  pbVar14 = local_328;
  for (iVar13 = 10; iVar13 != 0; iVar13 = iVar13 + -1) {
    pbVar14[0] = 0x5a;
    pbVar14[1] = 0x5a;
    pbVar14[2] = 0x5a;
    pbVar14[3] = 0x5a;
    pbVar14 = pbVar14 + 4;
  }
  pbVar14 = local_3c8;
  for (iVar13 = 0x1e; iVar13 != 0; iVar13 = iVar13 + -1) {
    pbVar14[0] = 0;
    pbVar14[1] = 0;
    pbVar14[2] = 0;
    pbVar14[3] = 0;
    pbVar14 = pbVar14 + 4;
  }
  uVar17 = RD32(DAT_00b53575);
  iVar13 = (int)RD32(DAT_00b53579);
  if (0 < (int)RD32(DAT_00b69a89)) {
    uVar5 = RD32(DAT_00b69a89);
    pbVar14 = local_3c8;
    pbVar15 = DAT_00b63f9d;
    do {
      *pbVar14 = *pbVar15;
      uVar5 = uVar5 - 1u;
      pbVar14[1] = pbVar15[-8];
      pbVar14[2] = pbVar15[8];
      pbVar14 = pbVar14 + 3;
      pbVar15 = pbVar15 + 0x238;
    } while (uVar5 != 0);
  }
  do {
    if (iVar13 < 1) {
      uVar17 = RD32(DAT_00b53575);
      iVar13 = (int)RD32(DAT_00b53579);
      if (0x3fe < (int)RD32(DAT_00b53579)) {
        for (; 0 < iVar13; iVar13 = (int)((unsigned int)iVar13 - 1u)) {
          iVar6 = (int)(uVar17 * 0x210u);
          if (RD32(REC(uVar17)) == 0) {
            uVar17 = uVar17 + 1u;
          }
          else {
            uVar5 = (unsigned int)REC(uVar17)[0xe];
            cVar2 = (char)REC(uVar17)[0xd];
            uVar11 = FUN_004d23b0(RD32(REC(uVar17)));
            if ((uVar11 < RD32(DAT_00b69a89)) && (uVar11 < 0x28)) {
              if (cVar2 != '\0') {
                if (cVar2 != '\x02') {
                  iVar12 = (int)RD32(PLR(uVar11));
                }
                else {
                  iVar12 = (int)RD32(PLR(uVar11) + 0x10);
                }
              }
              else {
                iVar12 = (int)RD32(PLR(uVar11) + 8);
              }
              local_3f0 = (unsigned int)iVar12 + 1u;
              if (0x7f < (int)local_3f0) {
                local_3f0 = 0;
              }
            }
            iVar12 = (int)(uVar5 - local_3f0);
            if ((uVar11 < RD32(DAT_00b69a89)) && ((iVar12 < -0x1b || ((-1 < iVar12 && (iVar12 < 100)))))) {
              if (REC(uVar17)[0xc] != 0) {
                if (cVar2 != '\0') {
                  if (cVar2 != '\x02') {
                    _sprintf(local_300, "(ROOS %u) ", uVar5);
                    WR32(PLR(uVar11), uVar5);
                  }
                  else {
                    _sprintf(local_300, "(ROOG %u) ", uVar5);
                    WR32(PLR(uVar11) + 0x10, uVar5);
                  }
                }
                else {
                  _sprintf(local_300, "(ROOB %u) ", uVar5);
                  WR32(PLR(uVar11) + 8, uVar5);
                }
                WR32(PLR(uVar11) + 0x224, RD32(PLR(uVar11) + 0x224) + 1u);
                puVar18 = REC(uVar17);
                puVar20 = DAT_00b63d81;
                for (iVar13 = 0x84; iVar13 != 0; iVar13 = iVar13 + -1) {
                  puVar20[0] = puVar18[0];
                  puVar20[1] = puVar18[1];
                  puVar20[2] = puVar18[2];
                  puVar20[3] = puVar18[3];
                  puVar18 = puVar18 + 4;
                  puVar20 = puVar20 + 4;
                }
                FUN_004d2170(uVar17);
                *param_1 = RD32(DAT_00b63d81);
                *param_2 = RD32(DAT_00b63d81 + 4);
                return DAT_00b63d91;
              }
              uVar17 = uVar17 + 1u;
            }
            else {
              iVar6 = FUN_004d2170(uVar17);
              if (iVar6 != 0) {
                uVar17 = uVar17 + 1u;
              }
            }
          }
          if (0x3ff < (int)uVar17) {
            uVar17 = 0;
          }
        }
      }
      return (unsigned char *)0;
    }
    iVar6 = (int)(uVar17 * 0x210u);
    if (RD32(REC(uVar17)) == 0) {
      if (RD32(DAT_00b53575) == uVar17) {
        puVar18 = REC(uVar17);
        puVar20 = DAT_00b63d81;
        for (iVar13 = 0x84; iVar13 != 0; iVar13 = iVar13 + -1) {
          puVar20[0] = puVar18[0];
          puVar20[1] = puVar18[1];
          puVar20[2] = puVar18[2];
          puVar20[3] = puVar18[3];
          puVar18 = puVar18 + 4;
          puVar20 = puVar20 + 4;
        }
        *param_1 = RD32(REC(uVar17));
        *param_2 = RD32(REC(uVar17) + 4);
        WR32(DAT_00b53579, RD32(DAT_00b53579) - 1u);
        WR32(DAT_00b53575, RD32(DAT_00b53575) + 1u);
        if (0x3ff < (int)RD32(DAT_00b53575)) {
          WR32(DAT_00b53575, 0);
        }
        return DAT_00b63d91;
      }
joined_r0x004d0616:
      uVar17 = uVar17 + 1u;
      if (0x3ff < (int)uVar17) {
        uVar17 = 0;
      }
    }
    else {
      bVar1 = REC(uVar17)[0xe];
      uVar7 = (unsigned int)bVar1;
      uVar11 = (unsigned int)(REC(uVar17)[0xd] == 0);
      uVar3 = (unsigned int)(REC(uVar17)[0xd] == 2);
      uVar5 = RD32(REC(uVar17) + 0x10);
      uVar8 = FUN_004d23b0(RD32(REC(uVar17)));
      if ((local_3d4 == uVar8) && (uVar8 < 0x28)) {
        iVar12 = (int)(uVar8 * 3u);
        local_3c8[iVar12] = 0x7f;
        local_3c8[iVar12 + 1] = 0x7f;
        local_3c8[iVar12 + 2] = 0x7f;
      }
      if (RD32(DAT_00b69a89) <= uVar8) {
LAB_004d07c7:
        iVar12 = (int)(uVar7 - local_3f0);
        if ((uVar8 < RD32(DAT_00b69a89)) && ((iVar12 < -0x1c || ((-1 < iVar12 && (iVar12 < 100)))))) {
          if (local_3f0 == uVar7) {
            DVar9 = GetTickCount();
            WR32(PLR(uVar8) + 0x220, DVar9);
            if (uVar11 == 0) {
              if (uVar3 == 0) {
                _sprintf(local_300, "(RS %u) ", uVar7);
                WR32(PLR(uVar8), uVar7);
              }
              else {
                _sprintf(local_300, "(RG %u) ", uVar7);
                WR32(PLR(uVar8) + 0x10, uVar7);
              }
            }
            else {
              _sprintf(local_300, "(RB %u) ", uVar7);
              WR32(PLR(uVar8) + 8, uVar7);
            }
            WR32(PLR(uVar8) + 0x224, RD32(PLR(uVar8) + 0x224) + 1u);
            puVar18 = REC(uVar17);
            puVar20 = DAT_00b63d81;
            for (iVar13 = 0x84; iVar13 != 0; iVar13 = iVar13 + -1) {
              puVar20[0] = puVar18[0];
              puVar20[1] = puVar18[1];
              puVar20[2] = puVar18[2];
              puVar20[3] = puVar18[3];
              puVar18 = puVar18 + 4;
              puVar20 = puVar20 + 4;
            }
            FUN_004d2170(uVar17);
            *param_1 = RD32(DAT_00b63d81);
            *param_2 = RD32(DAT_00b63d81 + 4);
            return DAT_00b63d91;
          }
          if (REC(uVar17)[0xf] == 0) {
            if (uVar11 == 0) {
              local_3d4 = 2u - (unsigned int)(uVar3 == 0);
            }
            else {
              local_3d4 = 0;
            }
            uVar5 = (unsigned int)local_3c8[uVar8 * 3u + local_3d4] + 1u;
            local_3c8[uVar8 * 3u + local_3d4] = bVar1;
            if (0x7f < uVar5) {
              uVar5 = 0;
            }
            local_3d8 = (int)(uVar7 - uVar5);
            if (local_3d8 < 0) {
              local_3d8 = (int)((unsigned int)local_3d8 + 0x80u);
            }
            pbVar14 = local_328 + uVar8;
            bVar4 = 0;
            uVar16 = uVar5;
            if ((int)(unsigned int)*pbVar14 < local_3d8) {
              *pbVar14 = 0;
            }
            else {
              *pbVar14 = (unsigned char)(*pbVar14 - (char)local_3d8);
            }
            while (uVar7 != uVar16) {
              uVar10 = FUN_004d2080(uVar17, uVar16, uVar11, uVar3, uVar8);
              if (((int)uVar10 < 0x400) && (-1 < (int)uVar10)) {
                if (local_3f0 == uVar16) {
                  DVar9 = GetTickCount();
                  WR32(PLR(uVar8) + 0x220, DVar9);
                  puVar18 = REC(uVar10);
                  puVar20 = DAT_00b63d81;
                  for (iVar13 = 0x84; iVar13 != 0; iVar13 = iVar13 + -1) {
                    puVar20[0] = puVar18[0];
                    puVar20[1] = puVar18[1];
                    puVar20[2] = puVar18[2];
                    puVar20[3] = puVar18[3];
                    puVar18 = puVar18 + 4;
                    puVar20 = puVar20 + 4;
                  }
                  if (uVar11 == 0) {
                    if (uVar3 == 0) {
                      _sprintf(local_300, "(ROOS %u) ", local_3f0);
                      WR32(PLR(uVar8), local_3f0);
                    }
                    else {
                      _sprintf(local_300, "(ROOG %u) ", local_3f0);
                      WR32(PLR(uVar8) + 0x10, local_3f0);
                    }
                  }
                  else {
                    _sprintf(local_300, "(ROOB %u) ", local_3f0);
                    WR32(PLR(uVar8) + 8, local_3f0);
                  }
                  WR32(PLR(uVar8) + 0x224, RD32(PLR(uVar8) + 0x224) + 1u);
                  *param_1 = RD32(DAT_00b63d81);
                  *param_2 = RD32(DAT_00b63d81 + 4);
                  if (local_3d8 < 2) {
                    REC(uVar17)[0xc] = 0;
                    WR32(REC(uVar17) + 8, 0);
                  }
                  FUN_004d2170(uVar10);
                  return DAT_00b63d91;
                }
                local_3d8 = (int)((unsigned int)local_3d8 - 1u);
              }
              else {
                if (REC(uVar17)[0xc] == 0) {
                  _sprintf(local_300, "(RP %d) ", uVar16);
                  if (0x14 < RD32(PLR(uVar8) + 0x224)) {
                    WR32(PLR(uVar8) + 0x22c, RD32(PLR(uVar8) + 0x22c) + 1u);
                  }
                }
                else {
                  DVar9 = GetTickCount();
                  if (RD32(DAT_00acf56d) == 1) {
                    bVar1 = 0;
                    uVar10 = 20000;
                  }
                  else {
                    bVar1 = 0x14;
                    uVar10 = 1000;
                  }
                  if (DVar9 - RD32(REC(uVar17) + 8) <= uVar10) goto LAB_004d09c8;
                  if (bVar1 < REC(uVar17)[0xc]) {
                    DVar9 = GetTickCount();
                    WR32(PLR(uVar8) + 0x220, DVar9);
                    while ((uVar16 = FUN_004d2080(uVar17, uVar5, uVar11, uVar3, uVar8), (int)uVar16 < 0
                           || (0x400 < (int)uVar16))) {
                      if (uVar7 == uVar5) {
                        if (uVar11 == 0) {
                          if (uVar3 == 0) {
                            _sprintf(local_300, "(ROOS %u) ", uVar7);
                            WR32(PLR(uVar8), uVar7);
                          }
                          else {
                            _sprintf(local_300, "(ROOG %u) ", uVar7);
                            WR32(PLR(uVar8) + 0x10, uVar7);
                          }
                        }
                        else {
                          _sprintf(local_300, "(ROOB %u) ", uVar7);
                          WR32(PLR(uVar8) + 8, uVar7);
                        }
                        WR32(PLR(uVar8) + 0x224, RD32(PLR(uVar8) + 0x224) + 1u);
                        puVar18 = REC(uVar17);
                        puVar20 = DAT_00b63d81;
                        for (iVar13 = 0x84; iVar13 != 0; iVar13 = iVar13 + -1) {
                          puVar20[0] = puVar18[0];
                          puVar20[1] = puVar18[1];
                          puVar20[2] = puVar18[2];
                          puVar20[3] = puVar18[3];
                          puVar18 = puVar18 + 4;
                          puVar20 = puVar20 + 4;
                        }
                        FUN_004d2170(uVar17);
                        *param_1 = RD32(DAT_00b63d81);
                        *param_2 = RD32(DAT_00b63d81 + 4);
                        return DAT_00b63d91;
                      }
                      uVar5 = uVar5 + 1u;
                      if (0x7f < (int)uVar5) {
                        uVar5 = 0;
                      }
                    }
                    puVar18 = REC(uVar16);
                    puVar20 = DAT_00b63d81;
                    for (iVar13 = 0x84; iVar13 != 0; iVar13 = iVar13 + -1) {
                      puVar20[0] = puVar18[0];
                      puVar20[1] = puVar18[1];
                      puVar20[2] = puVar18[2];
                      puVar20[3] = puVar18[3];
                      puVar18 = puVar18 + 4;
                      puVar20 = puVar20 + 4;
                    }
                    FUN_004d2170(uVar16);
                    if (uVar11 == 0) {
                      if (uVar3 == 0) {
                        _sprintf(local_300, "(ROOS %u) ", local_3f0);
                        WR32(PLR(uVar8), uVar5);
                      }
                      else {
                        _sprintf(local_300, "(ROOG %u) ", local_3f0);
                        WR32(PLR(uVar8) + 0x10, uVar5);
                      }
                    }
                    else {
                      _sprintf(local_300, "(ROOB %u) ", local_3f0);
                      WR32(PLR(uVar8) + 8, uVar5);
                    }
                    WR32(PLR(uVar8) + 0x224, RD32(PLR(uVar8) + 0x224) + 1u);
                    *param_1 = RD32(DAT_00b63d81);
                    *param_2 = RD32(DAT_00b63d81 + 4);
                    return DAT_00b63d91;
                  }
                  _sprintf(local_300, "(RP %d) ", uVar16);
                }
                local_200[2] = 0;
                if (uVar11 == 0) {
                  local_200[2] = 2u - (unsigned int)(uVar3 == 0);
                }
                local_200[0] = 0x38;
                local_200[1] = uVar16;
                FUN_004cf830(RD32(REC(uVar17)), local_200, 0xc);
                bVar4 = 1;
              }
LAB_004d09c8:
              uVar16 = uVar16 + 1u;
              if (0x7f < (int)uVar16) {
                uVar16 = 0;
              }
            }
            if (local_3d8 < 1) {
              REC(uVar17)[0xc] = 0;
              WR32(REC(uVar17) + 8, 0);
            }
            else if (bVar4) {
              REC(uVar17)[0xc] = (unsigned char)(REC(uVar17)[0xc] + 1);
              DVar9 = GetTickCount();
              WR32(REC(uVar17) + 8, DVar9);
            }
          }
          else if (RD32(DAT_00b53575) == uVar17) {
            iVar6 = FUN_004d2170(uVar17);
            goto joined_r0x004d0a36;
          }
        }
        else {
          iVar6 = FUN_004d2170(uVar17);
          if (iVar6 == 0) goto LAB_004d0a5b;
        }
        goto joined_r0x004d0616;
      }
      if (0x32 < uVar5) {
        if (local_328[uVar8] < local_350[uVar8]) goto joined_r0x004d0616;
        DVar9 = GetTickCount();
        WR32(PLR(uVar8) + 0x230, DVar9);
        if (REC(uVar17)[0xf] == 0) {
          local_350[uVar8] = (unsigned char)(local_350[uVar8] + 1);
        }
        if (uVar11 == 0) {
          if (uVar3 == 0) {
            iVar12 = (int)RD32(PLR(uVar8));
          }
          else {
            iVar12 = (int)RD32(PLR(uVar8) + 0x10);
          }
        }
        else {
          iVar12 = (int)RD32(PLR(uVar8) + 8);
        }
        local_3f0 = (unsigned int)iVar12 + 1u;
        if (0x7f < (int)local_3f0) {
          local_3f0 = 0;
        }
        goto LAB_004d07c7;
      }
      if (REC(uVar17)[0xf] == 0) {
        if (uVar11 == 0) {
          if (uVar3 == 0) {
            WR32(PLR(uVar8), uVar7);
            local_3c8[uVar8 * 3u + 1u] = bVar1;
          }
          else {
            WR32(PLR(uVar8) + 0x10, uVar7);
            local_3c8[uVar8 * 3u + 2u] = bVar1;
          }
        }
        else {
          local_3c8[uVar8 * 3u] = bVar1;
          WR32(PLR(uVar8) + 8, uVar7);
        }
      }
      iVar6 = FUN_004d2170(uVar17);
joined_r0x004d0a36:
      if (iVar6 != 0) goto joined_r0x004d0616;
    }
LAB_004d0a5b:
    iVar13 = (int)((unsigned int)iVar13 - 1u);
  } while (1);
}
