/* SPDX-License-Identifier: MIT */
/* z_bop 004a9380 */
extern bool __cdecl FUN_0042d600(void *param_1);
extern char DAT_00a0adf0[];

bool __cdecl FUN_004a9380(int param_1)
{
  if (3 < param_1 && param_1 < 0x346) {
    return FUN_0042d600(DAT_00a0adf0 + param_1 * 0x18);
  }
  return 0;
}
