/* SPDX-License-Identifier: MIT */
/* z_bop 004d2250 */
extern int DAT_00665050;
extern int DAT_00665410;

uint __cdecl FUN_004d2250(int param_1)
{
    uint uVar1;
    int iVar2;
    int *piVar3;
    
    iVar2 = 0;
    piVar3 = &DAT_00665050;
    do {
        if (*piVar3 == param_1) {
            uVar1 = *(uint *)(iVar2 * 0x18 + 0x665064);
            if (uVar1 == 0) {
                return 1;
            }
            return *(uint *)(iVar2 * 0x18 + 0x665054) / uVar1;
        }
        piVar3 += 6;
        iVar2 += 1;
    } while (piVar3 < &DAT_00665410);
    return 0;
}
