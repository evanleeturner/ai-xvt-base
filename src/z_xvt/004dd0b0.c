/* SPDX-License-Identifier: MIT */
/* z_xvt 004dd0b0 */
extern int DAT_007596b7;
extern int DAT_007596bb;
extern int DAT_007596bf;
extern int DAT_007596c3;

unsigned int __cdecl FUN_004dd0b0(int *param_1)
{
    int iVar1;
    unsigned int uVar2;

    iVar1 = DAT_007596b7;
    uVar2 = 0;
    if (*param_1 < DAT_007596b7) {
        *param_1 = DAT_007596b7;
        if (param_1[2] < iVar1) {
            param_1[2] = (int)((unsigned int)iVar1 - 1u);
        }
        uVar2 = 1;
    }
    iVar1 = DAT_007596bb;
    if (DAT_007596bb < param_1[2]) {
        param_1[2] = DAT_007596bb;
        if (iVar1 < *param_1) {
            *param_1 = (int)((unsigned int)iVar1 + 1u);
        }
        uVar2 = uVar2 | 4;
    }
    iVar1 = DAT_007596bf;
    if (param_1[1] < DAT_007596bf) {
        param_1[1] = DAT_007596bf;
        if (param_1[3] < iVar1) {
            param_1[3] = (int)((unsigned int)iVar1 - 1u);
        }
        uVar2 = uVar2 | 2;
    }
    iVar1 = DAT_007596c3;
    if (DAT_007596c3 < param_1[3]) {
        param_1[3] = DAT_007596c3;
        if (iVar1 < param_1[1]) {
            param_1[1] = (int)((unsigned int)iVar1 + 1u);
        }
        uVar2 = uVar2 | 8;
    }
    return uVar2;
}
