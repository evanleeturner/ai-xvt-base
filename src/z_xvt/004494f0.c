/* SPDX-License-Identifier: MIT */
/* z_xvt 004494f0 */
extern int DAT_005efc40[];

void __cdecl FUN_004491e0(unsigned int param_1, unsigned int param_2, int param_3);
long long __ftol(void);

long long __fastcall FUN_004494f0(unsigned int param_1, unsigned int param_2, int param_3)
{
    if (DAT_005efc40[param_3] == 0) {
        FUN_004491e0(param_1, param_2, param_3);
    }
    return __ftol();
}
