/* SPDX-License-Identifier: MIT */
/* z_xvt 004d8a70 */
/* Prototypes */
void FUN_004ca200(void);
int FUN_004c96a0(int, int, int, int, char *, char *, int, char *, unsigned int, int);
void FUN_004d8520(void);
int FUN_004cdbb0(void);
void FUN_004cb830(int, unsigned int *, int);
int FUN_004e0750(void);
void FUN_004e43d0(int);
void FUN_004e4820(unsigned char *);

/* Globals (types inferred from the decompile) */
extern unsigned char DAT_00757e40[];
extern char DAT_0064f658[];
extern unsigned char DAT_0085e6f0;
extern char DAT_0085e731[];
extern char DAT_0085e6f1[];
extern char DAT_0081b740[];
extern unsigned char DAT_0081e016;
extern int DAT_00520278;
extern int DAT_0052027c;
extern int DAT_00520280;
extern int DAT_00520284;
extern char DAT_0081bcf0[];
extern int DAT_00532a34;
extern unsigned int DAT_00758618;
extern int DAT_00758820;
extern int DAT_00758828;
extern unsigned char *DAT_00757cc4;
extern int DAT_00757cc0;
extern int DAT_00757cc8;
extern int DAT_00757ccc;
extern int DAT_005330fc;
extern unsigned int DAT_00758620;
extern int DAT_007585a0;
extern unsigned char DAT_007585a4;
extern unsigned char DAT_00757e34;
extern unsigned char DAT_0081bfba[];
extern unsigned char DAT_0081bfbe[];
extern unsigned char DAT_00757e70[];
extern unsigned char DAT_00757e74[];
extern unsigned char DAT_00757e78[];
extern unsigned char DAT_00757e79[];
extern unsigned char DAT_00757e7a[];

unsigned int __cdecl FUN_004d8a70(int param_1)
{
    char cVar1;
    unsigned char uVar2;
    unsigned char uVar3;
    int iVar4;
    DWORD DVar5;
    int iVar6;
    unsigned int uVar7;
    unsigned int uVar8;
    int *piVar9;
    char *pcVar10;
    char *pcVar11;
    unsigned int *puVar12;
    unsigned char *pbVar13;
    char local_a[2];
    int local_8;
    DWORD local_4;
    int iTmp;
    int iCmp;
    unsigned int uTmp;

    FUN_004ca200();
    iVar6 = (int)((unsigned int)param_1 * 0x3bU);
    uVar7 = 0xffffffffU;
    pcVar10 = (char *)(DAT_00757e40 + iVar6);
    do {
        pcVar11 = pcVar10;
        if (uVar7 == 0) break;
        uVar7 = uVar7 - 1;
        pcVar11 = pcVar10 + 1;
        cVar1 = *pcVar10;
        pcVar10 = pcVar11;
    } while (cVar1 != '\0');
    uVar7 = ~uVar7;
    pcVar10 = pcVar11 + -uVar7;
    pcVar11 = DAT_0064f658;
    for (uVar8 = uVar7 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
        __builtin_memcpy(pcVar11, pcVar10, 4);
        pcVar10 = pcVar10 + 4;
        pcVar11 = pcVar11 + 4;
    }
    for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
        *pcVar11 = *pcVar10;
        pcVar10 = pcVar10 + 1;
        pcVar11 = pcVar11 + 1;
    }
    if (DAT_0085e6f0 == 0) {
        DAT_0081b740[0] = 0;
    }
    else {
        if (DAT_0085e6f0 == 1) {
            pcVar10 = DAT_0085e731;
        }
        else {
            if (DAT_0085e6f0 != 2) goto LAB_004d8b08;
            pcVar10 = DAT_0085e6f1;
        }
        uVar7 = 0xffffffffU;
        do {
            pcVar11 = pcVar10;
            if (uVar7 == 0) break;
            uVar7 = uVar7 - 1;
            pcVar11 = pcVar10 + 1;
            cVar1 = *pcVar10;
            pcVar10 = pcVar11;
        } while (cVar1 != '\0');
        uVar7 = ~uVar7;
        pcVar10 = pcVar11 + -uVar7;
        pcVar11 = DAT_0081b740;
        for (uVar8 = uVar7 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
            __builtin_memcpy(pcVar11, pcVar10, 4);
            pcVar10 = pcVar10 + 4;
            pcVar11 = pcVar11 + 4;
        }
        for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
            *pcVar11 = *pcVar10;
            pcVar10 = pcVar10 + 1;
            pcVar11 = pcVar11 + 1;
        }
    }
