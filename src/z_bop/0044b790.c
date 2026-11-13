/* SPDX-License-Identifier: MIT */
/* z_bop 0044b790 */
HFILE FUN_004cc4a0(char *param_1, void *param_2);
void FUN_004cc5c0(HFILE param_1, long param_2, short param_3);
size_t FUN_004cc5e0(HFILE param_1);
void *_malloc(size_t size);
int FUN_004cc6c0(HFILE param_1, int *param_2, size_t param_3);
int FUN_0044bb90(int *param_1, int *param_2, int *param_3, unsigned int *param_4);
void FUN_004cc590(HFILE param_1);
void FUN_0049eb60(short param_1);
extern int DAT_0051a854;
extern int *DAT_005569d4;

int FUN_0044b790(int param_1, char *param_2, int *param_3, int *param_4, unsigned int *param_5) {
    HFILE pFVar2;
    size_t _Size;
    int *piVar3;
    int bVar1;
    int iVar4;

    pFVar2 = FUN_004cc4a0(param_2, &DAT_0051a854);
    if (pFVar2 != 0) {
        FUN_004cc5c0(pFVar2, 0, 2);
        _Size = FUN_004cc5e0(pFVar2);
        FUN_004cc5c0(pFVar2, 0, 0);
        piVar3 = _malloc(_Size);
        DAT_005569d4 = piVar3;
        if (piVar3 == 0) {
            FUN_0049eb60(0);
        } else {
            bVar1 = FUN_004cc6c0(pFVar2, piVar3, _Size);
            if (bVar1 != 0) {
                iVar4 = FUN_0044bb90(piVar3, param_3, param_4, param_5);
                if (iVar4 != 0) {
                    FUN_004cc590(pFVar2);
                    return 1;
                }
            }
        }
        FUN_004cc590(pFVar2);
    }
    return 0;
}
