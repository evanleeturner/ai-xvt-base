/* SPDX-License-Identifier: MIT */
/* z_bop 004bf3a0 */
extern int __cdecl _sprintf(char *, const char *, ...);
extern void * __cdecl FID_conflict___wfopen(char *, char *);
extern int __cdecl _fclose(void *);
extern void * __cdecl FUN_004cc4a0(char *, char *);
extern char * __cdecl FUN_004ddc10(int);
extern void __cdecl FUN_004e5000(int);
extern void __cdecl _free(void *);
extern void __cdecl FUN_004cc790(void *, void *, int);
extern void __cdecl FUN_004cc590(void *);
extern unsigned int __cdecl FUN_004bf650(int);

extern char DAT_0051a854[];
extern char DAT_00527004[];
extern char DAT_0052b364[];

extern unsigned int DAT_00b6a2e0[];
extern char DAT_00b6bc9e[];
extern char DAT_00b6a5c6[];
extern char DAT_00b6a5e6[];
extern void *DAT_00aa6114;
extern unsigned int DAT_00b69e30;

/* 4-byte globals at addresses = 2 mod 4: packed, accessed via memcpy */
extern unsigned char DAT_00b6bc32[4];
extern unsigned char DAT_00b6a5ae[4];
extern unsigned char DAT_00b6d972[4];
extern unsigned char DAT_00b6d962[4];
extern unsigned char DAT_00b7e352[4];
extern unsigned char DAT_00b8ed32[4];
extern unsigned char DAT_00b6a5ba[4];
extern unsigned char DAT_00b6d97e[4];
extern unsigned char DAT_00b7e35e[4];

