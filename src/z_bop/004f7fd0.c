/* SPDX-License-Identifier: MIT */
/* z_bop 004f7fd0 */
extern short DAT_00669204;
extern short DAT_00669206;
extern short DAT_0066920c;
extern short DAT_0066920e;

int __cdecl FUN_004d6e30(int a, int b, int c);
void __cdecl FUN_004d6480(int *rect, unsigned int *param_1);
void __cdecl FUN_00506550(int a, int b, int c, short d);
void __cdecl FUN_005063b0(int a, int b, int c, short d);

void __cdecl FUN_004f7fd0(unsigned int *param_1)
{
    int bVar1;
    int iVar2;
    int iVar3;
    short sVar4;
    short sVar5;
    short sVar6;
    unsigned short uVar7;
    unsigned short local_18;
    unsigned short local_16;
    struct {
        int local_10;
        int local_c;
        int local_8;
        int local_4;
    } rc;

    iVar2 = FUN_004d6e30(0x96, 0, 0);
    iVar3 = FUN_004d6e30(0x50, 0, 0);
    FUN_004d6480(&rc.local_10, param_1);

    local_18 = (unsigned short)(((int)DAT_00669204 + (((int)DAT_00669204 >> 0x1f) & 0xff)) >> 8);
    if ((0 < DAT_00669204) && ((char)DAT_00669204 != '\0')) {
        local_18 = (unsigned short)(local_18 + 1);
    }
    for (sVar4 = (short)((short)(((-(int)DAT_00669204) & 0xff) * (int)DAT_0066920c >> 8) +
                         (short)((int)((unsigned int)rc.local_8 - (unsigned int)rc.local_10) >> 1) +
                         (short)rc.local_10);
         rc.local_10 < sVar4;
         sVar4 = (short)(sVar4 - DAT_0066920c)) {
        local_18 = (unsigned short)(local_18 - 1);
    }

    local_16 = (unsigned short)(((int)DAT_00669206 + (((int)DAT_00669206 >> 0x1f) & 0xff)) >> 8);
    if ((0 < DAT_00669206) && ((char)DAT_00669206 != '\0')) {
        local_16 = (unsigned short)(local_16 + 1);
    }
    for (sVar5 = (short)((short)(((-(int)DAT_00669206) & 0xff) * (int)DAT_0066920e >> 8) +
                         (short)((int)((unsigned int)rc.local_4 - (unsigned int)rc.local_c) >> 1) +
                         (short)rc.local_c);
         rc.local_c < sVar5;
         sVar5 = (short)(sVar5 - DAT_0066920e)) {
        local_16 = (unsigned short)(local_16 - 1);
    }

    if (0xf < DAT_0066920c) {
        bVar1 = 0x1f < DAT_0066920c;
        sVar6 = sVar4;
        uVar7 = local_18;
        if (sVar4 < rc.local_8) {
            do {
                if (((uVar7 & 3) != 0) && (((uVar7 & 3) == 2 || (bVar1)))) {
                    FUN_00506550(rc.local_c, rc.local_4, (int)sVar6, (short)iVar3);
                }
                sVar6 = (short)(sVar6 + DAT_0066920c);
                uVar7 = (unsigned short)(uVar7 + 1);
            } while (sVar6 < rc.local_8);
        }
        sVar6 = sVar5;
        uVar7 = local_16;
        if (sVar5 < rc.local_4) {
            do {
                if (((uVar7 & 3) != 0) && (((uVar7 & 3) == 2 || (bVar1)))) {
                    FUN_005063b0(rc.local_10, rc.local_8, (int)sVar6, (short)iVar3);
                }
                sVar6 = (short)(sVar6 + DAT_0066920e);
                uVar7 = (unsigned short)(uVar7 + 1);
            } while (sVar6 < rc.local_4);
        }
    }
    if (sVar4 < rc.local_8) {
        do {
            if ((local_18 & 3) == 0) {
                FUN_00506550(rc.local_c, rc.local_4, (int)sVar4, (short)iVar2);
            }
            sVar4 = (short)(sVar4 + DAT_0066920c);
            local_18 = (unsigned short)(local_18 + 1);
        } while (sVar4 < rc.local_8);
    }
    if (sVar5 < rc.local_4) {
        do {
            if ((local_16 & 3) == 0) {
                FUN_005063b0(rc.local_10, rc.local_8, (int)sVar5, (short)iVar2);
            }
            sVar5 = (short)(sVar5 + DAT_0066920e);
            local_16 = (unsigned short)(local_16 + 1);
        } while (sVar5 < rc.local_4);
    }
    return;
}
