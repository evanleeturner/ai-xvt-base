/* SPDX-License-Identifier: MIT */
/* z_bop 004b47e0 */
byte __cdecl FUN_004b47e0(uint param_1)
{
  byte bVar1;
  
  bVar1 = (param_1 & 0x4000) != 0;
  if ((param_1 & 0x2000) != 0) {
    bVar1 = bVar1 | 2;
  }
  if ((param_1 & 0x1000) != 0) {
    bVar1 = bVar1 | 4;
  }
  if ((param_1 & 0x800) != 0) {
    bVar1 = bVar1 | 8;
  }
  if ((param_1 & 0x400) != 0) {
    bVar1 = bVar1 | 0x10;
  }
  if ((param_1 & 0x200) != 0) {
    bVar1 = bVar1 | 0x20;
  }
  if ((param_1 & 0x100) != 0) {
    bVar1 = bVar1 | 0x40;
  }
  return bVar1;
}
