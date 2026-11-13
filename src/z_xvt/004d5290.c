/* SPDX-License-Identifier: MIT */
/* z_xvt 004d5290 */
extern int DAT_007806ee;

void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);

void __cdecl FUN_004d5290(int param_1)
{
    if (param_1 == 0) {
        RaiseException(0xC0000094UL, 0, 0, NULL);
    } else {
        DAT_007806ee = (int)(1000LL / (long long)param_1);
    }
    return;
}
