/* SPDX-License-Identifier: MIT */
/* z_xvt 004491e0 */
extern unsigned short DAT_006da850[];
extern unsigned char DAT_005ef2d0;
extern unsigned char DAT_005ef2d4;
extern unsigned char DAT_005ef2d8;
extern unsigned char DAT_005eff68;
extern unsigned char DAT_005eff6c;
extern unsigned char DAT_005eff70;
extern unsigned char DAT_005efc40;

/* Calling convention and argument registers inferred from P-code: only EDX is
   set before the call (low 16 bits replaced). ECX handling is unresolved. */
int * __fastcall FUN_0048f4f0(unsigned int param_1, unsigned int param_2);
void  FUN_00485fa0(int *param_1);
int   FUN_00449140(int param_1);
void  FUN_0048f510(void);

void __fastcall FUN_004491e0(unsigned int param_1, unsigned int param_2, int param_3)
{
    int iVar1;
    int *piVar2;
    int iVar3;
    float *pfVar4;
    int iVar5;
    int iVar6;
    float local_20;
    float local_1c;
    float local_18;
    float local_14;
    float local_10;
    float local_c;
    int local_8;
    int local_4;
    int count;
    char *arr;

    local_18 = 1.0737418e+09f;
    local_1c = 1.0737418e+09f;
    local_20 = 1.0737418e+09f;
    local_c = -1.0737418e+09f;
    local_10 = -1.0737418e+09f;
    local_14 = -1.0737418e+09f;
    piVar2 = FUN_0048f4f0(param_1,
                          (param_2 & 0xffff0000u) | (unsigned int)DAT_006da850[param_3]);
    if ((int *)*piVar2 != piVar2) {
        FUN_00485fa0(piVar2);
    }
    local_4 = 0;
    /* packed field at +6 (count) and +10 (array pointer) */
    __builtin_memcpy(&count, (char *)piVar2 + 6, 4);
    if (0 < count) {
        local_8 = 0;
        do {
            __builtin_memcpy(&arr, (char *)piVar2 + 10, 4);
            iVar1 = *(int *)(arr + local_8);
            if ((iVar1 != 0) && (*(int *)(iVar1 + 4) != 0x14)) {
                if (iVar1 == 0) {
LAB_00449296:
                    iVar3 = 0;
                }
                else {
                    iVar3 = iVar1;
                    if (*(int *)(iVar1 + 4) != 3) {
                        iVar6 = 0;
                        iVar5 = 0;
                        if (0 < *(int *)(iVar1 + 8)) {
                            do {
                                iVar3 = *(int *)(*(int *)(iVar1 + 0xc) + iVar6);
                                if ((iVar3 != 0) && (iVar3 = FUN_00449140(iVar3), iVar3 != 0)) goto LAB_00449298;
                                iVar6 = (int)((unsigned int)iVar6 + 4u);
                                iVar5 = (int)((unsigned int)iVar5 + 1u);
                            } while (iVar5 < *(int *)(iVar1 + 8));
                        }
                        goto LAB_00449296;
                    }
                }
LAB_00449298:
                if (iVar3 != 0) {
                    if (1 < *(int *)(iVar3 + 0x10)) {
                        pfVar4 = (float *)(*(unsigned int *)(iVar3 + 0x14)
                                           + (unsigned int)*(int *)(iVar3 + 0x10) * 0xcu
                                           - 0x18u);
                        if (*pfVar4 < local_20) {
                            local_20 = *pfVar4;
                        }
                        if (pfVar4[1] < local_1c) {
                            local_1c = pfVar4[1];
                        }
                        if (pfVar4[2] < local_18) {
                            local_18 = pfVar4[2];
                        }
                        if (local_14 < pfVar4[3]) {
                            local_14 = pfVar4[3];
                        }
                        if (local_10 < pfVar4[4]) {
                            local_10 = pfVar4[4];
                        }
                        if (local_c < pfVar4[5]) {
                            local_c = pfVar4[5];
                        }
                    }
                }
            }
            local_8 = (int)((unsigned int)local_8 + 4u);
            local_4 = (int)((unsigned int)local_4 + 1u);
            __builtin_memcpy(&count, (char *)piVar2 + 6, 4);
        } while (local_4 < count);
    }
    *(float *)(&DAT_005ef2d0 + (int)((unsigned int)param_3 * 0xcu)) = local_20;
    iVar1 = (int)((unsigned int)param_3 * 0xcu);
    *(float *)(&DAT_005ef2d4 + iVar1) = local_1c;
    *(float *)(&DAT_005eff68 + iVar1) = local_14;
    *(float *)(&DAT_005ef2d8 + iVar1) = local_18;
    *(float *)(&DAT_005eff6c + iVar1) = local_10;
    *(unsigned int *)(&DAT_005efc40 + (int)((unsigned int)param_3 * 4u)) = 1;
    *(float *)(&DAT_005eff70 + iVar1) = local_c;
    FUN_0048f510();
    return;
}
