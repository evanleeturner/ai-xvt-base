/* SPDX-License-Identifier: MIT */
/* z_bop 00411e00 */
typedef struct _iobuf FILE;

extern char DAT_005506f8[];
extern char DAT_0051a854[];
extern char DAT_0051bd9c[];
extern FILE *DAT_00a07bd8;

int __cdecl _fclose(FILE *);
int __cdecl _fscanf(FILE *, const char *, ...);
int __cdecl __strnicmp(const char *, const char *, unsigned int);

void FUN_0049e720(char *, char *, int, int);
unsigned int FUN_004742a0(char *);
void FUN_00449130(void);
short FUN_004762f0(unsigned int);
unsigned short FUN_00412040(FILE *);
unsigned short FUN_00412030(void);
unsigned short FUN_00477950(unsigned int);
void FUN_004766c0(char *, unsigned int);

short __cdecl FUN_00411e00(char *param_1)
{
    char cVar1;
    FILE *_File;
    unsigned short uVar2;
    unsigned short uVar3;
    short sVar4;
    char *puVar5;
    unsigned int uVar6;
    unsigned int uVar7;
    int iVar8;
    char *pcVar9;
    char *pcVar10;
    char *pcVar11;
    unsigned int uTmp;

    uVar6 = 0xffffffff;
    pcVar9 = param_1;
    do {
        pcVar11 = pcVar9;
        if (uVar6 == 0) break;
        uVar6 = uVar6 - 1;
        pcVar11 = pcVar9 + 1;
        cVar1 = *pcVar9;
        pcVar9 = pcVar11;
    } while (cVar1 != '\0');
    uVar6 = ~uVar6;
    pcVar9 = pcVar11 - uVar6;
    pcVar11 = DAT_005506f8;
    for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
        __builtin_memcpy(&uTmp, pcVar9, 4);
        __builtin_memcpy(pcVar11, &uTmp, 4);
        pcVar9 = pcVar9 + 4;
        pcVar11 = pcVar11 + 4;
    }
    for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
        *pcVar11 = *pcVar9;
        pcVar9 = pcVar9 + 1;
        pcVar11 = pcVar11 + 1;
    }
    FUN_0049e720(DAT_005506f8, DAT_0051a854, 0, 0);
    if (DAT_00a07bd8 != (FILE *)0) {
        _fclose(DAT_00a07bd8);
        uVar6 = FUN_004742a0(DAT_005506f8);
        FUN_00449130();
        sVar4 = FUN_004762f0(uVar6 & 0xffff);
        FUN_00449130();
        return sVar4;
    }
    puVar5 = DAT_005506f8;
    cVar1 = DAT_005506f8[0];
    while (cVar1 != '.') {
        pcVar9 = puVar5 + 1;
        puVar5 = puVar5 + 1;
        cVar1 = *pcVar9;
    }
    uVar6 = 0xffffffff;
    *puVar5 = 0;
    pcVar9 = DAT_0051bd9c;
    do {
        pcVar11 = pcVar9;
        if (uVar6 == 0) break;
        uVar6 = uVar6 - 1;
        pcVar11 = pcVar9 + 1;
        cVar1 = *pcVar9;
        pcVar9 = pcVar11;
    } while (cVar1 != '\0');
    uVar6 = ~uVar6;
    iVar8 = -1;
    pcVar9 = DAT_005506f8;
    do {
        pcVar10 = pcVar9;
        if (iVar8 == 0) break;
        iVar8 = iVar8 + -1;
        pcVar10 = pcVar9 + 1;
        cVar1 = *pcVar9;
        pcVar9 = pcVar10;
    } while (cVar1 != '\0');
    pcVar9 = pcVar11 - uVar6;
    pcVar11 = pcVar10 - 1;
    for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
        __builtin_memcpy(&uTmp, pcVar9, 4);
        __builtin_memcpy(pcVar11, &uTmp, 4);
        pcVar9 = pcVar9 + 4;
        pcVar11 = pcVar11 + 4;
    }
    for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
        *pcVar11 = *pcVar9;
        pcVar9 = pcVar9 + 1;
        pcVar11 = pcVar11 + 1;
    }
    FUN_0049e720(DAT_005506f8, DAT_0051a854, 1, 0);
    _File = DAT_00a07bd8;
    if (DAT_00a07bd8 == (FILE *)0) {
        return 0;
    }
    iVar8 = _fscanf(DAT_00a07bd8, "%256s", DAT_005506f8);
    if (iVar8 != 1) {
        return 0;
    }
    iVar8 = __strnicmp(DAT_005506f8, "#inventor", 9);
    if (iVar8 != 0) {
        return 0;
    }
    iVar8 = _fscanf(_File, " %256s", DAT_005506f8);
    if (iVar8 != 1) {
        return 0;
    }
    iVar8 = _fscanf(_File, " %256s", DAT_005506f8);
    if (iVar8 != 1) {
        return 0;
    }
    iVar8 = __strnicmp(DAT_005506f8, "ascii", 5);
    if (iVar8 == 0) {
        uVar2 = FUN_00412040(_File);
    } else {
        iVar8 = __strnicmp(DAT_005506f8, "binary", 6);
        if (iVar8 != 0) {
            _fclose(_File);
            return 0;
        }
        uVar2 = FUN_00412030();
    }
    _fclose(_File);
    uVar3 = FUN_00477950((unsigned int)uVar2);
    uVar6 = 0xffffffff;
    do {
        pcVar9 = param_1;
        if (uVar6 == 0) break;
        uVar6 = uVar6 - 1;
        pcVar9 = param_1 + 1;
        cVar1 = *param_1;
        param_1 = pcVar9;
    } while (cVar1 != '\0');
    uVar6 = ~uVar6;
    pcVar9 = pcVar9 - uVar6;
    pcVar11 = DAT_005506f8;
    for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
        __builtin_memcpy(&uTmp, pcVar9, 4);
        __builtin_memcpy(pcVar11, &uTmp, 4);
        pcVar9 = pcVar9 + 4;
        pcVar11 = pcVar11 + 4;
    }
    for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
        *pcVar11 = *pcVar9;
        pcVar9 = pcVar9 + 1;
        pcVar11 = pcVar11 + 1;
    }
    FUN_004766c0(DAT_005506f8, (unsigned int)uVar3);
    sVar4 = FUN_004762f0((unsigned int)uVar3);
    return sVar4;
}
