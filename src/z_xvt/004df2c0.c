/* SPDX-License-Identifier: MIT */
/* z_xvt 004df2c0 */
extern unsigned char DAT_00758ee8[];
extern unsigned char DAT_00758eee[];

unsigned long long __fastcall FUN_004df2c0(unsigned int param_1, unsigned int param_2, int param_3)
{
  unsigned int uVar1;
  
  if (1 < param_3) {
    return ((unsigned long long)param_2 << 32) | 0xFFFFFFFF;
  }
  if (DAT_00758ee8[param_3] != '\0') {
    return ((unsigned long long)param_2 << 32) | 0xFFFFFFFF;
  }
  uVar1 = 0;
  do {
    if (DAT_00758eee[param_3 * 0x20 + uVar1] == '\0') {
      return uVar1;
    }
    uVar1 = uVar1 + 1;
  } while ((int)uVar1 < 0x20);
  return 0xFFFFFFFF;
}
