/* SPDX-License-Identifier: MIT */
/* z_xvt 004c9370 */
extern char DAT_0085e731[];
extern char DAT_0085e770;
extern char DAT_0085e771[];
extern char DAT_0085e6f0;
extern char DAT_0085e78f;
extern int DAT_0081b6dc;
extern int DAT_0081b6e8;
extern char s_joiner_00532308[];
extern char DAT_00532300[];
extern char *PTR_DAT_00531c78[];

char *__cdecl _strncpy(char *, const char *, unsigned int);
int __cdecl FUN_004c0640(char *param_1);
void __cdecl FUN_004c02a0(char *param_1);
DWORD __stdcall GetTickCount(void);
void __cdecl _srand(unsigned int);
int __cdecl _rand(void);

unsigned int __cdecl FUN_004c9370(char *param_1)
{
    char cVar1;
    int bVar2;
    DWORD _Seed;
    unsigned int uVar3;
    int iVar4;
    int iVar5;
    unsigned int uVar6;
    char *pcVar7;
    int iVar8;
    char *pcVar9;
    int bVar10;
    unsigned int uVal;
    char local_110[16];
    char local_100[256];

    bVar2 = 0;
    uVar3 = 0xffffffff;
    bVar10 = 0;
    pcVar7 = param_1;
    do {
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
    } while (cVar1 != '\0');
    iVar8 = 0;
    iVar5 = (int)(~uVar3 - 1);
    if (0 < iVar5) {
        do {
            if (param_1[iVar8] == '\"') {
                cVar1 = param_1[iVar8 + 1];
                iVar8 = iVar8 + 1;
                for (iVar4 = 0; ((cVar1 != '\"' && (iVar8 < iVar5)) && (iVar4 < 0xff)); iVar4 = iVar4 + 1) {
                    pcVar7 = param_1 + iVar8;
                    iVar8 = iVar8 + 1;
                    local_100[iVar4] = *pcVar7;
                    cVar1 = param_1[iVar8];
                }
                local_100[iVar4] = 0;
                if ((local_100[0] == 1) && (local_100[1] == '=')) {
                    bVar2 = 1;
                    _strncpy(DAT_0085e731, &local_100[2], 0x3f);
                    DAT_0085e770 = 0;
                } else {
                    local_100[0] = (char)(local_100[1] == '=');
                    if (local_100[1] == '=') {
                        _strncpy(local_110, &local_100[2], 0xc);
                        local_110[12] = 0;
                        bVar10 = 1;
                    }
                }
            }
            iVar8 = iVar8 + 1;
        } while (iVar8 < iVar5);
    }
    if (!bVar10) {
        DAT_0081b6dc = 0;
        DAT_0081b6e8 = 0;
        return 1;
    }
    if (DAT_0085e771[0] == '\0') {
        iVar5 = 7;
        bVar10 = 1;
        pcVar7 = local_110;
        pcVar9 = s_joiner_00532308;
        do {
            if (iVar5 == 0) break;
            iVar5 = iVar5 + -1;
            bVar10 = (*pcVar7 == *pcVar9);
            pcVar7 = pcVar7 + 1;
            pcVar9 = pcVar9 + 1;
        } while (bVar10);
        if (bVar10) goto LAB_004c94bc;
        iVar5 = 5;
        bVar10 = 0;
        pcVar7 = local_110;
        pcVar9 = DAT_00532300;
        do {
            if (iVar5 == 0) break;
            iVar5 = iVar5 + -1;
            bVar10 = (*pcVar7 == *pcVar9);
            pcVar7 = pcVar7 + 1;
            pcVar9 = pcVar9 + 1;
        } while (bVar10);
        if (bVar10) goto LAB_004c94bc;
        pcVar7 = local_110;
    } else {
        iVar5 = FUN_004c0640(DAT_0085e771);
        if (iVar5 != 0) goto LAB_004c950d;
        iVar5 = 7;
        bVar10 = 1;
        pcVar7 = local_110;
        pcVar9 = s_joiner_00532308;
        do {
            if (iVar5 == 0) break;
            iVar5 = iVar5 + -1;
            bVar10 = (*pcVar7 == *pcVar9);
            pcVar7 = pcVar7 + 1;
            pcVar9 = pcVar9 + 1;
        } while (bVar10);
        if (!bVar10) {
            iVar5 = 5;
            pcVar7 = local_110;
            pcVar9 = DAT_00532300;
            do {
                if (iVar5 == 0) break;
                iVar5 = iVar5 + -1;
                bVar10 = (*pcVar7 == *pcVar9);
                pcVar7 = pcVar7 + 1;
                pcVar9 = pcVar9 + 1;
            } while (bVar10);
            if (!bVar10) {
                pcVar7 = local_110;
                goto LAB_004c94de;
            }
        }
LAB_004c94bc:
        _Seed = GetTickCount();
        _srand(_Seed);
        iVar5 = _rand();
        pcVar7 = PTR_DAT_00531c78[iVar5 % 0x28];
    }
LAB_004c94de:
    uVar3 = 0xffffffff;
    do {
        pcVar9 = pcVar7;
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        pcVar9 = pcVar7 + 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar9;
    } while (cVar1 != '\0');
    uVar3 = ~uVar3;
    pcVar7 = pcVar9 - uVar3;
    pcVar9 = DAT_0085e771;
    for (uVar6 = uVar3 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        __builtin_memcpy(&uVal, pcVar7, 4);
        __builtin_memcpy(pcVar9, &uVal, 4);
        pcVar7 = pcVar7 + 4;
        pcVar9 = pcVar9 + 4;
    }
    for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar9 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar9 = pcVar9 + 1;
    }
    FUN_004c02a0(DAT_0085e771);
LAB_004c950d:
    if (!bVar2) {
        DAT_0081b6e8 = 0;
        DAT_0081b6dc = 0;
        return 1;
    }
    DAT_0085e6f0 = 1;
    DAT_0085e78f = 1;
    return 1;
}
