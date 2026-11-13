/* SPDX-License-Identifier: MIT */
/* z_bop 004dc2a0 */
extern int DAT_00aa73a2;
extern unsigned char DAT_00aa73a0;

unsigned char __cdecl FUN_004dc2a0(int param_1)
{
  if ((param_1 != DAT_00aa73a2) && (DAT_00aa73a2 != 0)) {
    return 0;
  }
  return DAT_00aa73a0;
}
