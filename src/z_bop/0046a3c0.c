/* SPDX-License-Identifier: MIT */
/* z_bop 0046a3c0 */
long double sinl(long double);
long long __ftol(void);
float _DAT_0051815c;

long long __cdecl FUN_0046a3c0(short param_1)
{
    long long lVar1;
    sinl((long double)param_1 * (long double)_DAT_0051815c);
    lVar1 = __ftol();
    return lVar1;
}
