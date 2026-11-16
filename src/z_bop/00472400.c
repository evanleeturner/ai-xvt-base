/* SPDX-License-Identifier: MIT */
/* z_bop 00472400 */
extern unsigned short DAT_009a7ed0[];
extern unsigned int _DAT_005233a0;
extern unsigned char DAT_009e9a5b[];
extern unsigned char DAT_009e9a5f[];
extern unsigned char DAT_009e9a63[];
extern int DAT_009ecc34;
extern int DAT_00a004a8;
extern int DAT_00a004a4;
extern int DAT_00a004b0;
extern int DAT_00a004b8;
extern int DAT_00a004cc;
extern int DAT_00a00488;
extern int DAT_009fe7f4;
extern int DAT_00a00490;
extern int DAT_00a00480;
extern int DAT_009d77b8;
extern int DAT_009d77b4;
extern int DAT_009d77b0;
extern int DAT_009d77ac;
extern int DAT_009d7690;
extern int DAT_009d768c;
extern int _DAT_009d7688;
extern int _DAT_009d77a4;
extern int _DAT_009d77a0;
extern float _DAT_005181d8;
extern float _DAT_005181d0;
extern int DAT_0060f1c0;
extern unsigned int *DAT_005271d8;
extern unsigned int DAT_0060f210;
extern int _DAT_0060f224;
extern int DAT_0060f220;
extern int DAT_0060f214;
extern unsigned char DAT_0060f228[];
extern unsigned char DAT_005271e0[];
extern int DAT_0060f1e8;
extern int _DAT_0060f208;
extern int _DAT_0060f1f0;
extern unsigned int *DAT_0060f1ec;
extern int DAT_0060f1d8;
extern int DAT_0060f204;
extern int DAT_005271d4;
extern int DAT_005272a0;

int *FUN_004ac720(int param_1);
void FUN_00471ff0(int *param_1);
void FUN_00420fe0(float *param_1, float *param_2);
void FUN_004720d0(void *param_1, void *param_2, int param_3, int param_4);
int FUN_004afa30(int param_1);
void FUN_00472360(int *param_1, int param_2, int param_3, int param_4);
void FUN_00472c90(int param_1, unsigned int *param_2, void *param_3);
void FUN_004ac740(void);

