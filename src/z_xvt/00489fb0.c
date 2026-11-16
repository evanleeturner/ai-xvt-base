/* SPDX-License-Identifier: MIT */
/* z_xvt 00489fb0 */
extern int DAT_0052a54c;
extern undefined4 *DAT_006026b0;
extern undefined4 *DAT_00602694;
extern int DAT_00602678;
extern int DAT_00602684;
extern int DAT_006026a0;
extern int DAT_006026ac;
extern int DAT_006026b8;

int __cdecl __strcmpi(char *, char *);
undefined4 * __cdecl FUN_0048e520(undefined4 *, char *);
void * __cdecl FID_conflict__memcpy(void *, void *, int);
int __cdecl FUN_00489dd0(int, int, int);
int __cdecl FUN_00489ed0(int, int, int);
void __cdecl FUN_00489fb0(int param_1, undefined4 *param_2, undefined4 *param_3, int param_4, int param_5);

void __cdecl
FUN_00489fb0(int param_1, undefined4 *param_2, undefined4 *param_3, int param_4, int param_5)
{
    char *_Str2;
    int iVar1;
    undefined4 *puVar2;
    int *piVar3;
    int *piVar4;
    int *piVar5;
    int iVar6;
    int *piVar7;
    int local_c;
    int local_8;
    int *local_4;
    int tmpCount;
    int tmpArr;

    if (param_3 != (undefined4 *)0x0) {
        while (param_3[1] == 7) {
            _Str2 = (char *)param_3[5];
            local_c = 0;
            __builtin_memcpy(&tmpCount, (char *)param_4 + 6, 4);
            if (0 < tmpCount) {
                local_8 = 0;
                do {
                    __builtin_memcpy(&tmpArr, (char *)param_4 + 10, 4);
                    puVar2 = *(undefined4 **)(tmpArr + local_8);
                    if (puVar2 == (undefined4 *)0x0) {
LAB_0048a035:
                        puVar2 = (undefined4 *)0x0;
                    }
                    else if (((char *)*puVar2 == (char *)0x0) ||
                             (iVar1 = __strcmpi((char *)*puVar2, _Str2), iVar1 != 0)) {
                        iVar1 = 0;
                        iVar6 = 0;
                        if (0 < (int)puVar2[2]) {
                            do {
                                param_3 = FUN_0048e520(*(undefined4 **)(puVar2[3] + iVar1), _Str2);
                                if (param_3 != (undefined4 *)0x0) goto LAB_0048a05c;
                                iVar1 = (int)((unsigned int)iVar1 + 4u);
                                iVar6 = (int)((unsigned int)iVar6 + 1u);
                            } while (iVar6 < (int)puVar2[2]);
                        }
                        goto LAB_0048a035;
                    }
                    param_3 = puVar2;
                    if (puVar2 != (undefined4 *)0x0) goto LAB_0048a05c;
                    local_8 = (int)((unsigned int)local_8 + 4u);
                    local_c = (int)((unsigned int)local_c + 1u);
                    __builtin_memcpy(&tmpCount, (char *)param_4 + 6, 4);
                } while (local_c < tmpCount);
            }
            param_3 = (undefined4 *)0x0;
LAB_0048a05c:
            if (param_3 == (undefined4 *)0x0) {
                return;
            }
        }
        if ((DAT_0052a54c == 0) && (param_3 == param_2)) {
            DAT_0052a54c = 1;
        }
        switch (param_3[1]) {
        case 1:
        case 0xf:
        case 0x10:
        case 0x11:
            if (((DAT_0052a54c != 0) && (DAT_006026b0 == DAT_00602694)) && (0 < *(int *)param_3[5])) {
                iVar1 = **(int **)(param_1 + 0x14);
                piVar7 = *(int **)(param_1 + 0x14) + 1;
                FID_conflict__memcpy(piVar7 + param_3[4] * 0x10, piVar7,
                                     (int)((unsigned int)*(int *)(param_1 + 0x10) * 100u));
                local_4 = *(int **)(param_5 + 0x84);
                if (local_4 == (int *)0x0) {
                    local_4 = (int *)(param_3[5] + 4) + param_3[4] * 0x15;
                }
                local_8 = 0;
                piVar4 = piVar7;
                piVar3 = (int *)(param_3[5] + 4);
                if (0 < (int)param_3[4]) {
                    do {
                        iVar6 = FUN_00489dd0(DAT_006026b8, DAT_00602678, *piVar3);
                        *piVar4 = iVar6;
                        iVar6 = FUN_00489dd0(DAT_006026b8, DAT_00602678, piVar3[1]);
                        piVar4[1] = iVar6;
                        iVar6 = FUN_00489dd0(DAT_006026b8, DAT_00602678, piVar3[2]);
                        piVar4[2] = iVar6;
                        iVar6 = FUN_00489dd0(DAT_006026b8, DAT_00602678, piVar3[3]);
                        piVar4[3] = iVar6;
                        piVar4[4] = (int)((unsigned int)piVar3[4] + (unsigned int)iVar1);
                        piVar4[5] = (int)((unsigned int)piVar3[5] + (unsigned int)iVar1);
                        piVar4[6] = (int)((unsigned int)piVar3[6] + (unsigned int)iVar1);
                        if (piVar3[7] == -1) {
                            piVar4[7] = -1;
                        }
                        else {
                            piVar4[7] = (int)((unsigned int)iVar1 + (unsigned int)piVar3[7]);
                        }
                        iVar6 = FUN_00489ed0(DAT_006026ac, DAT_006026a0, piVar3[8]);
                        piVar4[8] = iVar6;
                        iVar6 = FUN_00489ed0(DAT_006026ac, DAT_006026a0, piVar3[9]);
                        piVar4[9] = iVar6;
                        iVar6 = FUN_00489ed0(DAT_006026ac, DAT_006026a0, piVar3[10]);
                        piVar4[10] = iVar6;
                        iVar6 = FUN_00489ed0(DAT_006026ac, DAT_006026a0, piVar3[0xb]);
                        piVar4[0xb] = iVar6;
                        iVar6 = FUN_00489dd0(DAT_00602684, (int)local_4, *piVar3);
                        piVar4[0xc] = iVar6;
                        iVar6 = FUN_00489dd0(DAT_00602684, (int)local_4, piVar3[1]);
                        piVar4[0xd] = iVar6;
                        iVar6 = FUN_00489dd0(DAT_00602684, (int)local_4, piVar3[2]);
                        piVar4[0xe] = iVar6;
                        iVar6 = FUN_00489dd0(DAT_00602684, (int)local_4, piVar3[3]);
                        local_8 = (int)((unsigned int)local_8 + 1u);
                        piVar4[0xf] = iVar6;
                        piVar4 = piVar4 + 0x10;
                        piVar3 = piVar3 + 0xc;
                    } while (local_8 < (int)param_3[4]);
                }
                piVar7 = piVar7 + ((unsigned int)*(int *)(param_1 + 0x10) + param_3[4]) * 0x10;
                FID_conflict__memcpy(piVar7 + param_3[4] * 3, piVar7,
                                     (int)((unsigned int)*(int *)(param_1 + 0x10) * 0x24u));
                piVar4 = (int *)(param_3[4] * 0x30 + 4 + param_3[5]);
                iVar1 = 0;
                piVar3 = piVar7;
                if (0 < (int)param_3[4]) {
                    do {
                        *piVar3 = *piVar4;
                        piVar5 = piVar4 + 2;
                        piVar3[1] = piVar4[1];
                        piVar4 = piVar4 + 3;
                        piVar3[2] = *piVar5;
                        piVar3 = piVar3 + 3;
                        iVar1 = (int)((unsigned int)iVar1 + 1u);
                    } while (iVar1 < (int)param_3[4]);
                }
                piVar7 = piVar7 + ((unsigned int)*(int *)(param_1 + 0x10) + param_3[4]) * 3;
                FID_conflict__memcpy(piVar7 + param_3[4] * 6, piVar7,
                                     (int)((unsigned int)*(int *)(param_1 + 0x10) * 0x18u));
                iVar1 = 0;
                piVar4 = (int *)(param_3[4] * 0x3c + 4 + param_3[5]);
                if (0 < (int)param_3[4]) {
                    do {
                        *piVar7 = *piVar4;
                        piVar7[1] = piVar4[1];
                        piVar7[2] = piVar4[2];
                        piVar7[3] = piVar4[3];
                        piVar3 = piVar4 + 5;
                        piVar7[4] = piVar4[4];
                        piVar4 = piVar4 + 6;
                        piVar7[5] = *piVar3;
                        piVar7 = piVar7 + 6;
                        iVar1 = (int)((unsigned int)iVar1 + 1u);
                    } while (iVar1 < (int)param_3[4]);
                }
                *(int *)(param_1 + 0x10) =
                    (int)((unsigned int)*(int *)(param_1 + 0x10) + (unsigned int)param_3[4]);
                piVar7 = (int *)param_3[5];
                **(int **)(param_1 + 0x14) =
                    (int)((unsigned int)**(int **)(param_1 + 0x14) + (unsigned int)*piVar7);
                *piVar7 = -1;
            }
            break;
        case 3:
            DAT_00602678 = param_3[5];
            break;
        case 0xb:
            *(undefined4 *)(param_5 + 0x84) = param_3[5];
            break;
        case 0xd:
            DAT_006026a0 = param_3[5];
            break;
        case 0x14:
            DAT_006026b0 = param_3;
        }
        iVar1 = 0;
        if ((param_3[2] != 0) && (iVar6 = 0, 0 < (int)param_3[2])) {
            do {
                iVar1 = (int)((unsigned int)iVar1 + 4u);
                iVar6 = (int)((unsigned int)iVar6 + 1u);
                FUN_00489fb0(param_1, param_2, *(undefined4 **)(param_3[3] + -4 + iVar1), param_4, param_5);
            } while (iVar6 < (int)param_3[2]);
        }
    }
    return;
}
