/* SPDX-License-Identifier: MIT */
/* z_xvt 004ed020 */
extern unsigned int DAT_00758620;
extern unsigned int DAT_00757cf6;
extern unsigned int DAT_00757e44;
extern unsigned int DAT_00757680;

unsigned int __cdecl FUN_004cb830(int param_1, unsigned int *param_2, unsigned int param_3);

unsigned int __stdcall FUN_004ed020(void)
{
    unsigned char uVar1;
    unsigned int uVar2;
    unsigned int uVar3;
    int iVar4;
    int iVar5;
    unsigned int *puVar6;
    unsigned int *puVar7;
    unsigned char *puVar8;
    unsigned char *puVar9;
    unsigned char *puVar10;
    int iVar11;

    iVar5 = 1;
    DAT_00758620 = 0x61;
    puVar6 = &DAT_00757cf6;
    do {
        uVar2 = puVar6[1];
        *(unsigned int *)((unsigned char *)&DAT_00758620 + iVar5 * 4) = *puVar6;
        uVar3 = puVar6[2];
        puVar7 = (unsigned int *)((int)puVar6 + 0x2a);
        *(unsigned int *)((iVar5 + 4) * 4 + 0x758614) = uVar2;
        iVar5 = iVar5 + 5;
        uVar2 = puVar6[3];
        *(unsigned int *)(iVar5 * 4 + 0x758614) = uVar3;
        uVar3 = puVar6[4];
        *(unsigned int *)(iVar5 * 4 + 0x758618) = uVar2;
        *(unsigned int *)(iVar5 * 4 + 0x75861c) = uVar3;
        puVar6 = puVar7;
    } while (puVar7 < (unsigned int *)((int)&DAT_00757e44 + 2));

    puVar10 = (unsigned char *)&DAT_00758620 + iVar5 * 4;
    puVar8 = (unsigned char *)&DAT_00757680;
    iVar11 = 10;
    do {
        iVar4 = 8;
        puVar9 = puVar10;
        do {
            uVar1 = *puVar8;
            puVar10 = puVar9 + 1;
            puVar8 = puVar8 + 4;
            iVar4 = iVar4 - 1;
            *puVar9 = uVar1;
            puVar9 = puVar10;
        } while (iVar4 != 0);
        iVar11 = iVar11 - 1;
    } while (iVar11 != 0);

    FUN_004cb830(0, (unsigned int *)&DAT_00758620, iVar5 * 4 + 0x50);
    return 1;
}
