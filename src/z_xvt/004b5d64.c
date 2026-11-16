/* SPDX-License-Identifier: MIT */
/* z_xvt 004b5d64 */
extern unsigned int *DAT_0064ee60;
extern int DAT_0052f20c;
extern unsigned int DAT_0052f208;

void __cdecl FUN_004b821c(unsigned int param_1);

unsigned int __cdecl FUN_004b5d64(unsigned int *param_1, unsigned int param_2)
{
    int iVar1;
    unsigned int local_24;
    unsigned int local_1c;
    unsigned int *local_18;
    unsigned int *local_14;
    unsigned int local_10;
    unsigned int local_c;

    local_18 = param_1;
    for (local_c = 0; local_c < param_2; local_c = local_c + local_24) {
        local_24 = 0;
        if (local_18[3] == 0) {
            local_14 = local_18;
            local_1c = local_c;
            for (; ((local_1c < param_2 && (local_14[3] == 0)) && (local_14[4] == local_18[4]));
                 local_14 = local_14 + 5) {
                local_24 = local_24 + 1;
                local_1c = local_1c + 1;
            }
            FUN_004b821c(local_18[4]);
            if (DAT_0052f20c != 0) {
                *(unsigned char *)DAT_0064ee60 = 8;
                *(unsigned char *)((int)DAT_0064ee60 + 1) = 8;
                *(unsigned short *)((int)DAT_0064ee60 + 2) = 1;
                DAT_0064ee60 = DAT_0064ee60 + 1;
                *DAT_0064ee60 = 1;
                DAT_0064ee60[1] = 0;
                DAT_0064ee60 = DAT_0064ee60 + 2;
                DAT_0052f20c = 0;
            }
            *(unsigned char *)DAT_0064ee60 = 3;
            *(unsigned char *)((int)DAT_0064ee60 + 1) = 8;
            *(short *)((int)DAT_0064ee60 + 2) = (short)local_24;
            DAT_0064ee60 = DAT_0064ee60 + 1;
            for (local_10 = 0; local_10 < local_24; local_10 = local_10 + 1) {
                *(short *)DAT_0064ee60 = (short)*local_18;
                *(short *)((int)DAT_0064ee60 + 2) = (short)local_18[1];
                *(short *)(DAT_0064ee60 + 1) = (short)local_18[2];
                *(unsigned short *)((int)DAT_0064ee60 + 6) = 0x700;
                DAT_0064ee60 = DAT_0064ee60 + 2;
                local_18 = local_18 + 5;
            }
        }
        else {
            local_14 = local_18;
            local_1c = local_c;
            iVar1 = (int)local_18[3];
            for (; ((local_1c < param_2 && ((int)local_14[3] == iVar1)) && (local_14[4] == local_18[4]));
                 local_14 = local_14 + 5) {
                local_24 = local_24 + 1;
                local_1c = local_1c + 1;
            }
            FUN_004b821c(local_18[4]);
            if ((int)local_18[3] != DAT_0052f20c) {
                *(unsigned char *)DAT_0064ee60 = 8;
                *(unsigned char *)((int)DAT_0064ee60 + 1) = 8;
                *(unsigned short *)((int)DAT_0064ee60 + 2) = 1;
                DAT_0064ee60 = DAT_0064ee60 + 1;
                *DAT_0064ee60 = 1;
                DAT_0064ee60[1] = *(unsigned int *)(iVar1 + 0x88);
                DAT_0064ee60 = DAT_0064ee60 + 2;
                DAT_0052f20c = iVar1;
            }
            *(unsigned char *)DAT_0064ee60 = 3;
            *(unsigned char *)((int)DAT_0064ee60 + 1) = 8;
            *(short *)((int)DAT_0064ee60 + 2) = (short)local_24;
            DAT_0064ee60 = DAT_0064ee60 + 1;
            for (local_10 = 0; local_10 < local_24; local_10 = local_10 + 1) {
                *(short *)DAT_0064ee60 = (short)*local_18;
                *(short *)((int)DAT_0064ee60 + 2) = (short)local_18[1];
                *(short *)(DAT_0064ee60 + 1) = (short)local_18[2];
                *(unsigned short *)((int)DAT_0064ee60 + 6) = 0x700;
                DAT_0064ee60 = DAT_0064ee60 + 2;
                local_18 = local_18 + 5;
            }
        }
    }
    DAT_0052f208 = DAT_0052f208 + param_2;
    return 1;
}
