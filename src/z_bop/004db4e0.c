/* SPDX-License-Identifier: MIT */
/* z_bop 004db4e0 */
uint __cdecl FUN_004b6780(int *param_1, int param_2, int param_3, uint param_4, int param_5);

uint __cdecl FUN_004db4e0(int param_1, char *param_2, int param_3, int param_4, uint param_5)
{
    char cVar1;
    byte bVar2;
    int *piVar3;
    uint uVar4;
    uint uVar5;
    int iVar6;
    uint local_418;
    uint local_414[5];
    /* local_414 = [0], local_410 = [1], local_40c = [2], local_408 = [3], local_404 = [4] */

    if (param_2 == (char *)0x0) {
        return 0;
    }
    if ((-1 < param_1) && (param_1 < 0x100)) {
        /* DAT_00aabd11 is an unaligned table of pointers */
        __builtin_memcpy(&piVar3, (unsigned char *)0x00aabd11 + param_1 * 4, 4);
        if (piVar3 != (int *)0x0) {
            iVar6 = 0;
            local_418 = 0;
            cVar1 = *param_2;
            uVar4 = param_5;
            while ((cVar1 != '\0') && (param_3 < 0x280)) {
                bVar2 = (byte)param_2[iVar6];
                uVar5 = param_5;
                if (bVar2 != 1) {
                    if ((bVar2 < 2) || (7 < bVar2)) {
                        uVar5 = (uint)bVar2;
                        local_414[0] = (uint)*(byte *)(uVar5 + 0x504 + (int)piVar3);
                        local_414[1] = (uint)*(byte *)((int)piVar3 + uVar5 + 0x404);
                        local_414[3] = 0;
                        local_414[2] = 1;
                        local_414[4] = (uint)piVar3[uVar5 + 1] + (uint)*piVar3;
                        uVar5 = FUN_004b6780((int *)&local_414[0], param_3, param_4, uVar4, 1);
                        local_418 = local_418 | uVar5;
                        param_3 = (int)((uint)param_3 +
                                        ((uint)*(byte *)((byte)param_2[iVar6] + 0x504 + (int)piVar3) +
                                         (uint)*(byte *)((int)piVar3 + 0x609)));
                        uVar5 = uVar4;
                    }
                    else {
                        /* DAT_00aa7f9b is an unaligned uint table */
                        __builtin_memcpy(&uVar5, (unsigned char *)0x00aa7f9b + (uint)bVar2 * 4, 4);
                    }
                }
                iVar6 = iVar6 + 1;
                cVar1 = param_2[iVar6];
                uVar4 = uVar5;
            }
            return local_418;
        }
        return 0;
    }
    return 0;
}
