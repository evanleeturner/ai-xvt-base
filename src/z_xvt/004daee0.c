/* SPDX-License-Identifier: MIT */
/* z_xvt 004daee0 */
extern unsigned char DAT_0075d846[];
extern unsigned char DAT_00759ad0[];

void __cdecl FUN_004d5180(unsigned int *param_1);
void __cdecl FUN_004d51b0(int *param_1);
int __cdecl FUN_004db2f0(char *param_1, int param_2);
void __cdecl FUN_004d2190(int *param_1, int param_2, int param_3, unsigned int param_4, int param_5);

int __cdecl FUN_004daee0(int param_1, char *param_2, int *param_3, unsigned int param_4, int param_5, int param_6)
{
    unsigned char bVar1;
    int *piVar2;
    _Bool bVar3;
    int iVar4;
    unsigned int uVar5;
    int iVar6;
    int iStack_548;
    int iStack_544;
    int iStack_540;
    int iStack_53c;
    int iStack_538;
    unsigned int uStack_530;
    unsigned int uStack_52c;
    unsigned int auStack_524[4];
    unsigned char abStack_514[256];
    /* [0]=uStack_414, [1]=uStack_410, [2]=uStack_40c, [3]=uStack_408, [4]=iStack_404 */
    unsigned int auStack_414[5];

    if (param_2 == (char *)0) {
        return 0;
    }
    if (*param_2 == '\0') {
        return 0;
    }
    if ((param_1 < 0) || (0xff < param_1)) {
        return 0;
    }
    /* DAT_0075d846 is not 4-byte aligned: read unaligned */
    __builtin_memcpy(&piVar2, DAT_0075d846 + (unsigned int)param_1 * 4u, 4);
    if (piVar2 == (int *)0) {
        return 0;
    }
    iVar6 = *param_3;
    uStack_52c = param_4;
    iStack_53c = 0;
    iStack_540 = param_3[1];
    iStack_544 = 0;
    bVar3 = 1;
    iStack_548 = 0;
    FUN_004d5180(auStack_524);
    FUN_004d51b0(param_3);
LAB_004dafa5:
    do {
        bVar1 = (unsigned char)param_2[iStack_53c];
        uStack_530 = uStack_52c;
        if (bVar1 == 1) {
            uStack_530 = param_4;
        }
        else if ((bVar1 < 2) || (7 < bVar1)) {
            if ((bVar1 == 0x20) ||
                (((bVar1 == 10 || (param_2[iStack_53c + 1] == '\0')) || (bVar1 == 0x24)))) {
                if ((bVar3) && (bVar1 == 0x20)) {
                    iStack_53c = iStack_53c + 1;
                    goto LAB_004dafa5;
                }
                if ((bVar1 != 10) && (bVar1 != 0x24)) {
                    abStack_514[iStack_544] = bVar1;
                    iStack_544 = iStack_544 + 1;
                }
                abStack_514[iStack_544] = 0;
                iVar4 = FUN_004db2f0((char *)abStack_514, param_1);
                if ((int)((unsigned int)param_3[2] - (unsigned int)param_1) <
                    (int)((unsigned int)iVar4 + (unsigned int)iVar6)) {
                    if (param_6 <= iStack_548) {
                        iStack_540 = (int)((unsigned int)iStack_540 + (unsigned int)param_5 + (unsigned int)param_1);
                    }
                    iVar6 = *param_3;
                    bVar3 = 1;
                    iStack_548 = iStack_548 + 1;
                }
                iStack_538 = 0;
                if (0 < iStack_544) {
                    do {
                        if (iStack_548 < param_6) {
                            iVar6 = (int)((unsigned int)iVar6 +
                                          (unsigned int)*(unsigned char *)(abStack_514[iStack_538] + 0x504 + (unsigned char *)piVar2) +
                                          (unsigned int)*(unsigned char *)((unsigned char *)piVar2 + 0x609));
                            if (param_3[2] < iVar6) {
joined_r0x004db1f2:
                                if (param_6 <= iStack_548) {
                                    iStack_540 = (int)((unsigned int)iStack_540 + (unsigned int)param_5 + (unsigned int)param_1);
                                }
                                iVar6 = *param_3;
                                iStack_548 = iStack_548 + 1;
                            }
                        }
                        else {
                            uVar5 = (unsigned int)abStack_514[iStack_538];
                            auStack_414[0] = (unsigned int)*(unsigned char *)(uVar5 + 0x504 + (unsigned char *)piVar2);
                            auStack_414[1] = (unsigned int)*(unsigned char *)(uVar5 + 0x404 + (unsigned char *)piVar2);
                            auStack_414[3] = 0;
                            auStack_414[2] = 1;
                            auStack_414[4] = (unsigned int)piVar2[uVar5 + 1] + (unsigned int)*piVar2;
                            FUN_004d2190((int *)auStack_414, iVar6, iStack_540, uStack_52c, 1);
                            iVar6 = (int)((unsigned int)iVar6 +
                                          (unsigned int)*(unsigned char *)((unsigned char *)piVar2 + 0x609) +
                                          (unsigned int)*(unsigned char *)(uVar5 + 0x504 + (unsigned char *)piVar2));
                            if ((int)((unsigned int)param_3[2] - (unsigned int)param_1) < iVar6) goto joined_r0x004db1f2;
                        }
                        iStack_538 = iStack_538 + 1;
                    } while (iStack_538 < iStack_544);
                }
                if ((bVar1 == 10) || (bVar1 == 0x24)) {
                    if (param_6 <= iStack_548) {
                        iStack_540 = (int)((unsigned int)iStack_540 + (unsigned int)param_5 + (unsigned int)param_1);
                    }
                    iVar6 = *param_3;
                    bVar3 = 1;
                    iStack_548 = iStack_548 + 1;
                }
                iStack_544 = 0;
            }
            else {
                bVar3 = 0;
                abStack_514[iStack_544] = bVar1;
                iStack_544 = iStack_544 + 1;
            }
        }
        else {
            uStack_530 = *(unsigned int *)(DAT_00759ad0 + (unsigned int)bVar1 * 4u);
        }
        uStack_52c = uStack_530;
        iStack_53c = iStack_53c + 1;
        if (param_2[iStack_53c] == '\0') {
            FUN_004d51b0((int *)auStack_524);
            return iStack_548;
        }
    } while (1);
}
