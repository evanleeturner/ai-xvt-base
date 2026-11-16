/* SPDX-License-Identifier: MIT */
/* z_xvt 004e6630 */
typedef struct _iobuf FILE;

extern unsigned int DAT_00757ccc;
extern int DAT_00757cc0;
extern unsigned int DAT_00757cc8;
extern char DAT_0081b740[];
extern char DAT_0052a330[];
extern char s__s__s_005331e4[];
extern char *PTR_s_train_005330e0[];
extern char DAT_0085e7b7;
extern int DAT_0085e7b8;

/* Globals at addresses that are not 4-aligned (all are 2 mod 4): accessed via __builtin_memcpy */
extern unsigned char DAT_0081bfba[4];   /* 0x0081bfba */
extern unsigned char DAT_0081bfbe[];    /* 0x0081bfbe, array of 4-byte entries */
extern unsigned char DAT_0081c012[4];   /* 0x0081c012 */
extern unsigned char DAT_0081c016[4];   /* 0x0081c016 */
extern unsigned char DAT_0081d32a[4];   /* 0x0081d32a */
extern unsigned char DAT_0081d406[4];   /* 0x0081d406 */
extern unsigned char DAT_0081d40a[4];   /* 0x0081d40a */
extern unsigned char DAT_0081d80e[4];   /* 0x0081d80e (_DAT_0081d80e) */
extern unsigned char DAT_0081dc0e[4];   /* 0x0081dc0e */

/* Callee conventions not confirmed by evidence; cdecl assumed (no callee-cleanup seen). */
int __cdecl _sprintf(char *, const char *, ...);
FILE * __cdecl FUN_004be160(char *, char *);
char * __cdecl _fgets(char *, int, FILE *);
void __cdecl FUN_004be1f0(FILE *);
int __cdecl _atoi(const char *);
DWORD __stdcall GetTickCount(void);
void __cdecl _srand(unsigned int);
int __cdecl _rand(void);
int __cdecl _tolower(int);
void __cdecl FUN_004e43d0(int);
void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);

