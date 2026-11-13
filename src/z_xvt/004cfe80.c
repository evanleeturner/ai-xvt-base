/* SPDX-License-Identifier: MIT */
/* z_xvt 004cfe80 */
extern int DAT_00759b08;
extern int DAT_00759b10;
extern unsigned int DAT_00759b34[];

int __cdecl FUN_004cfe80(int param_1)
{
  unsigned int uVar1;
  
  if (DAT_00759b08 == 0) {
    return 0;
  }
  if ((0 < param_1) && (param_1 <= DAT_00759b10)) {
    uVar1 = DAT_00759b34[param_1];
    return ((uVar1 >> 0x10 & 0xff) * 1000) / 0x4b +
           ((uVar1 & 0xff) * 0x3c + (uVar1 >> 8 & 0xff)) * 1000;
  }
  return 0;
}
