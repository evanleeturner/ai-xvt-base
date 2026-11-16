/* SPDX-License-Identifier: MIT */
/* z_xvt 0051e3b0 */
void __cdecl FUN_0051e000(unsigned int *param_1, int param_2, int param_3, int param_4, int *param_5, int param_6);

void __cdecl
FUN_0051e3b0(unsigned int *param_1, int param_2, int param_3, int param_4, int *param_5, int param_6)
{
  unsigned char *pbVar1;
  unsigned char *pbVar2;
  unsigned char *pbVar3;
  unsigned char bVar4;
  unsigned char bVar5;
  unsigned char bVar6;
  int *piVar7;
  short *psVar8;
  int iVar9;
  int iVar10;
  int iVar11;
  unsigned short uVar12;
  short sVar13;
  unsigned short uVar14;
  int iVar15;
  int iVar16;
  unsigned short uVar17;
  int iVar18;
  unsigned short uVar19;
  unsigned short uVar20;
  short *psVar21;
  short *psVar22;
  short sVar23;
  int bVar24;
  unsigned char *local_24;
  short *local_20;
  unsigned char *local_1c;
  unsigned char *local_18;
  int local_4;

  if ((param_3 < 3) || (param_2 < 3)) {
    FUN_0051e000(param_1, param_2, param_3, param_4, param_5, param_6);
  }
  iVar15 = (int)((unsigned int)param_4 - (unsigned int)param_2);
  iVar16 = (int)((unsigned int)param_6 - (unsigned int)param_2);
  local_1c = (unsigned char *)((unsigned int)param_4 + (unsigned int)(int)param_1);
  local_18 = (unsigned char *)param_1;
  bVar4 = local_18[1];
  bVar6 = *local_18;
  bVar5 = bVar6;
  if ((unsigned short)*local_1c == (unsigned short)bVar4) {
    bVar5 = bVar4;
  }
  *(unsigned short *)param_5 = (unsigned short)((bVar6 << 8) | bVar6);
  *(short *)((unsigned int)param_6 + (unsigned int)(int)param_5) = (short)((bVar6 << 8) | bVar5);
  iVar18 = (int)((unsigned int)param_2 - 3u);
  local_24 = local_18 + 2;
  iVar11 = iVar18;
  psVar21 = (short *)((unsigned int)param_6 + (unsigned int)(int)param_5);
  iVar10 = (int)((unsigned int)param_2 - 2u);
  uVar14 = (unsigned short)bVar4;
  uVar19 = (unsigned short)bVar6;
  while (1) {
    uVar17 = uVar14;
    iVar9 = iVar11;
    psVar8 = psVar21 + 1;
    piVar7 = (int *)((unsigned int)(int)param_5 + 2u);
    local_1c = local_1c + 1;
    if (iVar10 < 1) break;
    uVar14 = (unsigned short)*local_24;
    if (*local_1c == uVar19) {
      if (uVar19 == uVar14) {
LAB_0051e4d2:
        sVar23 = (short)(uVar17 * 0x101);
      }
      else {
        sVar23 = (short)(uVar19 * 0x100 + uVar17);
      }
    }
    else {
      if (*local_1c != uVar14) goto LAB_0051e4d2;
      sVar23 = (short)(uVar17 * 0x100 + (unsigned short)*local_24);
    }
    *(unsigned short *)piVar7 = (unsigned short)(uVar17 * 0x101);
    *psVar8 = sVar23;
    local_24 = local_24 + 1;
    iVar11 = (int)((unsigned int)iVar9 - 1u);
    param_5 = piVar7;
    psVar21 = psVar8;
    iVar10 = iVar9;
    uVar19 = uVar17;
  }
  if (*local_1c == uVar19) {
    sVar23 = (short)(uVar19 * 0x100 + uVar17);
  }
  else {
    sVar23 = (short)(uVar17 * 0x101);
  }
  *(unsigned short *)piVar7 = (unsigned short)(uVar17 * 0x101);
  *psVar8 = sVar23;
  local_24 = local_24 + iVar15;
  psVar21 = psVar21 + (int)((unsigned int)iVar16 + 2u);
  local_20 = (short *)((unsigned int)(int)param_5 + (unsigned int)iVar16 * 2u + 4u);
  bVar24 = param_3 != 2;
  param_3 = (int)((unsigned int)param_3 - 3u);
  if (bVar24) {
    do {
      local_1c = local_1c + (int)((unsigned int)iVar15 + 1u);
      bVar4 = *local_24;
      uVar14 = (unsigned short)bVar4;
      pbVar1 = local_18 + 1;
      pbVar2 = local_24 + 2;
      bVar6 = local_24[1];
      uVar19 = (unsigned short)bVar6;
      pbVar3 = local_1c + 1;
      if (uVar19 == *local_18) {
        if ((unsigned short)*local_1c == (unsigned short)*local_18) {
LAB_0051e5ba:
          sVar23 = (short)((unsigned short)bVar4 * 0x101);
          sVar13 = sVar23;
        }
        else {
          sVar23 = (short)((bVar4 << 8) | bVar6);
          sVar13 = (short)((bVar4 << 8) | bVar4);
        }
      }
      else {
        if (*local_1c != uVar19) goto LAB_0051e5ba;
        sVar23 = (short)((bVar4 << 8) | bVar4);
        sVar13 = (short)((bVar4 << 8) | bVar6);
      }
      psVar22 = psVar21 + 1;
      psVar8 = local_20 + 1;
      *local_20 = sVar23;
      *psVar21 = sVar13;
      psVar21 = psVar22;
      local_24 = pbVar2;
      local_20 = psVar8;
      local_1c = pbVar3;
      local_18 = pbVar1;
      local_4 = iVar18;
      uVar17 = uVar14;
      if (param_2 != 2) {
        do {
          uVar14 = uVar19;
          uVar12 = (unsigned short)*local_18;
          local_18 = local_18 + 1;
          pbVar1 = local_24 + 1;
          bVar4 = *local_24;
          uVar19 = (unsigned short)bVar4;
          pbVar2 = local_1c + 1;
          uVar20 = (unsigned short)*local_1c;
          if (uVar19 == uVar12) {
            if ((uVar17 == uVar12) || (uVar20 == uVar12)) goto LAB_0051e697;
            if (uVar20 == uVar17) {
              sVar23 = (short)(uVar14 * 0x100 + (unsigned short)bVar4);
              sVar13 = (short)(uVar17 * 0x100 + uVar14);
            }
            else {
              sVar23 = (short)(uVar14 * 0x100 + (unsigned short)bVar4);
              sVar13 = (short)(uVar14 * 0x101);
            }
          }
          else if (uVar17 == uVar12) {
            if (uVar20 == uVar12) goto LAB_0051e697;
            if (uVar20 == uVar19) {
              sVar23 = (short)(uVar17 * 0x100 + uVar14);
              sVar13 = (short)(uVar14 * 0x100 + (unsigned short)bVar4);
            }
            else {
              sVar23 = (short)(uVar17 * 0x100 + uVar14);
              sVar13 = (short)(uVar14 * 0x101);
            }
          }
          else if (uVar20 == uVar17) {
            if (uVar17 == uVar19) {
LAB_0051e697:
              sVar23 = (short)(uVar14 * 0x101);
              sVar13 = sVar23;
            }
            else {
              sVar23 = (short)(uVar14 * 0x101);
              sVar13 = (short)(uVar17 * 0x100 + uVar14);
            }
          }
          else {
            if (uVar20 != uVar19) goto LAB_0051e697;
            sVar23 = (short)(uVar14 * 0x101);
            sVar13 = (short)(uVar14 * 0x100 + (unsigned short)bVar4);
          }
          psVar22 = psVar21 + 1;
          *local_20 = sVar23;
          local_20 = local_20 + 1;
          *psVar21 = sVar13;
          bVar24 = local_4 != 0;
          psVar21 = psVar22;
          local_24 = pbVar1;
          local_1c = pbVar2;
          local_4 = (int)((unsigned int)local_4 - 1u);
          uVar17 = uVar14;
        } while (bVar24);
      }
      if (uVar14 == *local_18) {
        if ((unsigned short)*local_1c == (unsigned short)*local_18) {
LAB_0051e704:
          sVar23 = (short)(uVar19 * 0x101);
          sVar13 = sVar23;
        }
        else {
          sVar23 = (short)(uVar14 * 0x100 + uVar19);
          sVar13 = (short)(uVar19 * 0x101);
        }
      }
      else {
        if (*local_1c != uVar14) goto LAB_0051e704;
        sVar23 = (short)(uVar19 * 0x101);
        sVar13 = (short)(uVar14 * 0x100 + uVar19);
      }
      *local_20 = sVar23;
      *psVar22 = sVar13;
      local_20 = local_20 + (int)((unsigned int)iVar16 + 1u);
      psVar21 = psVar22 + (int)((unsigned int)iVar16 + 1u);
      local_18 = local_18 + (int)((unsigned int)iVar15 + 1u);
      local_24 = local_24 + iVar15;
      bVar24 = param_3 != 0;
      param_3 = (int)((unsigned int)param_3 - 1u);
    } while (bVar24);
  }
  bVar4 = *local_24;
  uVar14 = (unsigned short)bVar4;
  pbVar1 = local_18 + 1;
  uVar19 = (unsigned short)local_24[1];
  bVar6 = bVar4;
  if (uVar19 == *local_18) {
    bVar6 = local_24[1];
  }
  psVar22 = psVar21 + 1;
  psVar8 = local_20 + 1;
  *local_20 = (short)((bVar4 << 8) | bVar6);
  *psVar21 = (short)((bVar4 << 8) | bVar4);
  psVar21 = psVar22;
  local_24 = local_24 + 2;
  local_20 = psVar8;
  local_18 = pbVar1;
  uVar17 = uVar14;
  if (param_2 != 2) {
    do {
      uVar14 = uVar19;
      pbVar1 = local_18 + 1;
      uVar12 = (unsigned short)*local_18;
      uVar19 = (unsigned short)*local_24;
      if (uVar19 == uVar12) {
        if (uVar17 == uVar12) {
LAB_0051e7f1:
          sVar23 = (short)(uVar14 * 0x101);
        }
        else {
          sVar23 = (short)(uVar14 * 0x100 + (unsigned short)*local_24);
        }
      }
      else {
        if (uVar17 != uVar12) goto LAB_0051e7f1;
        sVar23 = (short)(uVar17 * 0x100 + uVar14);
      }
      psVar22 = psVar21 + 1;
      *local_20 = sVar23;
      local_20 = local_20 + 1;
      *psVar21 = (short)(uVar14 * 0x101);
      bVar24 = iVar18 != 0;
      iVar18 = (int)((unsigned int)iVar18 - 1u);
      psVar21 = psVar22;
      local_24 = local_24 + 1;
      local_18 = pbVar1;
      uVar17 = uVar14;
    } while (bVar24);
  }
  if (*local_18 == uVar14) {
    sVar23 = (short)(uVar14 * 0x100 + uVar19);
  }
  else {
    sVar23 = (short)(uVar19 * 0x101);
  }
  *local_20 = sVar23;
  *psVar22 = (short)(uVar19 * 0x101);
  return;
}
