/* SPDX-License-Identifier: MIT */
/* z_xvt 004d3030 */
extern int DAT_0064f440;
extern uint __cdecl FUN_004df760(byte, byte, byte);

uint __cdecl FUN_004d3030(byte *param_1, int param_2)
{
  short sVar1;
  uint uVar2;

  sVar1 = *(short *)((int)&DAT_0064f440 + param_2 * 2);
  uVar2 = (uint)sVar1;
  if (0xff < sVar1) {
    if (param_2 == 0) {
      return uVar2 & 0xffffff00;
    }
    uVar2 = FUN_004df760(param_1[2], param_1[1], *param_1);
    *(short *)((int)&DAT_0064f440 + param_2 * 2) = (short)uVar2;
  }
  return uVar2;
}
