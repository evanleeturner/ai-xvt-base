/* SPDX-License-Identifier: MIT */
/* z_bop 004afa90 */
extern int DAT_00a00870;
extern int DAT_00a07ba4;

int FUN_004adc40(int);
int FUN_004ae2a0(int, int);
int FUN_004ae200(int, int);

void __stdcall FUN_004afa90(void)
{
    int *local_4;
    int iVar4;
    int iVar5;
    int iVar2;
    int *piVar1;
    int iVar3;

    local_4 = &DAT_00a00870;
    iVar4 = 0;
    do {
        iVar5 = 0;
        iVar2 = FUN_004adc40(iVar4);
        *local_4 = iVar2;
        piVar1 = local_4;
        if (iVar2 > 0) {
            do {
                iVar3 = FUN_004ae2a0(iVar4, iVar5);
                piVar1[1] = iVar3;
                iVar5 = iVar5 + 1;
                iVar5 = FUN_004ae200(iVar4, iVar5);
                piVar1[0x33] = iVar5;
            } while (iVar5 < iVar2);
        }
        iVar4 = iVar4 + 1;
        local_4 = local_4 + 0x65;
    } while (local_4 < &DAT_00a07ba4);
    return;
}
