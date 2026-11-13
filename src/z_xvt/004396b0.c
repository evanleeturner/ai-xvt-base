/* SPDX-License-Identifier: MIT */
/* z_xvt 004396b0 */
extern uint FUN_00491810(char *param_1, int param_2);
extern uint FUN_00491240(char *param_1, int param_2);
extern uint FUN_004914a0(char *param_1, int param_2);
extern uint FUN_00491700(char *param_1, int param_2);

uint __cdecl FUN_004396b0(int param_1, int param_2, int param_3)
{
    if (param_1 < 4 || 0x342 < param_1) {
        return 0;
    }
    if (param_2 < 0x601) {
        if (param_2 == 0x600) {
            return FUN_00491240((char *)(0x73ba30 + param_1 * 0x18), param_3);
        }
        if (param_2 != 0x500) {
            return 0;
        }
        return FUN_00491810((char *)(0x73ba30 + param_1 * 0x18), param_3);
    }
    if (param_2 == 0x700) {
        return FUN_004914a0((char *)(0x73ba30 + param_1 * 0x18), param_3);
    }
    if (param_2 != 0x777) {
        return 0;
    }
    return FUN_00491700((char *)(0x73ba30 + param_1 * 0x18), param_3);
}
