/* SPDX-License-Identifier: MIT */
/* z_bop 004fcf30 */
extern void *DAT_00b6a2b4;
extern void *DAT_00aa6114;
extern unsigned int DAT_00aa6138;
extern char DAT_00b6a5be[4];    /* unaligned int at 0xb6a5be */
extern char DAT_00b6bba6[4];    /* unaligned int at 0xb6bba6 */
extern char DAT_00b6bbda[];     /* unaligned int array at 0xb6bbda */
extern int DAT_00b6a244;
extern char DAT_00b69d20[];
extern char DAT_00a91b90[];
extern char DAT_0051c4bc;
extern char *PTR_s_battle_0052c1f8;
extern char s__s__s_0052c300[];

extern void __cdecl _free(void *);
extern void __cdecl FUN_004e5000(int);
extern int __cdecl _sprintf(char *, const char *, ...);
extern void *__cdecl FUN_004cc4a0(char *, char *);
extern char *__cdecl _fgets(char *, int, void *);
extern void __cdecl FUN_004cc590(void *);
extern int __cdecl _atoi(const char *);
extern void *__cdecl _malloc(unsigned int);
extern int __cdecl __strcmpi(const char *, const char *);

unsigned int __stdcall FUN_004fcf30(void)
{
    char local_80[128];
    void *_File;
    char *pcVar4;
    char *pcVar13;
    char cVar1;
    int iVar5;
    unsigned int uVar6;
    unsigned int uVar7;
    int iVar8;
    int iVar9;
    unsigned int uVar10;
    int iVar11;
    unsigned int *puVar12;
    unsigned int *puVar14;
    int bVar2;
    int local_b5be;
    int local_bba6;
    int local_val;
    int *piVar3;
    char *pbVar3;

    if (DAT_00b6a2b4 != (void *)0) {
        _free(DAT_00b6a2b4);
        DAT_00b6a2b4 = (void *)0;
    }
    uVar7 = DAT_00aa6138;
    iVar9 = (int)DAT_00aa6114;
    uVar10 = 0;
    DAT_00aa6114 = (void *)0;
    FUN_004e5000(4);
    if (DAT_00aa6138 != 0) {
        piVar3 = (int *)((char *)DAT_00aa6114 + 0x140);
        do {
            __builtin_memcpy(&local_b5be, DAT_00b6a5be, 4);
            if (*piVar3 == local_b5be) break;
            piVar3 = piVar3 + 0x52;
            uVar10 = uVar10 + 1;
        } while (uVar10 < DAT_00aa6138);
    }
    _sprintf(local_80, s__s__s_0052c300, PTR_s_battle_0052c1f8,
             (char *)DAT_00aa6114 + uVar10 * 0x148u);
    uVar6 = 0xffffffffu;
    pcVar4 = (char *)DAT_00aa6114 + (uVar10 * 0x148u + 0x40u);
    do {
        pcVar13 = pcVar4;
        if (uVar6 == 0) break;
        uVar6 = uVar6 - 1;
        pcVar13 = pcVar4 + 1;
        cVar1 = *pcVar4;
        pcVar4 = pcVar13;
    } while (cVar1 != '\0');
    uVar6 = ~uVar6;
    pcVar4 = pcVar13 - uVar6;
    pcVar13 = DAT_00a91b90;
    for (uVar10 = uVar6 >> 2; uVar10 != 0; uVar10 = uVar10 - 1) {
        *(unsigned int *)pcVar13 = *(unsigned int *)pcVar4;
        pcVar4 = pcVar4 + 4;
        pcVar13 = pcVar13 + 4;
    }
    for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
        *pcVar13 = *pcVar4;
        pcVar4 = pcVar4 + 1;
        pcVar13 = pcVar13 + 1;
    }
    _free(DAT_00aa6114);
    DAT_00aa6114 = (void *)iVar9;
    DAT_00aa6138 = uVar7;
    _File = FUN_004cc4a0(local_80, &DAT_0051c4bc);
    if (_File == (void *)0) {
        return 0;
    }
    pcVar4 = _fgets(DAT_00b69d20, 0xff, _File);
    if (pcVar4 == (char *)0) {
        FUN_004cc590(_File);
        return 0;
    }
    DAT_00b6a244 = _atoi(DAT_00b69d20);
    iVar9 = 0;
    DAT_00b6a2b4 = _malloc((unsigned int)DAT_00b6a244 * 0x148u);
    if (DAT_00b6a244 < 1) {
        return 1;
    }
    iVar8 = 0;
