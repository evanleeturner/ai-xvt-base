/* SPDX-License-Identifier: MIT */
/* z_xvt 004dd070 */
void __cdecl FUN_004dd070(int *param_1, int param_2, int param_3)
{
    *param_1 = (int)((unsigned int)*param_1 + (unsigned int)param_2);
    param_1[2] = (int)((unsigned int)param_1[2] - (unsigned int)param_2);
    param_1[1] = (int)((unsigned int)param_1[1] + (unsigned int)param_3);
    param_1[3] = (int)((unsigned int)param_1[3] - (unsigned int)param_3);
}
