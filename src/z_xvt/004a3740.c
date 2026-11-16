/* SPDX-License-Identifier: MIT */
/* z_xvt 004a3740 */
extern undefined4 FUN_004ac6f0(uint param_1, undefined4 *param_2);
extern void FUN_004a1d70(undefined4 param_1, undefined4 param_2);

extern undefined4 DAT_006d4988;
extern uint _DAT_00732e74;
extern ushort DAT_00732fe2;
extern ushort _DAT_00732950;
extern ushort _DAT_00732952;
extern ushort _DAT_00732954;
extern ushort DAT_00732956;
extern undefined4 _DAT_00732e70;
extern undefined4 DAT_0071ecf4;
extern char DAT_0070ad47;
extern char DAT_0070acf0;
extern char DAT_0052b070;

void __cdecl FUN_004a3740(uint param_1, undefined4 *param_2, undefined2 param_3)
{
  undefined4 uVar1;
  uint uVar2;
  
  uVar2 = (uint)*(byte *)((param_1 & 0xffff) * 0x23 + 0x16 + DAT_006d4988);
  _DAT_00732e74 = uVar2 * 0x566;
  DAT_00732fe2 = (ushort)(byte)(&DAT_0070ad47)[_DAT_00732e74];
  _DAT_00732950 = 0x8000;
  _DAT_00732e74 = (uint)&DAT_0070acf0 + _DAT_00732e74;
  _DAT_00732952 = 0x8001;
  _DAT_00732e70 = (undefined4)(&DAT_0052b070 + (uint)*(byte *)(param_2 + 1) * 0xef);
  uVar1 = FUN_004ac6f0(uVar2, param_2);
  _DAT_00732954 = (short)uVar1;
  if (_DAT_00732954 != 0) {
    DAT_00732956 = param_3;
    FUN_004a1d70(0x9c, DAT_0071ecf4);
    return;
  }
  _DAT_00732954 = param_3;
  FUN_004a1d70(0x9d, DAT_0071ecf4);
  return;
}
