/* SPDX-License-Identifier: MIT */
/* z_bop 004d97f0 */
extern unsigned char DAT_00aa62b0[];
extern unsigned char DAT_00aa62e0[];
extern unsigned char DAT_00aa62e4[];
extern unsigned char DAT_00aa62e8[];
extern unsigned char DAT_00aa62ec[];
extern unsigned char DAT_00aa62ed[];
extern unsigned int  DAT_00665440[8];
extern char          DAT_00bb72d0;
extern char          DAT_00bb72d1[];
extern char          DAT_00bb7311[];
extern char          DAT_00b69d20[];
extern int           DAT_0052bf50;
extern unsigned int  DAT_00aa6ae8;
extern unsigned int  DAT_00aa6cf0;
extern unsigned char *DAT_00aa6118;
extern int           DAT_00aa6cf8;
extern unsigned int  DAT_00aa6af0;
extern unsigned int  DAT_00aa6a70;
extern unsigned char DAT_00aa6a74;
extern unsigned int  DAT_00aa62a4;
extern int           DAT_00aa6114;
extern int           DAT_00aa6134;
extern int           DAT_00aa6138;
extern int           DAT_0052c208;
extern unsigned char DAT_00b6a5aa[];   /* packed int at 0xb6a5aa (unaligned) */
extern unsigned char DAT_00b6a5ae[];   /* packed array of 4-byte values (unaligned) */

void  FUN_004cd9d0(void);
int   FUN_004ccf90(void);
void  FUN_004d92b0(void);
DWORD __stdcall GetTickCount(void);
int   FUN_004d11d0(void);
void  FUN_004cef70(int param_1, unsigned int *param_2, int param_3);
int   FUN_004e0490(void);
void  FUN_004e5000(int param_1);
void  FUN_004e5590(unsigned char *param_1);

unsigned int __cdecl FUN_004d97f0(int param_1)
{
    char cVar1;
    int iVar2;
    unsigned int uVar3;
    unsigned char uVar4;
    int iVar5;
    DWORD DVar6;
    DWORD DVar7;
    unsigned int uVar8;
    unsigned int uVar9;
    int *piVar10;
    char *pcVar11;
    char *pcVar12;
    unsigned int *puVar13;
    unsigned char *pbVar14;
    unsigned int uTmp;
    unsigned int uTmp2;

    FUN_004cd9d0();
    uVar8 = 0xffffffffU;
    iVar2 = (int)((unsigned int)param_1 * 0x3eU);
    pcVar11 = (char *)(DAT_00aa62b0 + iVar2);
    do {
        pcVar12 = pcVar11;
        if (uVar8 == 0) break;
        uVar8 = uVar8 - 1;
        pcVar12 = pcVar11 + 1;
        cVar1 = *pcVar11;
        pcVar11 = pcVar12;
    } while (cVar1 != '\0');
    uVar8 = ~uVar8;
    pcVar11 = pcVar12 + -uVar8;
    pcVar12 = (char *)DAT_00665440;
    for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
        __builtin_memcpy(&uTmp, pcVar11, 4);
        __builtin_memcpy(pcVar12, &uTmp, 4);
        pcVar11 = pcVar11 + 4;
        pcVar12 = pcVar12 + 4;
    }
    for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
        *pcVar12 = *pcVar11;
        pcVar11 = pcVar11 + 1;
        pcVar12 = pcVar12 + 1;
    }
    if (DAT_00bb72d0 == '\0') {
        DAT_00b69d20[0] = 0;
    }
    else {
        if (DAT_00bb72d0 == '\x01') {
            pcVar11 = DAT_00bb7311;
        }
        else {
            if (DAT_00bb72d0 != '\x02') goto LAB_004d987f;
            pcVar11 = DAT_00bb72d1;
        }
        uVar8 = 0xffffffffU;
        do {
            pcVar12 = pcVar11;
            if (uVar8 == 0) break;
            uVar8 = uVar8 - 1;
            pcVar12 = pcVar11 + 1;
            cVar1 = *pcVar11;
            pcVar11 = pcVar12;
        } while (cVar1 != '\0');
        uVar8 = ~uVar8;
        pcVar11 = pcVar12 + -uVar8;
        pcVar12 = DAT_00b69d20;
        for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
            __builtin_memcpy(&uTmp, pcVar11, 4);
            __builtin_memcpy(pcVar12, &uTmp, 4);
            pcVar11 = pcVar11 + 4;
            pcVar12 = pcVar12 + 4;
        }
        for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
            *pcVar12 = *pcVar11;
            pcVar11 = pcVar11 + 1;
            pcVar12 = pcVar12 + 1;
        }
    }
