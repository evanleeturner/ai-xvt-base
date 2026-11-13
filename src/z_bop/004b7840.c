/* SPDX-License-Identifier: MIT */
/* z_bop 004b7840 */
int __cdecl FUN_004b7840(int *param_1, byte *param_2, int param_3)
{
    int iVar1;
    byte *pbVar2;
    byte *pbVar3;
    byte bVar4;
    int iVar5;
    byte bVar6;
    int iVar7;
    int iVar8;
    int iVar9;
    byte *local_c;

    iVar1 = 0;
    iVar9 = 0;
    bVar6 = *param_2;
    *(byte *)(param_1 + 1) = 0;
    local_c = (byte *)(param_1 + 1);
    pbVar2 = local_c;
    if (param_3 < 1) {
        *local_c = 0x80;
        *param_1 = 5;
        return 5;
    }
    do {
        iVar5 = 0;
        do {
            if ((param_3 <= iVar9) || (param_2[iVar5] != bVar6)) break;
            iVar9 = iVar9 + 1;
            iVar5 = iVar5 + 1;
        } while (iVar5 < 0x3f);
        bVar4 = (byte)iVar5;
        if (bVar6 == 0) {
            iVar1 = iVar1 + 1;
            *pbVar2 = bVar4 | 0x40;
            pbVar3 = pbVar2 + 1;
            local_c = pbVar2;
        }
        else if (iVar5 < 3) {
            if ((*local_c & 0x80) == 0) {
                iVar1 = iVar1 + 1;
                *pbVar2 = bVar4 | 0x80;
                pbVar3 = pbVar2 + 1;
                iVar8 = 0;
                local_c = pbVar2;
                if (0 < iVar5) {
                    do {
                        iVar7 = iVar8 + 1;
                        pbVar3[iVar8] = param_2[iVar8];
                        iVar8 = iVar7;
                    } while (iVar7 < iVar5);
                }
            }
            else {
                bVar6 = (*local_c & 0x7f) + bVar4;
                if (bVar6 < 0x80) {
                    *local_c = bVar6 | 0x80;
                    pbVar3 = pbVar2;
                    iVar8 = 0;
                    if (0 < iVar5) {
                        do {
                            iVar7 = iVar8 + 1;
                            pbVar2[iVar8] = param_2[iVar8];
                            iVar8 = iVar7;
                        } while (iVar7 < iVar5);
                    }
                }
                else {
                    iVar1 = iVar1 + 1;
                    *pbVar2 = bVar4 | 0x80;
                    pbVar3 = pbVar2 + 1;
                    iVar8 = 0;
                    local_c = pbVar2;
                    if (0 < iVar5) {
                        do {
                            iVar7 = iVar8 + 1;
                            pbVar3[iVar8] = param_2[iVar8];
                            iVar8 = iVar7;
                        } while (iVar7 < iVar5);
                    }
                }
            }
            pbVar3 = pbVar3 + iVar5;
            iVar1 = iVar1 + iVar5;
        }
        else {
            *pbVar2 = bVar4;
            iVar1 = iVar1 + 2;
            pbVar2[1] = bVar6;
            pbVar3 = pbVar2 + 2;
            local_c = pbVar2;
        }
        if (param_3 <= iVar9) {
            *pbVar3 = 0x80;
            *param_1 = iVar1 + 5;
            return iVar1 + 5;
        }
        param_2 = param_2 + iVar5;
        bVar6 = *param_2;
        pbVar2 = pbVar3;
    } while (1);
}
