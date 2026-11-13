/* SPDX-License-Identifier: MIT */
/* z_xvt 00425280 */
extern int DAT_0081f7de;
extern int DAT_0081fa9e;
extern int DAT_0071bcb8;

extern int *FUN_00431c90(int *param_1);
extern int FUN_00432c10(int param_1);

int __stdcall FUN_00425280(void)
{
    int *puVar1;
    int *piVar2;
    int iVar3;
    int iVar4;
    int *piVar5;
    int local_4;
    
    iVar4 = 0;
    puVar1 = FUN_00431c90(&local_4);
    if (0 < local_4) {
        piVar5 = puVar1 + 8;
        do {
            iVar3 = 0;
            piVar2 = &DAT_0081f7de;
            do {
                if ((*piVar2 != 0) && (*piVar5 == *piVar2)) break;
                piVar2 = piVar2 + 0x16;
                iVar3 = iVar3 + 1;
            } while (piVar2 < &DAT_0081fa9e);
            if (iVar3 < 8) {
                iVar3 = FUN_00432c10(*piVar5);
                *(int *)((int)&DAT_0071bcb8 + iVar3 * 0x5bd) = *piVar5;
            }
            piVar5 = piVar5 + 10;
            iVar4 = iVar4 + 1;
        } while (iVar4 < local_4);
    }
    return 1;
}
