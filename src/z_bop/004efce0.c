/* SPDX-License-Identifier: MIT */
/* z_bop 004efce0 */
/* FILE is not declared by xvt.h; declare an opaque CRT stream type here. */
typedef struct _iobuf FILE;

/* Unresolved: FUN_004f07d0's real prototype is unknown (assumed void(void); stored only as a value).
   The block passed to FUN_004ef100 is the contiguous stack area local_1b4..local_188, modeled as
   unsigned int[12]; indices 7 (local_198) and 10 (local_18c) are never written. Pointers are 32-bit on i386. */

extern char s_movies__0052c850[];
extern char DAT_0052c848[];
extern char DAT_0051a854[];
extern unsigned int DAT_0052c83c;
extern unsigned int DAT_0052c840;
extern unsigned short DAT_0052c844;
extern unsigned char DAT_0052c846;
extern unsigned int DAT_0052c858;
extern unsigned int DAT_0052c85c;
extern int DAT_00b69e30;
extern int DAT_00b69cb0;
extern int DAT_00b69cbc;

/* packed (unaligned) globals: accessed through __builtin_memcpy */
extern unsigned char DAT_00aa7b36[4];
extern unsigned char DAT_00aa7b3a[4];
extern unsigned char DAT_00aa7b32[4];
extern unsigned char DAT_00aa7fb7[4];
extern unsigned char DAT_00aa7b26[4];
extern unsigned char DAT_00aa7b9a[4];

extern void LAB_004f0070(void);
extern void LAB_004f0140(void);

FILE *FID_conflict___wfopen(char *_Filename, char *_Mode);
char FUN_004ccb90(void);
void FUN_004d4f60(void);
void FUN_004d4f70(void);
void FUN_004d4c10(void);
char *FUN_004ddc10(int param_1);
int FUN_004dcb90(char *a, char *b, char *c, char *d, char *e);
void _fclose(FILE *_File);
void FUN_004f07d0(void);
unsigned char FUN_004ef100(unsigned int *param_1);

