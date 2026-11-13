/* SPDX-License-Identifier: MIT */
/* z_xvt 00446fe0 */
extern short DAT_00525ade;
extern short DAT_00525b0e;
extern uint FUN_00478320(uint, ushort);

short __cdecl FUN_00446fe0(int param_1)
{
  short sVar1;
  uint uVar2;
  
  sVar1 = *(short *)((char *)&DAT_00525ade + param_1 * 2);
  uVar2 = FUN_00478320((param_1 & 0xFFFF0000) | (ushort)*(short *)((char *)&DAT_00525b0e + param_1 * 2), 0xec);
  return (short)uVar2 + sVar1 * 0xec;
}
