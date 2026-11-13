/* SPDX-License-Identifier: MIT */
/* z_bop 00427490 */
extern int DAT_00a00480;
extern int DAT_00a004a4;
extern int DAT_00a004b8;

int __cdecl FUN_00427490(int param_1, int param_2, int param_3)
{
    long long v1 = (long long)DAT_00a004a4 * (long long)param_1;
    int term1 = ((unsigned int)v1 >> 15) | ((int)(v1 >> 32) << 17);

    long long v2 = (long long)DAT_00a004b8 * (long long)param_2;
    int term2 = ((unsigned int)v2 >> 15) | ((int)(v2 >> 32) << 17);

    long long v3 = (long long)DAT_00a00480 * (long long)param_3;
    int term3 = ((unsigned int)v3 >> 15) | ((int)(v3 >> 32) << 17);

    return (term1 + term2) + term3;
}