LAB_004d8b08:
    local_a[0] = (char)DAT_0081e016 + '\x01';
    local_a[1] = 0;
    iVar4 = FUN_004c96a0(DAT_00520278, DAT_0052027c, DAT_00520280, DAT_00520284, local_a,
                         DAT_0081bcf0, 0, DAT_0064f658, (unsigned int)DAT_0085e6f0, 0);
    if (iVar4 == 0) {
        if (DAT_00532a34 != -1) {
            DAT_00532a34 = -1;
            DAT_00758618 = 0xffffffffU;
            DAT_00758820 = 0;
            puVar12 = (unsigned int *)DAT_0064f658;
            for (iVar4 = 8; iVar4 != 0; iVar4 = iVar4 + -1) {
                *puVar12 = 0;
                puVar12 = puVar12 + 1;
            }
            pbVar13 = DAT_00757cc4;
            for (iVar4 = 0x100; iVar4 != 0; iVar4 = iVar4 + -1) {
                pbVar13[0] = 0;
                pbVar13[1] = 0;
                pbVar13[2] = 0;
                pbVar13[3] = 0;
                pbVar13 = pbVar13 + 4;
            }
            FUN_004d8520();
        }
        iTmp = 0;
        __builtin_memcpy(DAT_00757e70 + iVar6, &iTmp, 4);
        DVar5 = GetTickCount();
        __builtin_memcpy(DAT_00757e74 + iVar6, &DVar5, 4);
        DAT_00757e78[iVar6] = 0;
        DAT_00757e79[iVar6] = 0;
        DAT_00757e7a[iVar6] = 0;
        return 0;
    }
    if ((DAT_00532a34 != -1) && (param_1 == DAT_00532a34)) {
        DAT_00758828 = 0;
        DAT_00758618 = 0xffffffffU;
    }
    iVar4 = FUN_004cdbb0();
    DAT_00758620 = 0x40;
    FUN_004cb830(iVar4, &DAT_00758620, 4);
    if ((DAT_00532a34 != -1) && (param_1 == DAT_00532a34)) {
        DAT_00758620 = 0x6c;
        FUN_004cb830(iVar4, &DAT_00758620, 4);
    }
    DAT_007585a0 = 0;
    DAT_00758820 = 0;
    local_4 = GetTickCount();
    local_8 = iVar6;
    do {
        iVar6 = FUN_004e0750();
        DVar5 = GetTickCount();
        if ((iVar6 == 0x3a) || (iVar6 == 0x72)) break;
        if (iVar6 == 0x41) {
            if ((DAT_00532a34 == -1) || (param_1 != DAT_00532a34)) {
                DAT_00758618 = 0xffffffffU;
            }
            else {
                iTmp = DAT_00758828;
                __builtin_memcpy(DAT_0081bfba, &iTmp, 4);
                uTmp = DAT_00758618;
                __builtin_memcpy(DAT_0081bfbe + (unsigned int)DAT_00758828 * 4U, &uTmp, 4);
                FUN_004e43d0(iTmp);
                FUN_004e4820(DAT_00757cc4);
                if ((DAT_00757cc0 != 0) && (DAT_00757cc8 = 0, DAT_00757ccc != 0)) {
                    piVar9 = (int *)(DAT_00757cc0 + 0x140);
                    do {
                        __builtin_memcpy(&iTmp, DAT_0081bfba, 4);
                        __builtin_memcpy(&iCmp, DAT_0081bfbe + (unsigned int)iTmp * 4U, 4);
                        if (*piVar9 == iCmp) break;
                        piVar9 = piVar9 + 0x51;
                        DAT_00757cc8 = (int)((unsigned int)DAT_00757cc8 + 1U);
                    } while (DAT_00757cc8 < DAT_00757ccc);
                }
                DAT_005330fc = 0;
            }
        }
    } while (DVar5 - local_4 < 2000);
    iVar4 = local_8;
    if (iVar6 == 0x3a) {
        DVar5 = GetTickCount();
        uVar3 = DAT_007585a4;
        uVar2 = DAT_00757e34;
        __builtin_memcpy(DAT_00757e74 + iVar4, &DVar5, 4);
        iTmp = DAT_007585a0;
        __builtin_memcpy(DAT_00757e70 + iVar4, &iTmp, 4);
        DAT_00757e78[iVar4] = uVar2;
        DAT_00757e79[iVar4] = uVar3;
        DAT_00757e7a[iVar4] = 1;
    }
    else if (iVar6 == 0x72) {
        DVar5 = GetTickCount();
        uVar3 = DAT_007585a4;
        uVar2 = DAT_00757e34;
        __builtin_memcpy(DAT_00757e74 + iVar4, &DVar5, 4);
        iTmp = DAT_007585a0;
        __builtin_memcpy(DAT_00757e70 + iVar4, &iTmp, 4);
        DAT_00757e78[iVar4] = uVar2;
        DAT_00757e79[iVar4] = uVar3;
        DAT_00757e7a[iVar4] = 0;
    }
    else {
        DVar5 = GetTickCount();
        __builtin_memcpy(DAT_00757e74 + iVar4, &DVar5, 4);
        iTmp = 0;
        __builtin_memcpy(DAT_00757e70 + iVar4, &iTmp, 4);
        DAT_00757e78[iVar4] = 0;
        DAT_00757e79[iVar4] = 0;
        DAT_00757e7a[iVar4] = 0;
    }
    return 1;
}
