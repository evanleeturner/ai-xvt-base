/* SPDX-License-Identifier: MIT */
/* z_bop 004bf650 */
typedef struct _iobuf FILE;

extern char DAT_00b6a2e0[];
extern char DAT_00b69d20[];
extern char DAT_0051a854[];
extern char DAT_00527004[];
extern char s___pl2_0052b398[];
extern char s___plt_0052b35c[];
extern char s___temp___tmp_0052b388[];
extern char s__s0_pl2_0052b380[];
extern char s__s0_plt_0052b378[];

unsigned int *FUN_004dfa10(char *param_1);
FILE *FUN_004cc4a0(char *param_1, char *param_2);
void FUN_004cc6c0(FILE *param_1, void *param_2, int param_3);
void FUN_004cc590(FILE *param_1);
int __cdecl __strcmpi(const char *param_1, const char *param_2);
void FUN_004dfc30(unsigned int *param_1);
void FUN_004ccf50(void);
void FUN_004ccf70(void);
int __cdecl _sprintf(char *param_1, const char *param_2, ...);
void FUN_004cc790(FILE *param_1, void *param_2, int param_3);
void FUN_004cb310(char *param_1, FILE *param_2);

unsigned int __cdecl FUN_004bf650(int param_1)
{
    char cVar1;
    unsigned int *puVar2;
    FILE *pFVar3;
    int iVar4;
    int iVar5;
    unsigned int uVar6;
    unsigned int uVar7;
    unsigned int *puVar8;
    char *pcVar9;
    char *pcVar10;
    char local_20[30];

    if (DAT_00b6a2e0[0] == '\0') {
        return 0;
    }
    pcVar9 = local_20;
    for (iVar5 = 7; iVar5 != 0; iVar5 = iVar5 + -1) {
        pcVar9[0] = '\0';
        pcVar9[1] = '\0';
        pcVar9[2] = '\0';
        pcVar9[3] = '\0';
        pcVar9 = pcVar9 + 4;
    }
    pcVar9[0] = '\0';
    pcVar9[1] = '\0';
    if (param_1 == 0) {
        puVar2 = FUN_004dfa10(s___pl2_0052b398);
        if (puVar2 != (unsigned int *)0) {
            puVar8 = (unsigned int *)*puVar2;
            iVar5 = 0;
            if (0 < (int)puVar2[1]) {
                do {
                    pFVar3 = FUN_004cc4a0((char *)*puVar8, DAT_0051a854);
                    if (pFVar3 != (FILE *)0) {
                        FUN_004cc6c0(pFVar3, DAT_00b69d20, 0xe);
                        FUN_004cc590(pFVar3);
                        iVar4 = __strcmpi(DAT_00b69d20, DAT_00b6a2e0);
                        if (iVar4 == 0) {
                            uVar7 = 0xffffffff;
                            pcVar9 = (char *)*puVar8;
                            goto code_r0x004bf6fc;
                        }
                    }
                    puVar8 = (unsigned int *)puVar8[1];
                    iVar5 = iVar5 + 1;
                } while (iVar5 < (int)puVar2[1]);
            }
            goto LAB_004bf716;
        }
    } else {
        local_20[0] = s___temp___tmp_0052b388[0];
        local_20[1] = s___temp___tmp_0052b388[1];
        local_20[2] = s___temp___tmp_0052b388[2];
        local_20[3] = s___temp___tmp_0052b388[3];
        local_20[4] = s___temp___tmp_0052b388[4];
        local_20[5] = s___temp___tmp_0052b388[5];
        local_20[6] = s___temp___tmp_0052b388[6];
        local_20[7] = s___temp___tmp_0052b388[7];
        local_20[8] = s___temp___tmp_0052b388[8];
        local_20[9] = s___temp___tmp_0052b388[9];
        local_20[10] = s___temp___tmp_0052b388[10];
        local_20[11] = s___temp___tmp_0052b388[11];
        local_20[12] = s___temp___tmp_0052b388[12];
    }
    goto LAB_004bf749;

    while (1) {
        uVar7 = uVar7 - 1;
        pcVar10 = pcVar9 + 1;
        cVar1 = *pcVar9;
        pcVar9 = pcVar10;
        if (cVar1 == '\0') break;
code_r0x004bf7d7:
        pcVar10 = pcVar9;
        if (uVar7 == 0) break;
    }
    uVar7 = ~uVar7;
    pcVar9 = pcVar10 + -uVar7;
    pcVar10 = local_20;
    for (uVar6 = uVar7 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        pcVar10[0] = pcVar9[0];
        pcVar10[1] = pcVar9[1];
        pcVar10[2] = pcVar9[2];
        pcVar10[3] = pcVar9[3];
        pcVar9 = pcVar9 + 4;
        pcVar10 = pcVar10 + 4;
    }
    for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
        *pcVar10 = *pcVar9;
        pcVar9 = pcVar9 + 1;
        pcVar10 = pcVar10 + 1;
    }
    goto LAB_004bf7f1;

    while (1) {
        uVar7 = uVar7 - 1;
        pcVar10 = pcVar9 + 1;
        cVar1 = *pcVar9;
        pcVar9 = pcVar10;
        if (cVar1 == '\0') break;
code_r0x004bf6fc:
        pcVar10 = pcVar9;
        if (uVar7 == 0) break;
    }
    uVar7 = ~uVar7;
    pcVar9 = pcVar10 + -uVar7;
    pcVar10 = local_20;
    for (uVar6 = uVar7 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        pcVar10[0] = pcVar9[0];
        pcVar10[1] = pcVar9[1];
        pcVar10[2] = pcVar9[2];
        pcVar10[3] = pcVar9[3];
        pcVar9 = pcVar9 + 4;
        pcVar10 = pcVar10 + 4;
    }
    for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
        *pcVar10 = *pcVar9;
        pcVar9 = pcVar9 + 1;
        pcVar10 = pcVar10 + 1;
    }
