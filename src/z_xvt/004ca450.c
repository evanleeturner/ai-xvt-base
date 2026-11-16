/* SPDX-License-Identifier: MIT */
/* z_xvt 004ca450 */
extern char DAT_00759645;
extern int *DAT_007806fa;
extern int DAT_0078075a;
extern int DAT_00780782;
extern int DAT_00780732;
extern int DAT_0078077a;
extern int DAT_00758830;

void __stdcall FUN_004d4d90(void);
int __stdcall FUN_004d4d00(void);

int __stdcall FUN_004ca450(void)
{
    char cVar1;
    int iVar2;
    int iVar3;
    int *piVar4;
    int *puVar5;
    int *piVar6;
    int aiStack_500[4];
    int aiStack_4f0[316];
    int LAB_004ca520;

    cVar1 = DAT_00759645;
    if (DAT_007806fa == (int *)0x0) {
        return 0;
    }
    FUN_004d4d90();
    piVar4 = &DAT_0078075a;
    piVar6 = aiStack_500;
    for (iVar3 = 0x140; iVar3 != 0; iVar3 = iVar3 + -1) {
        *piVar6 = *piVar4;
        piVar4 = piVar4 + 1;
        piVar6 = piVar6 + 1;
    }
    puVar5 = &DAT_00780782;
    for (iVar3 = 0x136; iVar3 != 0; iVar3 = iVar3 + -1) {
        *puVar5 = 0;
        puVar5 = puVar5 + 1;
    }
    iVar3 = 0;
    (*(void (__stdcall **)(int *, int, int *, int, int))(*DAT_007806fa + 0x30))(DAT_007806fa, 0, &LAB_004ca520, 0, 0);
    if (0 < DAT_00780732) {
        piVar4 = &DAT_0078077a;
        do {
            iVar2 = 0;
            piVar6 = aiStack_500 + 3;
            do {
                if (*piVar6 == *piVar4) {
                    piVar4[1] = aiStack_4f0[iVar2 * 10];
                    break;
                }
                piVar6 = piVar6 + 10;
                iVar2 = iVar2 + 1;
            } while (piVar6 < aiStack_4f0 + 316);
            piVar4 = piVar4 + 10;
            iVar3 = iVar3 + 1;
        } while (iVar3 < DAT_00780732);
    }
    if (cVar1 != '\0') {
        DAT_00758830 = FUN_004d4d00();
    }
    return 1;
}
