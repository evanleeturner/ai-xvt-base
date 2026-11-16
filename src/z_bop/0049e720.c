/* SPDX-License-Identifier: MIT */
/* z_bop 0049e720 */
typedef struct _iobuf FILE;

extern char DAT_009d8a60[];
extern char DAT_0052772c[4];
extern char DAT_00527744[];
extern char s_D__BalanceOfPower__00527730[19];
extern FILE *DAT_00a07bd8;

FILE *FID_conflict___wfopen(char *param_1, char *param_2);
char *FUN_004ccf40(void);
char FUN_004ccb90(void);
char FUN_0049e060(void);
void FUN_0049eb60(int param_1);

unsigned int __cdecl FUN_0049e720(char *param_1, char *param_2, int param_3, int param_4)
{
    char cVar2;
    char *pcVar3;
    unsigned int uVar4;
    unsigned int uVar5;
    int iVar6;
    char *pcVar7;
    char *pcVar8;

    uVar4 = 0xffffffff;
    pcVar3 = param_1;
    do {
        pcVar8 = pcVar3;
        if (uVar4 == 0) break;
        uVar4 = uVar4 - 1;
        pcVar8 = pcVar3 + 1;
        cVar2 = *pcVar3;
        pcVar3 = pcVar8;
    } while (cVar2 != '\0');
    uVar4 = ~uVar4;
    pcVar3 = pcVar8 - uVar4;
    pcVar8 = DAT_009d8a60;
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
    do {
        iVar6 = 4;
        if (param_4 != 2) {
            while (iVar6 != 0) {
                DAT_00a07bd8 = FID_conflict___wfopen(param_1, param_2);
                iVar6 = iVar6 + -1;
                if (DAT_00a07bd8 != (FILE *)0) {
                    return 1;
                }
            }
            pcVar3 = FUN_004ccf40();
            uVar4 = 0xffffffff;
            do {
                pcVar8 = pcVar3;
                if (uVar4 == 0) break;
                uVar4 = uVar4 - 1;
                pcVar8 = pcVar3 + 1;
                cVar2 = *pcVar3;
                pcVar3 = pcVar8;
            } while (cVar2 != '\0');
            uVar4 = ~uVar4;
            pcVar3 = pcVar8 - uVar4;
            pcVar8 = DAT_009d8a60;
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
            uVar4 = 0xffffffff;
            pcVar3 = DAT_00527744;
            do {
                pcVar8 = pcVar3;
                if (uVar4 == 0) break;
                uVar4 = uVar4 - 1;
                pcVar8 = pcVar3 + 1;
                cVar2 = *pcVar3;
                pcVar3 = pcVar8;
            } while (cVar2 != '\0');
            uVar4 = ~uVar4;
            iVar6 = -1;
            pcVar3 = DAT_009d8a60;
            do {
                pcVar7 = pcVar3;
                if (iVar6 == 0) break;
                iVar6 = iVar6 + -1;
                pcVar7 = pcVar3 + 1;
                cVar2 = *pcVar3;
                pcVar3 = pcVar7;
            } while (cVar2 != '\0');
            pcVar3 = pcVar8 - uVar4;
            pcVar8 = pcVar7 - 1;
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
            uVar4 = 0xffffffff;
            pcVar3 = param_1;
            do {
                pcVar8 = pcVar3;
                if (uVar4 == 0) break;
                uVar4 = uVar4 - 1;
                pcVar8 = pcVar3 + 1;
                cVar2 = *pcVar3;
                pcVar3 = pcVar8;
            } while (cVar2 != '\0');
            uVar4 = ~uVar4;
            iVar6 = -1;
            pcVar3 = DAT_009d8a60;
            do {
                pcVar7 = pcVar3;
                if (iVar6 == 0) break;
                iVar6 = iVar6 + -1;
                pcVar7 = pcVar3 + 1;
                cVar2 = *pcVar3;
                pcVar3 = pcVar7;
            } while (cVar2 != '\0');
            pcVar3 = pcVar8 - uVar4;
            pcVar8 = pcVar7 - 1;
            for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                __builtin_memcpy(pcVar8, pcVar3, 4);
                pcVar3 = pcVar3 + 4;
                pcVar8 = pcVar8 + 4;
            }
            for (uVar4 = uVar4 & 3; iVar6 = 4, uVar4 != 0; uVar4 = uVar4 - 1) {
                *pcVar8 = *pcVar3;
                pcVar3 = pcVar3 + 1;
                pcVar8 = pcVar8 + 1;
            }
            while (iVar6 != 0) {
                DAT_00a07bd8 = FID_conflict___wfopen(DAT_009d8a60, param_2);
                iVar6 = iVar6 + -1;
                if (DAT_00a07bd8 != (FILE *)0) {
                    return 1;
                }
            }
        }
        if (param_4 != 1) {
            DAT_009d8a60[0] = s_D__BalanceOfPower__00527730[0];
            DAT_009d8a60[1] = s_D__BalanceOfPower__00527730[1];
            DAT_009d8a60[2] = s_D__BalanceOfPower__00527730[2];
            DAT_009d8a60[3] = s_D__BalanceOfPower__00527730[3];
            DAT_009d8a60[4] = s_D__BalanceOfPower__00527730[4];
            DAT_009d8a60[5] = s_D__BalanceOfPower__00527730[5];
            DAT_009d8a60[6] = s_D__BalanceOfPower__00527730[6];
            DAT_009d8a60[7] = s_D__BalanceOfPower__00527730[7];
            DAT_009d8a60[8] = s_D__BalanceOfPower__00527730[8];
            DAT_009d8a60[9] = s_D__BalanceOfPower__00527730[9];
            DAT_009d8a60[10] = s_D__BalanceOfPower__00527730[10];
            DAT_009d8a60[11] = s_D__BalanceOfPower__00527730[0xb];
            DAT_009d8a60[12] = s_D__BalanceOfPower__00527730[0xc];
            DAT_009d8a60[13] = s_D__BalanceOfPower__00527730[0xd];
            DAT_009d8a60[14] = s_D__BalanceOfPower__00527730[0xe];
            DAT_009d8a60[15] = s_D__BalanceOfPower__00527730[0xf];
            DAT_009d8a60[16] = s_D__BalanceOfPower__00527730[0x10];
            DAT_009d8a60[17] = s_D__BalanceOfPower__00527730[0x11];
            DAT_009d8a60[18] = s_D__BalanceOfPower__00527730[0x12];
            DAT_009d8a60[0] = FUN_004ccb90();
            uVar4 = 0xffffffff;
            pcVar3 = param_1;
            do {
                pcVar8 = pcVar3;
                if (uVar4 == 0) break;
                uVar4 = uVar4 - 1;
                pcVar8 = pcVar3 + 1;
                cVar2 = *pcVar3;
                pcVar3 = pcVar8;
            } while (cVar2 != '\0');
            uVar4 = ~uVar4;
            iVar6 = -1;
            pcVar3 = DAT_009d8a60;
            do {
                pcVar7 = pcVar3;
                if (iVar6 == 0) break;
                iVar6 = iVar6 + -1;
                pcVar7 = pcVar3 + 1;
                cVar2 = *pcVar3;
                pcVar3 = pcVar7;
            } while (cVar2 != '\0');
            pcVar3 = pcVar8 - uVar4;
            pcVar8 = pcVar7 - 1;
            for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                __builtin_memcpy(pcVar8, pcVar3, 4);
                pcVar3 = pcVar3 + 4;
                pcVar8 = pcVar8 + 4;
            }
            for (uVar4 = uVar4 & 3; iVar6 = 4, uVar4 != 0; uVar4 = uVar4 - 1) {
                *pcVar8 = *pcVar3;
                pcVar3 = pcVar3 + 1;
                pcVar8 = pcVar8 + 1;
            }
            while (iVar6 != 0) {
                DAT_00a07bd8 = FID_conflict___wfopen(DAT_009d8a60, param_2);
                iVar6 = iVar6 + -1;
                if (DAT_00a07bd8 != (FILE *)0) {
                    return 1;
                }
            }
            __builtin_memcpy(DAT_009d8a60, DAT_0052772c, 4);
            DAT_009d8a60[0] = FUN_004ccb90();
            uVar4 = 0xffffffff;
            pcVar3 = param_1;
            do {
                pcVar8 = pcVar3;
                if (uVar4 == 0) break;
                uVar4 = uVar4 - 1;
                pcVar8 = pcVar3 + 1;
                cVar2 = *pcVar3;
                pcVar3 = pcVar8;
            } while (cVar2 != '\0');
            uVar4 = ~uVar4;
            iVar6 = -1;
            pcVar3 = DAT_009d8a60;
            do {
                pcVar7 = pcVar3;
                if (iVar6 == 0) break;
                iVar6 = iVar6 + -1;
                pcVar7 = pcVar3 + 1;
                cVar2 = *pcVar3;
                pcVar3 = pcVar7;
            } while (cVar2 != '\0');
            pcVar3 = pcVar8 - uVar4;
            pcVar8 = pcVar7 - 1;
            for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                __builtin_memcpy(pcVar8, pcVar3, 4);
                pcVar3 = pcVar3 + 4;
                pcVar8 = pcVar8 + 4;
            }
            for (uVar4 = uVar4 & 3; iVar6 = 4, uVar4 != 0; uVar4 = uVar4 - 1) {
                *pcVar8 = *pcVar3;
                pcVar3 = pcVar3 + 1;
                pcVar8 = pcVar8 + 1;
            }
            while (iVar6 != 0) {
                DAT_00a07bd8 = FID_conflict___wfopen(DAT_009d8a60, param_2);
                iVar6 = iVar6 + -1;
                if (DAT_00a07bd8 != (FILE *)0) {
                    return 1;
                }
            }
        }
        if (param_3 == 0) {
            DAT_00a07bd8 = (FILE *)0;
            return 0;
        }
        while ((cVar2 = FUN_0049e060(), cVar2 != 'R') && (cVar2 != 'r')) {
            if ((cVar2 == 'F') || (cVar2 == 'f')) {
                FUN_0049eb60(1);
                DAT_00a07bd8 = (FILE *)0;
                return 0;
            }
        }
    } while (1);
}
