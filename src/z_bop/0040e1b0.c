/* SPDX-License-Identifier: MIT */
/* z_bop 0040e1b0 */
extern unsigned int DAT_00523400;

void __cdecl FUN_0040e1b0(int param_1, int param_2, int param_3, int param_4)
{
    unsigned char bVar1;
    unsigned char bVar2;
    unsigned char bVar3;
    unsigned char bVar4;
    unsigned char bVar5;
    unsigned char bVar6;
    unsigned char *puVar7;
    unsigned int uVar8;
    unsigned int uVar9;
    unsigned char bVar10;
    unsigned char *puVar11;
    unsigned char bVar12;
    unsigned char *pbVar13;
    unsigned char *pbVar14;
    int bVar15;
    unsigned char local_11;
    unsigned char local_10;
    unsigned char local_e;
    unsigned char local_d;
    unsigned char local_4;

    if (DAT_00523400 == 0x100) {
        if (param_4 != 0) {
            puVar7 = (unsigned char *)((unsigned int)param_3 * 3u + (unsigned int)param_2);
            puVar11 = (unsigned char *)((unsigned int)param_1 + (unsigned int)param_3 * 3u);
            param_4 = param_4 - 1;
            do {
                *puVar7 = *puVar11;
                puVar7[1] = puVar11[1];
                puVar7[2] = puVar11[2];
                bVar15 = (param_4 != 0);
                param_4 = param_4 - 1;
                puVar7 = puVar7 + 3;
                puVar11 = puVar11 + 3;
            } while (bVar15);
            return;
        }
    }
    else if (param_4 != 0) {
        pbVar14 = (unsigned char *)((unsigned int)param_2 + (unsigned int)param_3 * 3u);
        pbVar13 = (unsigned char *)((unsigned int)param_1 + (unsigned int)param_3 * 3u);
        param_4 = param_4 - 1;
        do {
            bVar1 = *pbVar13;
            bVar2 = pbVar13[1];
            bVar3 = pbVar13[2];
            if (((bVar1 < bVar2) || (bVar10 = bVar1, bVar1 < bVar3)) &&
                ((bVar2 < bVar1) || (bVar10 = bVar2, bVar2 < bVar3))) {
                bVar10 = bVar3;
            }
            if (((bVar2 < bVar1) || (bVar12 = bVar1, bVar3 < bVar1)) &&
                ((bVar1 < bVar2) || (bVar12 = bVar2, bVar3 < bVar2))) {
                bVar12 = bVar3;
            }
            if (bVar10 == 0) {
                local_4 = 0;
            }
            else {
                local_4 = (unsigned char)((int)(((unsigned int)bVar10 - (unsigned int)bVar12) * 0x3fu) /
                                          (int)(unsigned int)bVar10);
            }
            if (local_4 != 0) {
                if (bVar1 == bVar10) {
                    if (bVar2 < bVar3) {
                        local_10 = 5;
                        local_11 = (unsigned char)((unsigned char)((int)(((unsigned int)bVar2 - (unsigned int)bVar3) * 0x3fu) /
                                                  (int)((unsigned int)bVar10 - (unsigned int)bVar12)) + 0x3f);
                    }
                    else {
                        local_10 = 0;
                        local_11 = (unsigned char)((int)(((unsigned int)bVar2 - (unsigned int)bVar3) * 0x3fu) /
                                                   (int)((unsigned int)bVar10 - (unsigned int)bVar12));
                    }
                }
                else if (bVar2 == bVar10) {
                    if (bVar3 < bVar1) {
                        local_10 = 1;
                        local_11 = (unsigned char)((unsigned char)((int)(((unsigned int)bVar3 - (unsigned int)bVar1) * 0x3fu) /
                                                  (int)((unsigned int)bVar10 - (unsigned int)bVar12)) + 0x3f);
                    }
                    else {
                        local_10 = 2;
                        local_11 = (unsigned char)((int)(((unsigned int)bVar3 - (unsigned int)bVar1) * 0x3fu) /
                                                   (int)((unsigned int)bVar10 - (unsigned int)bVar12));
                    }
                }
                else if (bVar1 < bVar2) {
                    local_10 = 3;
                    local_11 = (unsigned char)((unsigned char)((int)(((unsigned int)bVar1 - (unsigned int)bVar2) * 0x3fu) /
                                              (int)((unsigned int)bVar10 - (unsigned int)bVar12)) + 0x3f);
                }
                else {
                    local_10 = 4;
                    local_11 = (unsigned char)((int)(((unsigned int)bVar1 - (unsigned int)bVar2) * 0x3fu) /
                                               (int)((unsigned int)bVar10 - (unsigned int)bVar12));
                }
            }
            bVar10 = (unsigned char)(((unsigned int)bVar10 * DAT_00523400) >> 8);
            if (0x3f < bVar10) {
                bVar10 = 0x3f;
            }
            bVar12 = bVar10;
            local_e = bVar10;
            local_d = bVar10;
            if (local_4 != 0) {
                uVar8 = (unsigned int)local_4;
                uVar9 = (unsigned int)bVar10;
                bVar4 = (unsigned char)((int)((0x3fu - uVar8) * uVar9) / 0x3f);
                bVar5 = (unsigned char)((int)((((unsigned int)((int)((unsigned int)local_11 * uVar8) / -0x3f)) + 0x3fu) * uVar9) / 0x3f);
                bVar6 = (unsigned char)((int)((((unsigned int)((int)(((unsigned int)local_11 - 0x3fu) * uVar8) / 0x3f)) + 0x3fu) * uVar9) / 0x3f);
                bVar12 = bVar1;
                local_e = bVar3;
                local_d = bVar2;
                switch (local_10) {
                case 0:
                    bVar12 = bVar10;
                    local_e = bVar4;
                    local_d = bVar6;
                    break;
                case 1:
                    bVar12 = bVar5;
                    local_e = bVar4;
                    local_d = bVar10;
                    break;
                case 2:
                    bVar12 = bVar4;
                    local_e = bVar6;
                    local_d = bVar10;
                    break;
                case 3:
                    bVar12 = bVar4;
                    local_e = bVar10;
                    local_d = bVar5;
                    break;
                case 4:
                    bVar12 = bVar6;
                    local_e = bVar10;
                    local_d = bVar4;
                    break;
                case 5:
                    bVar12 = bVar10;
                    local_e = bVar5;
                    local_d = bVar4;
                }
            }
            *pbVar14 = bVar12;
            pbVar14[1] = local_d;
            pbVar14[2] = local_e;
            pbVar14 = pbVar14 + 3;
            pbVar13 = pbVar13 + 3;
            bVar15 = (param_4 != 0);
            param_4 = param_4 - 1;
        } while (bVar15);
    }
    return;
}
