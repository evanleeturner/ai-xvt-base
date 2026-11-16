/* SPDX-License-Identifier: MIT */
/* z_bop 0040b7b0 */
extern int *DAT_00803f80;
extern int *DAT_0066ddcc;
extern uint DAT_005233d8;
extern uint DAT_005233e0;
extern uint DAT_005233e8;
extern ushort DAT_0052747c;
extern uint DAT_0066ddc4;
extern uint DAT_0066dde0;
extern ushort DAT_009a1fe4;
extern uint DAT_009a7b60;
extern uint DAT_009a8074;
extern uint DAT_009a8e3c;
extern ushort DAT_009d682c;

void __stdcall FUN_0040b7b0(void)
{
  byte bVar1;
  int iVar2;
  uint uVar3;
  int iVar4;
  uint uVar5;
  uint *puVar6;
  uint *puVar7;
  uint uVar8;
  byte *pbVar9;
  byte *pbVar10;
  uint *puVar11;
  uint *puVar12;
  byte bStack_89;
  uint uStack_88;
  uint uStack_84;
  int iStack_7c;
  uint auStack_6c[27];
  /* auStack_6c[4] = pitch (iStack_5c), auStack_6c[9] = surface ptr (puStack_48) */

  puVar7 = auStack_6c;
  for (iVar4 = 0x1b; iVar4 != 0; iVar4 = iVar4 + -1) {
    *puVar7 = 0;
    puVar7 = puVar7 + 1;
  }
  auStack_6c[0] = 0x6c;
  while (iVar2 = (*(int (__stdcall **)(int *, int, void *, int, int))(*DAT_00803f80 + 100))(DAT_00803f80, 0, auStack_6c, 0, 0),
         puVar7 = (uint *)auStack_6c[9], iVar4 = (int)auStack_6c[4], iVar2 != 0) {
    if (iVar2 != -0x7789fde4) {
      return;
    }
  }
  puVar6 = auStack_6c;
  for (iVar2 = 0x1b; iVar2 != 0; iVar2 = iVar2 + -1) {
    *puVar6 = 0;
    puVar6 = puVar6 + 1;
  }
  auStack_6c[0] = 0x6c;
  while (iVar2 = (*(int (__stdcall **)(int *, int, void *, int, int))(*DAT_0066ddcc + 100))(DAT_0066ddcc, 0, auStack_6c, 0, 0), iVar2 != 0) {
    if (iVar2 != -0x7789fde4) {
      return;
    }
  }
  uVar8 = 0;
  puVar7 = (uint *)((int)puVar7 +
                    (int)(((DAT_0066dde0 - DAT_005233e8) >> 1) * (uint)iVar4 +
                          ((DAT_0066ddc4 - DAT_005233e0) >> 1) * DAT_005233d8));
  puVar6 = (uint *)auStack_6c[9];
  if (DAT_009a7b60 != 0) {
    do {
      if ((DAT_005233e0 & 1) == 0) {
        uVar3 = DAT_005233e0 * DAT_005233d8;
        puVar11 = puVar6;
        puVar12 = puVar7;
LAB_0040b8d4:
        for (uVar5 = uVar3 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
          *puVar12 = *puVar11;
          puVar11 = puVar11 + 1;
          puVar12 = puVar12 + 1;
        }
        for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
          *(byte *)puVar12 = *(byte *)puVar11;
          puVar11 = (uint *)((int)puVar11 + 1);
          puVar12 = (uint *)((int)puVar12 + 1);
        }
      }
      else {
        *(ushort *)puVar7 = *(ushort *)puVar6;
        if (DAT_005233e0 != 0) {
          uVar3 = (DAT_005233e0 - 1) * DAT_005233d8;
          puVar11 = (uint *)((int)puVar6 + 2);
          puVar12 = (uint *)((int)puVar7 + 2);
          goto LAB_0040b8d4;
        }
      }
      puVar6 = (uint *)((int)puVar6 + (int)auStack_6c[4]);
      puVar7 = (uint *)((int)puVar7 + iVar4);
      uVar8 = uVar8 + 1;
    } while (uVar8 < DAT_009a7b60);
  }
  iStack_7c = 0;
  pbVar9 = (byte *)((uint)DAT_0052747c + DAT_009a8074);
  if (DAT_009a1fe4 != 0) {
    do {
      if (DAT_009a8e3c != 0) {
        if ((DAT_009a8e3c & 1) == 0) {
          uVar8 = DAT_009a8e3c * DAT_005233d8;
          puVar11 = puVar6;
          puVar12 = puVar7;
LAB_0040b95e:
          for (uVar3 = uVar8 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
            *puVar12 = *puVar11;
            puVar11 = puVar11 + 1;
            puVar12 = puVar12 + 1;
          }
          for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
            *(byte *)puVar12 = *(byte *)puVar11;
            puVar11 = (uint *)((int)puVar11 + 1);
            puVar12 = (uint *)((int)puVar12 + 1);
          }
        }
        else {
          *(ushort *)puVar7 = *(ushort *)puVar6;
          if (DAT_009a8e3c != 0) {
            uVar8 = (DAT_009a8e3c - 1) * DAT_005233d8;
            puVar11 = (uint *)((int)puVar6 + 2);
            puVar12 = (uint *)((int)puVar7 + 2);
            goto LAB_0040b95e;
          }
        }
        puVar6 = (uint *)((int)puVar6 + (int)(DAT_009a8e3c * DAT_005233d8));
        puVar7 = (uint *)((int)puVar7 + (int)(DAT_009a8e3c * DAT_005233d8));
      }
      bStack_89 = *pbVar9;
      pbVar9 = pbVar9 + 1;
      uStack_84 = 0;
      pbVar10 = pbVar9;
      if (DAT_009d682c != 0) {
        do {
          uVar8 = (uint)*pbVar10;
          pbVar9 = pbVar10 + 1;
          if (uVar8 == 0) {
            uVar8 = (uint)*pbVar9;
            pbVar9 = pbVar10 + 2;
            if (uVar8 == 0) {
              bVar1 = *pbVar9;
              pbVar9 = pbVar10 + 3;
              uVar8 = (uint)bVar1 + 0x100;
            }
            uVar8 = uVar8 + 0xff;
          }
          if ((char)bStack_89 < '\0') {
            if ((uVar8 & 1) == 0) {
              uStack_88 = uVar8 * DAT_005233d8;
              puVar11 = puVar6;
              puVar12 = puVar7;
              for (uVar3 = uStack_88 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
                *puVar12 = *puVar11;
                puVar11 = puVar11 + 1;
                puVar12 = puVar12 + 1;
              }
            }
            else {
              *(ushort *)puVar7 = *(ushort *)puVar6;
              if (uVar8 == 0) goto LAB_0040ba1a;
              uStack_88 = (uVar8 - 1) * DAT_005233d8;
              puVar11 = (uint *)((int)puVar6 + 2);
              puVar12 = (uint *)((int)puVar7 + 2);
              for (uVar3 = uStack_88 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
                *puVar12 = *puVar11;
                puVar11 = puVar11 + 1;
                puVar12 = puVar12 + 1;
              }
            }
            for (uStack_88 = uStack_88 & 3; uStack_88 != 0; uStack_88 = uStack_88 - 1) {
              *(byte *)puVar12 = *(byte *)puVar11;
              puVar11 = (uint *)((int)puVar11 + 1);
              puVar12 = (uint *)((int)puVar12 + 1);
            }
          }
LAB_0040ba1a:
          puVar6 = (uint *)((int)puVar6 + (int)(uVar8 * DAT_005233d8));
          puVar7 = (uint *)((int)puVar7 + (int)(uVar8 * DAT_005233d8));
          uStack_84 = uStack_84 + uVar8;
          bStack_89 = (byte)(-bStack_89);
          pbVar10 = pbVar9;
        } while ((int)uStack_84 < (int)(uint)DAT_009d682c);
      }
      uStack_84 = uStack_84 + DAT_009a8e3c;
      if (uStack_84 < DAT_005233e0) {
        uStack_84 = DAT_005233e0 - uStack_84;
        if ((uStack_84 & 1) == 0) {
          uStack_88 = uStack_84 * DAT_005233d8;
          puVar11 = puVar6;
          puVar12 = puVar7;
          for (uVar8 = uStack_88 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
            *puVar12 = *puVar11;
            puVar11 = puVar11 + 1;
            puVar12 = puVar12 + 1;
          }
LAB_0040bab7:
          for (uStack_88 = uStack_88 & 3; uStack_88 != 0; uStack_88 = uStack_88 - 1) {
            *(byte *)puVar12 = *(byte *)puVar11;
            puVar11 = (uint *)((int)puVar11 + 1);
            puVar12 = (uint *)((int)puVar12 + 1);
          }
        }
        else {
          *(ushort *)puVar7 = *(ushort *)puVar6;
          if (uStack_84 != 0) {
            uStack_88 = (uStack_84 - 1) * DAT_005233d8;
            puVar11 = (uint *)((int)puVar6 + 2);
            puVar12 = (uint *)((int)puVar7 + 2);
            for (uVar8 = uStack_88 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
              *puVar12 = *puVar11;
              puVar11 = puVar11 + 1;
              puVar12 = puVar12 + 1;
            }
            goto LAB_0040bab7;
          }
        }
        puVar6 = (uint *)((int)puVar6 + (int)(uStack_84 * DAT_005233d8));
        puVar7 = (uint *)((int)puVar7 + (int)(uStack_84 * DAT_005233d8));
        uStack_84 = DAT_005233e0;
      }
      puVar6 = (uint *)((int)puVar6 + (int)((uint)auStack_6c[4] - uStack_84 * DAT_005233d8));
      puVar7 = (uint *)((int)puVar7 + (int)((uint)iVar4 - uStack_84 * DAT_005233d8));
      iStack_7c = iStack_7c + 1;
    } while (iStack_7c < (int)(uint)DAT_009a1fe4);
  }
  uVar8 = DAT_009a1fe4 + DAT_009a7b60;
  if (uVar8 < DAT_005233e8) {
    do {
      if ((DAT_005233e0 & 1) == 0) {
        uVar3 = DAT_005233e0 * DAT_005233d8;
        puVar11 = puVar6;
        puVar12 = puVar7;
LAB_0040bb59:
        for (uVar5 = uVar3 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
          *puVar12 = *puVar11;
          puVar11 = puVar11 + 1;
          puVar12 = puVar12 + 1;
        }
        for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
          *(byte *)puVar12 = *(byte *)puVar11;
          puVar11 = (uint *)((int)puVar11 + 1);
          puVar12 = (uint *)((int)puVar12 + 1);
        }
      }
      else {
        *(ushort *)puVar7 = *(ushort *)puVar6;
        if (DAT_005233e0 != 0) {
          uVar3 = (DAT_005233e0 - 1) * DAT_005233d8;
          puVar11 = (uint *)((int)puVar6 + 2);
          puVar12 = (uint *)((int)puVar7 + 2);
          goto LAB_0040bb59;
        }
      }
      puVar6 = (uint *)((int)puVar6 + (int)auStack_6c[4]);
      puVar7 = (uint *)((int)puVar7 + iVar4);
      uVar8 = uVar8 + 1;
    } while (uVar8 < DAT_005233e8);
  }
  (*(int (__stdcall **)(int *, uint))(*DAT_0066ddcc + 0x80))(DAT_0066ddcc, auStack_6c[9]);
  (*(int (__stdcall **)(int *, uint))(*DAT_00803f80 + 0x80))(DAT_00803f80, auStack_6c[4]);
  return;
}
