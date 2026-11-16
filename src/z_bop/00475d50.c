/* SPDX-License-Identifier: MIT */
/* z_bop 00475d50 */
extern int DAT_005272bc;
extern int DAT_005272b0;
extern undefined4 *DAT_0060f1f8;
extern undefined4 *DAT_0060f1dc;
extern int DAT_0060f1c0;
extern int DAT_0060f200;
extern int DAT_0060f1cc;
extern int DAT_0060f1e8;
extern int DAT_0060f1f4;

undefined4 *FUN_00479de0(int param_4, char *param_name);
void *FID_conflict__memcpy(void *dst, const void *src, unsigned int n);
int FUN_00475b70(int a, int b, int c);
int FUN_00475c70(int a, int b, int c);
void FUN_00475d50(int param_1, undefined4 *param_2, undefined4 *param_3, int param_4, int param_5);

void FUN_00475d50(int param_1, undefined4 *param_2, undefined4 *param_3, int param_4, int param_5)
{
    int *piVar1;
    int iVar2;
    int *piVar3;
    int *piVar4;
    int *piVar5;
    int iVar6;
    int local_c;
    int local_4;

    if (param_3 != (undefined4 *)0x0) {
        while (param_3[1] == 7) {
            param_3 = FUN_00479de0(param_4, (char *)param_3[5]);
            if (param_3 == (undefined4 *)0x0) {
                return;
            }
        }
        if ((DAT_005272bc == 0) && (param_3 == param_2)) {
            DAT_005272bc = 1;
        }
        switch (param_3[1]) {
        case 1:
        case 0xf:
        case 0x10:
        case 0x11:
            if (((DAT_005272bc != 0) && (DAT_0060f1f8 == DAT_0060f1dc)) && (0 < *(int *)param_3[5])) {
                iVar6 = **(int **)(param_1 + 0x14);
                piVar5 = *(int **)(param_1 + 0x14) + 1;
                FID_conflict__memcpy(piVar5 + param_3[4] * 0x10, piVar5,
                                     (unsigned int)*(int *)(param_1 + 0x10) * 100u);
                piVar3 = (int *)(param_3[5] + 4);
                local_c = *(int *)(param_5 + 0x84);
                if (local_c == 0) {
                    if (DAT_005272b0 == 0) {
                        iVar2 = (int)(param_3[4] << 6);
                    }
                    else {
                        iVar2 = (int)(param_3[4] * 0x30);
                    }
                    local_c = (int)((unsigned int)(int)piVar3 + param_3[4] * 0x24 + (unsigned int)iVar2);
                }
                local_4 = 0;
                piVar1 = piVar5;
                if (0 < (int)param_3[4]) {
                    do {
                        iVar2 = FUN_00475b70(DAT_0060f200, DAT_0060f1c0, *piVar3);
                        *piVar1 = iVar2;
                        iVar2 = FUN_00475b70(DAT_0060f200, DAT_0060f1c0, piVar3[1]);
                        piVar1[1] = iVar2;
                        iVar2 = FUN_00475b70(DAT_0060f200, DAT_0060f1c0, piVar3[2]);
                        piVar1[2] = iVar2;
                        iVar2 = FUN_00475b70(DAT_0060f200, DAT_0060f1c0, piVar3[3]);
                        piVar1[3] = iVar2;
                        piVar1[4] = (int)((unsigned int)piVar3[4] + (unsigned int)iVar6);
                        piVar1[5] = (int)((unsigned int)piVar3[5] + (unsigned int)iVar6);
                        piVar1[6] = (int)((unsigned int)piVar3[6] + (unsigned int)iVar6);
                        if (piVar3[7] == -1) {
                            piVar1[7] = -1;
                        }
                        else {
                            piVar1[7] = (int)((unsigned int)iVar6 + (unsigned int)piVar3[7]);
                        }
                        iVar2 = FUN_00475c70(DAT_0060f1f4, DAT_0060f1e8, piVar3[8]);
                        piVar1[8] = iVar2;
                        iVar2 = FUN_00475c70(DAT_0060f1f4, DAT_0060f1e8, piVar3[9]);
                        piVar1[9] = iVar2;
                        iVar2 = FUN_00475c70(DAT_0060f1f4, DAT_0060f1e8, piVar3[10]);
                        piVar1[10] = iVar2;
                        iVar2 = FUN_00475c70(DAT_0060f1f4, DAT_0060f1e8, piVar3[0xb]);
                        piVar1[0xb] = iVar2;
                        piVar4 = piVar3 + 0xc;
                        if (DAT_005272b0 != 0) {
                            piVar4 = piVar3;
                        }
                        iVar2 = FUN_00475b70(DAT_0060f1cc, local_c, *piVar4);
                        piVar1[0xc] = iVar2;
                        iVar2 = FUN_00475b70(DAT_0060f1cc, local_c, piVar4[1]);
                        piVar1[0xd] = iVar2;
                        iVar2 = FUN_00475b70(DAT_0060f1cc, local_c, piVar4[2]);
                        piVar3 = piVar4 + 4;
                        piVar1[0xe] = iVar2;
                        iVar2 = FUN_00475b70(DAT_0060f1cc, local_c, piVar4[3]);
                        piVar1[0xf] = iVar2;
                        if (DAT_005272b0 != 0) {
                            piVar3 = piVar4 + 0xc;
                        }
                        local_4 = (int)((unsigned int)local_4 + 1u);
                        piVar1 = piVar1 + 0x10;
                    } while (local_4 < (int)param_3[4]);
                }
                piVar5 = piVar5 + ((unsigned int)*(int *)(param_1 + 0x10) + param_3[4]) * 0x10;
                FID_conflict__memcpy(piVar5 + param_3[4] * 3, piVar5,
                                     (unsigned int)*(int *)(param_1 + 0x10) * 0x24u);
                if (DAT_005272b0 == 0) {
                    iVar6 = (int)(param_3[4] << 6);
                }
                else {
                    iVar6 = (int)(param_3[4] * 0x30);
                }
                iVar2 = 0;
                piVar1 = (int *)(param_3[5] + iVar6 + 4);
                piVar3 = piVar5;
                if (0 < (int)param_3[4]) {
                    do {
                        *piVar3 = *piVar1;
                        piVar4 = piVar1 + 2;
                        piVar3[1] = piVar1[1];
                        piVar1 = piVar1 + 3;
                        piVar3[2] = *piVar4;
                        piVar3 = piVar3 + 3;
                        iVar2 = (int)((unsigned int)iVar2 + 1u);
                    } while (iVar2 < (int)param_3[4]);
                }
                piVar5 = piVar5 + ((unsigned int)*(int *)(param_1 + 0x10) + param_3[4]) * 3;
                FID_conflict__memcpy(piVar5 + param_3[4] * 6, piVar5,
                                     (unsigned int)*(int *)(param_1 + 0x10) * 0x18u);
                if (DAT_005272b0 == 0) {
                    iVar6 = (int)(param_3[4] << 6);
                }
                else {
                    iVar6 = (int)(param_3[4] * 0x30);
                }
                piVar3 = (int *)(param_3[5] + iVar6 + 4 + param_3[4] * 0xc);
                iVar6 = 0;
                if (0 < (int)param_3[4]) {
                    do {
                        *piVar5 = *piVar3;
                        piVar5[1] = piVar3[1];
                        piVar5[2] = piVar3[2];
                        piVar5[3] = piVar3[3];
                        piVar1 = piVar3 + 5;
                        piVar5[4] = piVar3[4];
                        piVar3 = piVar3 + 6;
                        piVar5[5] = *piVar1;
                        piVar5 = piVar5 + 6;
                        iVar6 = (int)((unsigned int)iVar6 + 1u);
                    } while (iVar6 < (int)param_3[4]);
                }
                *(int *)(param_1 + 0x10) = (int)((unsigned int)*(int *)(param_1 + 0x10) + param_3[4]);
                piVar5 = (int *)param_3[5];
                **(int **)(param_1 + 0x14) = (int)((unsigned int)**(int **)(param_1 + 0x14) + (unsigned int)*piVar5);
                *piVar5 = -1;
            }
            break;
        case 3:
            DAT_0060f1c0 = (int)param_3[5];
            break;
        case 0xb:
            *(undefined4 *)(param_5 + 0x84) = param_3[5];
            break;
        case 0xd:
            DAT_0060f1e8 = (int)param_3[5];
            break;
        case 0x14:
            DAT_0060f1f8 = param_3;
        }
        iVar6 = 0;
        if ((param_3[2] != 0) && (iVar2 = 0, 0 < (int)param_3[2])) {
            do {
                iVar6 = (int)((unsigned int)iVar6 + 4u);
                iVar2 = (int)((unsigned int)iVar2 + 1u);
                FUN_00475d50(param_1, param_2, *(undefined4 **)(param_3[3] + -4 + iVar6), param_4, param_5);
            } while (iVar2 < (int)param_3[2]);
        }
    }
    return;
}
