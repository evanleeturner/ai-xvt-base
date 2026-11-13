/* SPDX-License-Identifier: MIT */
/* z_xvt 0042de20 */
extern unsigned int _DAT_005373d0;
extern unsigned int DAT_005373d4;
extern unsigned int DAT_005373d8;
extern unsigned int DAT_00732a18;

void __cdecl FUN_004310c0(unsigned int *param_1, unsigned int param_2);

void __cdecl FUN_0042de20(unsigned int *param_1, unsigned int *param_2, unsigned int param_3)
{
    unsigned int uVar1;
    int iVar2;
    unsigned int *puVar3;

    _DAT_005373d0 = 0x18;
    DAT_005373d4 = DAT_00732a18;
    puVar3 = &DAT_005373d8;
    for (uVar1 = param_3 & 0x3fffffff; uVar1 != 0; uVar1 = uVar1 - 1) {
        *puVar3 = *param_1;
        param_1 = param_1 + 1;
        puVar3 = puVar3 + 1;
    }
    for (iVar2 = 0; iVar2 != 0; iVar2 = iVar2 + -1) {
        *(unsigned char *)puVar3 = *(unsigned char *)param_1;
        param_1 = (unsigned int *)((int)param_1 + 1);
        puVar3 = (unsigned int *)((int)puVar3 + 1);
    }
    puVar3 = (unsigned int *)(&DAT_005373d8 + param_3 * 4);
    for (uVar1 = param_3 & 0x3fffffff; uVar1 != 0; uVar1 = uVar1 - 1) {
        *puVar3 = *param_2;
        param_2 = param_2 + 1;
        puVar3 = puVar3 + 1;
    }
    for (iVar2 = 0; iVar2 != 0; iVar2 = iVar2 + -1) {
        *(unsigned char *)puVar3 = *(unsigned char *)param_2;
        param_2 = (unsigned int *)((int)param_2 + 1);
        puVar3 = (unsigned int *)((int)puVar3 + 1);
    }
    FUN_004310c0(&_DAT_005373d0, param_3 * 8 + 8);
    return;
}
