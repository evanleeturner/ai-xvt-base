/* SPDX-License-Identifier: MIT */
/* z_xvt 004a3e40 */
unsigned int __cdecl FUN_004ac6f0(unsigned int a, unsigned int *b);

extern unsigned char *DAT_006d4988;
extern char DAT_0070acf0[];
extern char *DAT_0073b268[];
extern char *DAT_0073b1b4[];
extern char DAT_0052b070[];
extern char DAT_0052b07a[];

void __cdecl FUN_004a3e40(unsigned int param_1, short param_2, char *param_3)
{
    char *pcVar1;
    unsigned char *iVar2;
    unsigned int iVar6;
    unsigned char bVar3;
    char cVar4;
    unsigned int *puVar5;
    int bVar7;
    char *pcVar8;
    unsigned int uVar9;
    char *pcVar10;

    bVar7 = 0;
    *param_3 = '\0';
    iVar6 = (param_1 & 0xffff) * 0x23;
    __builtin_memcpy(&pcVar10, DAT_006d4988 + 0x1f + iVar6, sizeof(pcVar10));
    iVar2 = DAT_006d4988 + iVar6;
    if (pcVar10 == (char *)0) {
        if ((param_2 != 1) && (param_2 != 0)) {
            pcVar10 = DAT_0070acf0 + (unsigned int)iVar2[0x16] * 0x566;
            cVar4 = *pcVar10;
            while (cVar4 != '\0') {
                cVar4 = *pcVar10;
                pcVar10 = pcVar10 + 1;
                *param_3 = cVar4;
                param_3 = param_3 + 1;
                cVar4 = *pcVar10;
            }
            *param_3 = '\0';
            return;
        }
        pcVar10 = DAT_0073b268[(unsigned int)iVar2[3]];
        cVar4 = *pcVar10;
        pcVar8 = param_3;
        while (cVar4 != '\0') {
            cVar4 = *pcVar10;
            pcVar10 = pcVar10 + 1;
            *pcVar8 = cVar4;
            pcVar8 = pcVar8 + 1;
            cVar4 = *pcVar10;
        }
        *pcVar8 = '\0';
        if (DAT_0070acf0[(unsigned int)DAT_006d4988[0x16 + iVar6] * 0x566] == '\0') {
            return;
        }
        cVar4 = *param_3;
        pcVar10 = param_3;
        while (cVar4 != '\0') {
            pcVar8 = pcVar10 + 1;
            pcVar10 = pcVar10 + 1;
            cVar4 = *pcVar8;
        }
        *pcVar10 = ' ';
        pcVar10[1] = '\0';
        pcVar10 = DAT_0070acf0 + (unsigned int)DAT_006d4988[0x16 + iVar6] * 0x566;
        cVar4 = *param_3;
        while (cVar4 != '\0') {
            pcVar8 = param_3 + 1;
            param_3 = param_3 + 1;
            cVar4 = *pcVar8;
        }
        cVar4 = *pcVar10;
        while (cVar4 != '\0') {
            cVar4 = *pcVar10;
            pcVar10 = pcVar10 + 1;
            *param_3 = cVar4;
            param_3 = param_3 + 1;
            cVar4 = *pcVar10;
        }
        *param_3 = '\0';
        return;
    }
    bVar3 = iVar2[3];
    if (*pcVar10 != '\0') {
        if ((0x8e < bVar3) && (bVar3 < 0x9c)) {
            pcVar10 = DAT_0073b1b4[(unsigned int)bVar3];
            cVar4 = *pcVar10;
            while (cVar4 != '\0') {
                cVar4 = *pcVar10;
                pcVar10 = pcVar10 + 1;
                *param_3 = cVar4;
                param_3 = param_3 + 1;
                cVar4 = *pcVar10;
            }
            *param_3 = '\0';
            return;
        }
        if (bVar3 < 0x46) {
            return;
        }
        if (0x54 < bVar3) {
            return;
        }
        pcVar10 = DAT_0073b268[(unsigned int)bVar3];
        cVar4 = *pcVar10;
        while (cVar4 != '\0') {
            cVar4 = *pcVar10;
            pcVar10 = pcVar10 + 1;
            *param_3 = cVar4;
            param_3 = param_3 + 1;
            cVar4 = *pcVar10;
        }
        *param_3 = '\0';
        return;
    }
    __builtin_memcpy(&puVar5, pcVar10 + 0xa7, sizeof(puVar5));
    uVar9 = (unsigned int)iVar2[0x16];
    pcVar10 = param_3;
    if (param_2 == 1) {
        __builtin_memcpy(&pcVar8, DAT_0052b07a + (unsigned int)*((unsigned char *)puVar5 + 4) * 0xef, sizeof(pcVar8));
        cVar4 = *pcVar8;
        while (cVar4 != '\0') {
            cVar4 = *pcVar8;
            pcVar8 = pcVar8 + 1;
            *pcVar10 = cVar4;
            pcVar10 = pcVar10 + 1;
            cVar4 = *pcVar8;
        }
        *pcVar10 = '\0';
        bVar7 = 1;
    } else if (param_2 == 0) {
        pcVar8 = DAT_0052b070 + (unsigned int)*((unsigned char *)puVar5 + 4) * 0xef;
        cVar4 = DAT_0052b070[(unsigned int)*((unsigned char *)puVar5 + 4) * 0xef];
        while (cVar4 != '\0') {
            cVar4 = *pcVar8;
            pcVar8 = pcVar8 + 1;
            *pcVar10 = cVar4;
            pcVar10 = pcVar10 + 1;
            cVar4 = *pcVar8;
        }
        *pcVar10 = '\0';
        bVar7 = 1;
    }
    pcVar10 = DAT_0070acf0 + uVar9 * 0x566;
    if (DAT_0070acf0[uVar9 * 0x566] != '\0') {
        if (bVar7) {
            cVar4 = *param_3;
            pcVar8 = param_3;
            while (cVar4 != '\0') {
                pcVar1 = pcVar8 + 1;
                pcVar8 = pcVar8 + 1;
                cVar4 = *pcVar1;
            }
            *pcVar8 = ' ';
            pcVar8[1] = '\0';
        }
        cVar4 = *param_3;
        pcVar8 = param_3;
        while (cVar4 != '\0') {
            pcVar1 = pcVar8 + 1;
            pcVar8 = pcVar8 + 1;
            cVar4 = *pcVar1;
        }
        cVar4 = *pcVar10;
        while (cVar4 != '\0') {
            cVar4 = *pcVar10;
            pcVar10 = pcVar10 + 1;
            *pcVar8 = cVar4;
            pcVar8 = pcVar8 + 1;
            cVar4 = *pcVar10;
        }
        bVar7 = 1;
        *pcVar8 = '\0';
    }
    uVar9 = FUN_004ac6f0(uVar9, puVar5);
    if ((unsigned short)uVar9 == 0) {
        return;
    }
    if (bVar7) {
        cVar4 = *param_3;
        pcVar10 = param_3;
        while (cVar4 != '\0') {
            pcVar8 = pcVar10 + 1;
            pcVar10 = pcVar10 + 1;
            cVar4 = *pcVar8;
        }
        *pcVar10 = ' ';
        pcVar10[1] = '\0';
    }
    if (9 < (unsigned short)uVar9) {
        cVar4 = *param_3;
        pcVar10 = param_3;
        while (cVar4 != '\0') {
            pcVar8 = pcVar10 + 1;
            pcVar10 = pcVar10 + 1;
            cVar4 = *pcVar8;
        }
        *pcVar10 = (char)((uVar9 & 0xffff) / 10) + '0';
        pcVar10[1] = '\0';
        cVar4 = *param_3;
        while (cVar4 != '\0') {
            pcVar10 = param_3 + 1;
            param_3 = param_3 + 1;
            cVar4 = *pcVar10;
        }
        *param_3 = (char)((uVar9 & 0xffff) % 10) + '0';
        param_3[1] = '\0';
        return;
    }
    cVar4 = *param_3;
    while (cVar4 != '\0') {
        pcVar10 = param_3 + 1;
        param_3 = param_3 + 1;
        cVar4 = *pcVar10;
    }
    *param_3 = (char)uVar9 + '0';
    param_3[1] = '\0';
    return;
}