unsigned char __cdecl FUN_004efce0(char *param_1, int param_2)
{
    char cVar1;
    unsigned char bVar2;
    FILE *_File;
    char *pcVar3;
    char *pcVar4;
    char *pcVar5;
    char *pcVar6;
    char *pcVar7;
    unsigned int uVar8;
    unsigned int uVar9;
    int iVar10;
    unsigned int local_1b4[12];
    char local_180[0x80];
    unsigned int local_100[64];

    uVar8 = 0xffffffff;
    do {
        pcVar3 = param_1;
        if (uVar8 == 0) break;
        uVar8 = uVar8 - 1;
        pcVar3 = param_1 + 1;
        cVar1 = *param_1;
        param_1 = pcVar3;
    } while (cVar1 != '\0');
    uVar8 = ~uVar8;
    pcVar3 = pcVar3 + -uVar8;
    pcVar4 = (char *)local_100;
    for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
        __builtin_memcpy(pcVar4, pcVar3, 4);
        pcVar3 = pcVar3 + 4;
        pcVar4 = pcVar4 + 4;
    }
    for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
        *pcVar4 = *pcVar3;
        pcVar3 = pcVar3 + 1;
        pcVar4 = pcVar4 + 1;
    }
    __builtin_memcpy(local_180, s_movies__0052c850, 4);
    uVar8 = 0xffffffff;
    __builtin_memcpy(local_180 + 4, s_movies__0052c850 + 4, 4);
    pcVar3 = (char *)local_100;
    do {
        pcVar4 = pcVar3;
        if (uVar8 == 0) break;
        uVar8 = uVar8 - 1;
        pcVar4 = pcVar3 + 1;
        cVar1 = *pcVar3;
        pcVar3 = pcVar4;
    } while (cVar1 != '\0');
    uVar8 = ~uVar8;
    iVar10 = -1;
    pcVar3 = local_180;
    do {
        pcVar5 = pcVar3;
        if (iVar10 == 0) break;
        iVar10 = iVar10 + -1;
        pcVar5 = pcVar3 + 1;
        cVar1 = *pcVar3;
        pcVar3 = pcVar5;
    } while (cVar1 != '\0');
    pcVar3 = pcVar4 + -uVar8;
    pcVar4 = pcVar5 + -1;
    for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
        __builtin_memcpy(pcVar4, pcVar3, 4);
        pcVar3 = pcVar3 + 4;
        pcVar4 = pcVar4 + 4;
    }
    for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
        *pcVar4 = *pcVar3;
        pcVar3 = pcVar3 + 1;
        pcVar4 = pcVar4 + 1;
    }
    uVar8 = 0xffffffff;
    pcVar3 = DAT_0052c848;
    do {
        pcVar4 = pcVar3;
        if (uVar8 == 0) break;
        uVar8 = uVar8 - 1;
        pcVar4 = pcVar3 + 1;
        cVar1 = *pcVar3;
        pcVar3 = pcVar4;
    } while (cVar1 != '\0');
    uVar8 = ~uVar8;
    iVar10 = -1;
    pcVar3 = local_180;
    do {
        pcVar5 = pcVar3;
        if (iVar10 == 0) break;
        iVar10 = iVar10 + -1;
        pcVar5 = pcVar3 + 1;
        cVar1 = *pcVar3;
        pcVar3 = pcVar5;
    } while (cVar1 != '\0');
    pcVar3 = pcVar4 + -uVar8;
    pcVar4 = pcVar5 + -1;
    for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
        __builtin_memcpy(pcVar4, pcVar3, 4);
        pcVar3 = pcVar3 + 4;
        pcVar4 = pcVar4 + 4;
    }
    for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
        *pcVar4 = *pcVar3;
        pcVar3 = pcVar3 + 1;
        pcVar4 = pcVar4 + 1;
    }
    _File = FID_conflict___wfopen(local_180, DAT_0051a854);
    if ((param_2 == 0) || (DAT_00b69e30 == 2)) {
        if (_File == (FILE *)0) {
            __builtin_memcpy(local_180, &DAT_0052c83c, 4);
            __builtin_memcpy(local_180 + 4, &DAT_0052c840, 4);
            __builtin_memcpy(local_180 + 8, &DAT_0052c844, 2);
            __builtin_memcpy(local_180 + 10, &DAT_0052c846, 1);
            cVar1 = FUN_004ccb90();
            uVar8 = 0xffffffff;
            local_180[0] = cVar1;
            pcVar3 = (char *)local_100;
            do {
                pcVar4 = pcVar3;
                if (uVar8 == 0) break;
                uVar8 = uVar8 - 1;
                pcVar4 = pcVar3 + 1;
                cVar1 = *pcVar3;
                pcVar3 = pcVar4;
            } while (cVar1 != '\0');
            uVar8 = ~uVar8;
            iVar10 = -1;
            pcVar3 = local_180;
            do {
                pcVar5 = pcVar3;
                if (iVar10 == 0) break;
                iVar10 = iVar10 + -1;
                pcVar5 = pcVar3 + 1;
                cVar1 = *pcVar3;
                pcVar3 = pcVar5;
            } while (cVar1 != '\0');
            pcVar3 = pcVar4 + -uVar8;
            pcVar4 = pcVar5 + -1;
            for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
                __builtin_memcpy(pcVar4, pcVar3, 4);
                pcVar3 = pcVar3 + 4;
                pcVar4 = pcVar4 + 4;
            }
            for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
                *pcVar4 = *pcVar3;
                pcVar3 = pcVar3 + 1;
                pcVar4 = pcVar4 + 1;
            }
            uVar8 = 0xffffffff;
            pcVar3 = DAT_0052c848;
            do {
                pcVar4 = pcVar3;
                if (uVar8 == 0) break;
                uVar8 = uVar8 - 1;
                pcVar4 = pcVar3 + 1;
                cVar1 = *pcVar3;
                pcVar3 = pcVar4;
            } while (cVar1 != '\0');
            uVar8 = ~uVar8;
            iVar10 = -1;
            pcVar3 = local_180;
            do {
                pcVar5 = pcVar3;
                if (iVar10 == 0) break;
                iVar10 = iVar10 + -1;
                pcVar5 = pcVar3 + 1;
                cVar1 = *pcVar3;
                pcVar3 = pcVar5;
            } while (cVar1 != '\0');
            pcVar3 = pcVar4 + -uVar8;
            pcVar4 = pcVar5 + -1;
            for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
                __builtin_memcpy(pcVar4, pcVar3, 4);
                pcVar3 = pcVar3 + 4;
                pcVar4 = pcVar4 + 4;
            }
            for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
                *pcVar4 = *pcVar3;
                pcVar3 = pcVar3 + 1;
                pcVar4 = pcVar4 + 1;
            }
            while (_File = FID_conflict___wfopen(local_180, DAT_0051a854), _File == (FILE *)0) {
                FUN_004d4f60();
                pcVar3 = FUN_004ddc10(0x13);
                pcVar4 = FUN_004ddc10(0x20b);
                pcVar5 = FUN_004ddc10(0x33d);
                pcVar6 = FUN_004ddc10(0x33c);
                pcVar7 = FUN_004ddc10(0x33b);
                iVar10 = FUN_004dcb90(pcVar7, pcVar6, pcVar5, pcVar4, pcVar3);
                if (iVar10 == 0) {
                    FUN_004d4f70();
                    FUN_004d4c10();
                    return 2;
                }
                FUN_004d4f70();
                FUN_004d4c10();
            }
        }
    }
    else if (_File == (FILE *)0) {
        __builtin_memcpy(local_180, &DAT_0052c83c, 4);
        __builtin_memcpy(local_180 + 4, &DAT_0052c840, 4);
        __builtin_memcpy(local_180 + 8, &DAT_0052c844, 2);
        __builtin_memcpy(local_180 + 10, &DAT_0052c846, 1);
        cVar1 = FUN_004ccb90();
        uVar8 = 0xffffffff;
        local_180[0] = cVar1;
        pcVar3 = (char *)local_100;
        do {
            pcVar4 = pcVar3;
            if (uVar8 == 0) break;
            uVar8 = uVar8 - 1;
            pcVar4 = pcVar3 + 1;
            cVar1 = *pcVar3;
            pcVar3 = pcVar4;
        } while (cVar1 != '\0');
        uVar8 = ~uVar8;
        iVar10 = -1;
        pcVar3 = local_180;
        do {
            pcVar5 = pcVar3;
            if (iVar10 == 0) break;
            iVar10 = iVar10 + -1;
            pcVar5 = pcVar3 + 1;
            cVar1 = *pcVar3;
            pcVar3 = pcVar5;
        } while (cVar1 != '\0');
        pcVar3 = pcVar4 + -uVar8;
        pcVar4 = pcVar5 + -1;
        for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
            __builtin_memcpy(pcVar4, pcVar3, 4);
            pcVar3 = pcVar3 + 4;
            pcVar4 = pcVar4 + 4;
        }
        for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
            *pcVar4 = *pcVar3;
            pcVar3 = pcVar3 + 1;
            pcVar4 = pcVar4 + 1;
        }
        uVar8 = 0xffffffff;
        pcVar3 = DAT_0052c848;
        do {
            pcVar4 = pcVar3;
            if (uVar8 == 0) break;
            uVar8 = uVar8 - 1;
            pcVar4 = pcVar3 + 1;
            cVar1 = *pcVar3;
            pcVar3 = pcVar4;
        } while (cVar1 != '\0');
        uVar8 = ~uVar8;
        iVar10 = -1;
        pcVar3 = local_180;
        do {
            pcVar5 = pcVar3;
            if (iVar10 == 0) break;
            iVar10 = iVar10 + -1;
            pcVar5 = pcVar3 + 1;
            cVar1 = *pcVar3;
            pcVar3 = pcVar5;
        } while (cVar1 != '\0');
        pcVar3 = pcVar4 + -uVar8;
        pcVar4 = pcVar5 + -1;
        for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
            __builtin_memcpy(pcVar4, pcVar3, 4);
            pcVar3 = pcVar3 + 4;
            pcVar4 = pcVar4 + 4;
        }
        for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
            *pcVar4 = *pcVar3;
            pcVar3 = pcVar3 + 1;
            pcVar4 = pcVar4 + 1;
        }
        _File = FID_conflict___wfopen(local_180, DAT_0051a854);
        if (_File == (FILE *)0) {
            local_100[0] = DAT_0052c858;
            local_100[1] = DAT_0052c85c;
            goto LAB_004effc6;
        }
    }
    _fclose(_File);
LAB_004effc6:
    local_1b4[0] = (unsigned int)local_100;
    __builtin_memcpy(&local_1b4[3], DAT_00aa7b36, 4);
    local_1b4[1] = 0x280;
    local_1b4[2] = 0x1e0;
    __builtin_memcpy(&local_1b4[4], DAT_00aa7b3a, 4);
    if ((DAT_00b69cb0 == 0) && (DAT_00b69cbc == 0)) {
        __builtin_memcpy(&local_1b4[4], DAT_00aa7b32, 4);
    }
    __builtin_memcpy(&local_1b4[6], DAT_00aa7fb7, 4);
    __builtin_memcpy(&local_1b4[8], DAT_00aa7b26, 4);
    __builtin_memcpy(&local_1b4[5], DAT_00aa7b9a, 4);
    if ((param_2 == 0) || (DAT_00b69e30 == 2)) {
        local_1b4[9] = (unsigned int)&LAB_004f0070;
        local_1b4[11] = 0;
    }
    else {
        local_1b4[9] = (unsigned int)&LAB_004f0140;
        local_1b4[11] = (unsigned int)FUN_004f07d0;
    }
    bVar2 = FUN_004ef100(local_1b4);
    return bVar2;
}
