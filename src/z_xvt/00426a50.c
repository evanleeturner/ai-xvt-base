/* SPDX-License-Identifier: MIT */
/* z_xvt 00426a50 */
extern uint DAT_00525518;
extern uint DAT_0070acc8;
extern uint DAT_00525534;
extern uint DAT_007325e0;

void __cdecl FUN_00426a50(int param_1, ushort param_2, uint param_3, uint param_4, uint param_5, uint param_6, ushort param_7, ushort param_8, uint param_9)
{
  uint uVar1;
  uint uVar2;
  int iVar3;
  ushort uVar4;
  uint uVar5;
  ushort *puVar6;
  ushort *puVar7;
  uint uVar8;
  ushort *puVar9;
  ushort *puVar10;

  if (DAT_00525518 == 1) {
    uVar4 = (param_2 << 8) | param_2;
  }
  else {
    uVar4 = *(ushort *)(&DAT_007325e0 + (uint)param_2 * 2);
  }
  puVar7 = (ushort *)(DAT_00525534 + (param_6 & 0xffff) * DAT_0070acc8 + (param_5 & 0xffff) * DAT_00525518);
  uVar1 = param_9 & 0xffff;
  puVar6 = (ushort *)(param_1 + (param_3 & 0xffff) * DAT_00525518 + (param_4 & 0xffff) * uVar1);
  if (param_2 == 0xffff) {
    if (param_8 != 0) {
      uVar8 = (uint)param_8;
      do {
        uVar2 = DAT_00525518 * (uint)param_7;
        puVar9 = puVar6;
        puVar10 = puVar7;
        for (uVar5 = uVar2 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
          *(uint *)puVar10 = *(uint *)puVar9;
          puVar9 = puVar9 + 2;
          puVar10 = puVar10 + 2;
        }
        for (uVar2 = uVar2 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
          *(char *)puVar10 = (char)*puVar9;
          puVar9 = (ushort *)((int)puVar9 + 1);
          puVar10 = (ushort *)((int)puVar10 + 1);
        }
        puVar7 = (ushort *)((int)puVar7 + DAT_0070acc8);
        puVar6 = (ushort *)((int)puVar6 + uVar1);
        uVar8 = uVar8 - 1;
      } while (uVar8 != 0);
      return;
    }
  }
  else if (param_8 != 0) {
    uVar8 = (uint)param_8;
    do {
      uVar2 = (uint)param_7;
      if (param_7 != 0) {
        do {
          if (DAT_00525518 == 1) {
            if ((char)uVar4 != (char)*puVar6) {
              *(char *)puVar7 = (char)*puVar6;
            }
          }
          else if ((DAT_00525518 == 2) && (uVar4 != *puVar6)) {
            *puVar7 = *puVar6;
          }
          puVar6 = (ushort *)((int)puVar6 + DAT_00525518);
          puVar7 = (ushort *)((int)puVar7 + DAT_00525518);
          uVar2 = uVar2 - 1;
        } while (uVar2 != 0);
      }
      iVar3 = DAT_00525518 * (uint)param_7;
      puVar7 = (ushort *)((int)puVar7 + (DAT_0070acc8 - iVar3));
      puVar6 = (ushort *)((int)puVar6 + (uVar1 - iVar3));
      uVar8 = uVar8 - 1;
    } while (uVar8 != 0);
  }
  return;
}
