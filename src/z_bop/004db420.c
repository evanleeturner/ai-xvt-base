/* SPDX-License-Identifier: MIT */
/* z_bop 004db420 */
void _free(void *);

void __stdcall FUN_004db420(void)
{
    unsigned int *puVar2;
    int iVar1;
    unsigned int tmp;

    puVar2 = (unsigned int *)0x00aa80a3;
    do {
        if (*(char *)((char *)puVar2 + 0x608) == 1) {
            __builtin_memcpy(&tmp, puVar2, 4);
            if (tmp != 0) {
                _free((void *)tmp);
                tmp = 0;
                __builtin_memcpy(puVar2, &tmp, 4);
            }
            *(char *)((char *)puVar2 + 0x608) = 0;
        }
        puVar2 = (unsigned int *)((unsigned int)puVar2 + 0x60b);
    } while ((unsigned int)puVar2 < 0x00aabd11);

    puVar2 = (unsigned int *)0x00aabd11;
    for (iVar1 = 0x100; iVar1 != 0; iVar1--) {
        tmp = 0;
        __builtin_memcpy(puVar2, &tmp, 4);
        puVar2++;
    }
    return;
}
