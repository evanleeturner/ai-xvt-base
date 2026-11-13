/* SPDX-License-Identifier: MIT */
/* z_bop 004b1600 */
extern char s_Unknown_Error_005293b0[];

char * __cdecl FUN_004b1600(int param_1, int *param_2, int param_3)
{
    char *pcVar1;
    int *piVar2;
    int iVar3;

    pcVar1 = s_Unknown_Error_005293b0;
    iVar3 = 0;
    piVar2 = param_2;
    if (0 < param_3) {
        while (*piVar2 != param_1) {
            iVar3 = iVar3 + 1;
            piVar2 = piVar2 + 2;
            if (param_3 <= iVar3) {
                return pcVar1;
            }
        }
        pcVar1 = (char *)param_2[iVar3 * 2 + 1];
    }
    return pcVar1;
}
