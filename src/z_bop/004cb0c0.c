/* SPDX-License-Identifier: MIT */
/* z_bop 004cb0c0 */
typedef struct _iobuf FILE;

/* Prototypes for every called function */
FILE *FUN_004cc4a0(char *, char *);
void  FUN_004cc6c0(FILE *, void *, unsigned int);
void  FUN_004cc590(FILE *);
void  FUN_004ccf50(void);
void  FUN_004ccf70(void);
char *FUN_004ddc10(int);
void  FUN_004e5000(int);
void  FUN_004c9f80(FILE *);
void  _free(void *);
int   _sprintf(char *, char *, ...);

extern unsigned char DAT_00b6a2e0[];  /* big buffer, 0x4852e bytes cleared */
extern char          DAT_0051a854[];  /* fopen mode string */
extern char          DAT_0052b364[];  /* sprintf format */
extern void         *DAT_00aa6114;
extern int           DAT_00b69e30;
extern unsigned char DAT_00b6a5ae[4]; /* unaligned 4-byte field */
extern unsigned char DAT_00b6bc32[4]; /* unaligned 4-byte field */
extern unsigned char DAT_00b6d962[4]; /* unaligned 4-byte field */
extern unsigned char DAT_00b6d972[4]; /* unaligned 4-byte field */
extern unsigned char DAT_00b7e352[4]; /* unaligned 4-byte field */
extern unsigned char DAT_00b8ed32[4]; /* unaligned 4-byte field */
extern char          DAT_00b6bc9e[];
extern char          DAT_00b6a5c6[];
extern char          DAT_00b6a5e6[];

