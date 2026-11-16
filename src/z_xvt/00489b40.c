/* SPDX-License-Identifier: MIT */
/* z_xvt 00489b40 */
extern int DAT_006026bc;

int __cdecl __strcmpi(const char *_Str1, const char *_Str2);
undefined4 * __cdecl FUN_0048e520(undefined4 *param_1, char *param_2);
void __cdecl FUN_00489b40(int param_1, undefined4 *param_2, int param_3, int param_4);

void __cdecl FUN_00489b40(int param_1, undefined4 *param_2, int param_3, int param_4)
{
    char *_Str2;
    int iVar1;
    undefined4 *puVar2;
    float *pfVar3;
    float *pfVar4;
    int iVar5;
    int bVar6;
    int local_8;
    int local_4;
    int tmpCount;
    int tmpBase;

    if (param_2 == (undefined4 *)0x0) {
        return;
    }
    while (param_2[1] == 7) {
        _Str2 = (char *)param_2[5];
        local_4 = 0;
        __builtin_memcpy(&tmpCount, (char *)param_3 + 6, 4);
        if (0 < tmpCount) {
            local_8 = 0;
            do {
                __builtin_memcpy(&tmpBase, (char *)param_3 + 10, 4);
                puVar2 = *(undefined4 **)(tmpBase + local_8);
                if (puVar2 == (undefined4 *)0x0) {
LAB_00489bc5:
                    puVar2 = (undefined4 *)0x0;
                }
                else if (((char *)*puVar2 == (char *)0x0) ||
                         (iVar1 = __strcmpi((char *)*puVar2, _Str2), iVar1 != 0)) {
                    iVar1 = 0;
                    iVar5 = 0;
                    if (0 < (int)puVar2[2]) {
                        do {
                            param_2 = FUN_0048e520(*(undefined4 **)(puVar2[3] + iVar1), _Str2);
                            if (param_2 != (undefined4 *)0x0) goto LAB_00489bec;
                            iVar1 = iVar1 + 4;
                            iVar5 = iVar5 + 1;
                        } while (iVar5 < (int)puVar2[2]);
                    }
                    goto LAB_00489bc5;
                }
                param_2 = puVar2;
                if (puVar2 != (undefined4 *)0x0) goto LAB_00489bec;
                local_8 = local_8 + 4;
                local_4 = local_4 + 1;
                __builtin_memcpy(&tmpCount, (char *)param_3 + 6, 4);
            } while (local_4 < tmpCount);
        }
        param_2 = (undefined4 *)0x0;
LAB_00489bec:
        if (param_2 == (undefined4 *)0x0) {
            return;
        }
    }
    switch (param_2[1]) {
    case 1:
    case 0xf:
    case 0x10:
    case 0x11:
        if (*(int *)(param_4 + 0x84) == 0) {
            local_8 = 0;
            pfVar3 = (float *)(param_2[5] + 4 + param_2[4] * 0x54);
            if (0 < DAT_006026bc) {
                do {
                    pfVar4 = *(float **)(param_1 + 0x14);
                    iVar5 = 0;
                    iVar1 = *(int *)(param_1 + 0x10);
                    bVar6 = (iVar1 == 0);
                    if (0 < iVar1) {
                        do {
                            if (((*pfVar3 == *pfVar4) && (pfVar4[1] == pfVar3[1])) && (pfVar4[2] == pfVar3[2]))
                                break;
                            pfVar4 = pfVar4 + 3;
                            iVar5 = iVar5 + 1;
                        } while (iVar5 < iVar1);
                        bVar6 = (iVar5 == iVar1);
                    }
                    if (bVar6) {
                        *pfVar4 = *pfVar3;
                        pfVar4[1] = pfVar3[1];
                        pfVar4[2] = pfVar3[2];
                        *(int *)(param_1 + 0x10) = *(int *)(param_1 + 0x10) + 1;
                    }
                    pfVar3 = pfVar3 + 3;
                    local_8 = local_8 + 1;
                } while (local_8 < DAT_006026bc);
            }
            goto LAB_00489d64;
        }
        break;
    case 3:
        DAT_006026bc = param_2[4];
        break;
    case 0xb:
        local_8 = 0;
        *(undefined4 *)(param_4 + 0x84) = param_2[5];
        pfVar3 = (float *)param_2[5];
        if (0 < (int)param_2[4]) {
            do {
                pfVar4 = *(float **)(param_1 + 0x14);
                iVar5 = 0;
                iVar1 = *(int *)(param_1 + 0x10);
                bVar6 = (iVar1 == 0);
                if (0 < iVar1) {
                    do {
                        if (((*pfVar3 == *pfVar4) && (pfVar4[1] == pfVar3[1])) && (pfVar4[2] == pfVar3[2]))
                            break;
                        pfVar4 = pfVar4 + 3;
                        iVar5 = iVar5 + 1;
                    } while (iVar5 < iVar1);
                    bVar6 = (iVar5 == iVar1);
                }
                if (bVar6) {
                    *pfVar4 = *pfVar3;
                    pfVar4[1] = pfVar3[1];
                    pfVar4[2] = pfVar3[2];
                    *(int *)(param_1 + 0x10) = *(int *)(param_1 + 0x10) + 1;
                }
                pfVar3 = pfVar3 + 3;
                local_8 = local_8 + 1;
            } while (local_8 < (int)param_2[4]);
        }
        goto LAB_00489d64;
    }
LAB_00489d64:
    iVar5 = 0;
    iVar1 = 0;
    *(undefined4 *)(param_4 + 0x84) = 0;
    if (0 < (int)param_2[2]) {
        do {
            iVar5 = iVar5 + 4;
            iVar1 = iVar1 + 1;
            FUN_00489b40(param_1, *(undefined4 **)(param_2[3] + -4 + iVar5), param_3, param_4);
        } while (iVar1 < (int)param_2[2]);
    }
    return;
}
