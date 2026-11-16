/* SPDX-License-Identifier: MIT */
/* z_xvt 004810f0 */
extern unsigned int DAT_00703ca0[0x1540];
extern int DAT_006da9e8;
extern char *DAT_00732a40[256];
extern char DAT_006db7c0[];
extern char DAT_0052a328[];
extern char DAT_00523570[];
extern void *DAT_00732948;

void __cdecl FUN_0047ed40(char *param_1, char *param_2, int param_3, int param_4);
unsigned int __cdecl FUN_00480ae0(char *param_1);
unsigned int __cdecl _fread(void *param_1, unsigned int param_2, unsigned int param_3, void *param_4);
int __cdecl _fclose(void *param_1);

unsigned int __cdecl FUN_004810f0(char *param_1)
{
    char cVar1;
    void *_File;
    unsigned int sVar3;
    unsigned int uVar4;
    unsigned int uVar5;
    int iVar6;
    unsigned int *puVar7;
    char **ppcVar7;
    char *pcVar8;
    char *pcVar9;
    char *pcVar10;
    unsigned int local_104;
    char local_100[256];

    uVar4 = 0xffffffff;
    pcVar8 = param_1;
    do {
        pcVar10 = pcVar8;
        if (uVar4 == 0) break;
        uVar4 = uVar4 - 1;
        pcVar10 = pcVar8 + 1;
        cVar1 = *pcVar8;
        pcVar8 = pcVar10;
    } while (cVar1 != '\0');
    uVar4 = ~uVar4;
    pcVar8 = pcVar10 - uVar4;
    pcVar10 = local_100;
    for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
        __builtin_memcpy(pcVar10, pcVar8, 4);
        pcVar8 = pcVar8 + 4;
        pcVar10 = pcVar10 + 4;
    }
    for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
        *pcVar10 = *pcVar8;
        pcVar8 = pcVar8 + 1;
        pcVar10 = pcVar10 + 1;
    }

    uVar4 = 0xffffffff;
    pcVar8 = DAT_0052a328;
    do {
        pcVar10 = pcVar8;
        if (uVar4 == 0) break;
        uVar4 = uVar4 - 1;
        pcVar10 = pcVar8 + 1;
        cVar1 = *pcVar8;
        pcVar8 = pcVar10;
    } while (cVar1 != '\0');
    uVar4 = ~uVar4;
    iVar6 = -1;
    pcVar8 = local_100;
    do {
        pcVar9 = pcVar8;
        if (iVar6 == 0) break;
        iVar6 = iVar6 + -1;
        pcVar9 = pcVar8 + 1;
        cVar1 = *pcVar8;
        pcVar8 = pcVar9;
    } while (cVar1 != '\0');
    pcVar8 = pcVar10 - uVar4;
    pcVar10 = pcVar9 - 1;
    for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
        __builtin_memcpy(pcVar10, pcVar8, 4);
        pcVar8 = pcVar8 + 4;
        pcVar10 = pcVar10 + 4;
    }
    for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
        *pcVar10 = *pcVar8;
        pcVar8 = pcVar8 + 1;
        pcVar10 = pcVar10 + 1;
    }

    puVar7 = DAT_00703ca0;
    for (iVar6 = 0x1540; iVar6 != 0; iVar6 = iVar6 + -1) {
        *puVar7 = 0;
        puVar7 = puVar7 + 1;
    }
    DAT_006da9e8 = 0;
    puVar7 = (unsigned int *)DAT_00732a40;
    for (iVar6 = 0x40; iVar6 != 0; iVar6 = iVar6 + -1) {
        *puVar7 = 0;
        puVar7 = puVar7 + 1;
    }

    FUN_0047ed40(local_100, DAT_00523570, 0, 1);
    _File = DAT_00732948;
    if (DAT_00732948 == (void *)0) {
        return FUN_00480ae0(param_1);
    }

    sVar3 = _fread(&local_104, 4, 1, DAT_00732948);
    if (sVar3 != 1) {
        _fclose(_File);
        return FUN_00480ae0(param_1);
    }
    sVar3 = _fread(DAT_00703ca0, local_104, 1, _File);
    if (sVar3 != 1) {
        _fclose(_File);
        return FUN_00480ae0(param_1);
    }
    sVar3 = _fread(&local_104, 4, 1, _File);
    if (sVar3 != 1) {
        _fclose(_File);
        return FUN_00480ae0(param_1);
    }
    sVar3 = _fread(DAT_006db7c0, local_104, 1, _File);
    if (sVar3 != 1) {
        _fclose(_File);
        return FUN_00480ae0(param_1);
    }
    _fclose(_File);

    pcVar8 = (char *)DAT_00703ca0;
    ppcVar7 = DAT_00732a40;
    do {
        if (*pcVar8 != '\0') {
            int off;
            DAT_006da9e8 = DAT_006da9e8 + 1;
            __builtin_memcpy(&off, pcVar8 + 0x51, 4);
            *ppcVar7 = DAT_006db7c0 + off;
        }
        pcVar8 = pcVar8 + 0x55;
        ppcVar7 = ppcVar7 + 1;
    } while (ppcVar7 < DAT_00732a40 + 256);
    return 1;
}
