/* SPDX-License-Identifier: MIT */
/* z_xvt 004dbe80 */
extern uint DAT_007595d0;
extern uint DAT_007595d4;
extern unsigned char DAT_007591d0[];

uint __stdcall FUN_004dbe80(void)
{
  if (DAT_007595d0 != DAT_007595d4) {
    return DAT_007595d4 & 0xffffff00;
  }
  return (DAT_007595d4 & 0xffffff00) | (unsigned char)DAT_007591d0[DAT_007595d4];
}
