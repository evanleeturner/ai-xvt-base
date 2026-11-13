/* SPDX-License-Identifier: MIT */
/* z_xvt 0048e520 */
int __strcmpi(char *, char *);

undefined4 * __cdecl FUN_0048e520(undefined4 *param_1, char *param_2)
{
    int iVar1;
    undefined4 *puVar2;
    int iVar3;

    if (param_1 == (undefined4 *)0x0) {
        return (undefined4 *)0x0;
    }
    if (((char *)*param_1 != (char *)0x0) && (iVar1 = __strcmpi((char *)*param_1, param_2), iVar1 == 0)) {
        return param_1;
    }
    iVar1 = 0;
    iVar3 = 0;
    if (0 < (int)param_1[2]) {
        do {
            puVar2 = FUN_0048e520(*(undefined4 **)(param_1[3] + iVar1), param_2);
            if (puVar2 != (undefined4 *)0x0) {
                return puVar2;
            }
            iVar1 = iVar1 + 4;
            iVar3 = iVar3 + 1;
        } while (iVar3 < (int)param_1[2]);
    }
    return (undefined4 *)0x0;
}
