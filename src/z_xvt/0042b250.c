/* SPDX-License-Identifier: MIT */
/* z_xvt 0042b250 */
unsigned int __cdecl FUN_0042b250(int param_1)
{
    unsigned short *psVar1;
    unsigned short *puVar2;
    unsigned short field_9;

    psVar1 = *(unsigned short **)(param_1 + 0x432);
    if (psVar1 == NULL) {
        __builtin_memcpy(&field_9, (void *)(param_1 + 9), 2);
        return field_9;
    }
    if (*(unsigned short *)(param_1 + 0x384) != *psVar1) {
        *(unsigned int *)(param_1 + 0x432) = 0;
        __builtin_memcpy(&field_9, (void *)(param_1 + 9), 2);
        return ((unsigned int)psVar1 & 0xFFFF0000) | field_9;
    }
    {
        int field_1f;
        __builtin_memcpy(&field_1f, (void *)((char *)psVar1 + 0x1f), 4);
        if (field_1f == 0) {
            __builtin_memcpy(&field_9, (void *)(param_1 + 9), 2);
            return field_9;
        }
        __builtin_memcpy(&puVar2, (void *)(field_1f + 0xab), 4);
        if (puVar2 != NULL) {
            return ((unsigned int)puVar2 & 0xFFFF0000) | *puVar2;
        }
    }
    __builtin_memcpy(&field_9, (void *)(param_1 + 9), 2);
    return field_9;
}
