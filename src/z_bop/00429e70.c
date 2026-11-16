/* SPDX-License-Identifier: MIT */
/* z_bop 00429e70 */
extern void FUN_0042a350(void);
extern void FUN_0042af90(void);
extern unsigned int FUN_0049e720(char *param_1, char *param_2, int param_3, int param_4);
extern int __cdecl _fscanf(void *param_1, const char *param_2, ...);
extern int __cdecl _fclose(void *param_1);
extern int __cdecl __strnicmp(const char *param_1, const char *param_2, unsigned int param_3);
extern void FUN_004ac6e0(unsigned int param_1);
extern short FUN_00412040(void *param_1);
extern short FUN_00412030(void);
extern short FUN_00477950(short param_1);
extern void FUN_004766c0(char *param_1, short param_2);
extern unsigned int FUN_004742a0(char *param_1);
extern short FUN_004762f0(short param_1);
extern int *FUN_004ac720(short param_1);
extern void FUN_00471ff0(int *param_1);
extern long double FUN_0042ae30(int param_1, int param_2);
/* the two dwords at 0x5561e0/0x5561e4 are passed as one 8-byte value */
extern void FUN_0042ac50(int param_1, double param_2);
/* stack-check: original pushes 12 bytes and cleans them (cdecl); the callee
   (per its decompile) ignores them, so the pushed values are unrecovered */
extern void __cdecl FUN_0042b010(int param_1, int param_2, int param_3);

extern int DAT_005233ac;
extern int DAT_005233d8;
extern int *DAT_00520ec4;
extern short DAT_009a7ed0;
extern void *DAT_00a07bd8;
extern char DAT_00520ee0[];
extern char DAT_0051a854[];
extern char DAT_0051bd9c[];
extern char DAT_00555cc8[];
extern double _DAT_00520ed8;
extern double _DAT_005561e0;
extern double _DAT_005180f8;
extern unsigned char DAT_009ec476;
extern int DAT_00520ec8;
extern unsigned int DAT_00556218[];
extern unsigned int DAT_00555d78[];
extern unsigned char DAT_005561eb;
extern int DAT_005561ec;
extern short DAT_005561fa;
extern int DAT_005561f0;
extern short DAT_005561f8;
extern int DAT_005561f4;
extern short DAT_005561fc;
extern short DAT_00520ec0;
/* unaligned pointer globals, accessed via __builtin_memcpy */
extern unsigned char DAT_00556207[4];
extern unsigned char _DAT_005562c1[4];

