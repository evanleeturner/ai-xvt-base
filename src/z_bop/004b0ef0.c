/* SPDX-License-Identifier: MIT */
/* z_bop 004b0ef0 */
extern int DAT_006644f8;
extern int DAT_006644fc;
extern int DAT_00664504;
extern int DAT_0066450c;
extern int DAT_00664510;
extern int DAT_00664514;
extern int DAT_00664518;
extern int DAT_0066451c;
extern int DAT_00664520;
extern int DAT_00664524;
extern int DAT_00664528;
extern int DAT_0066452c;
extern int DAT_00664530;
extern int DAT_00664534;
extern int DAT_00664538;
extern int DAT_0066453c;
extern int DAT_00664540;
extern int DAT_00664508;
extern int DAT_00664500;
extern int *DAT_00a90be0;

void __cdecl FUN_004b0ef0(undefined4 param_1, int param_2, int param_3)
{
    int iVar1;
    undefined4 *puVar2;
    
    puVar2 = (undefined4 *)&DAT_006644f8;
    for (iVar1 = 0x13; iVar1 != 0; iVar1 = iVar1 + -1) {
        *puVar2 = 0;
        puVar2 = puVar2 + 1;
    }
    DAT_00a90be0 = &DAT_006644f8;
    DAT_006644f8 = param_1;
    DAT_006644fc = param_2;
    DAT_00664504 = param_3;
    DAT_0066450c = 1;
    DAT_00664510 = 0x10;
    DAT_00664514 = 5;
    DAT_00664518 = 6;
    DAT_0066451c = 5;
    DAT_00664520 = 0xb;
    DAT_00664524 = 5;
    DAT_00664528 = 0;
    DAT_0066452c = 3;
    DAT_00664530 = 2;
    DAT_00664534 = 3;
    DAT_00664538 = 0;
    DAT_0066453c = 0;
    DAT_00664540 = 0;
    DAT_00664508 = param_3 / 2;
    DAT_00664500 = param_2 * param_3;
    return;
}
