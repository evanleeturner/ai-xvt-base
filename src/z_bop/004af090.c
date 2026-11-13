/* SPDX-License-Identifier: MIT */
/* z_bop 004af090 */
void * __cdecl FUN_00479de0(int param_1, char *param_2);
extern int DAT_00528128;
extern int DAT_005270b0;

void * __cdecl FUN_004af090(void *param_1, int param_2, int param_3)
{
    void *puVar1;
    int iVar2;
    int iVar3;

    if (param_1 == NULL) {
        return NULL;
    }
    do {
        if (((unsigned int *)param_1)[1] != 7) {
            if (((unsigned int *)param_1)[1] == 0x16) {
                if (param_3 == DAT_00528128) {
                    return param_1;
                }
                DAT_00528128 = DAT_00528128 + 1;
            }
            iVar3 = 0;
            iVar2 = 0;
            if (0 < (int)((unsigned int *)param_1)[2]) {
                do {
                    puVar1 = FUN_004af090(*(void **)(((unsigned int *)param_1)[3] + iVar3), param_2, param_3);
                    if (puVar1 != NULL) {
                        return puVar1;
                    }
                    iVar3 = iVar3 + 4;
                    iVar2 = iVar2 + 1;
                } while (iVar2 < (int)((unsigned int *)param_1)[2]);
            }
            return NULL;
        }
        if (DAT_005270b0 == 0) {
            param_1 = FUN_00479de0(param_2, (char *)((unsigned int *)param_1)[5]);
        }
        else if (*(char *)((unsigned int *)param_1)[5] == '\0') {
            param_1 = *(void **)param_1;
        }
        else {
            puVar1 = FUN_00479de0(param_2, (char *)((unsigned int *)param_1)[5]);
            *(void **)param_1 = puVar1;
            *(char *)((unsigned int *)param_1)[5] = 0;
            param_1 = *(void **)param_1;
        }
    } while (param_1 != NULL);
    return NULL;
}
