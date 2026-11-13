/* SPDX-License-Identifier: MIT */
/* z_bop 004dc1e0 */
extern uchar DAT_00aa739e;
extern uchar DAT_00aa73a2;

uchar __stdcall FUN_004dc1e0(void)
{
  if (DAT_00aa73a2 != 0) {
    return 0;
  }
  return DAT_00aa739e;
}
