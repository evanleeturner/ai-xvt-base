/* SPDX-License-Identifier: MIT */
/* z_xvt 004526c0 */
typedef struct _iobuf FILE;

extern int   DAT_00525e14;
extern int   DAT_00525e10;
extern int   DAT_0071ecf4;
extern unsigned char DAT_0071bb1b[];
extern int   DAT_00709ffc;
extern unsigned int DAT_005254e8;
extern unsigned int DAT_005254f0;
extern int   DAT_005254f8;
extern int   DAT_005254fc;
extern int   DAT_00525504;
extern int   DAT_00525500;
extern int   DAT_005254f4;
extern int   DAT_005254ec;
extern int   DAT_00525518;
extern int  *DAT_00525e0c;
extern unsigned short DAT_006da850;
extern char  DAT_00525e28[];
extern char  DAT_0052356c[];
extern char  DAT_00523570[];
extern FILE *DAT_00732948;
extern char  DAT_005f0928[];
extern double DAT_00525e20;
extern double DAT_005f0e18;
extern double DAT_00520148;
extern int   DAT_0071e536;
extern unsigned int DAT_005f0e50[];
extern unsigned int DAT_005f09d8[];
extern unsigned char DAT_005f0e23;
extern int   DAT_005f0e24;
extern unsigned short DAT_005f0e32;
extern int   DAT_005f0e28;
extern unsigned short DAT_005f0e30;
extern int   DAT_005f0e2c;
extern unsigned short DAT_005f0e34;
extern unsigned short DAT_00525e08;
extern unsigned char DAT_005f0ef7[];
extern unsigned char DAT_005f0e3f[];
extern int   DAT_00703c6c;
extern int   DAT_00703c70;
extern int   DAT_00703c84;

extern const char s__256s_00523564[];
extern const char s__inventor_00523558[];
extern const char s__256s_00523550[];
extern const char s_ascii_00523548[];
extern const char s_binary_00523540[];

void FUN_00465d20(void);
void FUN_0047ed40(char *, char *, int, int);
int  _fscanf(FILE *, const char *, ...);
int  __strnicmp(const char *, const char *, unsigned int);
void FUN_0048f4b0(unsigned int);
unsigned short FUN_00402a80(FILE *);
unsigned short FUN_00402a70(void);
int  _fclose(FILE *);
unsigned short FUN_0048be40(unsigned int);
void FUN_0048aa90(char *, unsigned int);
unsigned short FUN_00488490(char *);
unsigned int FUN_0048a570(unsigned int);
int *FUN_0048f4f0(unsigned int);
void FUN_00485fa0(int *);
long double FUN_004537e0(int, int);
void FUN_004532d0(unsigned int *, int, unsigned int, unsigned int);
long long __ftol(void);

