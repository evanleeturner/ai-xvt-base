/* SPDX-License-Identifier: MIT */
/* z_xvt 00439530 */
extern int DAT_005ef2cc;
extern int DAT_005ef244;
extern unsigned int DAT_005ef24c;

int __cdecl FUN_00439530(int param_1)
{
  unsigned int uVar1;
  
  if (DAT_005ef2cc == 0) {
    return 0;
  }
  if ((0 < param_1) && (param_1 <= DAT_005ef244)) {
    uVar1 = *(unsigned int *)((char *)&DAT_005ef24c + param_1 * 4);
    return ((uVar1 >> 0x10 & 0xff) * 1000) / 0x4b +
           ((uVar1 & 0xff) * 0x3c + (uVar1 >> 8 & 0xff)) * 1000;
  }
  return 0;
}
