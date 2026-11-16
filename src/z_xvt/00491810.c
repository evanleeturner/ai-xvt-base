/* SPDX-License-Identifier: MIT */
/* z_xvt 00491810 */
extern char DAT_0065865c[];
extern int DAT_006a8814;
extern char DAT_006587a0[];

int __cdecl _strncmp(const char *s1, const char *s2, int n);

int __cdecl FUN_00491810(char *param_1, int param_2)
{
    int iVar1;
    int iVar2;
    int iVar3;
    int iVar4;
    char *local_4;

    local_4 = DAT_0065865c;
    iVar3 = DAT_006a8814 + -1;
    iVar2 = 0;
    while (-1 < iVar3) {
        iVar4 = iVar3 >> 1;
        iVar1 = _strncmp(local_4 + iVar4 * 0x145, param_1, 0x40);
        if (iVar1 == 0) {
            iVar2 = iVar2 + iVar4;
            goto LAB_0049187f;
        }
        if (iVar3 < 1) break;
        if (iVar1 >= 0) {
            iVar2 = iVar4 + 1 + iVar2;
            iVar3 = iVar3 - (iVar4 + 1);
            local_4 = local_4 + iVar4 * 0x145 + 0x145;
        }
        else {
            iVar3 = iVar4 + -1;
        }
    }
    iVar2 = -1;
LAB_0049187f:
    if (iVar2 == -1) {
        return 0;
    }
    if (0xff < param_2) {
        param_2 = 0xff;
    }
    if (param_2 < 0) {
        param_2 = 0;
    }
    DAT_006587a0[iVar2 * 0x145] = (char)param_2;
    return 1;
}
