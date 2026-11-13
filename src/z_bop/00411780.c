/* SPDX-License-Identifier: MIT */
/* z_bop 00411780 */
extern unsigned short DAT_00a08242;
extern unsigned short DAT_009e95f0;
extern unsigned short DAT_009d8c06;
extern unsigned short DAT_00a08244;
extern unsigned short _DAT_00a08130;

undefined4 __stdcall FUN_004acac0(void);
void __stdcall FUN_004117f0(void);

void __stdcall FUN_00411780(void)
{
    DAT_00a08242 = 0;
    DAT_009e95f0 = 0;
    DAT_009d8c06 = 0;
    DAT_00a08244 = 0;
    undefined4 uVar1 = FUN_004acac0();
    _DAT_00a08130 = (short)uVar1;
    DAT_00a08242 = (unsigned short)(_DAT_00a08130 != 0);
    FUN_004117f0();
    DAT_009e95f0 = 0;
    return;
}
