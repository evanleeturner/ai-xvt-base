/* SPDX-License-Identifier: MIT */
/* z_bop 004d3140 */
extern unsigned int DAT_00aa7fd3;
extern unsigned int DAT_00aa7fd7;
extern unsigned int DAT_00aa7fdf;
extern unsigned int DAT_00aa7ff7;

unsigned int __stdcall FUN_004d3140(void)
{
  unsigned char local_4[4];
  
  if (DAT_00aa7fd3 == 0) {
    return 0;
  }
  if (DAT_00aa7fd7 == 0) {
    return 0;
  }
  mciSendCommandA(DAT_00aa7fd3, 0x808, 0, (unsigned int)local_4);
  DAT_00aa7fd7 = 0;
  DAT_00aa7fdf = 0;
  DAT_00aa7ff7 = 0;
  return 1;
}
