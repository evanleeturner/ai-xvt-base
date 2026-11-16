/* SPDX-License-Identifier: MIT */
/* z_bop 004a6080 */
extern int DAT_005270b0;
extern int *DAT_00527e88;
extern int DAT_00527e84;
extern float DAT_00622c50[3];
extern float DAT_00622c60[3];
extern unsigned int DAT_00622c6c;
extern unsigned int DAT_00622c8c;
extern unsigned int DAT_00622c90;
extern float DAT_00622c98;
extern float DAT_00518208;

unsigned int *FUN_00479de0(int param_1, char *param_2);
int FUN_004a6560(float *param_1, float *param_2, float *param_3, float *param_4, float *param_5);
int FUN_004a66d0(float *param_1, int param_2, int *param_3, float *param_4);
void FUN_00421300(float *param_1, float *param_2);
void FUN_00420fe0(float *param_1, float *param_2);
unsigned int FUN_004a6080(int param_1, unsigned int *param_2);

unsigned int FUN_004a6080(int param_1, unsigned int *param_2)
{
    unsigned int *puVar1;
    int iVar2;
    float *pfVar3;
    int iVar4;
    int *piVar5;
    int iVar6;
    float local_68;
    int local_64;
    int local_60;
    float local_5c[3];
    float local_50[4];
    float local_40[16];
    int local_68_bits;

    do {
        if (param_2 == (unsigned int *)0) {
            return 0;
        }
        local_64 = 0;
        while (param_2[1] == 7) {
            if (DAT_005270b0 == 0) {
                param_2 = FUN_00479de0(param_1, (char *)param_2[5]);
            } else if (*(char *)param_2[5] == '\0') {
                param_2 = (unsigned int *)*param_2;
            } else {
                puVar1 = FUN_00479de0(param_1, (char *)param_2[5]);
                *param_2 = (unsigned int)puVar1;
                *(char *)param_2[5] = 0;
                param_2 = (unsigned int *)*param_2;
            }
            if (param_2 == (unsigned int *)0) {
                return 0;
            }
        }
        switch (param_2[1]) {
        case 1:
        case 0xf:
        case 0x10:
        case 0x11:
            piVar5 = (int *)(param_2[5] + 4);
            local_60 = 0;
            pfVar3 = (float *)(piVar5 + param_2[4] * 0x10);
            iVar6 = DAT_00527e88[5];
            if (0 < (int)param_2[4]) {
                do {
                    iVar4 = FUN_004a6560(pfVar3,
                                         (float *)((unsigned int)iVar6 + (unsigned int)*piVar5 * 0xc),
                                         DAT_00622c50, DAT_00622c60, &local_68);
                    if (iVar4 != 0) {
                        __builtin_memcpy(&local_68_bits, &local_68, 4);
                        if ((DAT_00527e84 == 0 || (0x3dcccccc < local_68_bits)) &&
                            (local_68 < DAT_00622c98)) {
                            local_5c[0] = (DAT_00622c60[0] - DAT_00622c50[0]) * local_68 + DAT_00622c50[0];
                            local_5c[1] = (DAT_00622c60[1] - DAT_00622c50[1]) * local_68 + DAT_00622c50[1];
                            local_5c[2] = (DAT_00622c60[2] - DAT_00622c50[2]) * local_68 + DAT_00622c50[2];
                            iVar4 = FUN_004a66d0(pfVar3, iVar6, piVar5, local_5c);
                            if (iVar4 != 0) {
                                DAT_00622c98 = local_68;
                                DAT_00622c6c = DAT_00622c8c;
                            }
                        }
                    }
                    pfVar3 = pfVar3 + 3;
                    piVar5 = piVar5 + 0x10;
                    local_60 = local_60 + 1;
                } while (local_60 < (int)param_2[4]);
            }
            break;
        case 3:
            DAT_00527e88 = (int *)param_2;
            pfVar3 = (float *)(param_2[5] - 0x18 + param_2[4] * 0xc);
            if ((DAT_00622c50[0] < pfVar3[0]) && (DAT_00622c60[0] < pfVar3[0])) {
                return 1;
            }
            if ((DAT_00622c50[1] < pfVar3[1]) && (DAT_00622c60[1] < pfVar3[1])) {
                return 1;
            }
            if ((DAT_00622c50[2] < pfVar3[2]) && (DAT_00622c60[2] < pfVar3[2])) {
                return 1;
            }
            if ((pfVar3[3] < DAT_00622c50[0]) && (pfVar3[3] < DAT_00622c60[0])) {
                return 1;
            }
            if ((pfVar3[4] < DAT_00622c50[1]) && (pfVar3[4] < DAT_00622c60[1])) {
                return 1;
            }
            if ((pfVar3[5] < DAT_00622c50[2]) && (pfVar3[5] < DAT_00622c60[2])) {
                return 1;
            }
            break;
        case 0x15:
            local_64 = 1;
            break;
        case 0x17:
            if ((DAT_00622c90 & 0x7fffffff) != 0) {
                pfVar3 = (float *)param_2[5];
                DAT_00622c50[0] = DAT_00622c50[0] - pfVar3[0];
                DAT_00622c50[1] = DAT_00622c50[1] - pfVar3[1];
                DAT_00622c50[2] = DAT_00622c50[2] - pfVar3[2];
                DAT_00622c60[0] = DAT_00622c60[0] - pfVar3[0];
                DAT_00622c60[1] = DAT_00622c60[1] - pfVar3[1];
                DAT_00622c60[2] = DAT_00622c60[2] - pfVar3[2];
                local_50[0] = pfVar3[3] * DAT_00518208;
                local_50[1] = pfVar3[4] * DAT_00518208;
                local_50[2] = pfVar3[5] * DAT_00518208;
                __builtin_memcpy(&local_50[3], &DAT_00622c90, 4);
                FUN_00421300(local_40, local_50);
                FUN_00420fe0(DAT_00622c50, local_40);
                FUN_00420fe0(DAT_00622c60, local_40);
                DAT_00622c50[0] = pfVar3[0] + DAT_00622c50[0];
                DAT_00622c50[1] = pfVar3[1] + DAT_00622c50[1];
                DAT_00622c50[2] = pfVar3[2] + DAT_00622c50[2];
                DAT_00622c60[0] = pfVar3[0] + DAT_00622c60[0];
                DAT_00622c60[1] = pfVar3[1] + DAT_00622c60[1];
                DAT_00622c60[2] = pfVar3[2] + DAT_00622c60[2];
            }
        }
        if (param_2[2] == 0) {
            return 0;
        }
        if (local_64 == 0) {
            iVar6 = 0;
            if (0 < (int)param_2[2]) {
                iVar4 = 0;
                do {
                    iVar2 = (int)FUN_004a6080(param_1,
                                              *(unsigned int **)(param_2[3] + (unsigned int)iVar4));
                    if (iVar2 != 0) {
                        return 1;
                    }
                    iVar4 = iVar4 + 4;
                    iVar6 = iVar6 + 1;
                } while (iVar6 < (int)param_2[2]);
            }
            return 0;
        }
        if (local_64 == -1) {
            return 0;
        }
        param_2 = *(unsigned int **)(param_2[3] - 4 + (unsigned int)local_64 * 4);
    } while (1);
}
