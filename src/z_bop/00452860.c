/* SPDX-License-Identifier: MIT */
/* z_bop 00452860 */
void __cdecl FUN_00452860(char param_1, char *param_2)
{
    char *pcVar1;
    char cVar2;

    cVar2 = *param_2;
    while (cVar2 != '\0') {
        pcVar1 = param_2 + 1;
        param_2 = param_2 + 1;
        cVar2 = *pcVar1;
    }
    *param_2 = param_1;
    param_2[1] = '\0';
    return;
}
