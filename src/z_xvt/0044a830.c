/* SPDX-License-Identifier: MIT */
/* z_xvt 0044a830 */
extern int DAT_00525cc8;
extern int DAT_0052a348;

void * __cdecl FUN_0048e480(int param_1, char *param_2);
unsigned int * __cdecl FUN_0044a830(unsigned int *param_1, int param_2, int param_3);

unsigned int * __cdecl FUN_0044a830(unsigned int *param_1, int param_2, int param_3)
{
    unsigned int *puVar1;
    int iVar2;
    int iVar3;

    if (param_1 == NULL) {
        return NULL;
    }
    do {
        if (param_1[1] != 7) {
            if (param_1[1] == 0x16) {
                if (param_3 == DAT_00525cc8) {
                    return param_1;
                }
                DAT_00525cc8 = DAT_00525cc8 + 1;
            }
            iVar3 = 0;
            iVar2 = 0;
            if (0 < (int)param_1[2]) {
                do {
                    puVar1 = FUN_0044a830(*(unsigned int **)(param_1[3] + iVar3), param_2, param_3);
                    if (puVar1 != NULL) {
                        return puVar1;
                    }
                    iVar3 = iVar3 + 4;
                    iVar2 = iVar2 + 1;
                } while (iVar2 < (int)param_1[2]);
            }
            return NULL;
        }
        if (DAT_0052a348 == 0) {
            param_1 = (unsigned int *)FUN_0048e480(param_2, (char *)param_1[5]);
        }
        else if (*(char *)param_1[5] == '\0') {
            param_1 = (unsigned int *)*param_1;
        }
        else {
            puVar1 = (unsigned int *)FUN_0048e480(param_2, (char *)param_1[5]);
            *param_1 = (unsigned int)puVar1;
            *(char *)param_1[5] = 0;
            param_1 = (unsigned int *)*param_1;
        }
    } while (param_1 != NULL);
    return NULL;
}
