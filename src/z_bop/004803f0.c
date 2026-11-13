/* SPDX-License-Identifier: MIT */
/* z_bop 004803f0 */
extern unsigned char DAT_009e9683[];
extern unsigned int DAT_009ecc34;
extern unsigned short DAT_00a08ba0[];
extern unsigned short DAT_0052154c;
extern unsigned short DAT_00521548;
extern unsigned short DAT_00a08baa;
extern unsigned short DAT_00a08ba8;

void __cdecl FUN_004803f0(unsigned short param_1)
{
    unsigned short *psVar1;
    unsigned short uVar2;
    unsigned short local_2;
    
    if (DAT_009e9683[DAT_009ecc34 * 0x5bd] == '\0') {
        psVar1 = DAT_00a08ba0 + param_1;
        if (*psVar1 == 0) {
            *psVar1 = 1;
            local_2 = DAT_0052154c;
            DAT_0052154c = DAT_00521548;
            DAT_00521548 = param_1;
        }
        else {
            *psVar1 = -2;
        }
        if ((DAT_00a08baa != 0) && (DAT_00a08ba8 != 0)) {
            if (param_1 == 5) {
                DAT_00a08ba8 = -2;
            }
            else {
                DAT_00a08baa = -2;
            }
            if ((local_2 != 5) && (local_2 != 4)) {
                DAT_0052154c = local_2;
                return;
            }
        }
    }
    else {
        if (param_1 != 2) {
            uVar2 = 0;
            do {
                if ((((param_1 != uVar2) && (uVar2 != 5)) && (uVar2 != 2)) && (DAT_00a08ba0[uVar2] != 0)) {
                    DAT_00a08ba0[uVar2] = -2;
                }
                uVar2 = uVar2 + 1;
            } while (uVar2 < 8);
        }
        psVar1 = DAT_00a08ba0 + param_1;
        if (*psVar1 != 0) {
            *psVar1 = -2;
            return;
        }
        *psVar1 = 1;
        DAT_0052154c = DAT_00521548;
        DAT_00521548 = param_1;
    }
    return;
}