unsigned int __cdecl FUN_004bf3a0(char *param_1)
{
    char cVar1;
    void *pFVar2;
    char *pcVar3;
    unsigned int uVar4;
    unsigned int uVar5;
    int iVar6;
    unsigned int *puVar7;
    char *pcVar8;
    char local_20[32];
    unsigned int tmp;

    iVar6 = 0;
    while (1) {
        _sprintf(local_20, "%s%d.pl2", param_1, iVar6);
        pFVar2 = FID_conflict___wfopen(local_20, DAT_0051a854);
        if (pFVar2 == (void *)0) break;
        iVar6 = iVar6 + 1;
        _fclose(pFVar2);
    }
    pFVar2 = FUN_004cc4a0(local_20, DAT_00527004);
    if (pFVar2 == (void *)0) {
        return 0;
    }
    puVar7 = DAT_00b6a2e0;
    for (iVar6 = 0x1214b; iVar6 != 0; iVar6 = iVar6 + -1) {
        *puVar7 = 0;
        puVar7 = puVar7 + 1;
    }
    *(unsigned short *)puVar7 = 0;

    uVar4 = 0xffffffff;
    pcVar3 = param_1;
    do {
        pcVar8 = pcVar3;
        if (uVar4 == 0) break;
        uVar4 = uVar4 - 1;
        pcVar8 = pcVar3 + 1;
        cVar1 = *pcVar3;
        pcVar3 = pcVar8;
    } while (cVar1 != '\0');
    uVar4 = ~uVar4;
    pcVar3 = pcVar8 + -uVar4;
    pcVar8 = (char *)DAT_00b6a2e0;
    for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
        __builtin_memcpy(pcVar8, pcVar3, 4);
        pcVar3 = pcVar3 + 4;
        pcVar8 = pcVar8 + 4;
    }
    for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
        *pcVar8 = *pcVar3;
        pcVar3 = pcVar3 + 1;
        pcVar8 = pcVar8 + 1;
    }

    tmp = 2;
    __builtin_memcpy(DAT_00b6bc32, &tmp, 4);
    pcVar3 = FUN_004ddc10(0x7c);
    uVar4 = 0xffffffff;
    do {
        pcVar8 = pcVar3;
        if (uVar4 == 0) break;
        uVar4 = uVar4 - 1;
        pcVar8 = pcVar3 + 1;
        cVar1 = *pcVar3;
        pcVar3 = pcVar8;
    } while (cVar1 != '\0');
    uVar4 = ~uVar4;
    pcVar3 = pcVar8 + -uVar4;
    pcVar8 = DAT_00b6bc9e;
    for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
        __builtin_memcpy(pcVar8, pcVar3, 4);
        pcVar3 = pcVar3 + 4;
        pcVar8 = pcVar8 + 4;
    }
    for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
        *pcVar8 = *pcVar3;
        pcVar3 = pcVar3 + 1;
        pcVar8 = pcVar8 + 1;
    }

    pcVar3 = FUN_004ddc10(0x1d6);
    _sprintf(DAT_00b6a5c6, DAT_0052b364, param_1, pcVar3);
    uVar4 = 0xffffffff;
    pcVar3 = DAT_00b6a5c6;
    do {
        pcVar8 = pcVar3;
        if (uVar4 == 0) break;
        uVar4 = uVar4 - 1;
        pcVar8 = pcVar3 + 1;
        cVar1 = *pcVar3;
        pcVar3 = pcVar8;
    } while (cVar1 != '\0');
    uVar4 = ~uVar4;
    pcVar3 = pcVar8 + -uVar4;
    pcVar8 = DAT_00b6a5e6;
    for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
        __builtin_memcpy(pcVar8, pcVar3, 4);
        pcVar3 = pcVar3 + 4;
        pcVar8 = pcVar8 + 4;
    }
    for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
        *pcVar8 = *pcVar3;
        pcVar3 = pcVar3 + 1;
        pcVar8 = pcVar8 + 1;
    }

    FUN_004e5000(0);
    if (DAT_00aa6114 != (void *)0) {
        tmp = *(unsigned int *)((int)DAT_00aa6114 + 0x140);
        __builtin_memcpy(DAT_00b6a5ae, &tmp, 4);
        tmp = *(unsigned int *)((int)DAT_00aa6114 + 0x140);
        __builtin_memcpy(DAT_00b6d972, &tmp, 4);
        _free(DAT_00aa6114);
        DAT_00aa6114 = (void *)0;
    }
    tmp = 1;
    __builtin_memcpy(DAT_00b6d962, &tmp, 4);
    FUN_004e5000(0);
    tmp = 0;
    __builtin_memcpy(DAT_00b6d962, &tmp, 4);
    if (DAT_00aa6114 != (void *)0) {
        tmp = *(unsigned int *)((int)DAT_00aa6114 + 0x140);
        __builtin_memcpy(DAT_00b7e352, &tmp, 4);
        _free(DAT_00aa6114);
        DAT_00aa6114 = (void *)0;
    }
    DAT_00b69e30 = 4;
    FUN_004e5000(0);
    if (DAT_00aa6114 != (void *)0) {
        tmp = *(unsigned int *)((int)DAT_00aa6114 + 0x140);
        __builtin_memcpy(DAT_00b8ed32, &tmp, 4);
        _free(DAT_00aa6114);
        DAT_00aa6114 = (void *)0;
    }
    DAT_00b69e30 = 0;
    FUN_004cc790(pFVar2, DAT_00b6a2e0, 0x4852e);
    FUN_004e5000(3);
    if (DAT_00aa6114 != (void *)0) {
        tmp = *(unsigned int *)((int)DAT_00aa6114 + 0x3d0);
        __builtin_memcpy(DAT_00b6a5ba, &tmp, 4);
        tmp = *(unsigned int *)((int)DAT_00aa6114 + 0x140);
        __builtin_memcpy(DAT_00b6d97e, &tmp, 4);
        _free(DAT_00aa6114);
        DAT_00aa6114 = (void *)0;
    }
    tmp = 1;
    __builtin_memcpy(DAT_00b6d962, &tmp, 4);
    FUN_004e5000(3);
    tmp = 0;
    __builtin_memcpy(DAT_00b6d962, &tmp, 4);
    if (DAT_00aa6114 != (void *)0) {
        tmp = *(unsigned int *)((int)DAT_00aa6114 + 0x140);
        __builtin_memcpy(DAT_00b7e35e, &tmp, 4);
        _free(DAT_00aa6114);
        DAT_00aa6114 = (void *)0;
    }
    FUN_004cc590(pFVar2);
    FUN_004bf650(0);
    return 1;
}