LAB_004fd0af:
    pcVar4 = _fgets(DAT_00b69d20, 0xff, _File);
    if (pcVar4 == (char *)0) {
        DAT_00b6a244 = iVar9;
        FUN_004cc590(_File);
        return 0;
    }
    uVar7 = 0xffffffffu;
    pcVar4 = DAT_00b69d20;
    do {
        if (uVar7 == 0) break;
        uVar7 = uVar7 - 1;
        cVar1 = *pcVar4;
        pcVar4 = pcVar4 + 1;
    } while (cVar1 != '\0');
    if (DAT_00b69d20[~uVar7 - 2u] == '\n') {
        uVar7 = 0xffffffffu;
        pcVar4 = DAT_00b69d20;
        do {
            if (uVar7 == 0) break;
            uVar7 = uVar7 - 1;
            cVar1 = *pcVar4;
            pcVar4 = pcVar4 + 1;
        } while (cVar1 != '\0');
        DAT_00b69d20[~uVar7 - 2u] = 0;
    }
    uVar7 = 0;
    if (DAT_00aa6138 != 0) {
        iVar11 = 0;
        do {
            iVar5 = __strcmpi(DAT_00b69d20, (char *)DAT_00aa6114 + iVar11);
            if (iVar5 == 0) {
                bVar2 = 0;
                __builtin_memcpy(&local_bba6, DAT_00b6bba6, 4);
                if ((int)((unsigned int)local_bba6 - 1u) < 0) goto LAB_004fd15e;
                pbVar3 = DAT_00b6bbda + ((unsigned int)local_bba6 - 1u) * 4u;
                goto LAB_004fd149;
            }
            iVar11 = (int)((unsigned int)iVar11 + 0x148u);
            uVar7 = uVar7 + 1;
        } while (uVar7 < DAT_00aa6138);
    }
    goto LAB_004fd1a5;
    while (pbVar3 = pbVar3 - 4, (unsigned int)pbVar3 > 0xb6bbd9u) {
LAB_004fd149:
        __builtin_memcpy(&local_val, pbVar3, 4);
        if (local_val == iVar9) {
            bVar2 = 1;
            break;
        }
    }
LAB_004fd15e:
    puVar12 = (unsigned int *)((char *)DAT_00aa6114 + uVar7 * 0x148u);
    puVar14 = (unsigned int *)((char *)DAT_00b6a2b4 + iVar8);
    for (iVar11 = 0x52; iVar11 != 0; iVar11 = iVar11 - 1) {
        *puVar14 = *puVar12;
        puVar12 = puVar12 + 1;
        puVar14 = puVar14 + 1;
    }
    puVar12 = (unsigned int *)((char *)DAT_00b6a2b4 + iVar8 + 0xc0);
    for (iVar11 = 0x20; iVar11 != 0; iVar11 = iVar11 - 1) {
        *puVar12 = 0;
        puVar12 = puVar12 + 1;
    }
    if (bVar2) {
        *(unsigned int *)((char *)DAT_00b6a2b4 + iVar8 + 0x144) = 1;
    }
LAB_004fd1a5:
    iVar8 = (int)((unsigned int)iVar8 + 0x148u);
    iVar9 = (int)((unsigned int)iVar9 + 1u);
    if (DAT_00b6a244 <= iVar9) {
        return 1;
    }
    goto LAB_004fd0af;
}
