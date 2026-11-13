/* SPDX-License-Identifier: MIT */
/* z_xvt 00413ca0 */
undefined8 __fastcall FUN_00413ca0(undefined4 param_1, undefined4 param_2, int param_3, int param_4)
{
  long long product = (long long)param_3 * (long long)param_4;
  unsigned long long uprod = (unsigned long long)product;
  unsigned int high = (unsigned int)(uprod >> 32);
  unsigned int low = (unsigned int)uprod;
  unsigned int new_low = (low >> 15) | (high << 17);
  return ((long long)high << 32) | (unsigned int)new_low;
}
