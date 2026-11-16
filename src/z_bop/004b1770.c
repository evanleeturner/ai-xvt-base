/* SPDX-License-Identifier: MIT */
/* z_bop 004b1770 */
void *_malloc(unsigned int);

void * __cdecl FUN_004b1770(int *param_1)
{
    int *puVar1;
    void *pvVar2;
    int iVar3;
    int *piVar4;
    int *puVar5;
    int *piVar6;

    puVar1 = (int *)_malloc(0xd8);
    puVar5 = puVar1;
    for (iVar3 = 0x36; iVar3 != 0; iVar3--) {
        *puVar5 = 0;
        puVar5++;
    }
    *puVar1 = 0;
    piVar4 = param_1;
    piVar6 = puVar1 + 3;
    for (iVar3 = 0x13; iVar3 != 0; iVar3--) {
        *piVar6 = *piVar4;
        piVar4++;
        piVar6++;
    }
    pvVar2 = _malloc(((unsigned int)param_1[6] >> 3) * param_1[1] * *param_1);
    puVar1[0x17] = (int)pvVar2;
    puVar1[6] = ((unsigned int)param_1[6] >> 3) * *param_1;
    return (void *)puVar1;
}
