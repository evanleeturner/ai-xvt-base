/* SPDX-License-Identifier: MIT */
/* z_bop 004a9a50 */
extern undefined8 __cdecl FUN_004278c0(undefined4 param_1, undefined4 param_2, byte *param_3);
extern void __fastcall FUN_004a9500(short param_1, short param_2);
extern void __fastcall FUN_004a9960(byte *param_1);
extern short DAT_009d8c00;
extern short DAT_009a6fe0;
extern short DAT_00a08102;

void __fastcall FUN_004a9a50(undefined4 param_1, undefined4 param_2, byte *param_3)
{
  ushort uVar1;
  undefined8 uVar2;
  
  uVar2 = FUN_004278c0(param_1, param_2, param_3);
  uVar1 = (short)(((int)DAT_009d8c00 + (int)(short)DAT_009a6fe0) / 2) - ((ushort)uVar2 >> 1);
  if ((int)(uint)uVar1 < (int)(short)DAT_009a6fe0) {
    uVar1 = DAT_009a6fe0;
  }
  FUN_004a9500(uVar1, DAT_00a08102);
  FUN_004a9960(param_3);
  return;
}
