/* SPDX-License-Identifier: MIT */
/* z_bop 00430930 */
extern unsigned char DAT_00a0a880[];
extern unsigned char DAT_00a0aaa0[];
extern unsigned char DAT_00a0aa20[];
extern unsigned int DAT_009a20a4;
extern unsigned int DAT_00a0fc80[];
extern unsigned short DAT_00a0a910[];

void __cdecl FUN_00430930(unsigned int param_1)
{
    unsigned int iVar5;
    unsigned int uVar2;
    unsigned char bVar6;
    unsigned int *puVar3;
    unsigned int *puVar4;
    unsigned short *puVar7;
    unsigned short *puVar8;
    unsigned char *pcVar1;

    iVar5 = 1;
    if (DAT_00a0a880[param_1 + 1] != 0) {
        uVar2 = param_1;
        do {
            if ((DAT_009a20a4 & 0xFF) <= uVar2 + 1) break;
            pcVar1 = &DAT_00a0a880[uVar2 + 2];
            iVar5 = iVar5 + 1;
            uVar2 = uVar2 + 1;
        } while (*pcVar1 != 0);
    }
    bVar6 = DAT_009a20a4 - (char)iVar5;
    DAT_009a20a4 = (DAT_009a20a4 & 0xFFFFFF00) | bVar6;
    if (param_1 < bVar6) {
        puVar4 = &DAT_00a0fc80[param_1 + iVar5];
        puVar3 = &DAT_00a0fc80[param_1];
        puVar7 = &DAT_00a0a910[param_1];
        puVar8 = &DAT_00a0a910[param_1 + iVar5];
        do {
            *puVar3 = *puVar4;
            puVar4++;
            DAT_00a0aa20[param_1] = DAT_00a0aa20[iVar5 + param_1];
            puVar3++;
            DAT_00a0aaa0[param_1] = DAT_00a0aaa0[iVar5 + param_1];
            DAT_00a0a880[param_1] = DAT_00a0a880[iVar5 + param_1];
            param_1++;
            *puVar7 = *puVar8;
            puVar7++;
            puVar8++;
        } while (param_1 < bVar6);
    }
    return;
}
