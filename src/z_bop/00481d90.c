/* SPDX-License-Identifier: MIT */
/* z_bop 00481d90 */
extern int DAT_009e9670;
extern int DAT_009a1fe8;

void __cdecl FUN_00481d90(short param_1, int param_2)
{
    unsigned short uVar1;
    int iVar2;
    unsigned short new_val;

    iVar2 = *(int *)(*(int *)(*(int *)((int)&DAT_009e9670 + param_2 * 0x5bd) * 0x23 + 0x1f + DAT_009a1fe8) + 0xa9);
    __builtin_memcpy(&uVar1, (void *)(iVar2 + 0xbd), 2);
    new_val = (unsigned short)(param_1 + uVar1);
    __builtin_memcpy((void *)(iVar2 + 0xbd), &new_val, 2);
    if (new_val < uVar1) {
        unsigned short wrap_val = 0xffff;
        __builtin_memcpy((void *)(iVar2 + 0xbd), &wrap_val, 2);
    }
    return;
}
