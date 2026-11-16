/* SPDX-License-Identifier: MIT */
/* z_bop 004b1de0 */
extern unsigned int *DAT_00664c80;
extern int DAT_00528ca8;
extern unsigned int _DAT_00528ca4;

void __cdecl FUN_004b2130(unsigned int param_1);

unsigned int __cdecl FUN_004b1de0(unsigned int *param_1, unsigned int param_2)
{
    unsigned int uVar1;
    int iVar2;
    unsigned int *puVar3;
    int iVar4;
    unsigned int local_c;

    local_c = 0;
    if (param_2 != 0) {
        do {
            iVar4 = 0;
            iVar2 = (int)param_1[4];
            if (iVar2 == 0) {
                uVar1 = local_c;
                for (puVar3 = param_1; ((uVar1 < param_2 && (puVar3[4] == 0)) && (puVar3[3] == param_1[3]));
                     puVar3 = puVar3 + 5) {
                    iVar4 = iVar4 + 1;
                    uVar1 = uVar1 + 1;
                }
                FUN_004b2130(param_1[3]);
                if (DAT_00528ca8 != 0) {
                    *(unsigned char *)DAT_00664c80 = 8;
                    *(unsigned char *)((int)DAT_00664c80 + 1) = 8;
                    *(unsigned short *)((int)DAT_00664c80 + 2) = 1;
                    DAT_00664c80 = DAT_00664c80 + 1;
                    *DAT_00664c80 = 1;
                    DAT_00664c80[1] = 0;
                    DAT_00528ca8 = 0;
                    DAT_00664c80 = DAT_00664c80 + 2;
                }
                *(unsigned char *)DAT_00664c80 = 3;
                *(unsigned char *)((int)DAT_00664c80 + 1) = 8;
                *(short *)((int)DAT_00664c80 + 2) = (short)iVar4;
                DAT_00664c80 = DAT_00664c80 + 1;
                for (iVar2 = iVar4; iVar2 != 0; iVar2 = iVar2 + -1) {
                    *(short *)DAT_00664c80 = (short)*param_1;
                    *(short *)((int)DAT_00664c80 + 2) = (short)param_1[1];
                    *(short *)(DAT_00664c80 + 1) = (short)param_1[2];
                    *(unsigned short *)((int)DAT_00664c80 + 6) = 0x700;
                    DAT_00664c80 = DAT_00664c80 + 2;
                    param_1 = param_1 + 5;
                }
            }
            else {
                uVar1 = local_c;
                for (puVar3 = param_1;
                     ((uVar1 < param_2 && ((int)puVar3[4] == iVar2)) && (puVar3[3] == param_1[3]));
                     puVar3 = puVar3 + 5) {
                    iVar4 = iVar4 + 1;
                    uVar1 = uVar1 + 1;
                }
                FUN_004b2130(param_1[3]);
                if ((int)param_1[4] != DAT_00528ca8) {
                    *(unsigned char *)DAT_00664c80 = 8;
                    *(unsigned char *)((int)DAT_00664c80 + 1) = 8;
                    *(unsigned short *)((int)DAT_00664c80 + 2) = 1;
                    DAT_00664c80 = DAT_00664c80 + 1;
                    *DAT_00664c80 = 1;
                    DAT_00664c80[1] = *(unsigned int *)(iVar2 + 0x74);
                    DAT_00664c80 = DAT_00664c80 + 2;
                    DAT_00528ca8 = iVar2;
                }
                *(unsigned char *)DAT_00664c80 = 3;
                *(unsigned char *)((int)DAT_00664c80 + 1) = 8;
                *(short *)((int)DAT_00664c80 + 2) = (short)iVar4;
                DAT_00664c80 = DAT_00664c80 + 1;
                for (iVar2 = iVar4; iVar2 != 0; iVar2 = iVar2 + -1) {
                    *(short *)DAT_00664c80 = (short)*param_1;
                    *(short *)((int)DAT_00664c80 + 2) = (short)param_1[1];
                    *(short *)(DAT_00664c80 + 1) = (short)param_1[2];
                    *(unsigned short *)((int)DAT_00664c80 + 6) = 0x700;
                    DAT_00664c80 = DAT_00664c80 + 2;
                    param_1 = param_1 + 5;
                }
            }
            local_c = local_c + iVar4;
        } while (local_c < param_2);
    }
    _DAT_00528ca4 = _DAT_00528ca4 + param_2;
    return 1;
}
