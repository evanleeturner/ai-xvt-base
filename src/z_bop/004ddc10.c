/* SPDX-License-Identifier: MIT */
/* z_bop 004ddc10 */
extern unsigned int DAT_00b69a9d;
extern char * s_No_text__0052c0f0;
extern unsigned int DAT_00b69a95;
extern int DAT_00b69a99;

char * __cdecl FUN_004ddc10(unsigned int param_1)
{
    if (DAT_00b69a9d <= param_1) {
        return s_No_text__0052c0f0;
    }
    return (char *)(*(int *)(DAT_00b69a95 + param_1 * 4) + DAT_00b69a99);
}
