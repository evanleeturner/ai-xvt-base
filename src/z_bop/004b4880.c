/* SPDX-License-Identifier: MIT */
/* z_bop 004b4880 */
byte __cdecl FUN_004b4880(uint param_1)
{
  byte bVar1;
  
  bVar1 = (param_1 & 1) != 0;
  if ((param_1 & 4) != 0) {
    bVar1 = bVar1 | 4;
  }
  if ((param_1 & 2) != 0) {
    bVar1 = bVar1 | 2;
  }
  if ((param_1 & 8) != 0) {
    bVar1 = bVar1 | 8;
  }
  if ((param_1 & 0x10) != 0) {
    bVar1 = bVar1 | 0x10;
  }
  if ((param_1 & 0x20) != 0) {
    bVar1 = bVar1 | 0x20;
  }
  if ((param_1 & 0x40) != 0) {
    bVar1 = bVar1 | 0x40;
  }
  if ((param_1 & 0x80) != 0) {
    bVar1 = bVar1 | 0x80;
  }
  return bVar1;
}
