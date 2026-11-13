/* SPDX-License-Identifier: MIT */
/* z_xvt 0048fce0 */
extern void __cdecl _fprintf(void *stream, const char *format);
extern void __cdecl _exit(int code);
extern void DAT_005344c0;

void __cdecl FUN_0048fce0(char *param_1, int param_2)
{
    _fprintf(&DAT_005344c0, param_1);
    _exit(param_2);
}
