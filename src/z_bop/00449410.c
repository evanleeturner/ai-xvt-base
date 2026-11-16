/* SPDX-License-Identifier: MIT */
/* z_bop 00449410 */
extern unsigned int DAT_00523400;

unsigned char __cdecl FUN_004ac4b0(void);

void __cdecl FUN_00449410(int param_1, int param_2, int param_3, int param_4)
{
    unsigned char uVar1;
    unsigned char bVar2;
    unsigned char bVar3;
    unsigned char bVar4;
    int iVar5;
    unsigned char bVar7;
    unsigned short uVar6;
    unsigned char *pbVar8;
    unsigned short *puVar9;
    unsigned char bStack_27;
    unsigned char bStack_26;
    unsigned char bStack_25;
    unsigned char bStack_24;
    unsigned char uStack_22;
    unsigned char bStack_21;
    unsigned char bStack_20;
    unsigned char bStack_18;
    unsigned char bStack_17;
    unsigned char bStack_16;
    unsigned int uStack_14;
    unsigned int uStack_10;
    unsigned int uStack_c;

    if (DAT_00523400 == 0x100) {
        if (param_3 < (int)((unsigned int)param_4 + (unsigned int)param_3)) {
            puVar9 = (unsigned short *)((unsigned int)param_2 + (unsigned int)param_3 * 2u);
            pbVar8 = (unsigned char *)((unsigned int)param_1 + (unsigned int)param_3 * 3u);
            do {
                uVar1 = FUN_004ac4b0();
                if (uVar1 == 0) {
                    bStack_18 = *pbVar8;
                    bStack_17 = pbVar8[1];
                    bStack_16 = pbVar8[2];
                    *puVar9 = (unsigned short)(((unsigned short)bStack_17 << 5) |
                              ((unsigned short)((unsigned char)((bStack_18 >> 1) << 3)) << 8 |
                               (unsigned short)(bStack_16 >> 1)));
                }
                else {
                    *puVar9 = (unsigned short)(((pbVar8[1] & 0xfe) << 4) |
                              (unsigned short)(pbVar8[2] >> 1) |
                              ((*pbVar8 & 0x3e) << 9));
                }
                puVar9 = puVar9 + 1;
                pbVar8 = pbVar8 + 3;
                param_4 = (int)((unsigned int)param_4 - 1u);
            } while (param_4 != 0);
            return;
        }
    }
    else if (param_4 != 0) {
        puVar9 = (unsigned short *)((unsigned int)param_2 + (unsigned int)param_3 * 2u);
        pbVar8 = (unsigned char *)((unsigned int)param_1 + (unsigned int)param_3 * 3u);
        do {
            param_4 = (int)((unsigned int)param_4 - 1u);
            bVar7 = *pbVar8;
            uVar6 = (unsigned short)bVar7;
            bStack_27 = pbVar8[1];
            bStack_26 = pbVar8[2];
            if (((bVar7 < bStack_27) || ((bStack_25 = bVar7), bVar7 < bStack_26)) &&
                ((bStack_27 < bVar7) || ((bStack_25 = bStack_27), bStack_27 < bStack_26))) {
                bStack_25 = bStack_26;
            }
            if (((bStack_27 < bVar7) || ((bStack_24 = bVar7), bStack_26 < bVar7)) &&
                ((bVar7 < bStack_27) || ((bStack_24 = bStack_27), bStack_26 < bStack_27))) {
                bStack_24 = bStack_26;
            }
            if (bStack_25 == 0) {
                bStack_20 = 0;
            }
            else {
                bStack_20 = (unsigned char)((int)(((unsigned int)bStack_25 - (unsigned int)bStack_24) * 0x3fu) /
                                            (int)(unsigned int)bStack_25);
            }
            if (bStack_20 != 0) {
                if (bStack_25 == bVar7) {
                    if (bStack_27 < bStack_26) {
                        uStack_22 = 5;
                        bStack_21 = (unsigned char)((unsigned char)((int)(((unsigned int)bStack_27 - (unsigned int)bStack_26) * 0x3fu) /
                                    (int)((unsigned int)bStack_25 - (unsigned int)bStack_24)) + 0x3f);
                    }
                    else {
                        uStack_22 = 0;
                        bStack_21 = (unsigned char)((int)(((unsigned int)bStack_27 - (unsigned int)bStack_26) * 0x3fu) /
                                    (int)((unsigned int)bStack_25 - (unsigned int)bStack_24));
                    }
                }
                else if (bStack_25 == bStack_27) {
                    if (bStack_26 < bVar7) {
                        uStack_22 = 1;
                        bStack_21 = (unsigned char)((unsigned char)((int)(((unsigned int)bStack_26 - (unsigned int)bVar7) * 0x3fu) /
                                    (int)((unsigned int)bStack_25 - (unsigned int)bStack_24)) + 0x3f);
                    }
                    else {
                        uStack_22 = 2;
                        bStack_21 = (unsigned char)((int)(((unsigned int)bStack_26 - (unsigned int)bVar7) * 0x3fu) /
                                    (int)((unsigned int)bStack_25 - (unsigned int)bStack_24));
                    }
                }
                else if (bVar7 < bStack_27) {
                    uStack_22 = 3;
                    bStack_21 = (unsigned char)((unsigned char)((int)(((unsigned int)bVar7 - (unsigned int)bStack_27) * 0x3fu) /
                                (int)((unsigned int)bStack_25 - (unsigned int)bStack_24)) + 0x3f);
                }
                else {
                    uStack_22 = 4;
                    bStack_21 = (unsigned char)((int)(((unsigned int)bVar7 - (unsigned int)bStack_27) * 0x3fu) /
                                (int)((unsigned int)bStack_25 - (unsigned int)bStack_24));
                }
            }
            bVar7 = (unsigned char)(((unsigned int)bStack_25 * DAT_00523400) >> 8);
            if (0x3f < bVar7) {
                bVar7 = 0x3f;
            }
            if (bStack_20 == 0) {
                uVar6 = (unsigned short)bVar7;
                bStack_27 = bVar7;
                bStack_26 = bVar7;
            }
            else {
                uStack_10 = (unsigned int)bStack_20;
                uStack_14 = (unsigned int)bVar7;
                bVar2 = (unsigned char)((int)((0x3fu - uStack_10) * uStack_14) / 0x3f);
                uStack_c = (unsigned int)bStack_21;
                bVar3 = (unsigned char)((int)(((unsigned int)((int)(uStack_10 * uStack_c) / -0x3f) + 0x3fu) * uStack_14) / 0x3f);
                iVar5 = (int)(((unsigned int)((int)((uStack_c - 0x3fu) * uStack_10) / 0x3f) + 0x3fu) * uStack_14) / 0x3f;
                bVar4 = (unsigned char)iVar5;
                switch (uStack_22) {
                case 0:
                    uVar6 = (unsigned short)bVar7;
                    bStack_27 = bVar4;
                    bStack_26 = bVar2;
                    break;
                case 1:
                    uVar6 = (unsigned short)bVar3;
                    bStack_27 = bVar7;
                    bStack_26 = bVar2;
                    break;
                case 2:
                    uVar6 = (unsigned short)bVar2;
                    bStack_27 = bVar7;
                    bStack_26 = bVar4;
                    break;
                case 3:
                    uVar6 = (unsigned short)bVar2;
                    bStack_27 = bVar3;
                    bStack_26 = bVar7;
                    break;
                case 4:
                    uVar6 = (unsigned short)iVar5 & 0xff;
                    bStack_27 = bVar2;
                    bStack_26 = bVar7;
                    break;
                case 5:
                    uVar6 = (unsigned short)bVar7;
                    bStack_27 = bVar2;
                    bStack_26 = bVar3;
                }
            }
            uVar1 = FUN_004ac4b0();
            if (uVar1 == 0) {
                bStack_18 = (unsigned char)uVar6;
                bStack_17 = bStack_27;
                bStack_16 = bStack_26;
                uVar6 = (unsigned short)(((unsigned short)bStack_27 << 5) |
                        ((unsigned short)((unsigned char)((bStack_18 >> 1) << 3)) << 8 |
                         (unsigned short)(bStack_26 >> 1)));
            }
            else {
                uVar6 = (unsigned short)(((bStack_27 & 0xfe) << 4) |
                        ((uVar6 & 0x3e) << 9) |
                        (unsigned short)(bStack_26 >> 1));
            }
            *puVar9 = uVar6;
            puVar9 = puVar9 + 1;
            pbVar8 = pbVar8 + 3;
        } while (param_4 != 0);
    }
    return;
}
