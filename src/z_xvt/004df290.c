/* SPDX-License-Identifier: MIT */
/* z_xvt 004df290 */
extern unsigned char DAT_00758ee8[];
extern unsigned char DAT_00758f2e[];

unsigned char __cdecl FUN_004df290(int param_1)
{
  if (param_1 > 1) {
    return 0;
  }
  if (DAT_00758ee8[param_1] == 0) {
    return DAT_00758f2e[param_1 * 0x20 + 1];
  }
  return 0;
}
