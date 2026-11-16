/* SPDX-License-Identifier: MIT */
/* z_xvt 004daac0 */
extern unsigned char DAT_0075d846[];
extern unsigned int DAT_00759ad0[];

int FUN_004db2f0(char *param_1, int param_2);
int FUN_004db2c0(int param_1);
unsigned int FUN_004d2190(int *param_1, int param_2, int param_3, unsigned int param_4, int param_5);

unsigned int FUN_004daac0(int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6)
{
    char cVar1;
    unsigned char bVar2;
    int *piVar3;
    int iVar4;
    int iVar9;
    int iVar6;
    unsigned int uVar7;
    int iVar8;
    int local_420;
    unsigned int local_41c;
    unsigned int local_418;
    int local_414[5]; /* local_414, local_410, local_40c, local_408, local_404 (contiguous on stack) */

    if (param_2 == (char *)0) {
        return 0;
    }
    if ((-1 < param_1) && (param_1 < 0x100)) {
        /* table at 0x75d846 is not 4-byte aligned: read unaligned via memcpy */
        __builtin_memcpy(&piVar3, DAT_0075d846 + (unsigned int)param_1 * 4, 4);
        if (piVar3 != (int *)0) {
            iVar6 = FUN_004db2f0(param_2, param_1);
            if (param_4 == 0) {
                iVar6 = *param_3;
            }
            else {
                iVar6 = (int)((unsigned int)(((int)((unsigned int)param_3[2] - (unsigned int)*param_3) >> 1) - (iVar6 >> 1)) + (unsigned int)*param_3);
            }
            if (param_5 == 0) {
                local_420 = param_3[1];
            }
            else {
                iVar8 = param_3[1];
                iVar4 = param_3[3];
                iVar9 = FUN_004db2c0(param_1);
                local_420 = (int)(((unsigned int)((int)((unsigned int)iVar4 - (unsigned int)iVar8) >> 1) + (unsigned int)iVar8) - (unsigned int)(iVar9 >> 1));
            }
            iVar8 = 0;
            cVar1 = *param_2;
            local_418 = 0;
            local_41c = param_6;
            while ((cVar1 != '\0') && (iVar6 < 0x280)) {
                bVar2 = (unsigned char)param_2[iVar8];
                if (bVar2 == 1) {
                    local_41c = param_6;
                }
                else if ((bVar2 < 2) || (7 < bVar2)) {
                    uVar7 = (unsigned int)bVar2;
                    local_414[0] = (int)(unsigned int)*((unsigned char *)piVar3 + uVar7 + 0x504);
                    local_414[1] = (int)(unsigned int)*((unsigned char *)piVar3 + uVar7 + 0x404);
                    local_414[3] = 0;
                    local_414[2] = 1;
                    local_414[4] = (int)((unsigned int)piVar3[uVar7 + 1] + (unsigned int)*piVar3);
                    uVar7 = FUN_004d2190(local_414, iVar6, local_420, local_41c, 1);
                    local_418 = local_418 | uVar7;
                    iVar6 = (int)((unsigned int)iVar6
                                  + (unsigned int)*((unsigned char *)piVar3 + (unsigned char)param_2[iVar8] + 0x504)
                                  + (unsigned int)*((unsigned char *)piVar3 + 0x609));
                }
                else {
                    local_41c = DAT_00759ad0[bVar2];
                }
                cVar1 = param_2[iVar8 + 1];
                iVar8 = iVar8 + 1;
            }
            return local_418;
        }
        return 0;
    }
    return 0;
}
