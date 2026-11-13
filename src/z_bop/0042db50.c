/* SPDX-License-Identifier: MIT */
/* z_bop 0042db50 */
extern int FUN_0042dda0(char *param_1);
extern char DAT_00a10660;

int __cdecl FUN_0042db50(char *param_1, int param_2)
{
  int iVar1;
  
  iVar1 = FUN_0042dda0(param_1);
  if (iVar1 == -1) {
    return 0;
  }
  if (0xff < param_2) {
    param_2 = 0xff;
  }
  if (param_2 < 0) {
    param_2 = 0;
  }
  (&DAT_00a10660)[iVar1 * 0x145] = (char)param_2;
  return 1;
}
