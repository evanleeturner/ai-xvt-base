/* SPDX-License-Identifier: MIT */
/* z_xvt 004cdc60 */
extern char DAT_0078077a;
extern char DAT_00780c7a;
extern char DAT_0078077e;

void __cdecl FUN_004cdc60(int param_1)
{
    int iVar1;
    char *piVar2;
    int local_val;
    int local_zero;

    iVar1 = 0;
    piVar2 = &DAT_0078077a;
    do {
        __builtin_memcpy(&local_val, piVar2, 4);
        if (local_val == param_1) {
            break;
        }
        piVar2 += 0x28;
        iVar1 += 1;
    } while (piVar2 < &DAT_00780c7a);

    if (iVar1 != 0x20) {
        local_zero = 0;
        __builtin_memcpy(&DAT_0078077e + iVar1 * 0x28, &local_zero, 4);
    }
    return;
}
