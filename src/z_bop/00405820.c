/* SPDX-License-Identifier: MIT */
/* z_bop 00405820 */
extern int DAT_009a1fe8;
extern int DAT_0051c573;

int __cdecl FUN_00405820(uint param_1)
{
  uint uAddr;
  uint uPtr1;
  uint uPtr2;
  byte bVal;
  uint uVal;
  uint tmp1;
  uint tmp2;
  int result;

  uAddr = (param_1 & 0xffff) * 0x23 + 0x1f + (uint)DAT_009a1fe8;
  __builtin_memcpy(&uPtr1, (void *)uAddr, 4);
  uPtr2 = uPtr1 + 0xa9;
  __builtin_memcpy(&tmp1, (void *)uPtr2, 4);
  uPtr2 = tmp1;
  bVal = *(byte *)(uPtr2 + 4);
  uAddr = (uint)DAT_0051c573 + bVal * 0x101;
  __builtin_memcpy(&tmp2, (void *)uAddr, 4);
  uVal = tmp2 * 2;
  result = (int)uVal;
  return result;
}
