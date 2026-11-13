/* SPDX-License-Identifier: MIT */
/* z_xvt 004d8ee0 */
extern uint DAT_00532adc;
extern uint DAT_0064f684;
extern uint DAT_0064f688;
extern uint DAT_0064f68c;
extern void __stdcall _memcpy(void *dst, const void *src, uint len);

uint __stdcall FUN_004d8ee0(void)
{
  uint uVar1;
  
  uVar1 = DAT_0064f688;
  if (DAT_00532adc == 0) {
    return 0;
  }
  _memcpy(&DAT_0064f688, &DAT_0064f68c, (DAT_00532adc * 4) - 4);
  (&DAT_0064f684)[DAT_00532adc] = uVar1;
  return 1;
}
