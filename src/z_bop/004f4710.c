/* SPDX-License-Identifier: MIT */
/* z_bop 004f4710 */
extern uint DAT_00aa5cb0;
extern uint DAT_00aa6af0;
extern uint DAT_00aa6af4;
extern void __cdecl FUN_004cef70(uint, uint *, uint);

uint __stdcall FUN_004f4710(void)
{
    uint iVar1;
    uint *puVar2;
    uint *puVar3;
    
    DAT_00aa6af0 = 0x48;
    puVar2 = &DAT_00aa5cb0;
    puVar3 = &DAT_00aa6af4;
    for (iVar1 = 0x50; iVar1 != 0; iVar1 = iVar1 - 1) {
        *puVar3 = *puVar2;
        puVar2 = puVar2 + 1;
        puVar3 = puVar3 + 1;
    }
    FUN_004cef70(0, &DAT_00aa6af0, 0x144);
    return 1;
}
