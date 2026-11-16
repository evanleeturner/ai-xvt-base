/* SPDX-License-Identifier: MIT */
/* z_xvt 004626c0 */
extern unsigned char DAT_006da600[];
extern unsigned char DAT_006da840;
extern int DAT_0071b014;

void __cdecl FUN_004290b0(int, int, unsigned int, int, int);

void __cdecl FUN_004626c0(int *param_1)
{
    unsigned char *pbVar1;
    unsigned char *pbVar2;
    unsigned char bVar3;
    char cVar4;
    char *pcVar5;
    int iVar6;
    int iVar7;
    int iVar8;
    unsigned char *pbVar9;
    int iVar10;
    unsigned char *pbVar11;
    char cVar12;
    char *local_20;
    unsigned int local_1c;
    int local_18;
    unsigned int local_10;

    local_20 = (char *)(*param_1 + (int)param_1);
    if (param_1[0xb] == 0x18) {
        pbVar9 = (unsigned char *)(param_1[3] + (int)param_1);
        param_1[5] = (int)local_20 - (int)param_1;
        local_1c = 0;
        if (param_1[0xc] != 0) {
            do {
                bVar3 = *pbVar9;
                pbVar1 = pbVar9 + 1;
                pbVar2 = pbVar9 + 2;
                cVar12 = '@';
                pbVar9 = pbVar9 + 4;
                pbVar11 = DAT_006da600;
                cVar4 = '@';
                local_18 = 0x7fffffff;
                do {
                    iVar6 = (int)(unsigned int)(bVar3 >> 2) - (int)(unsigned int)*pbVar11;
                    iVar7 = (int)(unsigned int)(*pbVar1 >> 2) - (int)(unsigned int)pbVar11[1];
                    iVar8 = (int)(unsigned int)(*pbVar2 >> 2) - (int)(unsigned int)pbVar11[2];
                    iVar6 = iVar6 * iVar6 + iVar8 * iVar8 + iVar7 * iVar7;
                    if (iVar6 < local_18) {
                        local_18 = iVar6;
                        cVar4 = cVar12;
                    }
                    pbVar11 = pbVar11 + 3;
                    cVar12 = cVar12 + '\x01';
                } while (pbVar11 < &DAT_006da840);
                pcVar5 = local_20 + 1;
                local_1c = local_1c + 1;
                *local_20 = cVar4;
                local_20 = pcVar5;
            } while (local_1c < (unsigned int)param_1[0xc]);
        }
    }
    local_10 = 0;
    if (param_1[6] != 0) {
        do {
            iVar6 = *(int *)(param_1[4] + local_10 * 4 + (int)param_1) + (int)param_1;
            *(int *)(iVar6 + 0xc) = (int)local_20 - iVar6;
            if (*(int *)(iVar6 + 0x24) == 0x18) {
                pbVar9 = (unsigned char *)(*(int *)(iVar6 + 4) + iVar6);
                if (DAT_0071b014 != 0) {
                    FUN_004290b0(*(int *)(iVar6 + 8) + iVar6, (int)pbVar9,
                                 *(unsigned int *)(iVar6 + 0x10),
                                 *(int *)(iVar6 + 0x14), *(int *)(iVar6 + 0x20));
                }
                local_1c = 0;
                if (*(int *)(iVar6 + 0x28) != 0) {
                    do {
                        bVar3 = *pbVar9;
                        pbVar1 = pbVar9 + 1;
                        pbVar2 = pbVar9 + 2;
                        cVar12 = '@';
                        pbVar9 = pbVar9 + 4;
                        pbVar11 = DAT_006da600;
                        cVar4 = '@';
                        local_18 = 0x7fffffff;
                        do {
                            iVar7 = (int)(unsigned int)(bVar3 >> 2) - (int)(unsigned int)*pbVar11;
                            iVar8 = (int)(unsigned int)(*pbVar1 >> 2) - (int)(unsigned int)pbVar11[1];
                            iVar10 = (int)(unsigned int)(*pbVar2 >> 2) - (int)(unsigned int)pbVar11[2];
                            iVar7 = iVar10 * iVar10 + iVar8 * iVar8 + iVar7 * iVar7;
                            if (iVar7 < local_18) {
                                local_18 = iVar7;
                                cVar4 = cVar12;
                            }
                            pbVar11 = pbVar11 + 3;
                            cVar12 = cVar12 + '\x01';
                        } while (pbVar11 < &DAT_006da840);
                        pcVar5 = local_20 + 1;
                        local_1c = local_1c + 1;
                        *local_20 = cVar4;
                        local_20 = pcVar5;
                    } while (local_1c < *(unsigned int *)(iVar6 + 0x28));
                }
            }
            local_10 = local_10 + 1;
        } while (local_10 < (unsigned int)param_1[6]);
    }
    return;
}