unsigned int __cdecl FUN_00429e70(char *param_1)
{
    char cVar1;
    void *_File;
    short uVar2;
    short sVar3;
    char *pcVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    int iVar7;
    char *pcVar8;
    char *pcVar9;
    unsigned int *puVar10;
    long double fVar11;
    void *pvTmp;
    char local_200[256];
    char local_100[256];

    FUN_0042a350();
    FUN_0042af90();
    DAT_005233ac = 1;
    if (DAT_00520ec4 == (int *)0) {
        DAT_009a7ed0 = 0;
    }
    uVar5 = 0xffffffff;
    do {
        pcVar4 = param_1;
        if (uVar5 == 0) break;
        uVar5 = uVar5 - 1;
        pcVar4 = param_1 + 1;
        cVar1 = *param_1;
        param_1 = pcVar4;
    } while (cVar1 != '\0');
    uVar5 = ~uVar5;
    pcVar4 = pcVar4 + -uVar5;
    pcVar9 = local_100;
    for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        *(unsigned int *)pcVar9 = *(unsigned int *)pcVar4;
        pcVar4 = pcVar4 + 4;
        pcVar9 = pcVar9 + 4;
    }
    for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
        *pcVar9 = *pcVar4;
        pcVar4 = pcVar4 + 1;
        pcVar9 = pcVar9 + 1;
    }
    pcVar4 = local_100;
    /* Ghidra's write to local_100[0] here is a register-alias artifact */
    while (*pcVar4 != '.') {
        pcVar4 = pcVar4 + 1;
    }
    uVar5 = 0xffffffff;
    *pcVar4 = '\0';
    pcVar4 = local_100;
    do {
        pcVar9 = pcVar4;
        if (uVar5 == 0) break;
        uVar5 = uVar5 - 1;
        pcVar9 = pcVar4 + 1;
        cVar1 = *pcVar4;
        pcVar4 = pcVar9;
    } while (cVar1 != '\0');
    uVar5 = ~uVar5;
    pcVar4 = pcVar9 + -uVar5;
    pcVar9 = local_200;
    for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        *(unsigned int *)pcVar9 = *(unsigned int *)pcVar4;
        pcVar4 = pcVar4 + 4;
        pcVar9 = pcVar9 + 4;
    }
    for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
        *pcVar9 = *pcVar4;
        pcVar4 = pcVar4 + 1;
        pcVar9 = pcVar9 + 1;
    }
    uVar5 = 0xffffffff;
    pcVar4 = DAT_00520ee0;
    do {
        pcVar9 = pcVar4;
        if (uVar5 == 0) break;
        uVar5 = uVar5 - 1;
        pcVar9 = pcVar4 + 1;
        cVar1 = *pcVar4;
        pcVar4 = pcVar9;
    } while (cVar1 != '\0');
    uVar5 = ~uVar5;
    iVar7 = -1;
    pcVar4 = local_200;
    do {
        pcVar8 = pcVar4;
        if (iVar7 == 0) break;
        iVar7 = iVar7 + -1;
        pcVar8 = pcVar4 + 1;
        cVar1 = *pcVar4;
        pcVar4 = pcVar8;
    } while (cVar1 != '\0');
    pcVar4 = pcVar9 + -uVar5;
    pcVar9 = pcVar8 + -1;
    for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        *(unsigned int *)pcVar9 = *(unsigned int *)pcVar4;
        pcVar4 = pcVar4 + 4;
        pcVar9 = pcVar9 + 4;
    }
    for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
        *pcVar9 = *pcVar4;
        pcVar4 = pcVar4 + 1;
        pcVar9 = pcVar9 + 1;
    }
    FUN_0049e720(local_200, DAT_0051a854, 0, 0);
    if (DAT_00a07bd8 == (void *)0) {
        uVar5 = 0xffffffff;
        pcVar4 = local_100;
        do {
            pcVar9 = pcVar4;
            if (uVar5 == 0) break;
            uVar5 = uVar5 - 1;
            pcVar9 = pcVar4 + 1;
            cVar1 = *pcVar4;
            pcVar4 = pcVar9;
        } while (cVar1 != '\0');
        uVar5 = ~uVar5;
        pcVar4 = pcVar9 + -uVar5;
        pcVar9 = local_200;
        for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
            *(unsigned int *)pcVar9 = *(unsigned int *)pcVar4;
            pcVar4 = pcVar4 + 4;
            pcVar9 = pcVar9 + 4;
        }
        for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
            *pcVar9 = *pcVar4;
            pcVar4 = pcVar4 + 1;
            pcVar9 = pcVar9 + 1;
        }
        uVar5 = 0xffffffff;
        pcVar4 = DAT_0051bd9c;
        do {
            pcVar9 = pcVar4;
            if (uVar5 == 0) break;
            uVar5 = uVar5 - 1;
            pcVar9 = pcVar4 + 1;
            cVar1 = *pcVar4;
            pcVar4 = pcVar9;
        } while (cVar1 != '\0');
        uVar5 = ~uVar5;
        iVar7 = -1;
        pcVar4 = local_200;
        do {
            pcVar8 = pcVar4;
            if (iVar7 == 0) break;
            iVar7 = iVar7 + -1;
            pcVar8 = pcVar4 + 1;
            cVar1 = *pcVar4;
            pcVar4 = pcVar8;
        } while (cVar1 != '\0');
        pcVar4 = pcVar9 + -uVar5;
        pcVar9 = pcVar8 + -1;
        for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
            *(unsigned int *)pcVar9 = *(unsigned int *)pcVar4;
            pcVar4 = pcVar4 + 4;
            pcVar9 = pcVar9 + 4;
        }
        for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
            *pcVar9 = *pcVar4;
            pcVar4 = pcVar4 + 1;
            pcVar9 = pcVar9 + 1;
        }
        FUN_0049e720(local_200, DAT_0051a854, 0, 0);
        _File = DAT_00a07bd8;
        if (DAT_00a07bd8 == (void *)0) {
            return 0;
        }
        iVar7 = _fscanf(DAT_00a07bd8, "%256s", local_200);
        if (iVar7 != 1) {
            return 0;
        }
        iVar7 = __strnicmp(local_200, "#inventor", 9);
        if (iVar7 != 0) {
            return 0;
        }
        iVar7 = _fscanf(_File, " %256s", local_200);
        if (iVar7 != 1) {
            return 0;
        }
        iVar7 = _fscanf(_File, " %256s", local_200);
        if (iVar7 != 1) {
            return 0;
        }
        iVar7 = __strnicmp(local_200, "ascii", 5);
        if (iVar7 == 0) {
            if (DAT_009a7ed0 != 0) {
                FUN_004ac6e0((unsigned int)DAT_009a7ed0);
            }
            uVar2 = FUN_00412040(_File);
        } else {
            iVar7 = __strnicmp(local_200, "binary", 6);
            if (iVar7 != 0) {
                _fclose(_File);
                return 0;
            }
            if (DAT_009a7ed0 != 0) {
                FUN_004ac6e0((unsigned int)DAT_009a7ed0);
            }
            uVar2 = FUN_00412030();
        }
        _fclose(_File);
        sVar3 = FUN_00477950(uVar2);
        uVar5 = 0xffffffff;
        pcVar4 = local_100;
        do {
            pcVar9 = pcVar4;
            if (uVar5 == 0) break;
            uVar5 = uVar5 - 1;
            pcVar9 = pcVar4 + 1;
            cVar1 = *pcVar4;
            pcVar4 = pcVar9;
        } while (cVar1 != '\0');
        uVar5 = ~uVar5;
        pcVar4 = pcVar9 + -uVar5;
        pcVar9 = local_200;
        for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
            *(unsigned int *)pcVar9 = *(unsigned int *)pcVar4;
            pcVar4 = pcVar4 + 4;
            pcVar9 = pcVar9 + 4;
        }
        for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
            *pcVar9 = *pcVar4;
            pcVar4 = pcVar4 + 1;
            pcVar9 = pcVar9 + 1;
        }
        uVar5 = 0xffffffff;
        pcVar4 = DAT_00520ee0;
        do {
            pcVar9 = pcVar4;
            if (uVar5 == 0) break;
            uVar5 = uVar5 - 1;
            pcVar9 = pcVar4 + 1;
            cVar1 = *pcVar4;
            pcVar4 = pcVar9;
        } while (cVar1 != '\0');
        uVar5 = ~uVar5;
        iVar7 = -1;
        pcVar4 = local_200;
        do {
            pcVar8 = pcVar4;
            if (iVar7 == 0) break;
            iVar7 = iVar7 + -1;
            pcVar8 = pcVar4 + 1;
            cVar1 = *pcVar4;
            pcVar4 = pcVar8;
        } while (cVar1 != '\0');
        pcVar4 = pcVar9 + -uVar5;
        pcVar9 = pcVar8 + -1;
        for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
            *(unsigned int *)pcVar9 = *(unsigned int *)pcVar4;
            pcVar4 = pcVar4 + 4;
            pcVar9 = pcVar9 + 4;
        }
        for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
            *pcVar9 = *pcVar4;
            pcVar4 = pcVar4 + 1;
            pcVar9 = pcVar9 + 1;
        }
        FUN_004766c0(local_200, sVar3);
        DAT_009a7ed0 = sVar3;
    } else {
        _fclose(DAT_00a07bd8);
        if (DAT_009a7ed0 != 0) {
            FUN_004ac6e0((unsigned int)DAT_009a7ed0);
        }
        uVar5 = FUN_004742a0(local_200);
        DAT_009a7ed0 = (short)(unsigned short)uVar5;
    }
    uVar5 = 0xffffffff;
    pcVar4 = local_100;
    do {
        pcVar9 = pcVar4;
        if (uVar5 == 0) break;
        uVar5 = uVar5 - 1;
        pcVar9 = pcVar4 + 1;
        cVar1 = *pcVar4;
        pcVar4 = pcVar9;
    } while (cVar1 != '\0');
    uVar5 = ~uVar5;
    pcVar4 = pcVar9 + -uVar5;
    pcVar9 = DAT_00555cc8;
    for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        *(unsigned int *)pcVar9 = *(unsigned int *)pcVar4;
        pcVar4 = pcVar4 + 4;
        pcVar9 = pcVar9 + 4;
    }
    for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
        *pcVar9 = *pcVar4;
        pcVar4 = pcVar4 + 1;
        pcVar9 = pcVar9 + 1;
    }
    uVar5 = 0xffffffff;
    pcVar4 = DAT_00520ee0;
    do {
        pcVar9 = pcVar4;
        if (uVar5 == 0) break;
        uVar5 = uVar5 - 1;
        pcVar9 = pcVar4 + 1;
        cVar1 = *pcVar4;
        pcVar4 = pcVar9;
    } while (cVar1 != '\0');
    uVar5 = ~uVar5;
    iVar7 = -1;
    pcVar4 = DAT_00555cc8;
    do {
        pcVar8 = pcVar4;
        if (iVar7 == 0) break;
        iVar7 = iVar7 + -1;
        pcVar8 = pcVar4 + 1;
        cVar1 = *pcVar4;
        pcVar4 = pcVar8;
    } while (cVar1 != '\0');
    pcVar4 = pcVar9 + -uVar5;
    pcVar9 = pcVar8 + -1;
    for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        *(unsigned int *)pcVar9 = *(unsigned int *)pcVar4;
        pcVar4 = pcVar4 + 4;
        pcVar9 = pcVar9 + 4;
    }
    for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
        *pcVar9 = *pcVar4;
        pcVar4 = pcVar4 + 1;
        pcVar9 = pcVar9 + 1;
    }
    if (DAT_005233ac != 0) {
        DAT_005233d8 = 2;
        DAT_009a7ed0 = FUN_004762f0(DAT_009a7ed0);
        DAT_005233d8 = 2;
    }
    DAT_00520ec4 = FUN_004ac720(DAT_009a7ed0);
    if ((int *)*DAT_00520ec4 != DAT_00520ec4) {
        FUN_00471ff0(DAT_00520ec4);
    }
    fVar11 = FUN_0042ae30((int)DAT_00520ec4, 0);
    _DAT_00520ed8 = (double)fVar11;
    _DAT_005561e0 = (double)((long double)_DAT_005180f8 / fVar11);
    FUN_0042ac50((int)DAT_00520ec4, _DAT_005561e0);
    DAT_009ec476 = 1;
    if (DAT_00520ec8 == 0) {
        puVar10 = DAT_00556218;
        for (iVar7 = 0x2c; iVar7 != 0; iVar7 = iVar7 + -1) {
            *puVar10 = 0;
            puVar10 = puVar10 + 1;
        }
        *(unsigned char *)puVar10 = 0;
        puVar10 = DAT_00555d78;
        for (iVar7 = 0x118; iVar7 != 0; iVar7 = iVar7 + -1) {
            *puVar10 = 0;
            puVar10 = puVar10 + 1;
        }
        *(unsigned short *)puVar10 = 0;
        DAT_005561eb = 0;
        DAT_005561ec = 0;
        DAT_005561fa = 0;
        DAT_005561f0 = 0;
        DAT_005561f8 = 0;
        DAT_005561f4 = 0;
        pvTmp = (void *)DAT_00556218;
        __builtin_memcpy(DAT_00556207, &pvTmp, 4);
        pvTmp = (void *)DAT_00555d78;
        __builtin_memcpy(_DAT_005562c1, &pvTmp, 4);
        DAT_005561fc = 0;
        DAT_00520ec0 = 0;
        FUN_0042af90();
        FUN_0042b010(0, 0, 0);
    }
    return 1;
}
