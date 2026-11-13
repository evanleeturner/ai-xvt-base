/* SPDX-License-Identifier: MIT */
/* z_bop 004dbf70 */
extern unsigned int DAT_00aabd11[];

unsigned char __cdecl FUN_004dbf70(int param_1)
{
    if (param_1 >= 0 && param_1 < 0x100) {
        unsigned int ptr = DAT_00aabd11[param_1];
        if (ptr != 0) {
            return *(unsigned char *)(ptr + 0x404);
        }
    }
    return 0;
}
