/* SPDX-License-Identifier: MIT */
/* z_bop 004d12b0 */
extern int DAT_00acebfd;
extern int DAT_00acec45;
extern int DAT_00acec49;

int __cdecl FUN_004d12b0(int param_1)
{
    int iVar1 = 0;
    int *piVar2;
    int bVar3;
    
    bVar3 = (DAT_00acebfd == 0);
    if (0 < DAT_00acebfd) {
        piVar2 = &DAT_00acec45;
        do {
            if (*piVar2 == param_1) {
                break;
            }
            piVar2 = piVar2 + 10;
            iVar1 = iVar1 + 1;
        } while (iVar1 < DAT_00acebfd);
        bVar3 = (DAT_00acebfd == iVar1);
    }
    if (bVar3) {
        return 0;
    }
    return (&DAT_00acec49)[iVar1 * 10];
}
