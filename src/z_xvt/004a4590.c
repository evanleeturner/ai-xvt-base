/* SPDX-License-Identifier: MIT */
/* z_xvt 004a4590 */
extern byte DAT_00645aee;
extern byte DAT_00645b46;
extern byte DAT_006d999e;

undefined8 __cdecl FUN_004b4060(undefined4 param_1, undefined4 param_2, byte *param_3);

undefined8 __fastcall FUN_004a4590(undefined4 param_1, undefined4 param_2, short param_3)
{
  byte bVar1;
  byte *pbVar2;
  ushort uVar3;
  ushort uVar4;
  undefined8 uVar5;
  byte local_50[80];

  if (((param_3 == 3) || (param_3 == 4)) || (param_3 == 7)) {
    pbVar2 = &DAT_00645aee;
  }
  else {
    pbVar2 = &DAT_00645b46;
    if (param_3 != 8) {
      pbVar2 = &DAT_006d999e;
    }
  }
  if (*pbVar2 < 9) {
    pbVar2 = pbVar2 + 1;
  }
  uVar3 = 0;
  bVar1 = *pbVar2;
  for (uVar4 = 0; (bVar1 != 0 && (uVar4 < 0x46)); uVar4 = uVar4 + 1) {
    bVar1 = *pbVar2;
    if ((bVar1 == 0x5b) || (bVar1 == 0x5d)) {
      if (bVar1 == 0xfe) {
        pbVar2 = pbVar2 + 2;
      }
    }
    else {
      local_50[uVar3] = bVar1;
      uVar3 = uVar3 + 1;
    }
    bVar1 = pbVar2[1];
    param_2 = (param_2 >> 8) | bVar1;
    pbVar2 = pbVar2 + 1;
  }
  local_50[uVar3] = 0;
  uVar5 = FUN_004b4060((undefined4)uVar3, param_2, local_50);
  return uVar5;
}
