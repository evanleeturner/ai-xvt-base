/* SPDX-License-Identifier: MIT */
/* z_bop 0046d300 */
extern int DAT_009994fc;
extern uint DAT_00999570[];

void __cdecl FUN_0046d350(uint param_1, uint *param_2, uint param_3);

void __cdecl FUN_0046d300(uint *param_1, uint param_2)
{
    int iVar1;
    uint *piVar2;
    
    iVar1 = 0;
    if (DAT_009994fc > 0) {
        piVar2 = DAT_00999570;
        do {
            if (*piVar2 != 0 && piVar2[1] != 0) {
                FUN_0046d350(*piVar2, param_1, param_2);
            }
            piVar2 += 10;
            iVar1++;
        } while (iVar1 < DAT_009994fc);
    }
}
