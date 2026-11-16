/* SPDX-License-Identifier: MIT */
/* z_bop 00504700 */
extern uint DAT_0066d968[10];
extern uint DAT_0066d8a0[10];
extern uint DAT_0066d8c0[];
extern uint DAT_0066d8d8[10];
extern uint DAT_0066d900;
extern uint DAT_0066d904;
extern uint DAT_0066d920[10];
extern uint DAT_0066d948[];
extern uint DAT_0066d990[8];
extern uint DAT_0066d9b0[];
extern uint DAT_0066d9c0[8];
extern uint DAT_0066d9e0[];
extern uint DAT_0066d9f8[8];
extern uint DAT_0066da18[10];
extern uint DAT_0066da40[];
extern uint DAT_0066da4c;

extern unsigned char DAT_00b6d5a6[];   /* slot array, stride 0x58, unaligned */
extern unsigned char DAT_00b6d5aa[];
extern unsigned char DAT_00b6d5b2[];
extern unsigned char DAT_00b6d866[];
extern unsigned char DAT_00b6d86a[];
extern unsigned char DAT_00b6d84a[];   /* stride 0x1c */
extern unsigned char DAT_00b6d84e[];
extern unsigned char DAT_00b6bad2[];   /* stride 0x14 */
extern unsigned char DAT_00b6bad6[];
extern unsigned char DAT_00b6bcc2[];
extern unsigned char DAT_00b6bce2[];
extern unsigned char DAT_00b6bda2[];
extern unsigned char DAT_00b6be82[];
extern unsigned char DAT_00b6bea2[];
extern unsigned char DAT_00b6bf62[];
extern unsigned char DAT_00b6a5aa[4];
extern unsigned char DAT_00b6a5a6[4];

extern unsigned char DAT_00a91d18[];   /* stride 0x562 */
extern short DAT_00aa5aac;
extern char DAT_00bb73a4;
extern int DAT_00b69e30;
extern int DAT_00aa5e10[];
extern uint DAT_00aa5e68;

#define LD32(p) ({ uint _t; __builtin_memcpy(&_t, (p), 4); _t; })
#define KEY(x) LD32(DAT_00b6d84e + (uint)DAT_00a91d18[LD32(DAT_00b6d5a6 + (uint)(x) * 0x58u) * 0x562u] * 0x1cu)

