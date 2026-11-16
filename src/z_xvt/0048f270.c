/* SPDX-License-Identifier: MIT */
/* z_xvt 0048f270 */
extern undefined8 __cdecl FUN_004b4060(undefined4 param_1, undefined4 param_2, byte *param_3);
extern void __fastcall FUN_0048f140(void *param_1, byte *param_2);
extern short DAT_0070acc0;
extern short DAT_006d9980;
extern short DAT_00732e58;

void __fastcall FUN_0048f270(undefined4 param_1, undefined4 param_2, byte *param_3)
{
  ushort uVar1;
  undefined8 uVar2;
  
  uVar2 = FUN_004b4060(param_1, param_2, param_3);
  uVar1 = (short)(((int)DAT_0070acc0 + (int)(short)DAT_006d9980) / 2) - ((ushort)uVar2 >> 1);
  DAT_00732e58 = uVar1;
  if ((int)(uint)uVar1 < (int)(short)DAT_006d9980) {
    DAT_00732e58 = DAT_006d9980;
  }
  FUN_0048f140((void *)(uint)uVar1, param_3);
  return;
}