void __cdecl FUN_00472400(int param_1)
{
    unsigned char bVar1;
    unsigned int *puVar2;
    int bVar3;
    int *piVar4;
    int iVar5;
    int iVar6;
    int *piVar7;
    int *piVar8;
    unsigned short uStack_1b4 = 0;
    int local_1b0;
    int local_1a8;
    int local_1a4;
    /* blk[0] = local_1a0 (int), blk[1] = local_19c, blk[2] = local_198, ... */
    float blk[52];
    int saved[52];
    unsigned char *p1 = (unsigned char *)param_1;
    int t0, t1, t2;
    int v;
    unsigned char *base;

    __builtin_memcpy(&t0, p1 + 0x1f, 4);
    if (t0 == 0) {
        _DAT_005233a0 = 0;
    } else {
        _DAT_005233a0 = (unsigned int)*((unsigned char *)t0 + 0x8a);
    }
    piVar4 = FUN_004ac720((int)(((unsigned int)uStack_1b4 << 16) | (unsigned int)DAT_009a7ed0[p1[3]]));
    if ((int *)*piVar4 != piVar4) {
        FUN_00471ff0(piVar4);
    }
    piVar7 = (int *)blk;
    for (iVar5 = 0x34; iVar5 != 0; iVar5 = (int)((unsigned int)iVar5 - 1U)) {
        *piVar7 = 0;
        piVar7 = piVar7 + 1;
    }
    __builtin_memcpy(&blk[0], &param_1, 4); /* local_1a0 */

    base = DAT_009e9a5b + (int)((unsigned int)DAT_009ecc34 * 0x5bdU);
    __builtin_memcpy(&v, base, 4);
    blk[2] = (float)(int)((unsigned int)*(int *)(p1 + 4) - (unsigned int)v);
    base = DAT_009e9a5f + (int)((unsigned int)DAT_009ecc34 * 0x5bdU);
    __builtin_memcpy(&v, base, 4);
    blk[3] = (float)(int)((unsigned int)*(int *)(p1 + 8) - (unsigned int)v);
    blk[5] = (float)DAT_00a004a8 * _DAT_005181d8;
    blk[6] = (float)DAT_00a004a4 * _DAT_005181d8;
    blk[7] = (float)DAT_00a004b0 * _DAT_005181d8;
    blk[9] = (float)DAT_00a004b8 * _DAT_005181d8;
    blk[8] = (float)DAT_00a004cc * _DAT_005181d8;
    blk[11] = (float)DAT_00a00488 * _DAT_005181d8;
    blk[10] = (float)DAT_009fe7f4 * _DAT_005181d8;
    blk[13] = (float)DAT_00a00490 * _DAT_005181d8;
    blk[12] = (float)DAT_00a00480 * _DAT_005181d8;
    base = DAT_009e9a63 + (int)((unsigned int)DAT_009ecc34 * 0x5bdU);
    __builtin_memcpy(&v, base, 4);
    blk[4] = (float)(int)((unsigned int)*(int *)(p1 + 0xc) - (unsigned int)v);
    FUN_00420fe0(&blk[2], &blk[5]);

    blk[6] = (float)DAT_009d77b8 * _DAT_005181d8;
    blk[7] = (float)DAT_009d77b4 * _DAT_005181d8;
    blk[8] = (float)DAT_009d77b0 * _DAT_005181d8;
    blk[5] = (float)DAT_009d77ac * _DAT_005181d8;
    blk[9] = (float)DAT_009d7690 * _DAT_005181d8;
    blk[10] = (float)DAT_009d768c * _DAT_005181d8;
    blk[11] = (float)_DAT_009d7688 * _DAT_005181d8;
    blk[12] = (float)_DAT_009d77a4 * _DAT_005181d8;
    blk[13] = (float)_DAT_009d77a0 * _DAT_005181d8;
    blk[14] = -blk[2];
    blk[15] = -blk[3];
    blk[16] = -blk[4];
    blk[17] = blk[5];
    blk[18] = blk[8];
    blk[19] = blk[11];
    blk[20] = blk[6];
    blk[21] = blk[9];
    blk[22] = blk[12];
    blk[23] = blk[7];
    blk[24] = blk[10];
    blk[25] = blk[13];
    FUN_00420fe0(&blk[14], &blk[17]);

    DAT_0060f1c0 = 0;
    if (DAT_005271d8 == (unsigned int *)0) {
        DAT_005271d8 = &DAT_0060f210;
        _DAT_0060f224 = 8;
        DAT_0060f220 = 8;
        DAT_0060f214 = 0x10;
        DAT_0060f210 = 0x100;
        FUN_004720d0(DAT_0060f228, DAT_005271e0, 8, 8);
    }
    DAT_0060f1e8 = 0;
    _DAT_0060f208 = 0;
    _DAT_0060f1f0 = 0;
    bVar3 = 0;
    DAT_0060f1ec = DAT_005271d8;
    local_1a4 = 0;
    DAT_0060f1d8 = 0;
    DAT_0060f204 = 0;
    local_1a8 = 0;
    __builtin_memcpy(&t0, (unsigned char *)piVar4 + 6, 4);
    if (0 < t0) {
        local_1b0 = 0;
        do {
            blk[1] = 0.0f;
            __builtin_memcpy(&t1, (unsigned char *)piVar4 + 10, 4);
            puVar2 = *(unsigned int **)((unsigned int)t1 + (unsigned int)local_1b0);
            iVar5 = local_1a8;
            if (puVar2[1] == 0x14) {
LAB_00472857:
                local_1a8 = iVar5;
                DAT_005271d4 = (int)((unsigned int)DAT_005271d4 + 1U);
                FUN_00472c90((int)piVar4, puVar2, blk);
                iVar5 = local_1a8;
                if (bVar3) {
                    piVar7 = saved;
                    piVar8 = (int *)blk;
                    for (iVar6 = 0x34; iVar6 != 0; iVar6 = (int)((unsigned int)iVar6 - 1U)) {
                        *piVar8 = *piVar7;
                        piVar7 = piVar7 + 1;
                        piVar8 = piVar8 + 1;
                    }
                    bVar3 = 0;
                }
            } else {
                iVar5 = (int)((unsigned int)local_1a8 + 1U);
                __builtin_memcpy(&t0, p1 + 0x1f, 4);
                if (t0 == 0) goto LAB_00472857;
                __builtin_memcpy(&iVar6, (unsigned char *)t0 + 0xa9, 4);
                if (iVar6 == 0) goto LAB_00472857;
                if (*(char *)((unsigned int)local_1a8 + 0x2d8U + (unsigned int)iVar6) == '\0') {
                    bVar1 = *(unsigned char *)((unsigned int)local_1a8 + 0x30aU + (unsigned int)iVar6);
                    if (*(char *)(p1 + 3) == '\x04') {
                        if (DAT_005272a0 == -1) {
                            DAT_005272a0 = FUN_004afa30((int)piVar4);
                            if (DAT_005272a0 == -1) goto LAB_00472841;
                        }
                        __builtin_memcpy(&t0, p1 + 0x1f, 4);
                        __builtin_memcpy(&t2, (unsigned char *)t0 + 0xa9, 4);
                        if (*(char *)((unsigned int)t2 + 0x30aU + (unsigned int)DAT_005272a0) != '\0') {
                            piVar7 = (int *)blk;
                            piVar8 = saved;
                            for (iVar6 = 0x34; iVar6 != 0; iVar6 = (int)((unsigned int)iVar6 - 1U)) {
                                *piVar8 = *piVar7;
                                piVar7 = piVar7 + 1;
                                piVar8 = piVar8 + 1;
                            }
                            bVar3 = 1;
                            FUN_00472360(piVar4, param_1, (int)blk, DAT_005272a0);
                        }
                    }
LAB_00472841:
                    blk[1] = (float)bVar1 * _DAT_005181d0;
                    goto LAB_00472857;
                }
            }
            local_1a8 = iVar5;
            local_1b0 = (int)((unsigned int)local_1b0 + 4U);
            local_1a4 = (int)((unsigned int)local_1a4 + 1U);
            __builtin_memcpy(&t0, (unsigned char *)piVar4 + 6, 4);
        } while (local_1a4 < t0);
    }
    FUN_004ac740();
    return;
}
