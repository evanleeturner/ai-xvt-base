/* SPDX-License-Identifier: MIT */
/* z_xvt 00406830 */
extern int DAT_00525518;
extern unsigned char DAT_007091b0[];
extern unsigned char DAT_00703ae0[];
extern unsigned char DAT_006db5c0[];

void __cdecl FUN_00406830(int param_1)
{
    unsigned char uVar1;
    int iVar2;
    int iVar3;
    int iVar4;
    unsigned char *puVar5;
    
    iVar2 = *(int *)(param_1 + 0x28);
    puVar5 = (unsigned char *)(*(int *)(param_1 + 0xc) + param_1);
    if (DAT_00525518 == 2) {
        iVar4 = 0;
        if (0 < iVar2) {
            do {
                DAT_007091b0[iVar4] = *puVar5;
                iVar3 = iVar4 + 1;
                DAT_00703ae0[iVar4] = puVar5[1];
                iVar4 = iVar3;
                puVar5 = puVar5 + 2;
            } while (iVar3 < iVar2);
            return;
        }
    }
    else {
        iVar4 = 0;
        if (0 < iVar2) {
            do {
                uVar1 = *puVar5;
                puVar5 = puVar5 + 1;
                DAT_006db5c0[iVar4] = uVar1;
                iVar4 = iVar4 + 1;
            } while (iVar4 < iVar2);
        }
    }
    return;
}
