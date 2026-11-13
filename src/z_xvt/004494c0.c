/* SPDX-License-Identifier: MIT */
/* z_xvt 004494c0 */
extern int DAT_005efc40[];
extern void FUN_004491e0(undefined4 param_1, undefined4 param_2, int param_3);
extern long long __ftol(void);

long long __fastcall FUN_004494c0(undefined4 param_1, undefined4 param_2, int param_3)
{
    if (DAT_005efc40[param_3] == 0) {
        FUN_004491e0(param_1, param_2, param_3);
    }
    return __ftol();
}
