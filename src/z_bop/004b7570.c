/* SPDX-License-Identifier: MIT */
/* z_bop 004b7570 */
extern short DAT_00664c88;
extern uint FUN_004d6e30(byte, byte, byte);

uint __cdecl FUN_004b7570(byte *param_1, int param_2)
{
  short *psVar1;
  short sVar2;
  uint uVar3;
  
  psVar1 = (short *)((int)&DAT_00664c88 + param_2 * 2);
  sVar2 = *psVar1;
  uVar3 = (uint)(unsigned short)sVar2;
  if (0xff < sVar2) {
    if (param_2 == 0) {
      uVar3 = FUN_004d6e30(*param_1, param_1[1], param_1[2]);
      *psVar1 = (short)uVar3;
    } else {
      return uVar3 & 0xffffff00;
    }
  }
  return uVar3;
}
