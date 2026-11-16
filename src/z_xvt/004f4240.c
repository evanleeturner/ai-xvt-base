/* SPDX-License-Identifier: MIT */
/* z_xvt 004f4240 */
extern unsigned int DAT_007577d0[];
extern unsigned int DAT_007577ec[];

int __cdecl FUN_004db2f0(char *, int);
void __cdecl FUN_004da7c0(int, char *, int, int, unsigned int);
void __cdecl FUN_004dcfd0(int *, int, int, int, int);
void __cdecl FUN_004dd3e0(int *, int, int, short, int);

void __cdecl
FUN_004f4240(char *param_1, unsigned int param_2, short param_3, short param_4, short param_5,
             short param_6)
{
    char cVar1;
    short sVar2;
    int iVar3;
    unsigned int uVar4;
    unsigned int uVar5;
    int iVar6;
    short sVar7;
    short sVar8;
    char *pcVar9;
    short sVar10;
    char *pcVar11;
    int local_50[4];
    char local_40[64];
    unsigned int uTmp;

    if (-1 < param_5) {
        iVar3 = -1;
        pcVar9 = param_1;
        do {
            if (iVar3 == 0) break;
            iVar3 = iVar3 + -1;
            cVar1 = *pcVar9;
            pcVar9 = pcVar9 + 1;
        } while (cVar1 != '\0');
        sVar2 = (short)(~(unsigned short)iVar3 - 1);
        uVar4 = 0xffffffffu;
        pcVar9 = param_1;
        do {
            pcVar11 = pcVar9;
            if (uVar4 == 0) break;
            uVar4 = uVar4 - 1;
            pcVar11 = pcVar9 + 1;
            cVar1 = *pcVar9;
            pcVar9 = pcVar11;
        } while (cVar1 != '\0');
        uVar4 = ~uVar4;
        pcVar9 = pcVar11 - uVar4;
        pcVar11 = local_40;
        for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
            __builtin_memcpy(&uTmp, pcVar9, 4);
            __builtin_memcpy(pcVar11, &uTmp, 4);
            pcVar9 = pcVar9 + 4;
            pcVar11 = pcVar11 + 4;
        }
        for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
            *pcVar11 = *pcVar9;
            pcVar9 = pcVar9 + 1;
            pcVar11 = pcVar11 + 1;
        }
        iVar3 = (int)param_5;
        sVar7 = (short)((unsigned int)param_6 * 8u);
        if (iVar3 < sVar2 + 2) {
            sVar10 = sVar2;
            if (param_5 <= sVar2) {
                local_40[iVar3] = '\0';
                sVar10 = param_5;
            }
            sVar8 = param_5;
            if (3 < param_5) {
                sVar8 = 3;
            }
            sVar8 = (short)((unsigned int)sVar7 + (unsigned int)((3 - sVar8) * 2));
            iVar3 = FUN_004db2f0(local_40, 10);
            for (; ((int)sVar8 <= sVar7 + 6 && (0 < sVar10)); sVar10 = (short)(sVar10 - 1)) {
                iVar6 = (int)sVar8;
                sVar8 = (short)(sVar8 + 1);
                uVar4 = DAT_007577d0[iVar6];
                local_40[sVar10] = '\0';
                FUN_004da7c0(10, local_40, (int)param_3, (int)param_4, uVar4);
            }
            iVar3 = (int)(short)iVar3 + (int)param_3;
            FUN_004dcfd0(local_50, iVar3 + 2, (int)param_4, iVar3 + 8, param_4 + 6);
            if (param_5 < sVar2) {
                FUN_004dd3e0(local_50, 0, 0, (short)DAT_007577ec[sVar7], 1);
                return;
            }
        }
        else {
            uVar4 = 0xffffffffu;
            do {
                pcVar9 = param_1;
                if (uVar4 == 0) break;
                uVar4 = uVar4 - 1;
                pcVar9 = param_1 + 1;
                cVar1 = *param_1;
                param_1 = pcVar9;
            } while (cVar1 != '\0');
            uVar4 = ~uVar4;
            pcVar9 = pcVar9 - uVar4;
            pcVar11 = local_40;
            for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                __builtin_memcpy(&uTmp, pcVar9, 4);
                __builtin_memcpy(pcVar11, &uTmp, 4);
                pcVar9 = pcVar9 + 4;
                pcVar11 = pcVar11 + 4;
            }
            for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
                *pcVar11 = *pcVar9;
                pcVar9 = pcVar9 + 1;
                pcVar11 = pcVar11 + 1;
            }
            if (iVar3 < sVar2 + 5) {
                sVar7 = (short)((unsigned int)((sVar7 - param_5) + 9) + (unsigned int)sVar2);
            }
            else {
                sVar7 = (short)((unsigned int)sVar7 + 4u);
            }
            FUN_004da7c0(10, local_40, (int)param_3, (int)param_4, DAT_007577d0[sVar7]);
        }
    }
    return;
}
