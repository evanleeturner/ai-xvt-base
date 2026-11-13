/* SPDX-License-Identifier: MIT */
/* z_xvt 004d3320 */
int __cdecl FUN_004d3320(int *param_1, byte *param_2, int param_3)
{
  int iVar1 = 0;
  int iVar9 = 0;
  byte bVar6 = *param_2;
  byte *local_c = (byte *)(param_1 + 1);
  byte *pbVar3 = local_c;
  byte *pbVar2;
  int iVar5;
  byte bVar4;
  int iVar7;
  int iVar8;

  *(byte *)(param_1 + 1) = 0;

  if (param_3 < 1) {
    *local_c = 0x80;
    *param_1 = 5;
    return 5;
  }

  while (1) {
    iVar5 = 0;
    do {
      if (param_3 <= iVar9 || param_2[iVar5] != bVar6) break;
      iVar9++;
      iVar5++;
    } while (iVar5 < 0x3f);
    bVar4 = (byte)iVar5;

    if (bVar6 == 0) {
      iVar1++;
      *pbVar3 = bVar4 | 0x40;
      pbVar2 = pbVar3 + 1;
      local_c = pbVar3;
    }
    else if (iVar5 < 3) {
      if ((*local_c & 0x80) == 0) {
        iVar1++;
        *pbVar3 = bVar4 | 0x80;
        pbVar2 = pbVar3 + 1;
        local_c = pbVar3;
        if (iVar5 > 0) {
          iVar8 = 0;
          do {
            iVar7 = iVar8 + 1;
            pbVar2[iVar8] = param_2[iVar8];
            iVar8 = iVar7;
          } while (iVar7 < iVar5);
        }
      }
      else {
        bVar6 = (*local_c & 0x7f) + bVar4;
        if (bVar6 < 0x80) {
          *local_c = bVar6 | 0x80;
          pbVar2 = pbVar3;
          if (iVar5 > 0) {
            iVar8 = 0;
            do {
              iVar7 = iVar8 + 1;
              pbVar3[iVar8] = param_2[iVar8];
              iVar8 = iVar7;
            } while (iVar7 < iVar5);
          }
        }
        else {
          iVar1++;
          *pbVar3 = bVar4 | 0x80;
          pbVar2 = pbVar3 + 1;
          local_c = pbVar3;
          if (iVar5 > 0) {
            iVar8 = 0;
            do {
              iVar7 = iVar8 + 1;
              pbVar2[iVar8] = param_2[iVar8];
              iVar8 = iVar7;
            } while (iVar7 < iVar5);
          }
        }
      }
      pbVar2 += iVar5;
      iVar1 += iVar5;
    }
    else {
      *pbVar3 = bVar4;
      iVar1 += 2;
      pbVar3[1] = bVar6;
      pbVar2 = pbVar3 + 2;
      local_c = pbVar3;
    }

    if (param_3 <= iVar9) {
      *pbVar2 = 0x80;
      *param_1 = iVar1 + 5;
      return iVar1 + 5;
    }

    param_2 += iVar5;
    bVar6 = *param_2;
    pbVar3 = pbVar2;
  }
}
