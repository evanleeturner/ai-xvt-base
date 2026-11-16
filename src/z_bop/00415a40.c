/* SPDX-License-Identifier: MIT */
/* z_bop 00415a40 */
extern unsigned char DAT_009a210a[];

uint __cdecl FUN_00415a40(uint param_1)
{
  uint v1 = param_1 & 0xffff;
  return ((((v1 * 0x49) >> 8) << 8) | DAT_009a210a[v1 * 0x126]) & 0xffff00ff;
}
