/* SPDX-License-Identifier: MIT */
/* z_bop 004e6750 */
extern int DAT_00aa6af0;
extern int DAT_00aa6af4;
extern int DAT_00aa6af8;
extern int DAT_00aa6ae8;

extern int __stdcall FUN_004d1400(void);
extern int * FUN_004cfe80(int *param_1);
extern unsigned int FUN_004d2250(int param_1);
extern int FUN_004cef70(int param_1, unsigned int *param_2, unsigned int param_3);

void __cdecl FUN_004e6750(int param_1)
{
    int *puVar1;
    unsigned int uVar2;
    int iVar3;
    int *piVar4;
    int iVar5;
    int local_4;

    DAT_00aa6af0 = 0x74;
    DAT_00aa6af4 = 8;
    iVar3 = 3;
    DAT_00aa6af8 = FUN_004d1400();
    iVar5 = 0;
    puVar1 = FUN_004cfe80(&local_4);
    if (0 < local_4) {
        piVar4 = puVar1 + 8;
        do {
            if (piVar4[1] == 1) {
                (&DAT_00aa6ae8)[iVar3 + 2] = *piVar4;
                iVar3 = iVar3 + 3;
                (&DAT_00aa6ae8)[iVar3] = *(unsigned char *)(piVar4 - 8) - 1;
                uVar2 = FUN_004d2250(*piVar4);
                *(unsigned int *)(iVar3 * 4 + 0xaa6aec) = uVar2;
            }
            piVar4 = piVar4 + 10;
            iVar5 = iVar5 + 1;
        } while (iVar5 < local_4);
    }
    FUN_004cef70(param_1, (unsigned int *)&DAT_00aa6af0, iVar3 << 2);
    return;
}
