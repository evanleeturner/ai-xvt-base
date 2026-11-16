/* SPDX-License-Identifier: MIT */
/* z_xvt 0048a790 */
extern int DAT_00525518;

int __cdecl __strcmpi(const char *, const char *);
undefined4 * __cdecl FUN_0048e520(undefined4 *, char *);
int __cdecl FUN_0048a980(int, int, int);
void __cdecl FUN_0048a790(undefined4 *param_1, int param_2, int param_3);

void __cdecl FUN_0048a790(undefined4 *param_1, int param_2, int param_3)
{
    char *_Str2;
    int *piVar1;
    int iVar2;
    undefined4 *puVar3;
    int iVar4;
    int iVar5;
    int local_8;
    int local_4;
    int tmp_a;
    int tmp_b;

    if (param_1 != (undefined4 *)0x0) {
        while (param_1[1] == 7) {
            _Str2 = (char *)param_1[5];
            local_4 = 0;
            __builtin_memcpy(&tmp_a, (char *)param_2 + 6, 4);
            if (0 < tmp_a) {
                local_8 = 0;
                do {
                    __builtin_memcpy(&tmp_b, (char *)param_2 + 10, 4);
                    puVar3 = *(undefined4 **)((unsigned int)tmp_b + (unsigned int)local_8);
                    if (puVar3 == (undefined4 *)0x0) {
LAB_0048a815:
                        puVar3 = (undefined4 *)0x0;
                    }
                    else if (((char *)*puVar3 == (char *)0x0) ||
                             (iVar2 = __strcmpi((char *)*puVar3, _Str2), iVar2 != 0)) {
                        iVar5 = 0;
                        iVar2 = 0;
                        if (0 < (int)puVar3[2]) {
                            do {
                                param_1 = FUN_0048e520(*(undefined4 **)(puVar3[3] + (unsigned int)iVar5), _Str2);
                                if (param_1 != (undefined4 *)0x0) goto LAB_0048a83a;
                                iVar5 = (int)((unsigned int)iVar5 + 4u);
                                iVar2 = (int)((unsigned int)iVar2 + 1u);
                            } while (iVar2 < (int)puVar3[2]);
                        }
                        goto LAB_0048a815;
                    }
                    param_1 = puVar3;
                    if (puVar3 != (undefined4 *)0x0) goto LAB_0048a83a;
                    local_8 = (int)((unsigned int)local_8 + 4u);
                    local_4 = (int)((unsigned int)local_4 + 1u);
                    __builtin_memcpy(&tmp_a, (char *)param_2 + 6, 4);
                } while (local_4 < tmp_a);
            }
            param_1 = (undefined4 *)0x0;
LAB_0048a83a:
            if (param_1 == (undefined4 *)0x0) {
                return;
            }
        }
        if (param_1[1] == 0x14) {
            piVar1 = (int *)param_1[5];
            if (piVar1[1] == 0) {
                iVar5 = *piVar1;
                iVar2 = iVar5;
                if (DAT_00525518 == 2) {
                    iVar2 = (int)((unsigned int)iVar5 + 0x1000u);
                }
                iVar4 = (int)((unsigned int)piVar1[5] * (unsigned int)piVar1[4]);
                if (piVar1[2] == iVar4) {
                    iVar4 = piVar1[3];
                }
                if ((int *)(((unsigned int)iVar2 + (unsigned int)-0x18) - (unsigned int)iVar4) != piVar1) {
                    iVar2 = 0;
                    local_4 = 0;
                    __builtin_memcpy(&tmp_a, (char *)param_3 + 6, 4);
                    if (0 < tmp_a) {
                        do {
                            int a0;
                            int a1;
                            __builtin_memcpy(&tmp_a, (char *)param_3 + 10, 4);
                            a0 = *(int *)((unsigned int)tmp_a + (unsigned int)iVar2);
                            __builtin_memcpy(&tmp_b, (char *)param_2 + 10, 4);
                            a1 = *(int *)((unsigned int)tmp_b + (unsigned int)iVar2);
                            iVar4 = FUN_0048a980(a0, a1, iVar5);
                            if (iVar4 != 0) {
                                iVar2 = *(int *)((unsigned int)iVar4 + 0x14u);
                                iVar5 = (int)((unsigned int)*(int *)((unsigned int)iVar2 + 0x14u) *
                                              (unsigned int)*(int *)((unsigned int)iVar2 + 0x10u));
                                if (*(int *)((unsigned int)iVar2 + 8u) == iVar5) {
                                    iVar5 = *(int *)((unsigned int)iVar2 + 0xcu);
                                }
                                iVar5 = (int)((unsigned int)iVar2 + 0x18u + (unsigned int)iVar5);
                                if (DAT_00525518 == 2) {
                                    iVar5 = (int)((unsigned int)iVar5 + (unsigned int)-0x1000);
                                }
                                *piVar1 = iVar5;
                                break;
                            }
                            iVar2 = (int)((unsigned int)iVar2 + 4u);
                            local_4 = (int)((unsigned int)local_4 + 1u);
                            __builtin_memcpy(&tmp_a, (char *)param_3 + 6, 4);
                        } while (local_4 < tmp_a);
                    }
                }
            }
            else {
                piVar1[1] = 0;
                iVar2 = (int)((unsigned int)piVar1[5] * (unsigned int)piVar1[4]);
                if (piVar1[2] == iVar2) {
                    iVar2 = piVar1[3];
                }
                iVar2 = (int)((unsigned int)(int)piVar1 + (unsigned int)iVar2 + 0x18u);
                if (DAT_00525518 == 2) {
                    iVar2 = (int)((unsigned int)iVar2 + (unsigned int)-0x1000);
                }
                *piVar1 = iVar2;
            }
        }
        iVar5 = 0;
        iVar2 = 0;
        if (0 < (int)param_1[2]) {
            do {
                puVar3 = (undefined4 *)(param_1[3] + (unsigned int)iVar5);
                iVar5 = (int)((unsigned int)iVar5 + 4u);
                iVar2 = (int)((unsigned int)iVar2 + 1u);
                FUN_0048a790((undefined4 *)*puVar3, param_2, param_3);
            } while (iVar2 < (int)param_1[2]);
        }
    }
    return;
}
