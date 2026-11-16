/* SPDX-License-Identifier: MIT */
/* z_bop 004b6fb0 */
extern char DAT_0051a854[];
extern unsigned char DAT_00aa7b1f[4]; /* packed 4-byte global at odd address, read via memcpy */
extern char DAT_00aa7b23;

void *FUN_004cc4a0(char *param_1, char *param_2);
void FUN_004cc6c0(void *param_1, void *param_2, int param_3);
void *malloc(unsigned int size);
void FUN_004d6dd0(void *param_1, unsigned int *param_2, int param_3);
int FUN_004b7240(void *param_1, int param_2, int param_3, int param_4);
int FUN_004b7320(void *param_1, int param_2, short *param_3, int param_4);
void FUN_004b7510(unsigned char *param_1, int param_2, int param_3);
void FUN_004cc590(void *param_1);
void free(void *param_1);
void FUN_004b75b0(int *param_1);

unsigned int __cdecl FUN_004b6fb0(char *param_1, unsigned int *param_2, int param_3, int param_4)
{
    void *pFVar1;
    int iVar2;
    unsigned int *puVar3;
    int iVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    unsigned char *_Memory;
    unsigned char *pbVar7;
    int local_43c;
    unsigned int local_438[10]; /* 0x28-byte info header: [1]=width (local_434), [2]=height (local_430), short[6]=planes (local_42c), short[7]=bitcount (local_42a) */
    short local_410[8];
    unsigned char local_400[1024];
    int DAT_00aa7b1f_val;

    _Memory = (unsigned char *)0;
    pFVar1 = FUN_004cc4a0(param_1, DAT_0051a854);
    local_43c = 0;
    pbVar7 = local_400;
    for (iVar4 = 0x100; iVar4 != 0; iVar4 = iVar4 - 1) {
        pbVar7[0] = 0;
        pbVar7[1] = 0;
        pbVar7[2] = 0;
        pbVar7[3] = 0;
        pbVar7 = pbVar7 + 4;
    }
    if (pFVar1 != (void *)0) {
        FUN_004cc6c0(pFVar1, local_410, 0xe);
        if (local_410[0] == 0x4d42) {
            FUN_004cc6c0(pFVar1, local_438, 0x28);
            uVar5 = (unsigned int)((int)local_438[1] >> 0x1f);
            iVar2 = (int)(((((local_438[1] ^ uVar5) - uVar5) & 3) ^ uVar5) - uVar5);
            iVar4 = 0;
            if (iVar2 != 0) {
                iVar4 = (int)(4u - (unsigned int)iVar2);
            }
            _Memory = (unsigned char *)malloc(local_438[1] * local_438[2] + (unsigned int)iVar4);
            if (_Memory != (unsigned char *)0) {
                uVar6 = local_438[1] * local_438[2] + (unsigned int)iVar4;
                pbVar7 = _Memory;
                for (uVar5 = uVar6 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                    pbVar7[0] = 0;
                    pbVar7[1] = 0;
                    pbVar7[2] = 0;
                    pbVar7[3] = 0;
                    pbVar7 = pbVar7 + 4;
                }
                for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
                    *pbVar7 = 0;
                    pbVar7 = pbVar7 + 1;
                }
                if (((short *)local_438)[6] == 1) {
                    if (((short *)local_438)[7] == 4) {
                        FUN_004d6dd0(pFVar1, (unsigned int *)local_400, 0x10);
                        local_43c = FUN_004b7240(pFVar1, (int)_Memory, (int)local_410, (int)local_438);
                    } else if (((short *)local_438)[7] == 8) {
                        FUN_004d6dd0(pFVar1, (unsigned int *)local_400, 0x100);
                        local_43c = FUN_004b7320(pFVar1, (int)_Memory, local_410, (int)local_438);
                    }
                    __builtin_memcpy(&DAT_00aa7b1f_val, DAT_00aa7b1f, 4);
                    if (DAT_00aa7b1f_val == 8) {
                        if ((local_43c == 1) && (param_3 == 1)) {
                            FUN_004b7510(_Memory, (int)local_400, (int)local_438);
                        }
                    } else if (DAT_00aa7b1f_val == 0x10) {
                        pbVar7 = local_400;
                        puVar3 = param_2 + 5;
                        do {
                            if (DAT_00aa7b23 == '\0') {
                                *puVar3 = (((unsigned int)(*pbVar7 >> 3) * 0x40u + (unsigned int)(pbVar7[1] >> 2)) * 0x20u) +
                                          (unsigned int)(pbVar7[2] >> 3);
                            } else {
                                *puVar3 = (((unsigned int)(pbVar7[1] >> 3) + (unsigned int)(*pbVar7 >> 3) * 0x20u) * 0x20u) +
                                          (unsigned int)(pbVar7[2] >> 3);
                            }
                            pbVar7 = pbVar7 + 4;
                            puVar3 = puVar3 + 1;
                        } while (pbVar7 < local_400 + 1024);
                    }
                }
            }
        }
        FUN_004cc590(pFVar1);
    }
    if (local_43c == 1) {
        *param_2 = local_438[1];
        param_2[1] = local_438[2];
        param_2[4] = (unsigned int)_Memory;
        param_2[2] = 0;
        param_2[3] = local_438[2] * local_438[1];
        if (param_4 == 1) {
            FUN_004b75b0((int *)param_2);
        }
        return 1;
    }
    if (_Memory != (unsigned char *)0) {
        free(_Memory);
    }
    return 0;
}
