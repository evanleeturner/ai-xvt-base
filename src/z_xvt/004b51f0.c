/* SPDX-License-Identifier: MIT */
/* z_xvt 004b51f0 */
extern char s_Unknown_Error_005300f8[];

char * __cdecl FUN_004b51f0(int param_1, int param_2, int param_3)
{
    int local_8;
    
    local_8 = 0;
    while (1) {
        if (param_3 <= local_8) {
            return s_Unknown_Error_005300f8;
        }
        if (*(int *)(param_2 + local_8 * 8) == param_1) {
            break;
        }
        local_8 = local_8 + 1;
    }
    return *(char **)(param_2 + 4 + local_8 * 8);
}