undefined4 __stdcall FUN_00504700(void)
{
  byte bVar1;
  uint uVar2;
  int iVar3;
  uint uVar4;
  int iVar5;
  uint *puVar6;
  uint uVar7;
  uint *puVar8;
  int *piVar9;
  uint uVar10;
  uint uVar11;
  int iVar12;
  uint uVar13;
  uint uVar14;
  int iVar15;
  int *piVar16;
  unsigned char *pbVar16;
  byte *pbVar17;
  uint *puVar18;
  uint local_10;
  unsigned char *local_c;
  char *local_8;
  int local_4;

  puVar18 = DAT_0066d968;
  for (iVar5 = 10; iVar5 != 0; iVar5 = iVar5 + -1) {
    *puVar18 = 0;
    puVar18 = puVar18 + 1;
  }
  puVar18 = DAT_0066d8a0;
  for (iVar5 = 10; iVar5 != 0; iVar5 = iVar5 + -1) {
    *puVar18 = 0;
    puVar18 = puVar18 + 1;
  }
  puVar18 = DAT_0066d8d8;
  for (iVar5 = 10; iVar5 != 0; iVar5 = iVar5 + -1) {
    *puVar18 = 0;
    puVar18 = puVar18 + 1;
  }
  DAT_0066d904 = 0;
  iVar5 = 0;
  pbVar16 = DAT_00b6d5a6;
  DAT_0066d900 = 0;
  do {
    if (LD32(pbVar16 + 4) != 0) {
      iVar5 = iVar5 + 1;
      if (DAT_0066d968[DAT_00a91d18[LD32(pbVar16) * 0x562u]] == 0) {
        DAT_0066d904 = DAT_0066d904 + 1;
      }
      DAT_0066d968[DAT_00a91d18[LD32(pbVar16) * 0x562u]] = 1;
    }
    pbVar16 = pbVar16 + 0x58;
  } while (pbVar16 < DAT_00b6d866);
  if ((LD32(DAT_00b6a5aa) == 1) || (LD32(DAT_00b6a5aa) == 2)) {
    if (((DAT_00bb73a4 != '\0') || ((DAT_00b69e30 == 2 || (iVar5 == 1)))) &&
       (iVar5 = (int)DAT_00aa5aac, iVar5 != 0)) {
      pbVar17 = DAT_00a91d18;
      do {
        if (pbVar17[0xc] != 0) {
          bVar1 = *pbVar17;
          if (DAT_0066d968[bVar1] == 0) {
            DAT_0066d904 = DAT_0066d904 + 1;
            DAT_0066d8d8[bVar1] = 1;
          }
          DAT_0066d968[bVar1] = 1;
        }
        pbVar17 = pbVar17 + 0x562;
        iVar5 = iVar5 + -1;
      } while (iVar5 != 0);
    }
    pbVar16 = DAT_00b6bad2;
    puVar8 = DAT_0066d8a0;
    do {
      iVar5 = (int)LD32(pbVar16);
      pbVar16 = pbVar16 + 0x14;
      puVar6 = puVar8 + 1;
      *puVar8 = (uint)(iVar5 != -1);
      puVar8 = puVar6;
    } while (puVar6 < DAT_0066d8c0);
  }
  puVar18 = DAT_0066da18;
  for (iVar5 = 10; iVar5 != 0; iVar5 = iVar5 + -1) {
    *puVar18 = 0xffffffff;
    puVar18 = puVar18 + 1;
  }
  puVar18 = DAT_0066d920;
  for (iVar5 = 10; iVar5 != 0; iVar5 = iVar5 + -1) {
    *puVar18 = 0xffffffff;
    puVar18 = puVar18 + 1;
  }
  puVar18 = DAT_0066d9f8;
  for (iVar5 = 8; iVar5 != 0; iVar5 = iVar5 + -1) {
    *puVar18 = 0xffffffff;
    puVar18 = puVar18 + 1;
  }
  puVar18 = DAT_0066d9c0;
  for (iVar5 = 8; iVar5 != 0; iVar5 = iVar5 + -1) {
    *puVar18 = 0xffffffff;
    puVar18 = puVar18 + 1;
  }
  puVar18 = DAT_0066d990;
  for (iVar5 = 8; iVar5 != 0; iVar5 = iVar5 + -1) {
    *puVar18 = 0xffffffff;
    puVar18 = puVar18 + 1;
  }
  local_10 = 0;
  do {
    if (DAT_0066d968[local_10] != 0) {
      iVar5 = 0;
      puVar8 = DAT_0066d920;
      uVar10 = local_10;
      do {
        uVar7 = *puVar8;
        if (uVar7 == 0xffffffff) {
          DAT_0066d920[iVar5] = uVar10;
          break;
        }
        if ((LD32(DAT_00b6a5aa) == 1) || (LD32(DAT_00b6a5aa) == 2)) {
          if ((int)LD32(DAT_00b6d84a + uVar7 * 0x1cu) < (int)LD32(DAT_00b6d84a + uVar10 * 0x1cu)) goto LAB_00504905;
        }
        else if (((LD32(DAT_00b6d84e + uVar7 * 0x1cu) == 0) && (LD32(DAT_00b6d84e + uVar10 * 0x1cu) == 1)) ||
                ((int)LD32(DAT_00b6d84a + uVar7 * 0x1cu) < (int)LD32(DAT_00b6d84a + uVar10 * 0x1cu))) {
LAB_00504905:
          *puVar8 = uVar10;
          uVar10 = uVar7;
        }
        puVar8 = puVar8 + 1;
        iVar5 = iVar5 + 1;
      } while (puVar8 < DAT_0066d948);
    }
    local_10 = local_10 + 1;
  } while (local_10 < 10);
  iVar5 = 0;
  piVar16 = (int *)DAT_0066d920;
  piVar9 = (int *)DAT_0066d968;
  do {
    if ((*piVar9 != 0) && (iVar12 = iVar5, (uint)*piVar16 == LD32(DAT_00b6a5a6))) break;
    piVar16 = piVar16 + 1;
    piVar9 = piVar9 + 1;
    iVar5 = iVar5 + 1;
    iVar12 = (int)DAT_0066d900;
  } while ((uint *)piVar9 < DAT_0066d990);
  DAT_0066d900 = iVar12;
  if ((LD32(DAT_00b6a5aa) == 1) || (LD32(DAT_00b6a5aa) == 2)) {
    uVar10 = 0;
    do {
      if (DAT_0066d8a0[uVar10] != 0) {
        iVar5 = 0;
        puVar8 = DAT_0066da18;
        uVar7 = uVar10;
        do {
          uVar2 = *puVar8;
          if (uVar2 == 0xffffffff) {
            DAT_0066da18[iVar5] = uVar7;
            break;
          }
          if ((int)LD32(DAT_00b6bad6 + uVar2 * 0x14u) < (int)LD32(DAT_00b6bad6 + uVar7 * 0x14u)) {
            *puVar8 = uVar7;
            uVar7 = uVar2;
          }
          puVar8 = puVar8 + 1;
          iVar5 = iVar5 + 1;
        } while (puVar8 < DAT_0066da40);
      }
      uVar10 = uVar10 + 1;
    } while (uVar10 < 10);
  }
  local_10 = 0;
  local_c = DAT_00b6d5aa;
  do {
    if (LD32(local_c) != 0) {
      local_4 = 0;
      piVar16 = (int *)DAT_0066d990;
      uVar10 = local_10;
      do {
        iVar5 = *piVar16;
        if (iVar5 == -1) {
          DAT_0066d990[local_4] = uVar10;
          break;
        }
        if (((KEY(uVar10) == 1) && (KEY(iVar5) == 0)) ||
           (((KEY(uVar10) != 0 || (KEY(iVar5) != 1)) &&
            ((int)LD32(DAT_00b6d5b2 + (uint)iVar5 * 0x58u) < (int)LD32(DAT_00b6d5b2 + uVar10 * 0x58u))))) {
          *piVar16 = uVar10;
          uVar10 = iVar5;
        }
        piVar16 = piVar16 + 1;
        local_4 = local_4 + 1;
      } while ((uint *)piVar16 < DAT_0066d9b0);
    }
    local_c = local_c + 0x58;
    local_10 = local_10 + 1;
  } while (local_c < DAT_00b6d86a);
  iVar5 = 0;
  pbVar16 = DAT_00b6d5aa;
  do {
    if (LD32(pbVar16) != 0) {
      iVar15 = 0;
      piVar9 = (int *)DAT_0066d9c0;
      iVar12 = iVar5;
      do {
        iVar3 = *piVar9;
        if (iVar3 == -1) {
          DAT_0066d9c0[iVar15] = iVar12;
          break;
        }
        if (LD32(DAT_00b6bcc2 + (uint)iVar3 * 4u) < LD32(DAT_00b6bcc2 + (uint)iVar12 * 4u)) {
          *piVar9 = iVar12;
          iVar12 = iVar3;
        }
        piVar9 = piVar9 + 1;
        iVar15 = iVar15 + 1;
      } while ((uint *)piVar9 < DAT_0066d9e0);
    }
    pbVar16 = pbVar16 + 0x58;
    iVar5 = iVar5 + 1;
  } while (pbVar16 < DAT_00b6d86a);
  if ((LD32(DAT_00b6a5aa) == 1) || (LD32(DAT_00b6a5aa) == 2)) {
    local_10 = 0;
    uVar10 = (uint)(int)DAT_00aa5aac;
    if (uVar10 != 0) {
      local_8 = (char *)DAT_00a91d18 + 0xc;
      do {
        uVar7 = local_10 + 8;
        if (*local_8 != '\0') {
          iVar5 = 0;
          pbVar16 = DAT_00b6d5aa;
          do {
            if ((LD32(pbVar16) != 0) && (LD32(pbVar16 - 4) == local_10)) break;
            pbVar16 = pbVar16 + 0x58;
            iVar5 = iVar5 + 1;
          } while (pbVar16 < DAT_00b6d86a);
          if (iVar5 == 8) {
            local_4 = 0;
            puVar8 = DAT_0066d9c0;
            do {
              uVar2 = *puVar8;
              if (uVar2 == 0xffffffff) {
                DAT_0066d9c0[local_4] = uVar7;
                break;
              }
              if (uVar7 < 8) {
                uVar11 = LD32(DAT_00b6bcc2 + uVar7 * 4u);
                uVar14 = LD32(DAT_00b6bce2 + uVar7 * 4u);
              }
              else {
                uVar11 = LD32(DAT_00b6bce2 + uVar7 * 4u);
                uVar14 = LD32(DAT_00b6bda2 + uVar7 * 4u);
              }
              if ((int)uVar2 < 8) {
                uVar13 = LD32(DAT_00b6bcc2 + uVar2 * 4u);
                uVar4 = LD32(DAT_00b6bce2 + uVar2 * 4u);
              }
              else {
                uVar13 = LD32(DAT_00b6bce2 + uVar2 * 4u);
                uVar4 = LD32(DAT_00b6bda2 + uVar2 * 4u);
              }
              if ((uVar13 < uVar11) || ((uVar13 == uVar11 && (uVar4 < uVar14)))) {
                *puVar8 = uVar7;
                uVar7 = uVar2;
              }
              puVar8 = puVar8 + 1;
              local_4 = local_4 + 1;
            } while (puVar8 < DAT_0066d9e0);
          }
        }
        local_8 = local_8 + 0x562;
        local_10 = local_10 + 1;
      } while (local_10 < uVar10);
    }
  }
  iVar5 = 0;
  pbVar16 = DAT_00b6d5aa;
  do {
    if (LD32(pbVar16) != 0) {
      iVar15 = 0;
      piVar9 = (int *)DAT_0066d9f8;
      iVar12 = iVar5;
      do {
        iVar3 = *piVar9;
        if (iVar3 == -1) {
          DAT_0066d9f8[iVar15] = iVar12;
          break;
        }
        if (LD32(DAT_00b6be82 + (uint)iVar3 * 4u) < LD32(DAT_00b6be82 + (uint)iVar12 * 4u)) {
          *piVar9 = iVar12;
          iVar12 = iVar3;
        }
        piVar9 = piVar9 + 1;
        iVar15 = iVar15 + 1;
      } while ((uint *)piVar9 < DAT_0066da18);
    }
    pbVar16 = pbVar16 + 0x58;
    iVar5 = iVar5 + 1;
  } while (pbVar16 < DAT_00b6d86a);
  if ((LD32(DAT_00b6a5aa) == 1) || (LD32(DAT_00b6a5aa) == 2)) {
    local_10 = 0;
    uVar10 = (uint)(int)DAT_00aa5aac;
    if (uVar10 != 0) {
      local_8 = (char *)DAT_00a91d18 + 0xc;
      do {
        uVar7 = local_10 + 8;
        if (*local_8 != '\0') {
          iVar5 = 0;
          pbVar16 = DAT_00b6d5aa;
          do {
            if ((LD32(pbVar16) != 0) && (LD32(pbVar16 - 4) == local_10)) break;
            pbVar16 = pbVar16 + 0x58;
            iVar5 = iVar5 + 1;
          } while (pbVar16 < DAT_00b6d86a);
          if (iVar5 == 8) {
            local_4 = 0;
            puVar8 = DAT_0066d9f8;
            do {
              uVar2 = *puVar8;
              if (uVar2 == 0xffffffff) {
                DAT_0066d9f8[local_4] = uVar7;
                break;
              }
              if (uVar7 < 8) {
                uVar11 = LD32(DAT_00b6be82 + uVar7 * 4u);
                uVar14 = LD32(DAT_00b6bea2 + uVar7 * 4u);
              }
              else {
                uVar11 = LD32(DAT_00b6bea2 + uVar7 * 4u);
                uVar14 = LD32(DAT_00b6bf62 + uVar7 * 4u);
              }
              if ((int)uVar2 < 8) {
                uVar13 = LD32(DAT_00b6be82 + uVar2 * 4u);
                uVar4 = LD32(DAT_00b6bea2 + uVar2 * 4u);
              }
              else {
                uVar13 = LD32(DAT_00b6bea2 + uVar2 * 4u);
                uVar4 = LD32(DAT_00b6bf62 + uVar2 * 4u);
              }
              if ((uVar13 < uVar11) || ((uVar13 == uVar11 && (uVar4 < uVar14)))) {
                *puVar8 = uVar7;
                uVar7 = uVar2;
              }
              puVar8 = puVar8 + 1;
              local_4 = local_4 + 1;
            } while (puVar8 < DAT_0066da18);
          }
        }
        local_8 = local_8 + 0x562;
        local_10 = local_10 + 1;
      } while (local_10 < uVar10);
    }
  }
  uVar10 = 0;
  DAT_0066da4c = 0;
  if (LD32(DAT_00b6a5aa) == 1) {
    if (DAT_00aa5e68 != 0) {
      piVar16 = DAT_00aa5e10;
      do {
        if (1 < *piVar16) break;
        piVar16 = piVar16 + 1;
        uVar10 = uVar10 + 1;
      } while (uVar10 < DAT_00aa5e68);
      if (DAT_00aa5e68 != uVar10) {
        DAT_0066da4c = 0;
        return 1;
      }
    }
    DAT_0066da4c = 1;
  }
  return 1;
}
