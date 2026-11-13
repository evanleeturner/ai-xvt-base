/* SPDX-License-Identifier: MIT */
/* z_bop 00427500 */
extern int DAT_00a004b0;
extern int DAT_009fe7f4;
extern int DAT_00a00490;

int __cdecl FUN_00427500(int param_1, int param_2, int param_3)
{
    long long v1 = (long long)DAT_00a004b0 * (long long)param_1;
    unsigned long long uv1 = (unsigned long long)v1;
    unsigned int t1 = ((unsigned int)uv1 >> 15) | ((unsigned int)(uv1 >> 32) << 17);

    long long v2 = (long long)DAT_009fe7f4 * (long long)param_2;
    unsigned long long uv2 = (unsigned long long)v2;
    unsigned int t2 = ((unsigned int)uv2 >> 15) | ((unsigned int)(uv2 >> 32) << 17);

    long long v3 = (long long)DAT_00a00490 * (long long)param_3;
    unsigned long long uv3 = (unsigned long long)v3;
    unsigned int t3 = ((unsigned int)uv3 >> 15) | ((unsigned int)(uv3 >> 32) << 17);

    return (t1 + t2) + t3;
}
