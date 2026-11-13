/* SPDX-License-Identifier: MIT */
/* z_bop 0041bbf0 */
void __cdecl FUN_0041bbf0(unsigned char *param_1, short param_2)
{
    unsigned char bVar1;
    unsigned char *pbVar2;
    unsigned int uVar3;
    int iVar4;
    unsigned char *pbVar5;
    
    uVar3 = 0;
    bVar1 = *param_1;
    if (bVar1 != 0) {
        pbVar2 = param_1 + 0x41;
        do {
            unsigned short tmp;
            __builtin_memcpy(&tmp, pbVar2, 2);
            if (tmp == param_2) break;
            pbVar2 = pbVar2 + 2;
            uVar3 = uVar3 + 1;
        } while (uVar3 < bVar1);
    }
    if (uVar3 != bVar1) {
        iVar4 = uVar3 + 1;
        if (iVar4 < bVar1) {
            pbVar2 = param_1 + iVar4 * 2 + 0x3f;
            pbVar5 = param_1 + iVar4 * 4 - 3;
            do {
                unsigned short tmp2;
                unsigned int tmp4;
                __builtin_memcpy(&tmp2, pbVar2 + 2, 2);
                __builtin_memcpy(pbVar2, &tmp2, 2);
                pbVar2 = pbVar2 + 2;
                iVar4 = iVar4 + 1;
                __builtin_memcpy(&tmp4, pbVar5 + 4, 4);
                __builtin_memcpy(pbVar5, &tmp4, 4);
                pbVar5 = pbVar5 + 4;
            } while (iVar4 < *param_1);
        }
        *param_1 = *param_1 - 1;
    }
    return;
}