unsigned int __cdecl FUN_004526c0(char *param_1)
{
    int *piVar1;
    int *piVar4;
    FILE *_File;
    unsigned short uVar5;
    unsigned short uVar6;
    char *pcVar7;
    char cVar3;
    unsigned int uVar8;
    unsigned int uVar9;
    int iVar10;
    int iVar11;
    char *pcVar12;
    char *pcVar13;
    unsigned int *puVar14;
    long double fVar15;
    long long lVar16;
    long long lVar17;
    double dVar2;
    unsigned int uStack_208;
    unsigned int uStack_204;
    char acStack_200[256];
    char acStack_100[256];
    unsigned int zero32 = 0;
    unsigned short zero16 = 0;
    unsigned int val32;
    unsigned short val16;
    int cnt;
    unsigned int arrp;
    unsigned int ret48a570;
    unsigned int tmp4;

    if (DAT_00525e14 != 0) {
        FUN_00465d20();
        DAT_00525e14 = 0;
    }
    iVar10 = DAT_0071ecf4;
    iVar11 = (int)((unsigned int)DAT_0071ecf4 * 5u);
    DAT_00525e10 = 0;
    __builtin_memcpy(&DAT_0071bb1b[(unsigned int)DAT_0071ecf4 * 0x5bdu], &zero32, 4);
    iVar10 = (int)((unsigned int)iVar10 * 0x5bdu);
    DAT_00709ffc = 0;
    val32 = 0xfffffb00;
    __builtin_memcpy(&DAT_0071bb1b[iVar10 + 4], &val32, 4);
    __builtin_memcpy(&DAT_0071bb1b[iVar10 + 8], &zero32, 4);
    __builtin_memcpy(&DAT_0071bb1b[iVar10 + 0x14], &zero16, 2);
    val16 = 0x4000;
    __builtin_memcpy(&DAT_0071bb1b[iVar10 + 0x10], &val16, 2);
    __builtin_memcpy(&DAT_0071bb1b[iVar10 + 0x12], &zero16, 2);
    DAT_005254e8 = 0x3f800000;
    DAT_005254f0 = 0x3f800000;
    DAT_005254f8 = 1;
    DAT_005254fc = 1;
    DAT_00525504 = 1;
    DAT_00525500 = 1;
    DAT_005254f4 = 1;
    DAT_005254ec = 1;
    if (DAT_00525e0c == (int *)0) {
        DAT_006da850 = 0;
    }

    uVar8 = 0xffffffff;
    do {
        pcVar7 = param_1;
        if (uVar8 == 0) break;
        uVar8 = uVar8 - 1;
        pcVar7 = param_1 + 1;
        cVar3 = *param_1;
        param_1 = pcVar7;
    } while (cVar3 != '\0');
    uVar8 = ~uVar8;
    pcVar7 = pcVar7 + -uVar8;
    pcVar13 = acStack_100;
    for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
        __builtin_memcpy(&tmp4, pcVar7, 4);
        __builtin_memcpy(pcVar13, &tmp4, 4);
        pcVar7 = pcVar7 + 4;
        pcVar13 = pcVar13 + 4;
    }
    for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
        *pcVar13 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar13 = pcVar13 + 1;
    }
    pcVar7 = acStack_100;
    while (acStack_100[0] != '.') {
        pcVar13 = pcVar7 + 1;
        pcVar7 = pcVar7 + 1;
        acStack_100[0] = *pcVar13;
    }
    uVar8 = 0xffffffff;
    *pcVar7 = '\0';
    pcVar7 = acStack_100;
    do {
        pcVar13 = pcVar7;
        if (uVar8 == 0) break;
        uVar8 = uVar8 - 1;
        pcVar13 = pcVar7 + 1;
        cVar3 = *pcVar7;
        pcVar7 = pcVar13;
    } while (cVar3 != '\0');
    uVar8 = ~uVar8;
    pcVar7 = pcVar13 + -uVar8;
    pcVar13 = acStack_200;
    for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
        __builtin_memcpy(&tmp4, pcVar7, 4);
        __builtin_memcpy(pcVar13, &tmp4, 4);
        pcVar7 = pcVar7 + 4;
        pcVar13 = pcVar13 + 4;
    }
    for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
        *pcVar13 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar13 = pcVar13 + 1;
    }
    uVar8 = 0xffffffff;
    pcVar7 = DAT_00525e28;
    do {
        pcVar13 = pcVar7;
        if (uVar8 == 0) break;
        uVar8 = uVar8 - 1;
        pcVar13 = pcVar7 + 1;
        cVar3 = *pcVar7;
        pcVar7 = pcVar13;
    } while (cVar3 != '\0');
    uVar8 = ~uVar8;
    iVar10 = -1;
    pcVar7 = acStack_200;
    do {
        pcVar12 = pcVar7;
        if (iVar10 == 0) break;
        iVar10 = iVar10 + -1;
        pcVar12 = pcVar7 + 1;
        cVar3 = *pcVar7;
        pcVar7 = pcVar12;
    } while (cVar3 != '\0');
    pcVar7 = pcVar13 + -uVar8;
    pcVar13 = pcVar12 + -1;
    for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
        __builtin_memcpy(&tmp4, pcVar7, 4);
        __builtin_memcpy(pcVar13, &tmp4, 4);
        pcVar7 = pcVar7 + 4;
        pcVar13 = pcVar13 + 4;
    }
    for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
        *pcVar13 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar13 = pcVar13 + 1;
    }
    FUN_0047ed40(acStack_200, DAT_00523570, 0, 0);

    if (DAT_00732948 == (FILE *)0) {
        uVar8 = 0xffffffff;
        pcVar7 = acStack_100;
        do {
            pcVar13 = pcVar7;
            if (uVar8 == 0) break;
            uVar8 = uVar8 - 1;
            pcVar13 = pcVar7 + 1;
            cVar3 = *pcVar7;
            pcVar7 = pcVar13;
        } while (cVar3 != '\0');
        uVar8 = ~uVar8;
        pcVar7 = pcVar13 + -uVar8;
        pcVar13 = acStack_200;
        for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
            __builtin_memcpy(&tmp4, pcVar7, 4);
            __builtin_memcpy(pcVar13, &tmp4, 4);
            pcVar7 = pcVar7 + 4;
            pcVar13 = pcVar13 + 4;
        }
        for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
            *pcVar13 = *pcVar7;
            pcVar7 = pcVar7 + 1;
            pcVar13 = pcVar13 + 1;
        }
        uVar8 = 0xffffffff;
        pcVar7 = DAT_0052356c;
        do {
            pcVar13 = pcVar7;
            if (uVar8 == 0) break;
            uVar8 = uVar8 - 1;
            pcVar13 = pcVar7 + 1;
            cVar3 = *pcVar7;
            pcVar7 = pcVar13;
        } while (cVar3 != '\0');
        uVar8 = ~uVar8;
        iVar10 = -1;
        pcVar7 = acStack_200;
        do {
            pcVar12 = pcVar7;
            if (iVar10 == 0) break;
            iVar10 = iVar10 + -1;
            pcVar12 = pcVar7 + 1;
            cVar3 = *pcVar7;
            pcVar7 = pcVar12;
        } while (cVar3 != '\0');
        pcVar7 = pcVar13 + -uVar8;
        pcVar13 = pcVar12 + -1;
        for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
            __builtin_memcpy(&tmp4, pcVar7, 4);
            __builtin_memcpy(pcVar13, &tmp4, 4);
            pcVar7 = pcVar7 + 4;
            pcVar13 = pcVar13 + 4;
        }
        for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
            *pcVar13 = *pcVar7;
            pcVar7 = pcVar7 + 1;
            pcVar13 = pcVar13 + 1;
        }
        FUN_0047ed40(acStack_200, DAT_00523570, 0, 0);
        _File = DAT_00732948;
        if (DAT_00732948 == (FILE *)0) {
            return 0;
        }
        iVar10 = _fscanf(DAT_00732948, s__256s_00523564, acStack_200);
        if (iVar10 != 1) {
            return 0;
        }
        iVar10 = __strnicmp(acStack_200, s__inventor_00523558, 9);
        if (iVar10 != 0) {
            return 0;
        }
        iVar10 = _fscanf(_File, s__256s_00523550, acStack_200);
        if (iVar10 != 1) {
            return 0;
        }
        iVar10 = _fscanf(_File, s__256s_00523550, acStack_200);
        if (iVar10 != 1) {
            return 0;
        }
        iVar10 = __strnicmp(acStack_200, s_ascii_00523548, 5);
        if (iVar10 == 0) {
            if (DAT_006da850 != 0) {
                FUN_0048f4b0((unsigned int)DAT_006da850);
            }
            uVar5 = FUN_00402a80(_File);
        } else {
            iVar10 = __strnicmp(acStack_200, s_binary_00523540, 6);
            if (iVar10 != 0) {
                _fclose(_File);
                return 0;
            }
            if (DAT_006da850 != 0) {
                FUN_0048f4b0((unsigned int)DAT_006da850);
            }
            uVar5 = FUN_00402a70();
        }
        _fclose(_File);
        uVar6 = FUN_0048be40((((unsigned int)pcVar13 >> 16) << 16) | (unsigned int)uVar5);
        uVar8 = 0xffffffff;
        pcVar7 = acStack_100;
        do {
            pcVar13 = pcVar7;
            if (uVar8 == 0) break;
            uVar8 = uVar8 - 1;
            pcVar13 = pcVar7 + 1;
            cVar3 = *pcVar7;
            pcVar7 = pcVar13;
        } while (cVar3 != '\0');
        uVar8 = ~uVar8;
        pcVar7 = pcVar13 + -uVar8;
        pcVar13 = acStack_200;
        for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
            __builtin_memcpy(&tmp4, pcVar7, 4);
            __builtin_memcpy(pcVar13, &tmp4, 4);
            pcVar7 = pcVar7 + 4;
            pcVar13 = pcVar13 + 4;
        }
        for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
            *pcVar13 = *pcVar7;
            pcVar7 = pcVar7 + 1;
            pcVar13 = pcVar13 + 1;
        }
        uVar8 = 0xffffffff;
        pcVar7 = DAT_00525e28;
        do {
            pcVar13 = pcVar7;
            if (uVar8 == 0) break;
            uVar8 = uVar8 - 1;
            pcVar13 = pcVar7 + 1;
            cVar3 = *pcVar7;
            pcVar7 = pcVar13;
        } while (cVar3 != '\0');
        uVar8 = ~uVar8;
        iVar10 = -1;
        pcVar7 = acStack_200;
        do {
            pcVar12 = pcVar7;
            if (iVar10 == 0) break;
            iVar10 = iVar10 + -1;
            pcVar12 = pcVar7 + 1;
            cVar3 = *pcVar7;
            pcVar7 = pcVar12;
        } while (cVar3 != '\0');
        pcVar7 = pcVar13 + -uVar8;
        pcVar13 = pcVar12 + -1;
        for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
            __builtin_memcpy(&tmp4, pcVar7, 4);
            __builtin_memcpy(pcVar13, &tmp4, 4);
            pcVar7 = pcVar7 + 4;
            pcVar13 = pcVar13 + 4;
        }
        for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
            *pcVar13 = *pcVar7;
            pcVar7 = pcVar7 + 1;
            pcVar13 = pcVar13 + 1;
        }
        FUN_0048aa90(acStack_200, (((unsigned int)iVar11 >> 16) << 16) | (unsigned int)uVar6);
        DAT_006da850 = uVar6;
    } else {
        _fclose(DAT_00732948);
        if (DAT_006da850 != 0) {
            FUN_0048f4b0((unsigned int)DAT_006da850);
        }
        DAT_006da850 = FUN_00488490(acStack_200);
    }

    uVar8 = 0xffffffff;
    pcVar7 = acStack_100;
    do {
        pcVar13 = pcVar7;
        if (uVar8 == 0) break;
        uVar8 = uVar8 - 1;
        pcVar13 = pcVar7 + 1;
        cVar3 = *pcVar7;
        pcVar7 = pcVar13;
    } while (cVar3 != '\0');
    uVar8 = ~uVar8;
    pcVar7 = pcVar13 + -uVar8;
    pcVar13 = DAT_005f0928;
    for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
        __builtin_memcpy(&tmp4, pcVar7, 4);
        __builtin_memcpy(pcVar13, &tmp4, 4);
        pcVar7 = pcVar7 + 4;
        pcVar13 = pcVar13 + 4;
    }
    for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
        *pcVar13 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar13 = pcVar13 + 1;
    }
    uVar8 = 0xffffffff;
    pcVar7 = DAT_00525e28;
    do {
        pcVar13 = pcVar7;
        if (uVar8 == 0) break;
        uVar8 = uVar8 - 1;
        pcVar13 = pcVar7 + 1;
        cVar3 = *pcVar7;
        pcVar7 = pcVar13;
    } while (cVar3 != '\0');
    uVar8 = ~uVar8;
    iVar11 = -1;
    pcVar7 = DAT_005f0928;
    do {
        pcVar12 = pcVar7;
        if (iVar11 == 0) break;
        iVar11 = iVar11 + -1;
        pcVar12 = pcVar7 + 1;
        cVar3 = *pcVar7;
        pcVar7 = pcVar12;
    } while (cVar3 != '\0');
    pcVar7 = pcVar13 + -uVar8;
    pcVar13 = pcVar12 + -1;
    for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
        __builtin_memcpy(&tmp4, pcVar7, 4);
        __builtin_memcpy(pcVar13, &tmp4, 4);
        pcVar7 = pcVar7 + 4;
        pcVar13 = pcVar13 + 4;
    }
    for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
        *pcVar13 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar13 = pcVar13 + 1;
    }

    uVar5 = 0;
    if (DAT_005254ec != 0) {
        DAT_00525518 = 2;
        ret48a570 = FUN_0048a570((((unsigned int)DAT_005254ec >> 16) << 16) | (unsigned int)DAT_006da850);
        DAT_006da850 = (unsigned short)ret48a570;
        DAT_00525518 = 2;
        uVar5 = (unsigned short)(ret48a570 >> 16);
    }
    DAT_00525e0c = FUN_0048f4f0(((unsigned int)uVar5 << 16) | (unsigned int)DAT_006da850);
    if ((int *)*DAT_00525e0c != DAT_00525e0c) {
        FUN_00485fa0(DAT_00525e0c);
    }
    iVar11 = 0;
    fVar15 = FUN_004537e0((int)DAT_00525e0c, 0);
    piVar4 = DAT_00525e0c;
    DAT_00525e20 = (double)fVar15;
    iVar10 = 0;
    piVar1 = (int *)((int)DAT_00525e0c + 6);
    DAT_005f0e18 = (double)((long double)DAT_00520148 / fVar15);
    dVar2 = (double)((long double)DAT_00520148 / fVar15);
    __builtin_memcpy(&cnt, piVar1, 4);
    if (0 < cnt) {
        do {
            __builtin_memcpy(&uStack_204, (unsigned char *)&dVar2 + 4, 4);
            __builtin_memcpy(&uStack_208, &dVar2, 4);
            iVar11 = iVar11 + 4;
            iVar10 = iVar10 + 1;
            __builtin_memcpy(&arrp, (unsigned char *)piVar4 + 10, 4);
            __builtin_memcpy(&arrp, (unsigned char *)(arrp + (unsigned int)iVar11 - 4u), 4);
            FUN_004532d0((unsigned int *)arrp, (int)piVar4, uStack_208, uStack_204);
            __builtin_memcpy(&cnt, piVar1, 4);
        } while (iVar10 < cnt);
    }
    DAT_0071e536 = 1;
    if (DAT_00525e10 == 0) {
        puVar14 = DAT_005f0e50;
        for (iVar11 = 0x2b; iVar11 != 0; iVar11 = iVar11 + -1) {
            *puVar14 = 0;
            puVar14 = puVar14 + 1;
        }
        *(unsigned short *)puVar14 = 0;
        *((unsigned char *)puVar14 + 2) = 0;
        puVar14 = DAT_005f09d8;
        for (iVar11 = 0x10d; iVar11 != 0; iVar11 = iVar11 + -1) {
            *puVar14 = 0;
            puVar14 = puVar14 + 1;
        }
        *(unsigned short *)puVar14 = 0;
        iVar11 = DAT_0071ecf4;
        DAT_005f0e23 = 0;
        DAT_005f0e24 = 0;
        DAT_005f0e32 = 0;
        DAT_005f0e28 = 0;
        DAT_005f0e30 = 0;
        DAT_005f0e2c = 0;
        DAT_005f0e34 = 0;
        DAT_00525e08 = 0;
        val32 = (unsigned int)DAT_005f09d8;
        __builtin_memcpy(DAT_005f0ef7, &val32, 4);
        val32 = (unsigned int)DAT_005f0e50;
        __builtin_memcpy(DAT_005f0e3f, &val32, 4);
        __builtin_memcpy(&DAT_0071bb1b[(unsigned int)DAT_0071ecf4 * 0x5bdu], &zero32, 4);
        iVar11 = (int)((unsigned int)iVar11 * 0x5bdu);
        DAT_00709ffc = 0;
        val32 = 0xfffffb00;
        __builtin_memcpy(&DAT_0071bb1b[iVar11 + 4], &val32, 4);
        __builtin_memcpy(&DAT_0071bb1b[iVar11 + 8], &zero32, 4);
        DAT_005254f8 = 1;
        __builtin_memcpy(&DAT_0071bb1b[iVar11 + 0x14], &zero16, 2);
        DAT_005254fc = 1;
        DAT_00525504 = 1;
        DAT_00525500 = 1;
        DAT_005254f4 = 1;
        val16 = 0x4000;
        __builtin_memcpy(&DAT_0071bb1b[iVar11 + 0x10], &val16, 2);
        __builtin_memcpy(&DAT_0071bb1b[iVar11 + 0x12], &zero16, 2);
        DAT_005254e8 = 0x3f800000;
        DAT_005254f0 = 0x3f800000;
        lVar16 = __ftol();
        DAT_00703c6c = (int)(short)lVar16;
        lVar17 = __ftol();
        DAT_00703c70 = (int)(short)lVar17;
        DAT_00703c84 = (int)(short)lVar16;
    }
    return 1;
}
