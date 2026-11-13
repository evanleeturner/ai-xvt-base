/* SPDX-License-Identifier: MIT */
/* z_bop 004b1d70 */
extern unsigned int *DAT_00664c80;
extern unsigned int *DAT_00664c84;
extern unsigned int DAT_00528ca0;

unsigned int __stdcall FUN_004b1d70(void)
{
  DAT_00664c84 = DAT_00664c80;
  *(unsigned char *)DAT_00664c80 = 9;
  *(unsigned char *)((unsigned int)DAT_00664c80 + 1) = 0x10;
  *(unsigned short *)((unsigned int)DAT_00664c80 + 2) = 1;
  DAT_00664c80 = (unsigned int *)((unsigned int)DAT_00664c80 + 4);
  *DAT_00664c80 = 2;
  *(unsigned short *)(DAT_00664c80 + 1) = 0;
  *(unsigned short *)((unsigned int)DAT_00664c80 + 6) = 0;
  DAT_00664c80[2] = DAT_00528ca0;
  DAT_00664c80[3] = 0;
  DAT_00664c80 = DAT_00664c80 + 4;
  return 1;
}
