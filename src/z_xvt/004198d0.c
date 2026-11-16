/* SPDX-License-Identifier: MIT */
/* z_xvt 004198d0 */
extern int DAT_0052a348;
extern int *DAT_00524774;
extern float _DAT_00536a20;
extern float _DAT_00536a24;
extern float _DAT_00536a28;
extern float _DAT_00536a30;
extern float _DAT_00536a34;
extern float _DAT_00536a38;
extern unsigned int DAT_00536a3c;
extern unsigned int DAT_00536a5c;
extern unsigned int DAT_00536a60;
extern float _DAT_00536a64;
extern float _DAT_005200b8;

unsigned int *FUN_0048e480(int param_1, char *param_2);
int FUN_00419da0(float *a, float *b, float *c, float *d, float *e);
int FUN_00419f10(float *a, int b, int *c, float *d);
void FUN_00415300(float *a, float *b);
void FUN_00414fe0(float *a, float *b);
unsigned int FUN_004198d0(int param_1, unsigned int *param_2);

unsigned int FUN_004198d0(int param_1, unsigned int *param_2)
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
    float local_5c;
    float local_58;
    float local_54;
    float local_50;
    float local_4c;
    float local_48;
    unsigned int local_44;
    float local_40[16];

    do {
        if (param_2 == (unsigned int *)0) {
            return 0;
        }
        local_64 = 0;
        while (param_2[1] == 7) {
            if (DAT_0052a348 == 0) {
                param_2 = FUN_0048e480(param_1, (char *)param_2[5]);
            }
            else if (*(char *)param_2[5] == '\0') {
                param_2 = (unsigned int *)*param_2;
            }
            else {
                puVar1 = FUN_0048e480(param_1, (char *)param_2[5]);
                *param_2 = (unsigned int)puVar1;
                *(unsigned char *)param_2[5] = 0;
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
            pfVar3 = (float *)(piVar5 + (int)((unsigned int)param_2[4] * 0x10U));
            iVar6 = DAT_00524774[5];
            if (0 < (int)param_2[4]) {
                do {
                    iVar4 = FUN_00419da0(pfVar3,
                                         (float *)(iVar6 + (int)((unsigned int)*piVar5 * 0xcU)),
                                         &_DAT_00536a20, &_DAT_00536a30, &local_68);
                    if ((iVar4 != 0) && (local_68 < _DAT_00536a64)) {
                        local_5c = (_DAT_00536a30 - _DAT_00536a20) * local_68 + _DAT_00536a20;
                        local_58 = (_DAT_00536a34 - _DAT_00536a24) * local_68 + _DAT_00536a24;
                        local_54 = (_DAT_00536a38 - _DAT_00536a28) * local_68 + _DAT_00536a28;
                        (void)local_58;
                        (void)local_54;
                        iVar4 = FUN_00419f10(pfVar3, iVar6, piVar5, &local_5c);
                        if (iVar4 != 0) {
                            _DAT_00536a64 = local_68;
                            DAT_00536a3c = DAT_00536a5c;
                        }
                    }
                    pfVar3 = pfVar3 + 3;
                    piVar5 = piVar5 + 0x10;
                    local_60 = local_60 + 1;
                } while (local_60 < (int)param_2[4]);
            }
            break;
        case 3:
            DAT_00524774 = (int *)param_2;
            pfVar3 = (float *)(param_2[5] - 0x18 + param_2[4] * 0xc);
            if ((_DAT_00536a20 < *(float *)(param_2[5] - 0x18 + param_2[4] * 0xc)) &&
                (_DAT_00536a30 < *pfVar3)) {
                return 1;
            }
            if ((_DAT_00536a24 < pfVar3[1]) && (_DAT_00536a34 < pfVar3[1])) {
                return 1;
            }
            if ((_DAT_00536a28 < pfVar3[2]) && (_DAT_00536a38 < pfVar3[2])) {
                return 1;
            }
            if ((pfVar3[3] < _DAT_00536a20) && (pfVar3[3] < _DAT_00536a30)) {
                return 1;
            }
            if ((pfVar3[4] < _DAT_00536a24) && (pfVar3[4] < _DAT_00536a34)) {
                return 1;
            }
            if ((pfVar3[5] < _DAT_00536a28) && (pfVar3[5] < _DAT_00536a38)) {
                return 1;
            }
            break;
        case 0x15:
            local_64 = 1;
            break;
        case 0x17:
            if ((DAT_00536a60 & 0x7fffffff) != 0) {
                pfVar3 = (float *)param_2[5];
                _DAT_00536a20 = _DAT_00536a20 - *pfVar3;
                _DAT_00536a24 = _DAT_00536a24 - pfVar3[1];
                _DAT_00536a28 = _DAT_00536a28 - pfVar3[2];
                _DAT_00536a30 = _DAT_00536a30 - *pfVar3;
                _DAT_00536a34 = _DAT_00536a34 - pfVar3[1];
                _DAT_00536a38 = _DAT_00536a38 - pfVar3[2];
                local_50 = pfVar3[3] * _DAT_005200b8;
                local_4c = pfVar3[4] * _DAT_005200b8;
                local_48 = pfVar3[5] * _DAT_005200b8;
                local_44 = DAT_00536a60;
                (void)local_4c;
                (void)local_48;
                (void)local_44;
                FUN_00415300(local_40, &local_50);
                FUN_00414fe0(&_DAT_00536a20, local_40);
                FUN_00414fe0(&_DAT_00536a30, local_40);
                _DAT_00536a20 = *pfVar3 + _DAT_00536a20;
                _DAT_00536a24 = pfVar3[1] + _DAT_00536a24;
                _DAT_00536a28 = pfVar3[2] + _DAT_00536a28;
                _DAT_00536a30 = *pfVar3 + _DAT_00536a30;
                _DAT_00536a34 = pfVar3[1] + _DAT_00536a34;
                _DAT_00536a38 = pfVar3[2] + _DAT_00536a38;
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
                    iVar2 = FUN_004198d0(param_1, *(unsigned int **)(param_2[3] + iVar4));
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
