/* SPDX-License-Identifier: MIT */
/* z_bop 004ccf70 */
extern int __chdir(const char *);
extern const char DAT_00b69aa7[];

unsigned int __stdcall FUN_004ccf70(void)
{
    __chdir(DAT_00b69aa7);
    return 1;
}
