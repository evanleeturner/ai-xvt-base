/* SPDX-License-Identifier: MIT */
/* z_xvt 004f2320 */
void __cdecl FUN_004dd000(int *param_1, undefined4 *param_2);
void __cdecl FUN_004dd070(int *param_1, int param_2, int param_3);
int __cdecl FUN_004f36b0(void);

int __cdecl FUN_004f2320(undefined4 *param_1, undefined4 *param_2, short param_3)
{
    int local_10[4];
    int local_20[4];
    ushort local_5b_val;
    
    FUN_004dd000(local_10, param_1);
    FUN_004dd070(local_10, 1, 1);
    FUN_004dd000(local_20, param_2);
    FUN_004dd070(local_20, 1, 1);
    if (param_3 != 0) {
        __builtin_memcpy(&local_5b_val, (void *)0x5b, 2);
        return (unsigned int)local_5b_val << 0x10;
    }
    return FUN_004f36b0();
}
