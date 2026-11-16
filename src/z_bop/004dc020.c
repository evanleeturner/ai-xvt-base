/* SPDX-License-Identifier: MIT */
/* z_bop 004dc020 */
extern char DAT_00527004;

void * __cdecl FUN_004cc4a0(char *param_1, char *param_2);
void __cdecl FUN_004cc790(void *param_1, void *param_2, size_t param_3);
void __cdecl FUN_004cc590(void *param_1);

void __cdecl FUN_004dc020(char *param_1, unsigned int *param_2, size_t param_3)
{
    void *pFVar1;
    int iVar2;
    unsigned int *puVar3;
    unsigned int *puVar4;
    unsigned int local_60c[0x183];

    if (param_2 != (unsigned int *)0x0) {
        pFVar1 = FUN_004cc4a0(param_1, &DAT_00527004);
        if (pFVar1 != (void *)0x0) {
            puVar3 = param_2;
            puVar4 = local_60c;
            for (iVar2 = 0x182; iVar2 != 0; iVar2 = iVar2 + -1) {
                *puVar4 = *puVar3;
                puVar3 = puVar3 + 1;
                puVar4 = puVar4 + 1;
            }
            *(unsigned short *)puVar4 = *(unsigned short *)puVar3;
            *(unsigned char *)((int)puVar4 + 2) = *(unsigned char *)((int)puVar3 + 2);
            local_60c[0] = (unsigned int)param_3;
            FUN_004cc790(pFVar1, local_60c, 0x60b);
            FUN_004cc790(pFVar1, (void *)*param_2, param_3);
            FUN_004cc590(pFVar1);
        }
    }
    return;
}
