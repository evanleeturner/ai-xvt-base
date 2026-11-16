/* SPDX-License-Identifier: MIT */
/* z_bop 004dbbd0 */
extern unsigned char DAT_00aabd11[];
extern unsigned char DAT_00aa7f9b[];

void __cdecl FUN_004d4cf0(void *param_1);
void __cdecl FUN_004d4d20(void *param_1);
int  __cdecl FUN_004dbfa0(char *param_1, int param_2);
void __cdecl FUN_004b6780(int *param_1, int param_2, int param_3, unsigned int param_4, int param_5);

int __cdecl
FUN_004dbbd0(int param_1, char *param_2, int *param_3, unsigned int param_4, int param_5, int param_6)
{
    unsigned char bVar1;
    int *piVar2;
    unsigned char *pbBase;
    int bVar3;
    int iVar4;
    unsigned int uVar5;
    int iVar6;
    int iVar7;
    int local_544;
    int local_540;
    int local_53c;
    int local_534;
    unsigned int local_530;
    unsigned int local_52c;
    unsigned int local_524[4];
    unsigned char local_514[256];
    unsigned int local_414[5];

    if (param_2 == (char *)0) {
        return 0;
    }
    if (*param_2 == '\0') {
        return 0;
    }
    if ((param_1 < 0) || (0xff < param_1)) {
        return 0;
    }
    __builtin_memcpy(&piVar2, DAT_00aabd11 + (unsigned int)param_1 * 4, 4);
    if (piVar2 == (int *)0) {
        return 0;
    }
    pbBase = (unsigned char *)piVar2;
    local_534 = 0;
    iVar7 = *param_3;
    iVar6 = 0;
    local_544 = 0;
    bVar3 = 1;
    local_52c = param_4;
    local_540 = param_3[1];
    FUN_004d4cf0(local_524);
    FUN_004d4d20(param_3);
LAB_004dbc9a:
    do {
        local_530 = local_52c;
        bVar1 = (unsigned char)param_2[local_534];
        if (bVar1 == 1) {
            local_530 = param_4;
        }
        else if ((bVar1 < 2) || (7 < bVar1)) {
            if ((bVar1 == 0x20) || (((bVar1 == 10 || ((param_2 + local_534)[1] == 0)) || (bVar1 == 0x24)))) {
                if ((bVar3) && (bVar1 == 0x20)) {
                    local_534 = local_534 + 1;
                    goto LAB_004dbc9a;
                }
                if ((bVar1 != 10) && (bVar1 != 0x24)) {
                    local_514[local_544] = bVar1;
                    local_544 = local_544 + 1;
                }
                local_514[local_544] = 0;
                iVar4 = FUN_004dbfa0((char *)local_514, param_1);
                if ((int)((unsigned int)param_3[2] - (unsigned int)param_1) < (int)((unsigned int)iVar4 + (unsigned int)iVar7)) {
                    if (param_6 <= iVar6) {
                        local_540 = (int)((unsigned int)local_540 + (unsigned int)param_5 + (unsigned int)param_1);
                    }
                    iVar6 = iVar6 + 1;
                    bVar3 = 1;
                    iVar7 = *param_3;
                }
                local_53c = 0;
                if (0 < local_544) {
                    do {
                        if (iVar6 < param_6) {
                            iVar7 = (int)((unsigned int)iVar7 +
                                          (unsigned int)*(unsigned char *)(pbBase + (unsigned int)local_514[local_53c] + 0x504) +
                                          (unsigned int)*(unsigned char *)(pbBase + 0x609));
                            if (param_3[2] < iVar7) {
joined_r0x004dbeb4:
                                if (param_6 <= iVar6) {
                                    local_540 = (int)((unsigned int)local_540 + (unsigned int)param_5 + (unsigned int)param_1);
                                }
                                iVar6 = iVar6 + 1;
                                iVar7 = *param_3;
                            }
                        }
                        else {
                            uVar5 = (unsigned int)local_514[local_53c];
                            local_414[0] = (unsigned int)*(unsigned char *)(pbBase + uVar5 + 0x504);
                            local_414[1] = (unsigned int)*(unsigned char *)(pbBase + uVar5 + 0x404);
                            local_414[3] = 0;
                            local_414[2] = 1;
                            local_414[4] = (unsigned int)piVar2[uVar5 + 1] + (unsigned int)*piVar2;
                            FUN_004b6780((int *)local_414, iVar7, local_540, local_52c, 1);
                            iVar7 = (int)((unsigned int)iVar7 +
                                          (unsigned int)*(unsigned char *)(pbBase + 0x609) +
                                          (unsigned int)*(unsigned char *)(pbBase + uVar5 + 0x504));
                            if ((int)((unsigned int)param_3[2] - (unsigned int)param_1) < iVar7) goto joined_r0x004dbeb4;
                        }
                        local_53c = local_53c + 1;
                    } while (local_53c < local_544);
                }
                if ((bVar1 == 10) || (bVar1 == 0x24)) {
                    if (param_6 <= iVar6) {
                        local_540 = (int)((unsigned int)local_540 + (unsigned int)param_5 + (unsigned int)param_1);
                    }
                    iVar6 = iVar6 + 1;
                    bVar3 = 1;
                    iVar7 = *param_3;
                }
                local_544 = 0;
            }
            else {
                bVar3 = 0;
                local_514[local_544] = bVar1;
                local_544 = local_544 + 1;
            }
        }
        else {
            __builtin_memcpy(&local_530, DAT_00aa7f9b + (unsigned int)bVar1 * 4, 4);
        }
        local_52c = local_530;
        local_534 = local_534 + 1;
        if (param_2[local_534] == '\0') {
            FUN_004d4d20(local_524);
            return iVar6;
        }
    } while (1);
}
