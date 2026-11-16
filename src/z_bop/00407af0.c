/* SPDX-License-Identifier: MIT */
/* z_bop 00407af0 */
/* Prototypes for every callee */
void FUN_004079e0(const char *param_1, ...);
unsigned char *FUN_004079f0(unsigned int param_1);
void FUN_004b3280(int param_1);
void FUN_004b0b60(unsigned int *param_1, unsigned int param_2);
void FUN_004b0b90(unsigned short *param_1, unsigned int param_2);
int FUN_004b2680(int *param_1, int param_2, int param_3, int param_4);

/* Globals (types inferred from use; names are the decompiler's placeholders) */
extern unsigned char DAT_0052f8f0[];
extern unsigned char DAT_0051a540[];
extern unsigned char DAT_0051a530[];
extern unsigned char DAT_00a00530[];
extern unsigned char DAT_005233c8;
extern int *DAT_00528c5c;
extern const char DAT_0051a638[];   /* "Error: Bitmap too large! (%d,%d)\n" */
extern const char DAT_0051a600[];   /* "AddToTextureCache returned NULL! (colorkey, (%d,%d))\n" */

unsigned char *FUN_00407af0(int param_1, int param_2, unsigned short *param_3,
                            unsigned char *param_4, int param_5)
{
    unsigned char *pbVar1;
    unsigned char bVar2;
    unsigned int uVar3;
    unsigned char bVar4;
    unsigned char *puVar5;
    unsigned int uVar6;
    int iVar7;
    unsigned char bVar8;
    unsigned int uVar9;
    unsigned char *pbVar10;
    unsigned char *pbVar11;
    int *piVar12;
    int local_f0;
    int local_ec;
    unsigned int local_e4;
    int local_d8[0x36];

    local_e4 = 0;
    if (0x10000 < (int)((unsigned int)param_2 * (unsigned int)param_1)) {
        FUN_004079e0(DAT_0051a638, param_1, param_2);
        return (unsigned char *)0;
    }
    puVar5 = FUN_004079f0((unsigned int)param_4);
    if (*(int *)(puVar5 + 0x78) != 0) {
        FUN_004b3280((int)puVar5);
        return puVar5;
    }
    pbVar10 = DAT_0052f8f0;
    bVar2 = 0;
    local_ec = 0;
    if (0 < param_2) {
        do {
            bVar4 = *param_4;
            if (bVar4 == 0xff) break;
            pbVar11 = pbVar10 + param_1;
            local_f0 = 0;
            while (bVar4 != 0xfe) {
                bVar4 = *param_4;
                if (bVar4 == 0xfb) {
                    bVar2 = param_4[1];
                    param_4 = param_4 + 3;
                }
                else if (bVar4 == 0xfc) {
                    pbVar1 = param_4 + 1;
                    param_4 = param_4 + 2;
                    bVar4 = (unsigned char)(*pbVar1 + 1);
                    if (local_f0 < param_1) {
                        local_f0 = (int)((unsigned int)local_f0 + (unsigned int)bVar4);
                        if (param_1 < local_f0) {
                            local_f0 = (int)(((unsigned int)local_f0 & 0xffffff00u) |
                                             (unsigned char)((unsigned char)local_f0 - bVar4));
                            bVar4 = (unsigned char)((unsigned char)param_1 - (unsigned char)local_f0);
                            local_f0 = param_1;
                        }
                        for (; bVar4 != 0; bVar4 = (unsigned char)(bVar4 - 1)) {
                            *pbVar10 = 0;
                            pbVar10 = pbVar10 + 1;
                        }
                    }
                }
                else {
                    if (bVar4 == 0xfd) {
                        bVar4 = param_4[1];
                        bVar8 = param_4[2];
                        param_4 = param_4 + 3;
                    }
                    else {
                        param_4 = param_4 + 1;
                        bVar8 = (unsigned char)((bVar4 >> (DAT_0051a540[param_5] & 0x1f)) + bVar2);
                        bVar4 = (unsigned char)(bVar4 & DAT_0051a530[param_5]);
                    }
                    bVar4 = (unsigned char)(bVar4 + 1);
                    if (local_e4 < bVar8) {
                        local_e4 = (unsigned int)bVar8;
                    }
                    if (local_f0 < param_1) {
                        local_f0 = (int)((unsigned int)local_f0 + (unsigned int)bVar4);
                        if (param_1 < local_f0) {
                            local_f0 = (int)(((unsigned int)local_f0 & 0xffffff00u) |
                                             (unsigned char)((unsigned char)local_f0 - bVar4));
                            bVar4 = (unsigned char)((unsigned char)param_1 - (unsigned char)local_f0);
                            local_f0 = param_1;
                        }
                        for (; bVar4 != 0; bVar4 = (unsigned char)(bVar4 - 1)) {
                            *pbVar10 = bVar8;
                            pbVar10 = pbVar10 + 1;
                        }
                    }
                }
                bVar4 = *param_4;
            }
            param_4 = param_4 + 1;
            if (pbVar10 < pbVar11) {
                uVar9 = (unsigned int)((int)pbVar11 - (int)pbVar10);
                pbVar11 = pbVar10;
                for (uVar6 = uVar9 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
                    pbVar11[0] = 0;
                    pbVar11[1] = 0;
                    pbVar11[2] = 0;
                    pbVar11[3] = 0;
                    pbVar11 = pbVar11 + 4;
                }
                pbVar10 = pbVar10 + uVar9;
                for (uVar9 = uVar9 & 3; uVar9 != 0; uVar9 = uVar9 - 1) {
                    *pbVar11 = 0;
                    pbVar11 = pbVar11 + 1;
                }
            }
            local_ec = local_ec + 1;
        } while (local_ec < param_2);
    }
    if (local_ec < param_2) {
        uVar9 = (unsigned int)(param_2 - local_ec) * (unsigned int)param_1;
        for (uVar6 = uVar9 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
            pbVar10[0] = 0;
            pbVar10[1] = 0;
            pbVar10[2] = 0;
            pbVar10[3] = 0;
            pbVar10 = pbVar10 + 4;
        }
        for (uVar9 = uVar9 & 3; uVar9 != 0; uVar9 = uVar9 - 1) {
            *pbVar10 = 0;
            pbVar10 = pbVar10 + 1;
        }
    }
    piVar12 = local_d8;
    for (iVar7 = 0x36; iVar7 != 0; iVar7 = iVar7 + -1) {
        *piVar12 = 0;
        piVar12 = piVar12 + 1;
    }
    local_d8[0] = 0;
    local_d8[3] = param_1;
    local_d8[4] = param_2;   /* local_c8 */
    local_d8[6] = param_1;   /* local_c0 */
    local_d8[0x17] = (int)DAT_0052f8f0;   /* local_7c */
    local_d8[9] = 8;         /* local_b4 */
    local_d8[8] = 0;         /* local_b8 */
    uVar3 = (unsigned int)DAT_00528c5c[4];
    if (DAT_00528c5c[3] != 0) {
        DAT_00528c5c[4] = 0;
    }
    if (DAT_00528c5c[4] == 0) {
        *param_3 = *(unsigned short *)(DAT_00a00530 + (unsigned int)DAT_005233c8 * 2);
        FUN_004b0b60((unsigned int *)param_3, local_e4 + 1);
    }
    else {
        *param_3 = *(unsigned short *)(DAT_00a00530 + (unsigned int)DAT_005233c8 * 2);
        FUN_004b0b90(param_3, local_e4 + 1);
    }
    iVar7 = FUN_004b2680(local_d8, (int)puVar5, 1, 0);
    if (iVar7 == 0) {
        FUN_004079e0(DAT_0051a600, param_1, param_2);
        if (DAT_00528c5c[3] != 0) {
            DAT_00528c5c[4] = (int)uVar3;
        }
        return (unsigned char *)0;
    }
    if (DAT_00528c5c[3] != 0) {
        DAT_00528c5c[4] = (int)uVar3;
    }
    return puVar5;
}
