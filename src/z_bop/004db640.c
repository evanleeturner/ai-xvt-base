/* SPDX-License-Identifier: MIT */
/* z_bop 004db640 */
extern unsigned char DAT_00aabd11[];
extern unsigned char DAT_00aa7f9b[];

int FUN_004dbfa0(char *param_1, int param_2);
unsigned int FUN_004dbf70(int param_1);
unsigned int FUN_004b6780(int *param_1, int param_2, int param_3, unsigned int param_4, int param_5);

unsigned int __cdecl FUN_004db640(int param_1, char *param_2, int *param_3, unsigned int param_4)
{
    char cVar1;
    unsigned char bVar2;
    int *piVar3;
    int iVar4;
    unsigned int uVar5;
    int iVar6;
    unsigned int uVar7;
    int iVar8;
    int iVar9;
    unsigned int local_420;
    unsigned int local_418;
    unsigned int local_414[5];

    if (param_2 == (char *)0) {
        return 0;
    }
    if ((-1 < param_1) && (param_1 < 0x100)) {
        /* DAT_00aabd11 is at an odd address: packed pointer table */
        __builtin_memcpy(&piVar3, &DAT_00aabd11[(unsigned int)param_1 * 4u], 4);
        if (piVar3 != (int *)0) {
            iVar6 = FUN_004dbfa0(param_2, param_1);
            iVar4 = param_3[3];
            iVar8 = (int)((unsigned int)(((int)((unsigned int)param_3[2] - (unsigned int)*param_3)) >> 1)
                          - (unsigned int)(iVar6 >> 1)
                          + (unsigned int)*param_3);
            iVar6 = param_3[1];
            iVar9 = 0;
            uVar5 = FUN_004dbf70(param_1);
            local_418 = 0;
            cVar1 = *param_2;
            local_420 = param_4;
            while ((cVar1 != '\0') && (iVar8 < 0x280)) {
                bVar2 = (unsigned char)param_2[iVar9];
                if (bVar2 == 1) {
                    local_420 = param_4;
                }
                else if ((bVar2 < 2) || (7 < bVar2)) {
                    uVar7 = (unsigned int)bVar2;
                    local_414[0] = (unsigned int)*(unsigned char *)(uVar7 + 0x504 + (unsigned int)(unsigned char *)piVar3);
                    local_414[1] = (unsigned int)*(unsigned char *)((unsigned char *)piVar3 + uVar7 + 0x404);
                    local_414[3] = 0;
                    local_414[2] = 1;
                    local_414[4] = (unsigned int)piVar3[uVar7 + 1] + (unsigned int)*piVar3;
                    uVar7 = FUN_004b6780((int *)&local_414[0], iVar8,
                                         (int)((unsigned int)(((int)((unsigned int)iVar4 - (unsigned int)iVar6)) >> 1)
                                               + (unsigned int)iVar6
                                               - (uVar5 >> 1)),
                                         local_420, 1);
                    local_418 = local_418 | uVar7;
                    iVar8 = (int)((unsigned int)iVar8
                                  + (unsigned int)*(unsigned char *)((unsigned int)(unsigned char)param_2[iVar9] + 0x504 + (unsigned int)(unsigned char *)piVar3)
                                  + (unsigned int)*(unsigned char *)((unsigned char *)piVar3 + 0x609));
                }
                else {
                    __builtin_memcpy(&local_420, &DAT_00aa7f9b[(unsigned int)bVar2 * 4u], 4);
                }
                iVar9 = iVar9 + 1;
                cVar1 = param_2[iVar9];
            }
            return local_418;
        }
        return 0;
    }
    return 0;
}
