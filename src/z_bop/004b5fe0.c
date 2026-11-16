/* SPDX-License-Identifier: MIT */
/* z_bop 004b5fe0 */
void __cdecl FUN_004b5fe0(int *param_1, int param_2, int param_3, int param_4,
                          int param_5, int param_6, int param_7);

/* DAT_00aa7b13 sits at an unaligned address, so it is read through memcpy. */
extern unsigned char DAT_00aa7b13[4];
extern int DAT_00aa6cfc;

void __cdecl
FUN_004b5fe0(int *param_1, int param_2, int param_3, int param_4,
             int param_5, int param_6, int param_7)
{
  unsigned char bVar1;
  unsigned char uVar2;
  int iVar3;
  int bVar4;
  unsigned char bVar5;
  unsigned int uVar6;
  unsigned int uVar7;
  int iVar8;
  unsigned int uVar9;
  int iVar10;
  unsigned char *pbVar11;
  unsigned char *pbVar12;
  int iVar13;
  int iVar14;
  unsigned char *pbVar15;
  unsigned int *puVar16;
  unsigned int *puVar17;
  int local_14;
  int *local_c;
  int local_8;
  int local_4;

  if (*param_1 == param_6) {
    pbVar11 = (unsigned char *)param_1[4];
    __builtin_memcpy(&local_14, DAT_00aa7b13, 4);
    local_14 = (int)((unsigned int)local_14 * (unsigned int)param_3 + (unsigned int)param_2 +
                     (unsigned int)DAT_00aa6cfc);
    if (0 < param_5) {
      do {
        pbVar11 = pbVar11 + *(int *)pbVar11;
        param_5 = (int)((unsigned int)param_5 - 1u);
      } while (param_5 != 0);
    }
    if (0 < param_7) {
      local_8 = param_7;
      do {
        iVar10 = 0;
        pbVar12 = pbVar11 + 4;
        while (1) {
          bVar5 = *pbVar12;
          pbVar11 = pbVar12 + 1;
          if (bVar5 == 0x80) break;
          if ((bVar5 & 0x80) == 0) {
            if ((bVar5 & 0x40) == 0) {
              bVar1 = *pbVar11;
              pbVar12 = pbVar12 + 2;
              pbVar11 = (unsigned char *)((unsigned int)local_14 + (unsigned int)iVar10);
              iVar10 = (int)((unsigned int)iVar10 + (unsigned int)bVar5);
              for (uVar7 = (unsigned int)(bVar5 >> 2); uVar7 != 0; uVar7 = uVar7 - 1) {
                *(unsigned int *)pbVar11 = (unsigned int)bVar1 * 0x01010101u;
                pbVar11 = pbVar11 + 4;
              }
              for (uVar7 = bVar5 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                *pbVar11 = bVar1;
                pbVar11 = pbVar11 + 1;
              }
            } else {
              bVar5 = bVar5 & 0x3f;
              puVar16 = (unsigned int *)((unsigned int)local_14 + (unsigned int)iVar10);
              iVar10 = (int)((unsigned int)iVar10 + (unsigned int)bVar5);
              for (uVar7 = (unsigned int)(bVar5 >> 2); uVar7 != 0; uVar7 = uVar7 - 1) {
                *puVar16 = 0;
                puVar16 = puVar16 + 1;
              }
              for (uVar7 = bVar5 & 3; pbVar12 = pbVar11, uVar7 != 0; uVar7 = uVar7 - 1) {
                *(unsigned char *)puVar16 = 0;
                puVar16 = (unsigned int *)((int)puVar16 + 1);
              }
            }
          } else {
            uVar7 = bVar5 & 0x7f;
            pbVar15 = (unsigned char *)((unsigned int)local_14 + (unsigned int)iVar10);
            iVar10 = (int)((unsigned int)iVar10 + uVar7);
            pbVar12 = pbVar11 + uVar7;
            for (uVar7 = uVar7 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
              *(unsigned int *)pbVar15 = *(unsigned int *)pbVar11;
              pbVar11 = pbVar11 + 4;
              pbVar15 = pbVar15 + 4;
            }
            for (uVar7 = bVar5 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
              *pbVar15 = *pbVar11;
              pbVar11 = pbVar11 + 1;
              pbVar15 = pbVar15 + 1;
            }
          }
        }
        __builtin_memcpy(&uVar7, DAT_00aa7b13, 4);
        local_14 = (int)((unsigned int)local_14 + uVar7);
        local_8 = (int)((unsigned int)local_8 - 1u);
      } while (local_8 != 0);
      return;
    }
  } else {
    local_c = (int *)param_1[4];
    __builtin_memcpy(&local_14, DAT_00aa7b13, 4);
    local_14 = (int)((unsigned int)local_14 * (unsigned int)param_3 + (unsigned int)param_2 +
                     (unsigned int)DAT_00aa6cfc);
    if (0 < param_5) {
      do {
        local_c = (int *)((int)local_c + *local_c);
        param_5 = (int)((unsigned int)param_5 - 1u);
      } while (param_5 != 0);
    }
    if (0 < param_7) {
      local_4 = param_7;
      do {
        bVar4 = 0;
        iVar3 = *local_c;
        iVar10 = 0;
        iVar14 = 4;
LAB_004b6130:
        do {
          iVar13 = (int)((unsigned int)iVar14 + 1u);
          bVar5 = *((unsigned char *)local_c + iVar14);
          uVar7 = (unsigned int)bVar5;
          iVar8 = iVar10;
          if (!bVar4) {
            if (bVar5 == 0x80) goto LAB_004b6317;
            if ((bVar5 & 0x80) == 0) {
              if ((bVar5 & 0x40) == 0) {
                iVar8 = (int)((unsigned int)bVar5 + (unsigned int)iVar10);
                if (iVar8 < param_4) {
                  iVar13 = (int)((unsigned int)iVar14 + 2u);
                } else {
                  bVar4 = 1;
                  iVar8 = 0;
                  bVar5 = bVar5 - ((char)param_4 - (char)iVar10);
                  uVar7 = (unsigned int)bVar5;
                  if (bVar5 == 0) {
                    iVar13 = (int)((unsigned int)iVar14 + 2u);
                  }
                }
              } else {
                uVar7 = uVar7 & 0xffffff3f;
                iVar8 = (int)((unsigned int)(bVar5 & 0x3f) + (unsigned int)iVar10);
                if (param_4 <= iVar8) {
                  bVar4 = 1;
                  iVar8 = 0;
                  bVar5 = (char)uVar7 - ((char)param_4 - (char)iVar10);
                  uVar7 = (unsigned int)bVar5;
                  if (bVar5 != 0) {
                    uVar7 = bVar5 | 0x40;
                  }
                }
              }
            } else {
              uVar7 = uVar7 & 0xffffff7f;
              iVar8 = (int)((unsigned int)(bVar5 & 0x7f) + (unsigned int)iVar10);
              if (iVar8 < param_4) {
                iVar13 = (int)((unsigned int)iVar13 + (unsigned int)(bVar5 & 0x7f));
              } else {
                bVar4 = 1;
                iVar8 = 0;
                iVar13 = (int)((unsigned int)iVar13 +
                               ((unsigned int)param_4 - (unsigned int)iVar10));
                bVar5 = (char)uVar7 - (char)((unsigned int)param_4 - (unsigned int)iVar10);
                uVar7 = (unsigned int)bVar5;
                if (bVar5 != 0) {
                  uVar7 = bVar5 | 0x80;
                }
              }
            }
          }
          iVar14 = iVar13;
          iVar10 = iVar8;
        } while ((!bVar4) || (bVar5 = (unsigned char)uVar7, bVar5 == 0));
        if (bVar5 != 0x80) {
          if ((uVar7 & 0x80) == 0) {
            if ((uVar7 & 0x40) == 0) {
              iVar13 = (int)((unsigned int)iVar14 + 1u);
              uVar2 = *((unsigned char *)local_c + iVar14);
              iVar10 = (int)(uVar7 + (unsigned int)iVar8);
              if (iVar10 < param_6) {
                puVar16 = (unsigned int *)((unsigned int)local_14 + (unsigned int)iVar8);
                for (uVar9 = uVar7 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
                  *puVar16 = (unsigned int)uVar2 * 0x01010101u;
                  puVar16 = puVar16 + 1;
                }
                for (uVar7 = uVar7 & 3; iVar14 = iVar13, uVar7 != 0; uVar7 = uVar7 - 1) {
                  *(unsigned char *)puVar16 = uVar2;
                  puVar16 = (unsigned int *)((int)puVar16 + 1);
                }
                goto LAB_004b6130;
              }
              uVar6 = (unsigned int)uVar2 * 0x01010101u;
            } else {
              bVar5 = bVar5 & 0x3f;
              iVar10 = (int)((unsigned int)bVar5 + (unsigned int)iVar8);
              if (iVar10 < param_6) {
                puVar16 = (unsigned int *)((unsigned int)local_14 + (unsigned int)iVar8);
                for (uVar7 = (unsigned int)(bVar5 >> 2); uVar7 != 0; uVar7 = uVar7 - 1) {
                  *puVar16 = 0;
                  puVar16 = puVar16 + 1;
                }
                for (uVar7 = bVar5 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                  *(unsigned char *)puVar16 = 0;
                  puVar16 = (unsigned int *)((int)puVar16 + 1);
                }
                goto LAB_004b6130;
              }
              uVar6 = 0;
            }
            bVar5 = (char)param_6 - (char)iVar8;
            puVar16 = (unsigned int *)((unsigned int)local_14 + (unsigned int)iVar8);
            for (uVar7 = (unsigned int)(bVar5 >> 2); uVar7 != 0; uVar7 = uVar7 - 1) {
              *puVar16 = uVar6;
              puVar16 = puVar16 + 1;
            }
            for (uVar7 = bVar5 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
              *(char *)puVar16 = (char)uVar6;
              puVar16 = (unsigned int *)((int)puVar16 + 1);
            }
          } else {
            uVar7 = (unsigned int)(bVar5 & 0x7f);
            iVar10 = (int)(uVar7 + (unsigned int)iVar8);
            if (iVar10 < param_6) {
              puVar16 = (unsigned int *)((int)local_c + iVar14);
              iVar14 = (int)((unsigned int)iVar14 + uVar7);
              puVar17 = (unsigned int *)((unsigned int)local_14 + (unsigned int)iVar8);
              for (uVar9 = (unsigned int)((bVar5 & 0x7f) >> 2); uVar9 != 0; uVar9 = uVar9 - 1) {
                *puVar17 = *puVar16;
                puVar16 = puVar16 + 1;
                puVar17 = puVar17 + 1;
              }
              for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                *(unsigned char *)puVar17 = *(unsigned char *)puVar16;
                puVar16 = (unsigned int *)((int)puVar16 + 1);
                puVar17 = (unsigned int *)((int)puVar17 + 1);
              }
              goto LAB_004b6130;
            }
            bVar5 = (char)param_6 - (char)iVar8;
            puVar16 = (unsigned int *)((int)local_c + iVar14);
            puVar17 = (unsigned int *)((unsigned int)local_14 + (unsigned int)iVar8);
            for (uVar7 = (unsigned int)(bVar5 >> 2); uVar7 != 0; uVar7 = uVar7 - 1) {
              *puVar17 = *puVar16;
              puVar16 = puVar16 + 1;
              puVar17 = puVar17 + 1;
            }
            for (uVar7 = bVar5 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
              *(unsigned char *)puVar17 = *(unsigned char *)puVar16;
              puVar16 = (unsigned int *)((int)puVar16 + 1);
              puVar17 = (unsigned int *)((int)puVar17 + 1);
            }
          }
        }
LAB_004b6317:
        local_c = (int *)((int)local_c + iVar3);
        __builtin_memcpy(&iVar3, DAT_00aa7b13, 4);
        local_14 = (int)((unsigned int)local_14 + (unsigned int)iVar3);
        local_4 = (int)((unsigned int)local_4 - 1u);
        if (local_4 == 0) {
          return;
        }
      } while (1);
    }
  }
  return;
}
