/* SPDX-License-Identifier: MIT */
/* z_bop 00479de0 */
void *FUN_00479e20(void *, char *);

void *FUN_00479de0(void *param_1, char *param_2)
{
    int count;
    __builtin_memcpy(&count, (char *)param_1 + 6, sizeof(int));
    if (count > 0) {
        int base_addr;
        __builtin_memcpy(&base_addr, (char *)param_1 + 10, sizeof(int));
        void **array_base = (void **)base_addr;
        int i = 0;
        do {
            void *res = FUN_00479e20(array_base[i], param_2);
            if (res != NULL) {
                return res;
            }
            i++;
        } while (i < count);
    }
    return NULL;
}
