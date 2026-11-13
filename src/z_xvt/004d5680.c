/* SPDX-License-Identifier: MIT */
/* z_xvt 004d5680 */
extern unsigned int DAT_0078073a;
extern unsigned int DAT_00780c5a;
extern unsigned int DAT_00758840;
extern unsigned int _DAT_0078109e;
extern unsigned int DAT_00780732;
extern unsigned int DAT_0078075a;
extern unsigned int DAT_007806fa;
extern unsigned int DAT_0078077a;
extern unsigned int DAT_00780726;
extern unsigned int DAT_00780c7a;

void __stdcall FUN_004ca450(void);

void __stdcall FUN_004d5680(void)
{
    unsigned int local_48[10];
    unsigned int local_20[8];
    unsigned int *puVar4;
    unsigned int *puVar5;
    int iVar2;
    unsigned int uVar1;
    unsigned int *puVar3;

    puVar4 = &DAT_0078073a;
    puVar5 = local_20;
    for (iVar2 = 8; iVar2 != 0; iVar2--) {
        *puVar5 = *puVar4;
        puVar4++;
        puVar5++;
    }
    puVar4 = &DAT_00780c5a;
    puVar5 = local_48;
    for (iVar2 = 10; iVar2 != 0; iVar2--) {
        *puVar5 = *puVar4;
        puVar4++;
        puVar5++;
    }
    puVar4 = &DAT_00758840;
    for (iVar2 = 0x30ba7; iVar2 != 0; iVar2--) {
        *puVar4 = 0;
        puVar4++;
    }
    puVar4 = local_20;
    puVar5 = &DAT_0078073a;
    for (iVar2 = 8; iVar2 != 0; iVar2--) {
        *puVar5 = *puVar4;
        puVar4++;
        puVar5++;
    }
    puVar4 = local_48;
    puVar5 = &DAT_00780c5a;
    for (iVar2 = 10; iVar2 != 0; iVar2--) {
        *puVar5 = *puVar4;
        puVar4++;
        puVar5++;
    }
    _DAT_0078109e = 1;
    DAT_00780732 = 1;
    puVar4 = local_48;
    puVar5 = &DAT_0078075a;
    for (iVar2 = 10; iVar2 != 0; iVar2--) {
        *puVar5 = *puVar4;
        puVar4++;
        puVar5++;
    }
    if (DAT_007806fa != 0) {
        FUN_004ca450();
        iVar2 = 0;
        puVar3 = &DAT_0078077a;
        do {
            if (*puVar3 == DAT_00780726) break;
            puVar3 += 10;
            iVar2++;
        } while (puVar3 < &DAT_00780c7a);
        if (iVar2 == 0x20) {
            puVar3 = &DAT_0078077a;
            DAT_00780726 = DAT_00780c7a;
            do {
                uVar1 = *puVar3;
                if (((uVar1 != 0) && (puVar3[1] != 0)) && (uVar1 < DAT_00780726)) {
                    DAT_00780726 = uVar1;
                }
                puVar3 += 10;
            } while (puVar3 < &DAT_00780c7a);
        }
    }
    return;
}
