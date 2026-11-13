/* SPDX-License-Identifier: MIT */
/* z_xvt 004d35a0 */
int _strncmp(char *s1, char *s2, unsigned int n);

int __cdecl FUN_004d35a0(char *param_1, int param_2, char *param_3)
{
    int iVar1;
    int iVar2;
    int iVar3;

    iVar2 = 0;
    while (1) {
        if (param_2 < 0) {
            return -1;
        }
        iVar3 = param_2 >> 1;
        iVar1 = _strncmp(param_1 + iVar3 * 0x44, param_3, 0x40);
        if (iVar1 == 0) {
            break;
        }
        if (param_2 < 1) {
            return -1;
        }
        if (iVar1 < 0) {
            iVar2 = iVar3 + 1 + iVar2;
            param_2 = param_2 - (iVar3 + 1);
            param_1 = param_1 + iVar3 * 0x44 + 0x44;
        } else {
            param_2 = iVar3 - 1;
        }
    }
    return iVar3 + iVar2;
}
