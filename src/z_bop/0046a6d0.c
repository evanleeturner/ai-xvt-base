/* SPDX-License-Identifier: MIT */
/* z_bop 0046a6d0 */
extern float _DAT_0051815c;
extern long long __ftol(void);
extern long double fcos(long double);

long long __cdecl FUN_0046a6d0(short param_1)
{
    long long lVar1;
    fcos((long double)(int)param_1 * (long double)_DAT_0051815c);
    lVar1 = __ftol();
    return lVar1;
}
