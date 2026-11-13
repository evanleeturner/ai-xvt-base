/* SPDX-License-Identifier: MIT */
/* z_bop 004ae1a0 */
int __cdecl FUN_004ae1a0(int param_1, unsigned int param_2);

int __cdecl FUN_004ae1a0(int param_1, unsigned int param_2)
{
    int iVar1;
    int iVar2;
    int iVar3;
    
    if (param_1 == 0) {
        return 0;
    }
    if (*(int *)(param_1 + 4) != 0x19) {
        iVar3 = 0;
        iVar2 = 0;
        if (0 < *(int *)(param_1 + 8)) {
            do {
                iVar1 = *(int *)(*(int *)(param_1 + 0xc) + iVar3);
                if ((iVar1 != 0) && (iVar1 = FUN_004ae1a0(iVar1, param_2), iVar1 != 0)) {
                    return iVar1;
                }
                iVar3 = iVar3 + 4;
                iVar2 = iVar2 + 1;
            } while (iVar2 < *(int *)(param_1 + 8));
        }
        return 0;
    }
    return *(int *)(param_1 + 0x14);
}