unsigned int __stdcall FUN_004e6630(void)
{
    int *puVar1;
    char cVar2;
    unsigned char bVar3;
    int iVar4;
    FILE *_File;
    char *pcVar5;
    int iVar6;
    DWORD _Seed;
    unsigned char *pbVar7;
    int *piVar8;
    unsigned int uVar9;
    unsigned char *pbVar10;
    unsigned int uVar11;
    int iVar12;
    unsigned int uVar13;
    char *pcVar14;
    int bVar15;
    unsigned char local_80[128];
    int tmp;
    int g_idx;
    int g_val;
    int g_val2;

    uVar11 = 0;
    if (DAT_00757ccc != 0) {
        piVar8 = (int *)((unsigned int)DAT_00757cc0 + 0x140u);
        do {
            __builtin_memcpy(&g_idx, DAT_0081bfba, 4);
            __builtin_memcpy(&g_val, DAT_0081bfbe + (unsigned int)g_idx * 4u, 4);
            if (*piVar8 == g_val) break;
            piVar8 = piVar8 + 0x51;
            uVar11 = uVar11 + 1;
        } while (uVar11 < DAT_00757ccc);
    }
    __builtin_memcpy(&g_idx, DAT_0081bfba, 4);
    _sprintf((char *)local_80, s__s__s_005331e4, PTR_s_train_005330e0[g_idx],
             uVar11 * 0x144u + (unsigned int)DAT_00757cc0);
    _File = FUN_004be160((char *)local_80, DAT_0052a330);
    if (_File == (FILE *)0) {
        return 0;
    }
    pcVar5 = _fgets(DAT_0081b740, 0xff, _File);
    if (pcVar5 == (char *)0) {
        FUN_004be1f0(_File);
        return 0;
    }
    iVar6 = _atoi(DAT_0081b740);
    __builtin_memcpy(&g_idx, DAT_0081bfba, 4);
    if (g_idx == 2) {
        iVar12 = 0;
        __builtin_memcpy(DAT_0081d32a, &iVar6, 4);
        __builtin_memcpy(&iVar4, DAT_0081d80e, 4);
    } else {
        _Seed = GetTickCount();
        _srand(_Seed);
        g_val = (int)((unsigned int)DAT_0085e7b8 + 2u);
        __builtin_memcpy(DAT_0081d40a, &g_val, 4);
        iVar12 = 0;
        iVar4 = iVar12;
        if (DAT_0085e7b7 != '\0') {
            tmp = _rand();
            if (iVar6 == 0 || (tmp == (int)0x80000000 && iVar6 == -1)) {
                RaiseException(0xC0000094UL, 0, 0, (const DWORD *)0);
            }
            iVar12 = tmp % iVar6;
            iVar4 = iVar12;
        }
    }
    __builtin_memcpy(DAT_0081d80e, &iVar4, 4);
    iVar12 = (int)((unsigned int)iVar12 + 1u);
    do {
        _fgets(DAT_0081b740, 0xff, _File);
        uVar11 = 0xffffffffu;
        pcVar5 = DAT_0081b740;
        do {
            if (uVar11 == 0) break;
            uVar11 = uVar11 - 1;
            cVar2 = *pcVar5;
            pcVar5 = pcVar5 + 1;
        } while (cVar2 != '\0');
        if (DAT_0081b740[~uVar11 - 2u] == '\n') {
            uVar11 = 0xffffffffu;
            pcVar5 = DAT_0081b740;
            do {
                if (uVar11 == 0) break;
                uVar11 = uVar11 - 1;
                cVar2 = *pcVar5;
                pcVar5 = pcVar5 + 1;
            } while (cVar2 != '\0');
            DAT_0081b740[~uVar11 - 2u] = 0;
        }
        iVar12 = (int)((unsigned int)iVar12 - 1u);
    } while (iVar12 != 0);
    FUN_004be1f0(_File);
    if (DAT_0081b740[0] == '\0') {
        return 0;
    }
    g_val = 1;
    __builtin_memcpy(DAT_0081c012, &g_val, 4);
    __builtin_memcpy(&g_idx, DAT_0081bfba, 4);
    g_idx = (int)((unsigned int)g_idx - 1u);
    __builtin_memcpy(DAT_0081bfba, &g_idx, 4);
    __builtin_memcpy(&g_idx, DAT_0081bfba, 4);
    __builtin_memcpy(&g_val, DAT_0081bfbe + (unsigned int)g_idx * 4u, 4);
    __builtin_memcpy(DAT_0081c016, &g_val, 4);
    uVar11 = 0xffffffffu;
    pcVar5 = DAT_0081b740;
    do {
        pcVar14 = pcVar5;
        if (uVar11 == 0) break;
        uVar11 = uVar11 - 1;
        pcVar14 = pcVar5 + 1;
        cVar2 = *pcVar5;
        pcVar5 = pcVar14;
    } while (cVar2 != '\0');
    uVar11 = ~uVar11;
    pbVar7 = (unsigned char *)(pcVar14 - uVar11);
    pbVar10 = local_80;
    for (uVar9 = uVar11 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
        *(unsigned int *)pbVar10 = *(unsigned int *)pbVar7;
        pbVar7 = pbVar7 + 4;
        pbVar10 = pbVar10 + 4;
    }
    for (uVar11 = uVar11 & 3u; uVar11 != 0; uVar11 = uVar11 - 1) {
        *pbVar10 = *pbVar7;
        pbVar7 = pbVar7 + 1;
        pbVar10 = pbVar10 + 1;
    }
    iVar6 = -1;
    pbVar7 = local_80;
    do {
        if (iVar6 == 0) break;
        iVar6 = (int)((unsigned int)iVar6 - 1u);
        bVar3 = *pbVar7;
        pbVar7 = pbVar7 + 1;
    } while (bVar3 != 0);
    uVar11 = 0;
    if (iVar6 != -2) {
        do {
            uVar13 = uVar11 + 1;
            iVar6 = _tolower((unsigned int)local_80[uVar11]);
            local_80[uVar11] = (unsigned char)iVar6;
            uVar9 = 0xffffffffu;
            pbVar7 = local_80;
            do {
                if (uVar9 == 0) break;
                uVar9 = uVar9 - 1;
                bVar3 = *pbVar7;
                pbVar7 = pbVar7 + 1;
            } while (bVar3 != 0);
            uVar11 = uVar13;
        } while (uVar13 < ~uVar9 - 1u);
    }
    iVar6 = 0;
    __builtin_memcpy(&g_idx, DAT_0081bfba, 4);
    FUN_004e43d0(g_idx);
    uVar11 = 0;
    if (DAT_00757ccc != 0) {
        do {
            pbVar7 = local_80;
            pbVar10 = (unsigned char *)((unsigned int)DAT_00757cc0 + (unsigned int)iVar6);
            do {
                bVar3 = *pbVar7;
                bVar15 = bVar3 < *pbVar10;
                if (bVar3 != *pbVar10) {
LAB_004e688c:
                    iVar12 = (int)((1u - (unsigned int)bVar15) - (unsigned int)(bVar15 != 0));
                    goto LAB_004e6891;
                }
                if (bVar3 == 0) break;
                bVar3 = pbVar7[1];
                bVar15 = bVar3 < pbVar10[1];
                if (bVar3 != pbVar10[1]) goto LAB_004e688c;
                pbVar7 = pbVar7 + 2;
                pbVar10 = pbVar10 + 2;
            } while (bVar3 != 0);
            iVar12 = 0;
LAB_004e6891:
            if (iVar12 == 0) {
                puVar1 = (int *)(uVar11 * 0x144u + 0x140u + (unsigned int)DAT_00757cc0);
                g_val = *puVar1;
                __builtin_memcpy(&g_idx, DAT_0081bfba, 4);
                __builtin_memcpy(DAT_0081bfbe + (unsigned int)g_idx * 4u, &g_val, 4);
                __builtin_memcpy(&g_idx, DAT_0081bfba, 4);
                if (g_idx == 3) {
                    g_val = *puVar1;
                    __builtin_memcpy(DAT_0081d406, &g_val, 4);
                }
                break;
            }
            iVar6 = (int)((unsigned int)iVar6 + 0x144u);
            uVar11 = uVar11 + 1;
        } while (uVar11 < DAT_00757ccc);
    }
    if ((DAT_00757cc0 != 0) && (DAT_00757cc8 = 0, DAT_00757ccc != 0)) {
        piVar8 = (int *)((unsigned int)DAT_00757cc0 + 0x140u);
        do {
            __builtin_memcpy(&g_idx, DAT_0081bfba, 4);
            __builtin_memcpy(&g_val2, DAT_0081bfbe + (unsigned int)g_idx * 4u, 4);
            if (*piVar8 == g_val2) {
                g_val = (int)DAT_00757cc8;
                __builtin_memcpy(DAT_0081dc0e, &g_val, 4);
                return 1;
            }
            piVar8 = piVar8 + 0x51;
            DAT_00757cc8 = DAT_00757cc8 + 1;
        } while (DAT_00757cc8 < DAT_00757ccc);
    }
    return 1;
}
