/* SPDX-License-Identifier: MIT */
/* z_xvt 004d0500 */
extern int DAT_00758d41;
void FUN_004dcfd0(int *param_2, int a, int b, int c, int d);
int FUN_004d3570(char *param_1);

int FUN_004d0500(char *param_1, int *param_2)
{
    int *puVar1;
    int iVar2;

    if (*param_1 == '\0') {
        FUN_004dcfd0(param_2, 0, 0, 0, 0);
        return 0;
    }
    iVar2 = FUN_004d3570(param_1);
    if (iVar2 == -1) {
        FUN_004dcfd0(param_2, 0, 0, 0, 0);
        return 0;
    }
    puVar1 = *(int **)(DAT_00758d41 + 0x40 + iVar2 * 0x44);
    FUN_004dcfd0(param_2, 0, 0, *puVar1, puVar1[1]);
    return 1;
}
