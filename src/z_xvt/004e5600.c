/* SPDX-License-Identifier: MIT */
/* z_xvt 004e5600 */
extern int DAT_00758618;
extern int DAT_00758620;
extern int DAT_00758624;
extern int DAT_00758628;

unsigned int __cdecl FUN_004cde00(void);
unsigned int * __cdecl FUN_004cc7b0(int *param_1);
unsigned int __cdecl FUN_004ced20(int param_1);
int __cdecl FUN_004cb830(int param_1, unsigned int *param_2, unsigned int param_3);

void __cdecl FUN_004e5600(int param_1)
{
    unsigned int *puVar1;
    unsigned int uVar2;
    int *piVar3;
    int iVar4;
    int iVar5;
    int local_4;

    DAT_00758620 = 0x74;
    DAT_00758624 = 8;
    iVar5 = 3;
    DAT_00758628 = FUN_004cde00();
    iVar4 = 0;
    puVar1 = FUN_004cc7b0(&local_4);
    if (0 < local_4) {
        piVar3 = (int *)(puVar1 + 8);
        do {
            if (piVar3[1] == 1) {
                (&DAT_00758618)[iVar5 + 2] = *piVar3;
                iVar5 = iVar5 + 3;
                (&DAT_00758618)[iVar5] = *(unsigned char *)(piVar3 - 8) - 1;
                uVar2 = FUN_004ced20(*piVar3);
                *(unsigned int *)(iVar5 * 4 + 0x75861c) = uVar2;
            }
            piVar3 = piVar3 + 10;
            iVar4 = iVar4 + 1;
        } while (iVar4 < local_4);
    }
    FUN_004cb830(param_1, (unsigned int *)&DAT_00758620, (unsigned int)(iVar5 << 2));
    return;
}
