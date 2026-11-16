/* SPDX-License-Identifier: MIT */
/* z_xvt 004f3890 */
extern short DAT_00650424;
extern short DAT_00650426;
extern short DAT_0065042c;
extern short DAT_0065042e;

int  __cdecl FUN_004df760(int param_1, int param_2, int param_3);
void __cdecl FUN_004dd000(int *param_1, unsigned int *param_2);
void __cdecl FUN_004f82f0(int param_1, int param_2, int param_3, short param_4);
void __cdecl FUN_004f8160(int param_1, int param_2, int param_3, short param_4);

void __cdecl FUN_004f3890(unsigned int *param_1)
{
    int bVar1;
    int iVar2;
    int iVar3;
    short sVar4;
    short sVar5;
    unsigned short uVar6;
    short sVar7;
    unsigned short local_18;
    unsigned short local_16;
    /* contiguous 4-int block filled by FUN_004dd000:
       [0]=local_10, [1]=local_c, [2]=local_8, [3]=local_4 */
    int local_10_blk[4];
#define local_10 local_10_blk[0]
#define local_c  local_10_blk[1]
#define local_8  local_10_blk[2]
#define local_4  local_10_blk[3]

    iVar2 = FUN_004df760(0x96, 0, 0);
    iVar3 = FUN_004df760(0x50, 0, 0);
    FUN_004dd000(local_10_blk, param_1);

    local_18 = (unsigned short)(((unsigned int)(int)DAT_00650424 +
                                 ((unsigned int)((int)DAT_00650424 >> 0x1f) & 0xffU)) >> 8);
    if ((0 < DAT_00650424) && ((char)DAT_00650424 != '\0')) {
        local_18 = (unsigned short)(local_18 + 1);
    }
    for (sVar4 = (short)((((0U - (unsigned int)(int)DAT_00650424) & 0xffU) *
                          (unsigned int)(int)DAT_0065042c) >> 8)
               + (short)((int)((unsigned int)local_8 - (unsigned int)local_10) >> 1)
               + (short)local_10;
         local_10 < sVar4;
         sVar4 = (short)(sVar4 - DAT_0065042c)) {
        local_18 = (unsigned short)(local_18 - 1);
    }

    local_16 = (unsigned short)(((unsigned int)(int)DAT_00650426 +
                                 ((unsigned int)((int)DAT_00650426 >> 0x1f) & 0xffU)) >> 8);
    if ((0 < DAT_00650426) && ((char)DAT_00650426 != '\0')) {
        local_16 = (unsigned short)(local_16 + 1);
    }
    for (sVar5 = (short)((((0U - (unsigned int)(int)DAT_00650426) & 0xffU) *
                          (unsigned int)(int)DAT_0065042e) >> 8)
               + (short)((int)((unsigned int)local_4 - (unsigned int)local_c) >> 1)
               + (short)local_c;
         local_c < sVar5;
         sVar5 = (short)(sVar5 - DAT_0065042e)) {
        local_16 = (unsigned short)(local_16 - 1);
    }

    if (0xf < DAT_0065042c) {
        bVar1 = 0x1f < DAT_0065042c;
        sVar7 = sVar4;
        uVar6 = local_18;
        if (sVar4 < local_8) {
            do {
                if (((uVar6 & 3) != 0) && (((uVar6 & 3) == 2) || (bVar1))) {
                    FUN_004f82f0(local_c, local_4, (int)sVar7, (short)iVar3);
                }
                sVar7 = (short)(sVar7 + DAT_0065042c);
                uVar6 = (unsigned short)(uVar6 + 1);
            } while (sVar7 < local_8);
        }
        sVar7 = sVar5;
        uVar6 = local_16;
        if (sVar5 < local_4) {
            do {
                if (((uVar6 & 3) != 0) && (((uVar6 & 3) == 2) || (bVar1))) {
                    FUN_004f8160(local_10, local_8, (int)sVar7, (short)iVar3);
                }
                sVar7 = (short)(sVar7 + DAT_0065042e);
                uVar6 = (unsigned short)(uVar6 + 1);
            } while (sVar7 < local_4);
        }
    }

    if (sVar4 < local_8) {
        do {
            if ((local_18 & 3) == 0) {
                FUN_004f82f0(local_c, local_4, (int)sVar4, (short)iVar2);
            }
            sVar4 = (short)(sVar4 + DAT_0065042c);
            local_18 = (unsigned short)(local_18 + 1);
        } while (sVar4 < local_8);
    }
    if (sVar5 < local_4) {
        do {
            if ((local_16 & 3) == 0) {
                FUN_004f8160(local_10, local_8, (int)sVar5, (short)iVar2);
            }
            sVar5 = (short)(sVar5 + DAT_0065042e);
            local_16 = (unsigned short)(local_16 + 1);
        } while (sVar5 < local_4);
    }
    return;
#undef local_10
#undef local_c
#undef local_8
#undef local_4
}
