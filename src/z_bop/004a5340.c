/* SPDX-License-Identifier: MIT */
/* z_bop 004a5340 */
extern int DAT_00622c4c;
extern int DAT_00622bc4;
extern int DAT_00622bcc;

int __cdecl FUN_004a5340(int param_1)
{
  unsigned int uVar1;
  
  if (DAT_00622c4c != 0) {
    return 0;
  }
  if ((0 < param_1) && (param_1 <= DAT_00622bc4)) {
    uVar1 = *(unsigned int *)(&DAT_00622bcc + param_1 * 4);
    return (((uVar1 >> 0x10) & 0xff) * 1000) / 0x4b +
           ((uVar1 & 0xff) * 0x3c + ((uVar1 >> 8) & 0xff)) * 1000;
  }
  return 0;
}
