/* SPDX-License-Identifier: MIT */
/* z_xvt 004da920 */
extern unsigned char DAT_0075d846[];
extern unsigned int DAT_00759ad0[];

int __cdecl FUN_004db2f0(char *param_1, int param_2);
unsigned int __cdecl FUN_004db2c0(int param_1);
unsigned int __cdecl FUN_004d2190(int *param_1, int param_2, int param_3, unsigned int param_4, int param_5);

unsigned int __cdecl FUN_004da920(int param_1, char *param_2, int *param_3, unsigned int param_4)
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
    unsigned char *pbBase;

    if (param_2 == (char *)0x0) {
        return 0;
    }
    if ((-1 < param_1) && (param_1 < 0x100)) {
        /* DAT_0075d846 is not 4-byte aligned: read unaligned via memcpy */
        __builtin_memcpy(&piVar3, DAT_0075d846 + (unsigned int)param_1 * 4, 4);
        if (piVar3 != (int *)0x0) {
            pbBase = (unsigned char *)piVar3;
            iVar6 = FUN_004db2f0(param_2, param_1);
            iVar4 = param_3[3];
            iVar8 = (int)((unsigned int)(((int)((unsigned int)param_3[2] - (unsigned int)*param_3)) >> 1)
                          - (unsigned int)(iVar6 >> 1) + (unsigned int)*param_3);
            iVar6 = param_3[1];
            uVar5 = FUN_004db2c0(param_1);
            iVar9 = 0;
            cVar1 = *param_2;
            local_418 = 0;
            local_420 = param_4;
            while ((cVar1 != '\0') && (iVar8 < 0x280)) {
                bVar2 = (unsigned char)param_2[iVar9];
                if (bVar2 == 1) {
                    local_420 = param_4;
                }
                else if ((bVar2 < 2) || (7 < bVar2)) {
                    uVar7 = (unsigned int)bVar2;
                    local_414[0] = (unsigned int)*(unsigned char *)(pbBase + uVar7 + 0x504);
                    local_414[1] = (unsigned int)*(unsigned char *)(pbBase + uVar7 + 0x404);
                    local_414[3] = 0;
                    local_414[2] = 1;
                    local_414[4] = (unsigned int)piVar3[uVar7 + 1] + (unsigned int)*piVar3;
                    uVar7 = FUN_004d2190((int *)local_414, iVar8,
                                         (int)((unsigned int)(((int)((unsigned int)iVar4 - (unsigned int)iVar6)) >> 1)
                                               + (unsigned int)iVar6
                                               - (uVar5 >> 1)),
                                         local_420, 1);
                    local_418 = local_418 | uVar7;
                    iVar8 = (int)((unsigned int)iVar8
                                  + (unsigned int)*(unsigned char *)(pbBase + (unsigned char)param_2[iVar9] + 0x504)
                                  + (unsigned int)*(unsigned char *)(pbBase + 0x609));
                }
                else {
                    local_420 = DAT_00759ad0[bVar2];
                }
                cVar1 = param_2[iVar9 + 1];
                iVar9 = iVar9 + 1;
            }
            return local_418;
        }
        return 0;
    }
    return 0;
}
