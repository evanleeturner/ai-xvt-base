/* SPDX-License-Identifier: MIT */
/* z_bop 00425d80 */
unsigned int __cdecl FUN_00425d80(short param_1, unsigned int param_2)
{
  unsigned int uVar1;
  unsigned int uVar2;
  unsigned int uVar3;
  
  uVar2 = (unsigned int)((int)param_1 * 0x1234 + 0x80);
  uVar3 = uVar2 >> 8;
  uVar1 = uVar3 / (param_2 & 0xffff);
  if (uVar2 >> 9 < (uVar3 & param_2 & 0xffff)) {
    uVar1 = uVar1 + 1;
  }
  return uVar1;
}
