/* SPDX-License-Identifier: MIT */
/* z_bop 0046b3d0 */
/* Prototypes */
int __cdecl FUN_0049e720(char *param_1, char *param_2, int param_3, int param_4);
unsigned int __cdecl FUN_0046aed0(char *param_1);
unsigned int __cdecl _fread(void *_DstBuf, unsigned int _ElementSize, unsigned int _Count, void *_File);
int __cdecl _fclose(void *_File);

/* Globals */
extern char DAT_00527008[];
extern char DAT_0051a854[];
extern unsigned int DAT_009d1320[0x1540];
extern int DAT_009a8068;
extern unsigned int DAT_00a07cf0[256];
extern void *DAT_00a07bd8;
extern unsigned char DAT_009a8e40[];

unsigned int __cdecl FUN_0046b3d0(char *param_1)
{
    char cVar1;
    void *_File;
    unsigned int uVar2;
    unsigned int sVar3;
    unsigned int *puVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    int iVar7;
    char *pcVar8;
    char *pcVar9;
    char *pcVar10;
    unsigned int local_104;
    char local_100[256];
    unsigned int uTmp;

    uVar5 = 0xffffffffUL;
    pcVar8 = param_1;
    do {
        pcVar10 = pcVar8;
        if (uVar5 == 0) break;
        uVar5 = uVar5 - 1;
        pcVar10 = pcVar8 + 1;
        cVar1 = *pcVar8;
        pcVar8 = pcVar10;
    } while (cVar1 != '\0');
    uVar5 = ~uVar5;
    pcVar8 = pcVar10 - uVar5;
    pcVar10 = local_100;
    for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        __builtin_memcpy(&uTmp, pcVar8, 4);
        __builtin_memcpy(pcVar10, &uTmp, 4);
        pcVar8 = pcVar8 + 4;
        pcVar10 = pcVar10 + 4;
    }
    for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
        *pcVar10 = *pcVar8;
        pcVar8 = pcVar8 + 1;
        pcVar10 = pcVar10 + 1;
    }
    uVar5 = 0xffffffffUL;
    pcVar8 = DAT_00527008;
    do {
        pcVar10 = pcVar8;
        if (uVar5 == 0) break;
        uVar5 = uVar5 - 1;
        pcVar10 = pcVar8 + 1;
        cVar1 = *pcVar8;
        pcVar8 = pcVar10;
    } while (cVar1 != '\0');
    uVar5 = ~uVar5;
    iVar7 = -1;
    pcVar8 = local_100;
    do {
        pcVar9 = pcVar8;
        if (iVar7 == 0) break;
        iVar7 = iVar7 + -1;
        pcVar9 = pcVar8 + 1;
        cVar1 = *pcVar8;
        pcVar8 = pcVar9;
    } while (cVar1 != '\0');
    pcVar8 = pcVar10 - uVar5;
    pcVar10 = pcVar9 - 1;
    for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        __builtin_memcpy(&uTmp, pcVar8, 4);
        __builtin_memcpy(pcVar10, &uTmp, 4);
        pcVar8 = pcVar8 + 4;
        pcVar10 = pcVar10 + 4;
    }
    for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
        *pcVar10 = *pcVar8;
        pcVar8 = pcVar8 + 1;
        pcVar10 = pcVar10 + 1;
    }
    puVar4 = DAT_009d1320;
    for (iVar7 = 0x1540; iVar7 != 0; iVar7 = iVar7 + -1) {
        *puVar4 = 0;
        puVar4 = puVar4 + 1;
    }
    DAT_009a8068 = 0;
    puVar4 = DAT_00a07cf0;
    for (iVar7 = 0x40; iVar7 != 0; iVar7 = iVar7 + -1) {
        *puVar4 = 0;
        puVar4 = puVar4 + 1;
    }
    FUN_0049e720(local_100, DAT_0051a854, 0, 1);
    _File = DAT_00a07bd8;
    if (DAT_00a07bd8 == 0) {
        uVar2 = FUN_0046aed0(param_1);
        return uVar2;
    }
    sVar3 = _fread(&local_104, 4, 1, DAT_00a07bd8);
    if (sVar3 != 1) {
        _fclose(_File);
        uVar2 = FUN_0046aed0(param_1);
        return uVar2;
    }
    sVar3 = _fread(DAT_009d1320, local_104, 1, _File);
    if (sVar3 != 1) {
        _fclose(_File);
        uVar2 = FUN_0046aed0(param_1);
        return uVar2;
    }
    sVar3 = _fread(&local_104, 4, 1, _File);
    if (sVar3 != 1) {
        _fclose(_File);
        uVar2 = FUN_0046aed0(param_1);
        return uVar2;
    }
    sVar3 = _fread(DAT_009a8e40, local_104, 1, _File);
    if (sVar3 != 1) {
        _fclose(_File);
        uVar2 = FUN_0046aed0(param_1);
        return uVar2;
    }
    _fclose(_File);
    pcVar8 = (char *)DAT_009d1320;
    puVar4 = DAT_00a07cf0;
    do {
        if (*pcVar8 != '\0') {
            DAT_009a8068 = DAT_009a8068 + 1;
            __builtin_memcpy(&iVar7, pcVar8 + 0x51, 4);
            *puVar4 = (unsigned int)(DAT_009a8e40 + iVar7);
        }
        pcVar8 = pcVar8 + 0x55;
        puVar4 = puVar4 + 1;
    } while (puVar4 < DAT_00a07cf0 + 256);
    return 1;
}
