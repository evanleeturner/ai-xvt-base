/* SPDX-License-Identifier: MIT */
/* z_bop 004d11f0 */
extern int DAT_00acebfd;
extern unsigned int DAT_00acec45;

int __cdecl FUN_004d11a0(unsigned int param_1);

void __cdecl FUN_004d11f0(int param_1)
{
    int iVar1;
    int iVar2 = 0;
    unsigned int *puVar3;
    
    if (DAT_00acebfd > 0) {
        puVar3 = &DAT_00acec45;
        while (iVar1 = FUN_004d11a0(*puVar3), param_1 != iVar1) {
            puVar3 += 10;
            iVar2 += 1;
            if (DAT_00acebfd <= iVar2) {
                return;
            }
        }
    }
    return;
}