unsigned int __cdecl FUN_004cb0c0(char *param_1)
{
    char cVar1;
    FILE *pFVar2;
    char *pcVar3;
    int iVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    unsigned int *puVar7;
    char *pcVar8;
    int bVar9;
    unsigned int v;
    char local_80[128];

    puVar7 = (unsigned int *)DAT_00b6a2e0;
    for (iVar4 = 0x1214b; iVar4 != 0; iVar4 = iVar4 + -1) {
        *puVar7 = 0;
        puVar7 = puVar7 + 1;
    }
    *(unsigned short *)puVar7 = 0;

    uVar5 = 0xffffffff;
    pcVar3 = param_1;
    do {
        pcVar8 = pcVar3;
        if (uVar5 == 0) break;
        uVar5 = uVar5 - 1;
        pcVar8 = pcVar3 + 1;
        cVar1 = *pcVar3;
        pcVar3 = pcVar8;
    } while (cVar1 != '\0');
    uVar5 = ~uVar5;
    pcVar3 = pcVar8 + -uVar5;
    pcVar8 = local_80;
    for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        __builtin_memcpy(&v, pcVar3, 4);
        __builtin_memcpy(pcVar8, &v, 4);
        pcVar3 = pcVar3 + 4;
        pcVar8 = pcVar8 + 4;
    }
    for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
        *pcVar8 = *pcVar3;
        pcVar3 = pcVar3 + 1;
        pcVar8 = pcVar8 + 1;
    }

    uVar5 = 0xffffffff;
    pcVar3 = local_80;
    do {
        if (uVar5 == 0) break;
        uVar5 = uVar5 - 1;
        cVar1 = *pcVar3;
        pcVar3 = pcVar3 + 1;
    } while (cVar1 != '\0');
    local_80[~uVar5 - 2] = '2';

    pFVar2 = FUN_004cc4a0(local_80, DAT_0051a854);
    bVar9 = (pFVar2 != (FILE *)0);
    if (bVar9) {
        FUN_004cc6c0(pFVar2, DAT_00b6a2e0, 0x4852e);
        FUN_004cc590(pFVar2);
    }
    FUN_004ccf50();
    pFVar2 = FUN_004cc4a0(param_1, DAT_0051a854);
    FUN_004ccf70();

    if (pFVar2 == (FILE *)0) {
        if (!bVar9) {
            return 0;
        }
    }
    else {
        if (!bVar9) {
            v = 2;
            __builtin_memcpy(DAT_00b6bc32, &v, 4);
            pcVar3 = FUN_004ddc10(0x7c);
            uVar5 = 0xffffffff;
            do {
                pcVar8 = pcVar3;
                if (uVar5 == 0) break;
                uVar5 = uVar5 - 1;
                pcVar8 = pcVar3 + 1;
                cVar1 = *pcVar3;
                pcVar3 = pcVar8;
            } while (cVar1 != '\0');
            uVar5 = ~uVar5;
            pcVar3 = pcVar8 + -uVar5;
            pcVar8 = DAT_00b6bc9e;
            for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
                __builtin_memcpy(&v, pcVar3, 4);
                __builtin_memcpy(pcVar8, &v, 4);
                pcVar3 = pcVar3 + 4;
                pcVar8 = pcVar8 + 4;
            }
            for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                *pcVar8 = *pcVar3;
                pcVar3 = pcVar3 + 1;
                pcVar8 = pcVar8 + 1;
            }

            FUN_004e5000(0);
            if (DAT_00aa6114 != (void *)0) {
                __builtin_memcpy(&v, (char *)DAT_00aa6114 + 0x140, 4);
                __builtin_memcpy(DAT_00b6a5ae, &v, 4);
                __builtin_memcpy(&v, (char *)DAT_00aa6114 + 0x140, 4);
                __builtin_memcpy(DAT_00b6d972, &v, 4);
                _free(DAT_00aa6114);
                DAT_00aa6114 = (void *)0;
            }
            v = 1;
            __builtin_memcpy(DAT_00b6d962, &v, 4);
            FUN_004e5000(0);
            v = 0;
            __builtin_memcpy(DAT_00b6d962, &v, 4);
            if (DAT_00aa6114 != (void *)0) {
                __builtin_memcpy(&v, (char *)DAT_00aa6114 + 0x140, 4);
                __builtin_memcpy(DAT_00b7e352, &v, 4);
                _free(DAT_00aa6114);
                DAT_00aa6114 = (void *)0;
            }
            DAT_00b69e30 = 4;
            FUN_004e5000(0);
            if (DAT_00aa6114 != (void *)0) {
                __builtin_memcpy(&v, (char *)DAT_00aa6114 + 0x140, 4);
                __builtin_memcpy(DAT_00b8ed32, &v, 4);
                _free(DAT_00aa6114);
                DAT_00aa6114 = (void *)0;
            }
            DAT_00b69e30 = 0;
        }
        FUN_004c9f80(pFVar2);
        FUN_004cc590(pFVar2);
        if (!bVar9) {
            pcVar3 = FUN_004ddc10(0x1d6);
            _sprintf(DAT_00b6a5c6, DAT_0052b364, DAT_00b6a2e0, pcVar3);
            uVar5 = 0xffffffff;
            pcVar3 = DAT_00b6a5c6;
            do {
                pcVar8 = pcVar3;
                if (uVar5 == 0) break;
                uVar5 = uVar5 - 1;
                pcVar8 = pcVar3 + 1;
                cVar1 = *pcVar3;
                pcVar3 = pcVar8;
            } while (cVar1 != '\0');
            uVar5 = ~uVar5;
            pcVar3 = pcVar8 + -uVar5;
            pcVar8 = DAT_00b6a5e6;
            for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
                __builtin_memcpy(&v, pcVar3, 4);
                __builtin_memcpy(pcVar8, &v, 4);
                pcVar3 = pcVar3 + 4;
                pcVar8 = pcVar8 + 4;
            }
            for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                *pcVar8 = *pcVar3;
                pcVar3 = pcVar3 + 1;
                pcVar8 = pcVar8 + 1;
            }
        }
    }
    return 1;
}
