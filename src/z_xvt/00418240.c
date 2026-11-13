/* SPDX-License-Identifier: MIT */
/* z_xvt 00418240 */
undefined4 __cdecl FUN_00418240(int *param_1, undefined4 *param_2, int param_3)
{
    int size1;
    uint uVar2;
    undefined4 *puVar3;
    undefined4 *puVar4;
    undefined4 local_10;
    undefined4 local_4;
    undefined4 local_8;
    int i;

    if (param_1 != NULL && param_2 != NULL && param_3 != 0) {
        size1 = ((int (__cdecl *)(int *, int, int, undefined4 *))(*param_1 + 0x2c))(param_1, 0, param_3, &local_4);
        if (size1 >= 0) {
            puVar3 = param_2;
            puVar4 = (undefined4 *)&local_8;
            uVar2 = (uint)size1 >> 2;
            while (uVar2 != 0) {
                *puVar4 = *puVar3;
                puVar3++;
                puVar4++;
                uVar2--;
            }
            i = size1 & 3;
            while (i != 0) {
                *(undefined1 *)puVar4 = *(undefined1 *)puVar3;
                puVar3 = (undefined4 *)((int)puVar3 + 1);
                puVar4 = (undefined4 *)((int)puVar4 + 1);
                i--;
            }
            if (local_4 != 0) {
                puVar4 = &local_10;
                uVar2 = (uint)local_4 >> 2;
                while (uVar2 != 0) {
                    *puVar4 = *puVar3;
                    puVar3++;
                    puVar4++;
                    uVar2--;
                }
                i = local_4 & 3;
                while (i != 0) {
                    *(undefined1 *)puVar4 = *(undefined1 *)puVar3;
                    puVar3 = (undefined4 *)((int)puVar3 + 1);
                    puVar4 = (undefined4 *)((int)puVar4 + 1);
                    i--;
                }
            }
            ((void (__cdecl *)(int *, int, undefined4 *, undefined4 *, undefined4))(*param_1 + 0x4c))(param_1, 0, &local_8, &local_10, local_4);
            return 1;
        }
    }
    return 0;
}
