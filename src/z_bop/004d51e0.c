/* SPDX-License-Identifier: MIT */
/* z_bop 004d51e0 */
extern unsigned int DAT_00acec05;
extern unsigned int DAT_00acf125;
extern unsigned int DAT_00aa6d00;
extern unsigned int _DAT_00acf569;
extern unsigned int DAT_00acebfd;
extern unsigned int DAT_00acec25;
extern unsigned int DAT_00acebc5;
extern unsigned int DAT_00acec45;
extern unsigned int DAT_00acebf1;
extern unsigned int DAT_00acf145;

void __stdcall FUN_004cdc20(void);

void __stdcall FUN_004d51e0(void)
{
    unsigned int uVar1;
    int iVar2;
    unsigned int *puVar3;
    unsigned int *puVar4;
    unsigned int *puVar5;
    unsigned int local_48[10];
    unsigned int local_20[8];
    
    puVar4 = &DAT_00acec05;
    puVar5 = local_20;
    for (iVar2 = 8; iVar2 != 0; iVar2 = iVar2 - 1) {
        *puVar5 = *puVar4;
        puVar4 = puVar4 + 1;
        puVar5 = puVar5 + 1;
    }
    puVar4 = &DAT_00acf125;
    puVar5 = local_48;
    for (iVar2 = 10; iVar2 != 0; iVar2 = iVar2 - 1) {
        *puVar5 = *puVar4;
        puVar4 = puVar4 + 1;
        puVar5 = puVar5 + 1;
    }
    puVar4 = &DAT_00aa6d00;
    for (iVar2 = 0x30be9; iVar2 != 0; iVar2 = iVar2 - 1) {
        *puVar4 = 0;
        puVar4 = puVar4 + 1;
    }
    *(unsigned short *)puVar4 = 0;
    *(unsigned char *)((int)puVar4 + 2) = 0;
    puVar4 = local_20;
    puVar5 = &DAT_00acec05;
    for (iVar2 = 8; iVar2 != 0; iVar2 = iVar2 - 1) {
        *puVar5 = *puVar4;
        puVar4 = puVar4 + 1;
        puVar5 = puVar5 + 1;
    }
    puVar4 = local_48;
    puVar5 = &DAT_00acf125;
    for (iVar2 = 10; iVar2 != 0; iVar2 = iVar2 - 1) {
        *puVar5 = *puVar4;
        puVar4 = puVar4 + 1;
        puVar5 = puVar5 + 1;
    }
    _DAT_00acf569 = 1;
    DAT_00acebfd = 1;
    puVar4 = local_48;
    puVar5 = &DAT_00acec25;
    for (iVar2 = 10; iVar2 != 0; iVar2 = iVar2 - 1) {
        *puVar5 = *puVar4;
        puVar4 = puVar4 + 1;
        puVar5 = puVar5 + 1;
    }
    if (DAT_00acebc5 != 0) {
        FUN_004cdc20();
        iVar2 = 0;
        puVar3 = &DAT_00acec45;
        do {
            if (*puVar3 == DAT_00acebf1) break;
            puVar3 = puVar3 + 10;
            iVar2 = iVar2 + 1;
        } while (puVar3 < &DAT_00acf145);
        if (iVar2 == 0x20) {
            puVar3 = &DAT_00acec45;
            DAT_00acebf1 = DAT_00acf145;
            do {
                uVar1 = *puVar3;
                if (((uVar1 != 0) && (puVar3[1] != 0)) && (uVar1 < DAT_00acebf1)) {
                    DAT_00acebf1 = uVar1;
                }
                puVar3 = puVar3 + 10;
            } while (puVar3 < &DAT_00acf145);
        }
    }
    return;
}
