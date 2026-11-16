/* SPDX-License-Identifier: MIT */
/* z_xvt 004153d0 */
extern uint DAT_00525540;

int __cdecl FUN_004a7360(void);
ushort __cdecl FUN_00413e80(int param_1, int param_2);

void __cdecl FUN_004153d0(int param_1, int param_2, int param_3, int param_4)
{
    int iVar1;
    byte bVar2;
    byte bVar3;
    byte bVar4;
    ushort uVar5;
    int iVar6;
    byte bVar7;
    byte *pbVar8;
    ushort *puVar9;
    byte bStack_15;
    byte uStack_14;
    byte bStack_12;
    byte bStack_11;
    struct {
        byte bStack_10;
        byte bStack_f;
        byte bStack_e;
    } rgb;
    uint uStack_c;
    uint uStack_8;
    uint uStack_4;

    if (DAT_00525540 == 0x100) {
        if (param_3 < (int)((uint)param_4 + (uint)param_3)) {
            puVar9 = (ushort *)((uint)param_2 + (uint)param_3 * 2u);
            pbVar8 = (byte *)((uint)param_1 + (uint)param_3 * 3u);
            do {
                iVar1 = FUN_004a7360();
                if (iVar1 == 0) {
                    rgb.bStack_10 = *pbVar8;
                    rgb.bStack_f = pbVar8[1];
                    rgb.bStack_e = pbVar8[2];
                    uVar5 = FUN_00413e80(0, (int)&rgb.bStack_10);
                    *puVar9 = uVar5;
                }
                else {
                    *puVar9 = (ushort)(((pbVar8[1] & 0xfe) << 4) | (ushort)(pbVar8[2] >> 1) | ((*pbVar8 & 0x3e) << 9));
                }
                puVar9 = puVar9 + 1;
                pbVar8 = pbVar8 + 3;
                param_4 = param_4 - 1;
            } while (param_4 != 0);
            return;
        }
    }
    else if (param_4 != 0) {
        puVar9 = (ushort *)((uint)param_2 + (uint)param_3 * 2u);
        pbVar8 = (byte *)((uint)param_1 + (uint)param_3 * 3u);
        do {
            param_4 = param_4 - 1;
            bVar7 = *pbVar8;
            uVar5 = (ushort)bVar7;
            bStack_11 = pbVar8[1];
            bStack_12 = pbVar8[2];
            if (((bVar7 < bStack_11) || (bVar2 = bVar7, bVar7 < bStack_12)) &&
                ((bStack_11 < bVar7) || (bVar2 = bStack_11, bStack_11 < bStack_12))) {
                bVar2 = bStack_12;
            }
            uStack_c = (uint)bVar2;
            if (((bStack_11 < bVar7) || (bVar3 = bVar7, bStack_12 < bVar7)) &&
                ((bVar7 < bStack_11) || (bVar3 = bStack_11, bStack_12 < bStack_11))) {
                bVar3 = bStack_12;
            }
            uStack_8 = (uint)bVar3;
            if (bVar2 == 0) {
                uStack_4 = 0;
            }
            else {
                uStack_4 = (uint)(byte)((int)(((uint)bVar2 - (uint)bVar3) * 0x3fu) / (int)(uint)bVar2);
            }
            if ((byte)uStack_4 != 0) {
                if (bVar7 == bVar2) {
                    if (bStack_11 < bStack_12) {
                        uStack_14 = 5;
                        bStack_15 = (byte)((byte)((int)(((uint)bStack_11 - (uint)bStack_12) * 0x3fu) /
                                                  (int)((uint)bVar2 - (uint)bVar3)) + 0x3f);
                    }
                    else {
                        uStack_14 = 0;
                        bStack_15 = (byte)((int)(((uint)bStack_11 - (uint)bStack_12) * 0x3fu) /
                                           (int)((uint)bVar2 - (uint)bVar3));
                    }
                }
                else if (bStack_11 == bVar2) {
                    if (bStack_12 < bVar7) {
                        uStack_14 = 1;
                        bStack_15 = (byte)((byte)((int)(((uint)bStack_12 - (uint)bVar7) * 0x3fu) /
                                                  (int)((uint)bVar2 - (uint)bVar3)) + 0x3f);
                    }
                    else {
                        uStack_14 = 2;
                        bStack_15 = (byte)((int)(((uint)bStack_12 - (uint)bVar7) * 0x3fu) /
                                           (int)((uint)bVar2 - (uint)bVar3));
                    }
                }
                else if (bVar7 < bStack_11) {
                    uStack_14 = 3;
                    bStack_15 = (byte)((byte)((int)(((uint)bVar7 - (uint)bStack_11) * 0x3fu) /
                                              (int)((uint)bVar2 - (uint)bVar3)) + 0x3f);
                }
                else {
                    uStack_14 = 4;
                    bStack_15 = (byte)((int)(((uint)bVar7 - (uint)bStack_11) * 0x3fu) /
                                       (int)((uint)bVar2 - (uint)bVar3));
                }
            }
            bVar7 = (byte)(((uint)bVar2 * DAT_00525540) >> 8);
            if (0x3f < bVar7) {
                bVar7 = 0x3f;
            }
            if ((byte)uStack_4 == 0) {
                uVar5 = (ushort)bVar7;
                bStack_12 = bVar7;
                bStack_11 = bVar7;
            }
            else {
                uStack_8 = uStack_4 & 0xff;
                uStack_c = (uint)bVar7;
                bVar2 = (byte)((int)((0x3fu - uStack_8) * uStack_c) / 0x3f);
                uStack_4 = (uint)bStack_15;
                bVar3 = (byte)((int)((uint)((int)(uStack_8 * uStack_4) / -0x3f + 0x3f) * uStack_c) / 0x3f);
                iVar6 = (int)((uint)((int)((uStack_4 - 0x3fu) * uStack_8) / 0x3f + 0x3f) * uStack_c) / 0x3f;
                bVar4 = (byte)iVar6;
                switch (uStack_14) {
                case 0:
                    uVar5 = (ushort)bVar7;
                    bStack_12 = bVar2;
                    bStack_11 = bVar4;
                    break;
                case 1:
                    uVar5 = (ushort)bVar3;
                    bStack_12 = bVar2;
                    bStack_11 = bVar7;
                    break;
                case 2:
                    uVar5 = (ushort)bVar2;
                    bStack_12 = bVar4;
                    bStack_11 = bVar7;
                    break;
                case 3:
                    uVar5 = (ushort)bVar2;
                    bStack_12 = bVar7;
                    bStack_11 = bVar3;
                    break;
                case 4:
                    uVar5 = (ushort)iVar6 & 0xff;
                    bStack_12 = bVar7;
                    bStack_11 = bVar2;
                    break;
                case 5:
                    uVar5 = (ushort)bVar7;
                    bStack_12 = bVar3;
                    bStack_11 = bVar2;
                }
            }
            iVar1 = FUN_004a7360();
            if (iVar1 == 0) {
                rgb.bStack_10 = (byte)uVar5;
                rgb.bStack_f = bStack_11;
                rgb.bStack_e = bStack_12;
                uVar5 = FUN_00413e80(0, (int)&rgb.bStack_10);
                *puVar9 = uVar5;
            }
            else {
                *puVar9 = (ushort)(((uVar5 & 0x3e) << 9) | ((bStack_11 & 0xfe) << 4) | (ushort)(bStack_12 >> 1));
            }
            puVar9 = puVar9 + 1;
            pbVar8 = pbVar8 + 3;
        } while (param_4 != 0);
    }
    return;
}