LAB_004d987f:
    iVar5 = FUN_004ccf90();
    if (iVar5 == 0) {
        if (DAT_0052bf50 != -1) {
            DAT_0052bf50 = -1;
            DAT_00aa6ae8 = 0xffffffffU;
            DAT_00aa6cf0 = 0;
            puVar13 = DAT_00665440;
            for (iVar5 = 8; iVar5 != 0; iVar5 = iVar5 + -1) {
                *puVar13 = 0;
                puVar13 = puVar13 + 1;
            }
            pbVar14 = DAT_00aa6118;
            for (iVar5 = 0x400; iVar5 != 0; iVar5 = iVar5 + -1) {
                pbVar14[0] = 0;
                pbVar14[1] = 0;
                pbVar14[2] = 0;
                pbVar14[3] = 0;
                pbVar14 = pbVar14 + 4;
            }
            FUN_004d92b0();
        }
        uTmp = 0;
        __builtin_memcpy(DAT_00aa62e0 + iVar2, &uTmp, 4);
        DVar6 = GetTickCount();
        __builtin_memcpy(DAT_00aa62e4 + iVar2, &DVar6, 4);
        uTmp = 0;
        __builtin_memcpy(DAT_00aa62e8 + iVar2, &uTmp, 4);
        DAT_00aa62ec[iVar2] = 0;
        DAT_00aa62ed[iVar2] = 0;
        return 0;
    }
    if ((DAT_0052bf50 != -1) && (param_1 == DAT_0052bf50)) {
        DAT_00aa6cf8 = 0;
        DAT_00aa6ae8 = 0xffffffffU;
    }
    iVar5 = FUN_004d11d0();
    DAT_00aa6af0 = 0x40;
    FUN_004cef70(iVar5, &DAT_00aa6af0, 4);
    if ((DAT_0052bf50 != -1) && (param_1 == DAT_0052bf50)) {
        DAT_00aa6af0 = 0x6c;
        FUN_004cef70(iVar5, &DAT_00aa6af0, 4);
    }
    DAT_00aa6a70 = 0;
    DAT_00aa6cf0 = 0;
    DVar6 = GetTickCount();
    do {
        iVar5 = FUN_004e0490();
        DVar7 = GetTickCount();
        if (iVar5 == 0x3a) goto LAB_004d9ad8;
        if (iVar5 == 0x72) break;
        if (iVar5 == 0x41) {
            if ((DAT_0052bf50 == -1) || (param_1 != DAT_0052bf50)) {
                DAT_00aa6ae8 = 0xffffffffU;
            }
            else {
                __builtin_memcpy(DAT_00b6a5aa, &DAT_00aa6cf8, 4);
                __builtin_memcpy(DAT_00b6a5ae + (unsigned int)DAT_00aa6cf8 * 4U, &DAT_00aa6ae8, 4);
                __builtin_memcpy(&iVar5, DAT_00b6a5aa, 4);
                FUN_004e5000(iVar5);
                FUN_004e5590(DAT_00aa6118);
                if ((DAT_00aa6114 != 0) && (DAT_00aa6134 = 0, DAT_00aa6138 != 0)) {
                    piVar10 = (int *)(DAT_00aa6114 + 0x140);
                    do {
                        __builtin_memcpy(&uTmp2, DAT_00b6a5aa, 4);
                        __builtin_memcpy(&uTmp, DAT_00b6a5ae + uTmp2 * 4U, 4);
                        if (*piVar10 == (int)uTmp) break;
                        piVar10 = piVar10 + 0x52;
                        DAT_00aa6134 = DAT_00aa6134 + 1;
                    } while (DAT_00aa6134 < DAT_00aa6138);
                }
                DAT_0052c208 = 0;
                iVar5 = 0x41;
            }
        }
    } while (DVar7 - DVar6 < 2000);
    if (iVar5 == 0x3a) {
LAB_004d9ad8:
        DVar6 = GetTickCount();
        uVar4 = DAT_00aa6a74;
        uVar3 = DAT_00aa62a4;
        __builtin_memcpy(DAT_00aa62e4 + iVar2, &DVar6, 4);
        __builtin_memcpy(DAT_00aa62e0 + iVar2, &DAT_00aa6a70, 4);
        __builtin_memcpy(DAT_00aa62e8 + iVar2, &uVar3, 4);
        DAT_00aa62ec[iVar2] = uVar4;
        DAT_00aa62ed[iVar2] = 1;
    }
    else if (iVar5 == 0x72) {
        DVar6 = GetTickCount();
        uVar4 = DAT_00aa6a74;
        uVar3 = DAT_00aa62a4;
        __builtin_memcpy(DAT_00aa62e4 + iVar2, &DVar6, 4);
        __builtin_memcpy(DAT_00aa62e0 + iVar2, &DAT_00aa6a70, 4);
        __builtin_memcpy(DAT_00aa62e8 + iVar2, &uVar3, 4);
        DAT_00aa62ec[iVar2] = uVar4;
        DAT_00aa62ed[iVar2] = 0;
    }
    else {
        DVar6 = GetTickCount();
        __builtin_memcpy(DAT_00aa62e4 + iVar2, &DVar6, 4);
        uTmp = 0;
        __builtin_memcpy(DAT_00aa62e0 + iVar2, &uTmp, 4);
        __builtin_memcpy(DAT_00aa62e8 + iVar2, &uTmp, 4);
        DAT_00aa62ec[iVar2] = 0;
        DAT_00aa62ed[iVar2] = 0;
    }
    return 1;
}
