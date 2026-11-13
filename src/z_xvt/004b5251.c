/* SPDX-License-Identifier: MIT */
/* z_xvt 004b5251 */
int __cdecl FUN_004b5251(int param_1)
{
    int local_8;

    local_8 = 0;
    for (; 1 < param_1; param_1 = param_1 >> 1) {
        local_8 = local_8 + 1;
    }
    return local_8;
}
