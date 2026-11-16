/* SPDX-License-Identifier: MIT */
/* z_bop 004d12f0 */
extern int DAT_00acebfd;
extern int DAT_00acec45;
extern int DAT_00acec25;

int * __cdecl FUN_004d12f0(int param_1)
{
    int *piVar1;
    int iVar2;
    int bVar3;
    
    iVar2 = 0;
    bVar3 = (DAT_00acebfd == 0);
    if (DAT_00acebfd > 0) {
        piVar1 = &DAT_00acec45;
        do {
            if (*piVar1 == param_1) {
                break;
            }
            piVar1 = piVar1 + 10;
            iVar2 = iVar2 + 1;
        } while (iVar2 < DAT_00acebfd);
        bVar3 = (DAT_00acebfd == iVar2);
    }
    if (bVar3) {
        return (int *)0x0;
    }
    return &DAT_00acec25 + iVar2 * 10;
}
