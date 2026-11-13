/* SPDX-License-Identifier: MIT */
/* z_bop 0044bad0 */
int __cdecl FUN_0044bad0(int *param_1, void *param_2, int param_3)
{
    int iVar1;
    int local_c;
    int local_8;
    int local_10;

    if (param_1 != 0 && param_2 != 0 && param_3 != 0) {
        iVar1 = (*(int (**)(int *, int, int, int *))(*(void **)param_1 + 0x2c))(param_1, 0, param_3, &local_c);
        if (iVar1 >= 0) {
            int *src = (int *)param_2;
            int *dst = &local_8;
            unsigned int uVar2 = (unsigned int)param_3 >> 2;
            while (uVar2 != 0) {
                *dst = *src;
                src++;
                dst++;
                uVar2--;
            }
            int rem = param_3 & 3;
            while (rem != 0) {
                *(char *)dst = *(char *)src;
                src = (int *)((char *)src + 1);
                dst = (int *)((char *)dst + 1);
                rem--;
            }

            if (local_c != 0) {
                src = (int *)((char *)param_2 + param_3);
                dst = &local_10;
                uVar2 = (unsigned int)local_c >> 2;
                while (uVar2 != 0) {
                    *dst = *src;
                    src++;
                    dst++;
                    uVar2--;
                }
                rem = local_c & 3;
                while (rem != 0) {
                    *(char *)dst = *(char *)src;
                    src = (int *)((char *)src + 1);
                    dst = (int *)((char *)dst + 1);
                    rem--;
                }
            }

            (*(void (**)(int *, int, int *, int *, int))(*(void **)param_1 + 0x4c))(param_1, 0, &local_8, &local_10, local_c);
            return 1;
        }
    }
    return 0;
}
