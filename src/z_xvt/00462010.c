/* SPDX-License-Identifier: MIT */
/* z_xvt 00462010 */
extern unsigned int DAT_00525540;

void __cdecl FUN_00462010(int param_1, int param_2, int param_3, int param_4)
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
    unsigned char *puVar10;
    unsigned char bVar11;
    unsigned char *pbVar12;
    unsigned char *pbVar13;
    int bVar14;
    unsigned char local_e;
    unsigned char local_d;
    unsigned char local_b;
    unsigned char local_a;
    unsigned char local_9;
    unsigned char local_4;

    if (DAT_00525540 == 0x100) {
        if (param_4 != 0) {
            puVar7 = (unsigned char *)(unsigned int)((unsigned int)param_3 * 3u + (unsigned int)param_2);
            puVar10 = (unsigned char *)(unsigned int)((unsigned int)param_1 + (unsigned int)param_3 * 3u);
            param_4 = param_4 - 1;
            do {
                *puVar7 = *puVar10;
                puVar7[1] = puVar10[1];
                puVar7[2] = puVar10[2];
                bVar14 = (param_4 != 0);
                param_4 = param_4 - 1;
                puVar7 = puVar7 + 3;
                puVar10 = puVar10 + 3;
            } while (bVar14);
            return;
        }
    }
    else if (param_4 != 0) {
        pbVar12 = (unsigned char *)(unsigned int)((unsigned int)param_1 + (unsigned int)param_3 * 3u);
        pbVar13 = (unsigned char *)(unsigned int)((unsigned int)param_2 + (unsigned int)param_3 * 3u);
        param_4 = param_4 - 1;
        do {
            bVar1 = *pbVar12;
            bVar2 = pbVar12[1];
            bVar3 = pbVar12[2];
            if (((bVar1 < bVar2) || (bVar11 = bVar1, bVar1 < bVar3)) &&
                ((bVar2 < bVar1 || (bVar11 = bVar2, bVar2 < bVar3)))) {
                bVar11 = bVar3;
            }
            if (((bVar2 < bVar1) || (bVar4 = bVar1, bVar3 < bVar1)) &&
                ((bVar1 < bVar2 || (bVar4 = bVar2, bVar3 < bVar2)))) {
                bVar4 = bVar3;
            }
            if (bVar11 == 0) {
                local_4 = 0;
            }
            else {
                local_4 = (unsigned char)((int)((((unsigned int)bVar11 - (unsigned int)bVar4) * 0x3fu)) /
                                          (int)(unsigned int)bVar11);
            }
            if (local_4 != 0) {
                if (bVar11 == bVar1) {
                    if (bVar2 < bVar3) {
                        local_d = 5;
                        local_e = (unsigned char)((char)((int)((((unsigned int)bVar2 - (unsigned int)bVar3) * 0x3fu)) /
                                                         (int)((unsigned int)bVar11 - (unsigned int)bVar4)) + 0x3f);
                    }
                    else {
                        local_d = 0;
                        local_e = (unsigned char)((int)((((unsigned int)bVar2 - (unsigned int)bVar3) * 0x3fu)) /
                                                  (int)((unsigned int)bVar11 - (unsigned int)bVar4));
                    }
                }
                else if (bVar11 == bVar2) {
                    if (bVar3 < bVar1) {
                        local_d = 1;
                        local_e = (unsigned char)((char)((int)((((unsigned int)bVar3 - (unsigned int)bVar1) * 0x3fu)) /
                                                         (int)((unsigned int)bVar11 - (unsigned int)bVar4)) + 0x3f);
                    }
                    else {
                        local_d = 2;
                        local_e = (unsigned char)((int)((((unsigned int)bVar3 - (unsigned int)bVar1) * 0x3fu)) /
                                                  (int)((unsigned int)bVar11 - (unsigned int)bVar4));
                    }
                }
                else if (bVar1 < bVar2) {
                    local_d = 3;
                    local_e = (unsigned char)((char)((int)((((unsigned int)bVar1 - (unsigned int)bVar2) * 0x3fu)) /
                                                     (int)((unsigned int)bVar11 - (unsigned int)bVar4)) + 0x3f);
                }
                else {
                    local_d = 4;
                    local_e = (unsigned char)((int)((((unsigned int)bVar1 - (unsigned int)bVar2) * 0x3fu)) /
                                              (int)((unsigned int)bVar11 - (unsigned int)bVar4));
                }
            }
            bVar11 = (unsigned char)(((unsigned int)bVar11 * DAT_00525540) >> 8);
            if (0x3f < bVar11) {
                bVar11 = 0x3f;
            }
            local_b = bVar11;
            local_a = bVar11;
            local_9 = bVar11;
            if (local_4 != 0) {
                uVar8 = (unsigned int)local_4;
                uVar9 = (unsigned int)bVar11;
                bVar4 = (unsigned char)((int)((0x3fu - uVar8) * uVar9) / 0x3f);
                bVar5 = (unsigned char)((int)(((unsigned int)((int)(uVar8 * (unsigned int)local_e) / -0x3f + 0x3f)) * uVar9) / 0x3f);
                bVar6 = (unsigned char)((int)(((unsigned int)((int)(((unsigned int)local_e - 0x3fu) * uVar8) / 0x3f + 0x3f)) * uVar9) / 0x3f);
                local_b = bVar3;
                local_a = bVar1;
                local_9 = bVar2;
                switch (local_d) {
                case 0:
                    local_b = bVar4;
                    local_a = bVar11;
                    local_9 = bVar6;
                    break;
                case 1:
                    local_b = bVar4;
                    local_a = bVar5;
                    local_9 = bVar11;
                    break;
                case 2:
                    local_b = bVar6;
                    local_a = bVar4;
                    local_9 = bVar11;
                    break;
                case 3:
                    local_b = bVar11;
                    local_a = bVar4;
                    local_9 = bVar5;
                    break;
                case 4:
                    local_b = bVar11;
                    local_a = bVar6;
                    local_9 = bVar4;
                    break;
                case 5:
                    local_b = bVar5;
                    local_a = bVar11;
                    local_9 = bVar4;
                }
            }
            *pbVar13 = local_a;
            pbVar13[1] = local_9;
            pbVar12 = pbVar12 + 3;
            pbVar13[2] = local_b;
            bVar14 = (param_4 != 0);
            param_4 = param_4 - 1;
            pbVar13 = pbVar13 + 3;
        } while (bVar14);
    }
    return;
}