LAB_004bf716:
    FUN_004dfc30(puVar2);
LAB_004bf749:
    if (local_20[0] == '\0') {
        FUN_004ccf50();
        puVar2 = FUN_004dfa10(s___plt_0052b35c);
        FUN_004ccf70();
        if (puVar2 != (unsigned int *)0) {
            puVar8 = (unsigned int *)*puVar2;
            iVar5 = 0;
            if (0 < (int)puVar2[1]) {
                do {
                    pFVar3 = FUN_004cc4a0((char *)*puVar8, DAT_0051a854);
                    if (pFVar3 != (FILE *)0) {
                        FUN_004cc6c0(pFVar3, DAT_00b69d20, 0xe);
                        FUN_004cc590(pFVar3);
                        iVar4 = __strcmpi(DAT_00b69d20, DAT_00b6a2e0);
                        if (iVar4 == 0) {
                            uVar7 = 0xffffffff;
                            pcVar9 = (char *)*puVar8;
                            goto code_r0x004bf7d7;
                        }
                    }
                    puVar8 = (unsigned int *)puVar8[1];
                    iVar5 = iVar5 + 1;
                } while (iVar5 < (int)puVar2[1]);
            }
LAB_004bf7f1:
            FUN_004dfc30(puVar2);
        }
        if (local_20[0] == '\0') {
            _sprintf(local_20, s__s0_pl2_0052b380, DAT_00b6a2e0);
        } else {
            uVar7 = 0xffffffff;
            pcVar9 = local_20;
            do {
                if (uVar7 == 0) break;
                uVar7 = uVar7 - 1;
                cVar1 = *pcVar9;
                pcVar9 = pcVar9 + 1;
            } while (cVar1 != '\0');
            local_20[~uVar7 - 2] = 0x32;
        }
    }
    pFVar3 = FUN_004cc4a0(local_20, DAT_00527004);
    if (pFVar3 == (FILE *)0) {
        return 0;
    }
    FUN_004cc790(pFVar3, DAT_00b6a2e0, 0x4852e);
    FUN_004cc590(pFVar3);
    if (local_20[0] == '\0') {
        _sprintf(local_20, s__s0_plt_0052b378, DAT_00b6a2e0);
    } else {
        uVar7 = 0xffffffff;
        pcVar9 = local_20;
        do {
            if (uVar7 == 0) break;
            uVar7 = uVar7 - 1;
            cVar1 = *pcVar9;
            pcVar9 = pcVar9 + 1;
        } while (cVar1 != '\0');
        local_20[~uVar7 - 2] = 0x74;
    }
    FUN_004cb310(local_20, pFVar3);
    return 1;
}
