/* SPDX-License-Identifier: MIT */
/* z_bop 004ddbc0 */
extern void * DAT_00b69a95;
extern void * DAT_00b69a99;
extern void * DAT_00b69a9d;
extern void * DAT_00b69aa1;

void _free(void *);

void __stdcall FUN_004ddbc0(void)
{
  if (DAT_00b69a95 != (void *)0x0) {
    _free(DAT_00b69a95);
    DAT_00b69a95 = (void *)0x0;
  }
  if (DAT_00b69a99 != (void *)0x0) {
    _free(DAT_00b69a99);
    DAT_00b69a99 = (void *)0x0;
  }
  DAT_00b69a9d = 0;
  DAT_00b69aa1 = 0;
  return;
}
