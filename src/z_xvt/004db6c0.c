/* SPDX-License-Identifier: MIT */
/* z_xvt 004db6c0 */
extern unsigned int DAT_0077dc52;
extern unsigned char DAT_0077dc56;

void __cdecl FUN_004dd000(unsigned int *param_1, unsigned int *param_2);

unsigned int __cdecl FUN_004db6c0(unsigned int param_1, unsigned int *param_2)
{
    DAT_0077dc52 = param_1;
    FUN_004dd000((unsigned int *)&DAT_0077dc56, param_2);
    return 1;
}
