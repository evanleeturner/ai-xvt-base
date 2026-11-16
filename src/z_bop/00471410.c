/* SPDX-License-Identifier: MIT */
/* z_bop 00471410 */
extern int DAT_0099943c;
extern unsigned char DAT_00999432[4]; /* unaligned 4-byte read at 0x00999432 */
extern float _DAT_005181c0;

void __cdecl FUN_00486980(float param_1, float param_2, uint param_3, int param_4);

#define F(p, i) (*(float *)&(p)[(i)])

void __cdecl FUN_00471410(int param_1)
{
  float fVar1;
  int *piVar2;
  int iVar3;
  int *piVar4;
  uint *puVar5;
  uint *puVar6;
  uint *puVar7;
  int iVar8;
  uint uVar9;
  uint uVar10;
  int bVar11;
  int local_1c;
  float *local_18;
  float local_14;
  float local_10;
  uint local_8;
  uint local_4;
  int tmpBase;

  piVar2 = (int *)(param_1 + 0x50);
  local_1c = *(int *)(param_1 + 100);
  puVar6 = (uint *)*piVar2;
  puVar7 = puVar6;
  if (1 < local_1c) {
    iVar3 = local_1c + -1;
    piVar4 = (int *)(param_1 + 0x54);
    do {
      puVar5 = (uint *)*piVar4;
      if ((int)puVar5[1] < (int)puVar7[1]) {
        puVar7 = puVar5;
      }
      if ((int)*puVar6 < (int)*puVar5) {
        puVar6 = puVar5;
      }
      piVar4 = piVar4 + 1;
      iVar3 = iVar3 + -1;
    } while (iVar3 != 0);
  }
  uVar10 = puVar7[1];
  *(uint *)(param_1 + 0x40) = uVar10;
  uVar9 = *puVar6;
  *(uint *)(param_1 + 0x44) = uVar9;
  iVar3 = (int)(uVar9 - uVar10);
  if (DAT_0099943c <= iVar3) {
    *(uint *)(param_1 + 0x44) = uVar10;
    return;
  }
  iVar8 = 0;
  DAT_0099943c = (int)((uint)DAT_0099943c - (uint)iVar3);
  iVar3 = *(int *)(param_1 + 100);
  __builtin_memcpy(&tmpBase, DAT_00999432, 4);
  *(int *)(param_1 + 0x68) = (int)((uint)tmpBase + (uint)DAT_0099943c * 4u);
  bVar11 = (iVar3 == 0);
  piVar4 = piVar2;
  if (0 < iVar3) {
    do {
      if ((puVar7 != (uint *)*piVar4) && (((uint *)*piVar4)[1] == puVar7[1])) {
        puVar6 = *(uint **)(param_1 + 0x50 + iVar8 * 4);
        break;
      }
      iVar8 = iVar8 + 1;
      piVar4 = piVar4 + 1;
    } while (iVar8 < iVar3);
    bVar11 = (iVar8 == iVar3);
  }
  if (bVar11) {
    *(uint *)(param_1 + 0x44) = *(uint *)(param_1 + 0x40);
    return;
  }
  if ((F(puVar6, 2) < F(puVar7, 2)) ||
      ((puVar5 = puVar6, F(puVar6, 2) == F(puVar7, 2)) &&
       (F(puVar6, 4) < F(puVar7, 4)))) {
    puVar5 = puVar7;
    puVar7 = puVar6;
  }
  *(uint **)(param_1 + 0x38) = puVar7;
  uVar10 = puVar7[1];
  uVar9 = *puVar5;
  if ((int)*puVar7 < (int)*puVar5) {
    uVar9 = *puVar7;
  }
  iVar3 = (int)(uVar9 - uVar10);
  local_14 = F(puVar7, 2);
  local_10 = F(puVar5, 2);
  local_8 = puVar7[3];
  local_4 = puVar5[3];
  if (_DAT_005181c0 < local_10 - local_14) {
    while (1) {
      iVar3 = (int)((uint)iVar3 - 1u);
      uVar9 = uVar10 + 1;
      *(float *)(param_1 + 0x34) =
          (F(puVar5, 3) - F(puVar7, 3)) / (F(puVar5, 2) - F(puVar7, 2));
      FUN_00486980(F(puVar7, 2), F(puVar5, 2), uVar10, param_1);
      if (iVar3 < 1) break;
      F(puVar7, 2) = F(puVar7, 4) + F(puVar7, 2);
      F(puVar7, 3) = F(puVar7, 5) + F(puVar7, 3);
      F(puVar5, 2) = F(puVar5, 4) + F(puVar5, 2);
      F(puVar5, 3) = F(puVar5, 5) + F(puVar5, 3);
      uVar10 = uVar9;
    }
  }
  else {
    fVar1 = (float)iVar3;
    *(float *)(param_1 + 0x34) =
        ((fVar1 * F(puVar5, 5) + F(puVar5, 3)) -
         (fVar1 * F(puVar7, 5) + F(puVar7, 3))) /
        ((fVar1 * F(puVar5, 4) + F(puVar5, 2)) -
         (fVar1 * F(puVar7, 4) + F(puVar7, 2)));
    while (1) {
      iVar3 = (int)((uint)iVar3 - 1u);
      uVar9 = uVar10 + 1;
      FUN_00486980(F(puVar7, 2), F(puVar5, 2), uVar10, param_1);
      if (iVar3 < 1) break;
      F(puVar7, 2) = F(puVar7, 2) + F(puVar7, 4);
      F(puVar7, 3) = F(puVar7, 5) + F(puVar7, 3);
      F(puVar5, 2) = F(puVar5, 2) + F(puVar5, 4);
      F(puVar5, 3) = F(puVar5, 5) + F(puVar5, 3);
      uVar10 = uVar9;
    }
  }
  do {
    local_18 = (float *)(param_1 + 0x34);
    if (local_1c < 1) {
LAB_004719cf:
      F(puVar7, 2) = local_14;
      F(puVar5, 2) = local_10;
      puVar7[3] = local_8;
      puVar5[3] = local_4;
      return;
    }
    if (*puVar5 == *puVar7) {
      local_1c = local_1c + -2;
      if (local_1c != 0) {
        F(puVar7, 2) = local_14;
        F(puVar5, 2) = local_10;
        iVar8 = 0;
        puVar7[3] = local_8;
        puVar5[3] = local_4;
        iVar3 = *(int *)(param_1 + 100);
        bVar11 = (iVar3 == 0);
        piVar4 = piVar2;
        if (0 < iVar3) {
          do {
            if (*(uint *)(*piVar4 + 4) == uVar9) {
              puVar7 = *(uint **)(param_1 + 0x50 + iVar8 * 4);
              break;
            }
            iVar8 = iVar8 + 1;
            piVar4 = piVar4 + 1;
          } while (iVar8 < iVar3);
          bVar11 = (iVar8 == iVar3);
        }
        if (!bVar11) {
          iVar8 = iVar8 + 1;
          if (iVar8 < iVar3) {
            piVar4 = (int *)(param_1 + 0x50 + iVar8 * 4);
            do {
              if (*(uint *)(*piVar4 + 4) == uVar9) {
                puVar5 = *(uint **)(param_1 + 0x50 + iVar8 * 4);
                break;
              }
              piVar4 = piVar4 + 1;
              iVar8 = iVar8 + 1;
            } while (iVar8 < iVar3);
          }
          if (iVar8 != iVar3) {
            if ((F(puVar5, 2) < F(puVar7, 2)) ||
                ((puVar6 = puVar5, F(puVar5, 2) == F(puVar7, 2)) &&
                 (F(puVar5, 4) < F(puVar7, 4)))) {
              puVar6 = puVar7;
              puVar7 = puVar5;
            }
            *(uint **)(param_1 + 0x38) = puVar7;
            local_8 = puVar7[3];
            local_14 = F(puVar7, 2);
            local_10 = F(puVar6, 2);
            local_4 = puVar6[3];
            goto LAB_00471853;
          }
        }
LAB_004719cc:
        *(uint *)(param_1 + 0x44) = uVar9;
      }
      goto LAB_004719cf;
    }
    local_1c = local_1c + -1;
    puVar6 = puVar5;
    if (uVar9 == *puVar7) {
      iVar8 = 0;
      F(puVar7, 2) = local_14;
      puVar7[3] = local_8;
      iVar3 = *(int *)(param_1 + 100);
      bVar11 = (iVar3 == 0);
      piVar4 = piVar2;
      if (0 < iVar3) {
        do {
          if (*(uint *)(*piVar4 + 4) == uVar9) {
            puVar7 = *(uint **)(param_1 + 0x50 + iVar8 * 4);
            break;
          }
          iVar8 = iVar8 + 1;
          piVar4 = piVar4 + 1;
        } while (iVar8 < iVar3);
        bVar11 = (iVar8 == iVar3);
      }
      if (bVar11) goto LAB_004719cc;
      *(uint **)(param_1 + 0x38) = puVar7;
      local_14 = F(puVar7, 2);
      local_8 = puVar7[3];
      F(puVar5, 2) = F(puVar5, 4) + F(puVar5, 2);
      F(puVar5, 3) = F(puVar5, 5) + F(puVar5, 3);
    }
    else {
      iVar8 = 0;
      F(puVar5, 2) = local_10;
      puVar5[3] = local_4;
      iVar3 = *(int *)(param_1 + 100);
      bVar11 = (iVar3 == 0);
      piVar4 = piVar2;
      if (0 < iVar3) {
        do {
          if (*(uint *)(*piVar4 + 4) == uVar9) {
            puVar5 = *(uint **)(param_1 + 0x50 + iVar8 * 4);
            break;
          }
          iVar8 = iVar8 + 1;
          piVar4 = piVar4 + 1;
        } while (iVar8 < iVar3);
        bVar11 = (iVar8 == iVar3);
        puVar6 = puVar5;
      }
      puVar5 = puVar6;
      if (bVar11) goto LAB_004719cc;
      local_10 = F(puVar6, 2);
      local_4 = puVar6[3];
      F(puVar7, 2) = F(puVar7, 4) + F(puVar7, 2);
      F(puVar7, 3) = F(puVar7, 5) + F(puVar7, 3);
    }
LAB_00471853:
    if (local_1c == 1) {
      return;
    }
    puVar5 = puVar6;
    if (local_1c == 2) {
      if (*puVar6 != *puVar7) {
        return;
      }
      iVar3 = (int)(*puVar7 - uVar9);
      uVar10 = uVar9;
      if (_DAT_005181c0 <
          ((F(puVar6, 4) * (float)iVar3 + F(puVar6, 2)) -
           (F(puVar7, 4) * (float)iVar3 + F(puVar7, 2)))) {
        while (1) {
          local_18 = (float *)(param_1 + 0x34);
          iVar3 = (int)((uint)iVar3 - 1u);
          uVar9 = uVar10 + 1;
          *local_18 = (F(puVar6, 3) - F(puVar7, 3)) / (F(puVar6, 2) - F(puVar7, 2));
          FUN_00486980(F(puVar7, 2), F(puVar6, 2), uVar10, param_1);
          if (iVar3 < 1) break;
          F(puVar7, 2) = F(puVar7, 2) + F(puVar7, 4);
          F(puVar7, 3) = F(puVar7, 5) + F(puVar7, 3);
          F(puVar6, 2) = F(puVar6, 2) + F(puVar6, 4);
          F(puVar6, 3) = F(puVar6, 5) + F(puVar6, 3);
          uVar10 = uVar9;
        }
      }
      else {
        *local_18 = (F(puVar6, 3) - F(puVar7, 3)) / (F(puVar6, 2) - F(puVar7, 2));
        while (1) {
          iVar3 = (int)((uint)iVar3 - 1u);
          uVar9 = uVar10 + 1;
          FUN_00486980(F(puVar7, 2), F(puVar6, 2), uVar10, param_1);
          if (iVar3 < 1) break;
          F(puVar7, 2) = F(puVar7, 2) + F(puVar7, 4);
          F(puVar7, 3) = F(puVar7, 5) + F(puVar7, 3);
          F(puVar6, 2) = F(puVar6, 2) + F(puVar6, 4);
          F(puVar6, 3) = F(puVar6, 5) + F(puVar6, 3);
          uVar10 = uVar9;
        }
      }
    }
    else {
      uVar10 = uVar9;
      if ((int)*puVar7 < (int)*puVar6) {
        iVar3 = (int)(*puVar7 - uVar9);
      }
      else {
        iVar3 = (int)(*puVar6 - uVar9);
      }
      while (1) {
        local_18 = (float *)(param_1 + 0x34);
        iVar3 = (int)((uint)iVar3 - 1u);
        uVar9 = uVar10 + 1;
        *local_18 = (F(puVar6, 3) - F(puVar7, 3)) / (F(puVar6, 2) - F(puVar7, 2));
        FUN_00486980(F(puVar7, 2), F(puVar6, 2), uVar10, param_1);
        if (iVar3 < 1) break;
        F(puVar7, 2) = F(puVar7, 2) + F(puVar7, 4);
        F(puVar7, 3) = F(puVar7, 5) + F(puVar7, 3);
        F(puVar6, 2) = F(puVar6, 2) + F(puVar6, 4);
        F(puVar6, 3) = F(puVar6, 5) + F(puVar6, 3);
        uVar10 = uVar9;
      }
    }
  } while (1);
}
