/* SPDX-License-Identifier: MIT */
/* z_bop 004facc0 */
extern int DAT_00b69e30;
extern int DAT_00a91ca0;
extern int DAT_00a91cc0;

int __stdcall FUN_004d1400(void);

int __stdcall FUN_004facc0(void)
{
    int iVar1;
    int *piVar2;
    int iVar3;
    
    iVar3 = 0;
    if (DAT_00b69e30 == 2) {
        return 0;
    }
    iVar1 = FUN_004d1400();
    piVar2 = &DAT_00a91ca0;
    do {
        if (*piVar2 != 0) {
            iVar3 = iVar3 + 1;
        }
        piVar2 = piVar2 + 1;
    } while (piVar2 < &DAT_00a91cc0);
    return iVar3 == iVar1;
}
