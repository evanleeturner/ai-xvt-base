/* SPDX-License-Identifier: MIT */
/* z_bop 004f8950 */
extern unsigned int DAT_00aa5c10[];
extern unsigned int DAT_00aa5c2c[];

int  __cdecl FUN_004dbfa0(char *a, int b);
void __cdecl FUN_004db4e0(int a, char *b, int c, int d, unsigned int e);
void __cdecl FUN_004d6460(int *a, int b, int c, int d, int e);
void __cdecl FUN_004d67e0(int *a, int b, int c, short d, int e);

void __cdecl
FUN_004f8950(char *param_1, unsigned int param_2, short param_3, short param_4,
             short param_5, short param_6)
{
    char cVar1;
    short sVar2;
    short sVar3;
    int iVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    int iVar7;
    short sVar8;
    char *pcVar9;
    short sVar10;
    char *pcVar11;
    int local_50[4];
    char local_40[64];

    if (-1 < param_5) {
        iVar4 = -1;
        pcVar9 = param_1;
        do {
            if (iVar4 == 0) break;
            iVar4 = iVar4 + -1;
            cVar1 = *pcVar9;
            pcVar9 = pcVar9 + 1;
        } while (cVar1 != '\0');
        sVar3 = (short)(~(unsigned short)iVar4 - 1);
        uVar5 = 0xffffffff;
        pcVar9 = param_1;
        do {
            pcVar11 = pcVar9;
            if (uVar5 == 0) break;
            uVar5 = uVar5 - 1;
            pcVar11 = pcVar9 + 1;
            cVar1 = *pcVar9;
            pcVar9 = pcVar11;
        } while (cVar1 != '\0');
        uVar5 = ~uVar5;
        pcVar9 = pcVar11 + -uVar5;
        pcVar11 = local_40;
        for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
            __builtin_memcpy(pcVar11, pcVar9, 4);
            pcVar9 = pcVar9 + 4;
            pcVar11 = pcVar11 + 4;
        }
        for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
            *pcVar11 = *pcVar9;
            pcVar9 = pcVar9 + 1;
            pcVar11 = pcVar11 + 1;
        }
        iVar4 = (int)param_5;
        sVar2 = (short)(param_6 * 8);
        if (iVar4 < sVar3 + 2) {
            sVar8 = sVar3;
            if (param_5 <= sVar3) {
                local_40[iVar4] = '\0';
                sVar8 = param_5;
            }
            sVar10 = param_5;
            if (3 < param_5) {
                sVar10 = 3;
            }
            sVar10 = (short)(sVar2 + (3 - sVar10) * 2);
            iVar4 = FUN_004dbfa0(local_40, 10);
            for (; ((int)sVar10 <= sVar2 + 6 && (0 < sVar8)); sVar8 = (short)(sVar8 - 1)) {
                iVar7 = (int)sVar10;
                sVar10 = (short)(sVar10 + 1);
                uVar5 = DAT_00aa5c10[iVar7];
                local_40[sVar8] = '\0';
                FUN_004db4e0(10, local_40, (int)param_3, (int)param_4, uVar5);
            }
            iVar4 = (int)(short)iVar4 + (int)param_3;
            FUN_004d6460(local_50, iVar4 + 2, (int)param_4, iVar4 + 8, param_4 + 6);
            if (param_5 < sVar3) {
                FUN_004d67e0(local_50, 0, 0, (short)DAT_00aa5c2c[sVar2], 1);
                return;
            }
        }
        else {
            uVar5 = 0xffffffff;
            do {
                pcVar9 = param_1;
                if (uVar5 == 0) break;
                uVar5 = uVar5 - 1;
                pcVar9 = param_1 + 1;
                cVar1 = *param_1;
                param_1 = pcVar9;
            } while (cVar1 != '\0');
            uVar5 = ~uVar5;
            pcVar9 = pcVar9 + -uVar5;
            pcVar11 = local_40;
            for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
                __builtin_memcpy(pcVar11, pcVar9, 4);
                pcVar9 = pcVar9 + 4;
                pcVar11 = pcVar11 + 4;
            }
            for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                *pcVar11 = *pcVar9;
                pcVar9 = pcVar9 + 1;
                pcVar11 = pcVar11 + 1;
            }
            if (iVar4 < sVar3 + 5) {
                sVar2 = (short)((sVar3 - param_5) + sVar2 + 9);
            }
            else {
                sVar2 = (short)(sVar2 + 4);
            }
            FUN_004db4e0(10, local_40, (int)param_3, (int)param_4, DAT_00aa5c10[sVar2]);
        }
    }
    return;
}
