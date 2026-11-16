/* SPDX-License-Identifier: MIT */
/* z_xvt 0051a1e0 */
extern char DAT_00742728[];
extern char DAT_0052a330[];

char *__cdecl __getcwd(char *, int);
void *__cdecl _fopen(const char *, const char *);
int __cdecl _fclose(void *);

unsigned int __cdecl FUN_0051a1e0(int param_1, char *param_2, int param_3)
{
    char cVar1;
    char bVar2;
    char *pcVar3;
    DWORD nBufferLength;
    UINT UVar4;
    void *_File;
    char *pcVar5;
    unsigned int uVar6;
    unsigned int uVar7;
    int iVar8;
    int iVar9;
    LPCSTR pCVar10;
    char *pcVar11;
    char *pcVar12;
    DWORD local_1b0;
    DWORD local_1ac;
    DWORD local_1a8;
    char local_1a4[80];
    char local_154[260];
    CHAR local_50[80];

    bVar2 = 0;
    if (param_3 == 1) {
        pcVar3 = __getcwd(local_154, 0x104);
        if (pcVar3 != (char *)0) {
            DAT_00742728[1] = 0x3a;
            DAT_00742728[2] = 0;
            DAT_00742728[0] = local_154[0];
            return 1;
        }
        return 0;
    }
    nBufferLength = GetLogicalDriveStringsA(0, (LPSTR)0);
    iVar9 = 0;
    GetLogicalDriveStringsA(nBufferLength, local_154);
    pcVar3 = local_154;
    if (0 < (int)nBufferLength) {
LAB_0051a272:
        UVar4 = GetDriveTypeA(pcVar3);
        if (UVar4 != 5) goto LAB_0051a2ec;
        uVar6 = 0xffffffff;
        pCVar10 = pcVar3;
        do {
            pcVar5 = (char *)pCVar10;
            if (uVar6 == 0) break;
            uVar6 = uVar6 - 1;
            pcVar5 = (char *)pCVar10 + 1;
            cVar1 = *pCVar10;
            pCVar10 = pcVar5;
        } while (cVar1 != '\0');
        uVar6 = ~uVar6;
        pcVar5 = pcVar5 + -uVar6;
        pcVar12 = local_1a4;
        for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
            __builtin_memcpy(pcVar12, pcVar5, 4);
            pcVar5 = pcVar5 + 4;
            pcVar12 = pcVar12 + 4;
        }
        for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
            *pcVar12 = *pcVar5;
            pcVar5 = pcVar5 + 1;
            pcVar12 = pcVar12 + 1;
        }
        uVar6 = 0xffffffff;
        pcVar5 = (char *)(param_1 + 1);
        do {
            pcVar12 = pcVar5;
            if (uVar6 == 0) break;
            uVar6 = uVar6 - 1;
            pcVar12 = pcVar5 + 1;
            cVar1 = *pcVar5;
            pcVar5 = pcVar12;
        } while (cVar1 != '\0');
        uVar6 = ~uVar6;
        iVar8 = -1;
        pcVar5 = local_1a4;
        do {
            pcVar11 = pcVar5;
            if (iVar8 == 0) break;
            iVar8 = iVar8 + -1;
            pcVar11 = pcVar5 + 1;
            cVar1 = *pcVar5;
            pcVar5 = pcVar11;
        } while (cVar1 != '\0');
        pcVar5 = pcVar12 + -uVar6;
        pcVar12 = pcVar11 + -1;
        for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
            __builtin_memcpy(pcVar12, pcVar5, 4);
            pcVar5 = pcVar5 + 4;
            pcVar12 = pcVar12 + 4;
        }
        for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
            *pcVar12 = *pcVar5;
            pcVar5 = pcVar5 + 1;
            pcVar12 = pcVar12 + 1;
        }
        _File = _fopen(local_1a4, DAT_0052a330);
        if (_File == (void *)0) goto LAB_0051a2ec;
        _fclose(_File);
        if (param_2 == (char *)0) goto LAB_0051a373;
        pcVar5 = local_1a4;
        for (iVar9 = 0x14; iVar9 != 0; iVar9 = iVar9 + -1) {
            pcVar5[0] = '\0';
            pcVar5[1] = '\0';
            pcVar5[2] = '\0';
            pcVar5[3] = '\0';
            pcVar5 = pcVar5 + 4;
        }
        GetVolumeInformationA(pcVar3, local_1a4, 0x50, &local_1b0, &local_1ac, &local_1a8, local_50, 0x50);
        pcVar5 = local_1a4;
        do {
            if (((*pcVar5 != *param_2) || (*pcVar5 == '\0')) ||
                (pcVar12 = pcVar5 + 1, *pcVar12 != param_2[1])) break;
            pcVar5 = pcVar5 + 2;
            param_2 = param_2 + 2;
        } while (*pcVar12 != '\0');
LAB_0051a373:
        bVar2 = 1;
    }
LAB_0051a37b:
    if (!bVar2) {
        return 0;
    }
    uVar6 = 0xffffffff;
    do {
        pcVar5 = pcVar3;
        if (uVar6 == 0) break;
        uVar6 = uVar6 - 1;
        pcVar5 = pcVar3 + 1;
        cVar1 = *pcVar3;
        pcVar3 = pcVar5;
    } while (cVar1 != '\0');
    uVar6 = ~uVar6;
    pcVar3 = pcVar5 + -uVar6;
    pcVar5 = DAT_00742728;
    for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
        __builtin_memcpy(pcVar5, pcVar3, 4);
        pcVar3 = pcVar3 + 4;
        pcVar5 = pcVar5 + 4;
    }
    for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
        *pcVar5 = *pcVar3;
        pcVar3 = pcVar3 + 1;
        pcVar5 = pcVar5 + 1;
    }
    DAT_00742728[2] = 0;
    return 1;
LAB_0051a2ec:
    pcVar3 = pcVar3 + 4;
    iVar9 = iVar9 + 4;
    if ((int)nBufferLength <= iVar9) goto LAB_0051a37b;
    goto LAB_0051a272;
}
