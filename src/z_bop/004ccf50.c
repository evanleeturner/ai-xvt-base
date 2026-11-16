/* SPDX-License-Identifier: MIT */
/* z_bop 004ccf50 */
extern char DAT_00b69ba7[];
int __chdir(const char *);

int __stdcall FUN_004ccf50(void)
{
  __chdir(DAT_00b69ba7);
  return 1;
}
