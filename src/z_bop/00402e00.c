/* SPDX-License-Identifier: MIT */
/* z_bop 00402e00 */
int __cdecl FUN_00402e00(unsigned short param_1)
{
  if (param_1 < 0x8000) {
    return 0;
  }
  return 2 - (unsigned int)(param_1 < 0xc000);
}
