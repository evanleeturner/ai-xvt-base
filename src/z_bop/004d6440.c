/* SPDX-License-Identifier: MIT */
/* z_bop 004d6440 */
extern undefined4 DAT_00aa73a6;

undefined4 __cdecl FUN_004d6440(int param_1)
{
  if (2 < param_1) {
    return 0;
  }
  return (&DAT_00aa73a6)[param_1];
}
