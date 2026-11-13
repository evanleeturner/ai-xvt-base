/* SPDX-License-Identifier: MIT */
/* z_bop 004dc2c0 */
extern undefined4 DAT_00aa73a2;
extern undefined1 DAT_00aa73a1;

undefined1 __cdecl FUN_004dc2c0(int param_1)
{
  if ((DAT_00aa73a2 != (undefined4)param_1) && (DAT_00aa73a2 != 0)) {
    return 0;
  }
  return DAT_00aa73a1;
}
