/* SPDX-License-Identifier: MIT */
/* z_bop 0042ac50 */
void __cdecl FUN_0042a920(undefined4 *, int, undefined4, undefined4);

void __cdecl FUN_0042ac50(int param_1, undefined4 param_2, undefined4 param_3)
{
    int count;
    int arr_ptr;
    int i;
    int idx;

    __builtin_memcpy(&count, (void *)(param_1 + 6), 4);
    idx = 0;
    if (0 < count) {
        i = 0;
        do {
            i = i + 4;
            idx = idx + 1;
            __builtin_memcpy(&arr_ptr, (void *)(param_1 + 10), 4);
            FUN_0042a920(*(undefined4 **)(arr_ptr - 4 + i), param_1, param_2, param_3);
        } while (idx < count);
    }
    return;
}
