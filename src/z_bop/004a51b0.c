/* SPDX-License-Identifier: MIT */
/* z_bop 004a51b0 */
extern unsigned int DAT_00622c4c;
extern unsigned int DAT_00622c48;
extern unsigned int _DAT_00622bc0;

unsigned int __stdcall FUN_004a51b0(void)
{
  unsigned char local_4[4];
  
  if (DAT_00622c4c == 0) {
    return 0;
  }
  if (DAT_00622c48 == 0) {
    return 0;
  }
  mciSendCommandA(DAT_00622c4c, 0x808, 0, (unsigned long)local_4);
  DAT_00622c48 = 0;
  _DAT_00622bc0 = 0;
  return 1;
}
