/* SPDX-License-Identifier: MIT */
/* z_xvt 004dcfa0 */
extern unsigned int DAT_0081b5d2;
extern char *s_No_text__00532bc8;
extern unsigned int DAT_0081b5ca;
extern unsigned int DAT_0081b5ce;

char * __cdecl FUN_004dcfa0(unsigned int param_1)
{
    if (DAT_0081b5d2 <= param_1) {
        return s_No_text__00532bc8;
    }
    return (char *)(*(int *)(DAT_0081b5ca + param_1 * 4) + DAT_0081b5ce);
}
