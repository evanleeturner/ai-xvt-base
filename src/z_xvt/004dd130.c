/* SPDX-License-Identifier: MIT */
/* z_xvt 004dd130 */
extern unsigned int DAT_00758830;
extern unsigned char DAT_0075964b[4];  /* packed uint at odd address */
extern unsigned char DAT_00759657[4];  /* packed uint at odd address */
extern char DAT_0075965b;

void __cdecl FUN_004dd000(int *param_1, int *param_2);
void __cdecl FUN_004dd030(int *param_1, int param_2, int param_3);
void __cdecl FUN_004dd0b0(int *param_1);

void __cdecl FUN_004dd130(int *param_1, int param_2, int param_3, unsigned int param_4)
{
    unsigned int uVar1;
    unsigned int uVar2;
    unsigned int uVar3;
    unsigned int uVar6;
    unsigned char *puVar4;
    unsigned char *puVar7;
    unsigned short *puVar8;
    unsigned short *local_18;
    int local_10;
    int local_c;
    int local_8;
    int local_4;
    int rect[4];
    unsigned int pitch;
    unsigned int bpp;
    unsigned char uVar5;
    unsigned int fill;

    if ((*param_1 < param_1[2]) && (param_1[1] < param_1[3])) {
        FUN_004dd000(rect, param_1);
        FUN_004dd030(rect, param_2, param_3);
        local_10 = rect[0];
        local_c = rect[1];
        local_8 = rect[2];
        local_4 = rect[3];
        if ((local_10 < 0x281) && (((-1 < local_8) && (local_c < 0x1e1)) && (-1 < local_4))) {
            FUN_004dd0b0(rect);
            local_10 = rect[0];
            local_c = rect[1];
            local_8 = rect[2];
            local_4 = rect[3];

            __builtin_memcpy(&pitch, DAT_0075964b, 4);
            __builtin_memcpy(&bpp, DAT_00759657, 4);
            uVar2 = pitch;
            local_4 = (int)((unsigned int)local_4 + 1u);
            uVar1 = ((unsigned int)local_8 - (unsigned int)local_10) + 1u;

            if (bpp == 8) {
                puVar4 = (unsigned char *)((unsigned int)local_c * pitch + (unsigned int)local_10 + DAT_00758830);
                if (local_c < local_4) {
                    local_4 = (int)((unsigned int)local_4 - (unsigned int)local_c);
                    do {
                        uVar5 = (unsigned char)param_4;
                        fill = ((unsigned int)uVar5 << 24) | ((unsigned int)uVar5 << 16) |
                               ((unsigned int)uVar5 << 8) | (unsigned int)uVar5;
                        puVar7 = puVar4;
                        for (uVar6 = uVar1 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
                            __builtin_memcpy(puVar7, &fill, 4);
                            puVar7 = puVar7 + 4;
                        }
                        for (uVar6 = uVar1 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
                            *puVar7 = uVar5;
                            puVar7 = puVar7 + 1;
                        }
                        puVar4 = puVar4 + uVar2;
                        local_4 = local_4 + -1;
                    } while (local_4 != 0);
                    return;
                }
            }
            else {
                if (bpp != 0x10) {
                    return;
                }
                local_18 = (unsigned short *)(pitch * (unsigned int)local_c + (unsigned int)local_10 * 2u + DAT_00758830);
                if (DAT_0075965b == '\0') {
                    if (local_c < local_4) {
                        local_4 = (int)((unsigned int)local_4 - (unsigned int)local_c);
                        uVar2 = pitch & 0xfffffffe;
                        do {
                            uVar6 = uVar1;
                            puVar8 = local_18;
                            if (0 < (int)uVar1) {
                                do {
                                    uVar3 = (unsigned int)*puVar8;
                                    uVar3 = ((((uVar3 & 0xf800) * 0x20 + (uVar3 & 0x7e0) * 8 + (uVar3 & 0x1f)) >> 1)) +
                                            ((((param_4 & 0xf800) * 0x20 + (param_4 & 0x7e0) * 8 + (param_4 & 0x1f)) >> 1));
                                    uVar6 = uVar6 - 1;
                                    *puVar8 = (unsigned short)(((unsigned short)(uVar3 >> 5) & 0xf800) +
                                                               ((unsigned short)(uVar3 >> 3) & 0x7e0) +
                                                               ((unsigned short)uVar3 & 0x1f));
                                    puVar8 = puVar8 + 1;
                                } while (uVar6 != 0);
                            }
                            local_18 = (unsigned short *)((unsigned char *)local_18 + uVar2);
                            local_4 = local_4 + -1;
                        } while (local_4 != 0);
                    }
                }
                else if (local_c < local_4) {
                    uVar2 = pitch & 0xfffffffe;
                    local_4 = (int)((unsigned int)local_4 - (unsigned int)local_c);
                    do {
                        uVar6 = uVar1;
                        puVar8 = local_18;
                        if (0 < (int)uVar1) {
                            do {
                                uVar3 = (unsigned int)*puVar8;
                                uVar3 = ((((uVar3 & 0x7c00) * 0x40 + (uVar3 & 0x3e0) * 8 + (uVar3 & 0x1f)) >> 1)) +
                                        ((((param_4 & 0x7c00) * 0x40 + (param_4 & 0x3e0) * 8 + (param_4 & 0x1f)) >> 1));
                                uVar6 = uVar6 - 1;
                                *puVar8 = (unsigned short)(((unsigned short)(uVar3 >> 6) & 0x7c00) +
                                                           ((unsigned short)(uVar3 >> 3) & 0x3e0) +
                                                           ((unsigned short)uVar3 & 0x1f));
                                puVar8 = puVar8 + 1;
                            } while (uVar6 != 0);
                        }
                        local_18 = (unsigned short *)((unsigned char *)local_18 + uVar2);
                        local_4 = local_4 + -1;
                    } while (local_4 != 0);
                    return;
                }
            }
        }
    }
    return;
}
