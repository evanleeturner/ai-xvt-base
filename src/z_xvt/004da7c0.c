/* SPDX-License-Identifier: MIT */
/* z_xvt 004da7c0 */
extern unsigned char DAT_0075d846[]; /* packed: table of pointers at odd address 0x0075d846 */
extern uint DAT_00759ad0[];

uint __cdecl FUN_004d2190(int *param_1, int param_2, int param_3, uint param_4, int param_5);

uint __cdecl FUN_004da7c0(int param_1, char *param_2, int param_3, int param_4, uint param_5)
{
    char cVar1;
    byte bVar2;
    int *piVar3;
    uint uVar4;
    int iVar5;
    uint local_41c;
    uint local_418;
    uint local_414[5]; /* local_414, local_410, local_40c, local_408, local_404 (contiguous) */

    if (param_2 == (char *)0) {
        return 0;
    }
    if ((-1 < param_1) && (param_1 < 0x100)) {
        __builtin_memcpy(&piVar3, DAT_0075d846 + (uint)param_1 * 4u, sizeof(piVar3));
        if (piVar3 != (int *)0) {
            iVar5 = 0;
            cVar1 = *param_2;
            local_418 = 0;
            local_41c = param_5;
            while ((cVar1 != '\0') && (param_3 < 0x280)) {
                bVar2 = (byte)param_2[iVar5];
                if (bVar2 == 1) {
                    local_41c = param_5;
                }
                else if ((bVar2 < 2) || (7 < bVar2)) {
                    uVar4 = (uint)bVar2;
                    local_414[0] = (uint)*(byte *)(uVar4 + 0x504 + (uint)piVar3);
                    local_414[1] = (uint)*(byte *)(uVar4 + 0x404 + (uint)piVar3);
                    local_414[3] = 0;
                    local_414[2] = 1;
                    local_414[4] = (uint)piVar3[uVar4 + 1] + (uint)*piVar3;
                    uVar4 = FUN_004d2190((int *)local_414, param_3, param_4, local_41c, 1);
                    local_418 = local_418 | uVar4;
                    param_3 = (int)((uint)param_3 +
                                    ((uint)*(byte *)((byte)param_2[iVar5] + 0x504 + (uint)piVar3) +
                                     (uint)*(byte *)((uint)piVar3 + 0x609)));
                }
                else {
                    local_41c = DAT_00759ad0[bVar2];
                }
                cVar1 = param_2[iVar5 + 1];
                iVar5 = iVar5 + 1;
            }
            return local_418;
        }
        return 0;
    }
    return 0;
}
