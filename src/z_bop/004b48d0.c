/* SPDX-License-Identifier: MIT */
/* z_bop 004b48d0 */
unsigned char __cdecl FUN_004b48d0(unsigned int param_1)
{
  unsigned char bVar1;
  
  bVar1 = (param_1 & 1) != 0;
  if ((param_1 & 4) != 0) {
    bVar1 = 3;
  }
  if ((param_1 & 2) != 0) {
    bVar1 = bVar1 | 2;
  }
  if ((param_1 & 8) != 0) {
    bVar1 = bVar1 | 4;
  }
  if ((param_1 & 0x10) != 0) {
    bVar1 = bVar1 | 5;
  }
  if ((param_1 & 0x20) != 0) {
    bVar1 = bVar1 | 6;
  }
  if ((param_1 & 0x40) != 0) {
    bVar1 = 7;
  }
  if ((param_1 & 0x80) != 0) {
    bVar1 = bVar1 | 8;
  }
  return bVar1;
}
