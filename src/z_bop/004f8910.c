/* SPDX-License-Identifier: MIT */
/* z_bop 004f8910 */
void FUN_004f8950(char *param_1, int param_2, short param_3, short param_4, short param_5, short param_6);

void __cdecl FUN_004f8910(char *param_1, short param_2, short param_3, short param_4, short param_5, short param_6)
{
    if (param_5 >= 0) {
        FUN_004f8950(param_1, (int)param_2, param_3, param_4, param_5 * 2, param_6);
    }
}
