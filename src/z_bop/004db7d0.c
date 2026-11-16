/* SPDX-License-Identifier: MIT */
/* z_bop 004db7d0 */
extern unsigned char DAT_00aabd11[];
extern unsigned char DAT_00aa7f9b[];

int __cdecl FUN_004dbfa0(char *param_1, int param_2);
unsigned char __cdecl FUN_004dbf70(int param_1);
uint __cdecl FUN_004b6780(int *param_1, int param_2, int param_3, uint param_4, int param_5);

uint __cdecl FUN_004db7d0(int param_1, char *param_2, int *param_3, int param_4, int param_5, uint param_6)
{
    char cVar1;
    unsigned char bVar2;
    int *piVar3;
    unsigned char *pbBase;
    int iVar4;
    unsigned char uVar5;
    int iVar6;
    uint uVar7;
    int iVar8;
    uint local_420;
    int local_41c;
    uint local_418;
    uint local_414[5];

    if (param_2 == (char *)0) {
        return 0;
    }
    if ((-1 < param_1) && (param_1 < 0x100)) {
        __builtin_memcpy(&piVar3, &DAT_00aabd11[(uint)param_1 * 4u], 4);
        if (piVar3 != (int *)0) {
            pbBase = (unsigned char *)piVar3;
            iVar6 = FUN_004dbfa0(param_2, param_1);
            if (param_4 == 0) {
                iVar6 = *param_3;
            }
            else {
                iVar6 = (int)((uint)((int)((uint)param_3[2] - (uint)*param_3) >> 1)
                              - (uint)(iVar6 >> 1) + (uint)*param_3);
            }
            if (param_5 == 0) {
                local_41c = param_3[1];
            }
            else {
                iVar8 = param_3[1];
                iVar4 = param_3[3];
                uVar5 = FUN_004dbf70(param_1);
                local_41c = (int)((uint)((int)((uint)iVar4 - (uint)iVar8) >> 1) + (uint)iVar8
                                  - (uint)((int)(uint)uVar5 >> 1));
            }
            iVar8 = 0;
            cVar1 = *param_2;
            local_418 = 0;
            local_420 = param_6;
            while ((cVar1 != '\0') && (iVar6 < 0x280)) {
                bVar2 = (unsigned char)param_2[iVar8];
                if (bVar2 == 1) {
                    local_420 = param_6;
                }
                else if ((bVar2 < 2) || (7 < bVar2)) {
                    uVar7 = (uint)bVar2;
                    local_414[0] = (uint)*(unsigned char *)(pbBase + uVar7 + 0x504);
                    local_414[1] = (uint)*(unsigned char *)(pbBase + uVar7 + 0x404);
                    local_414[3] = 0;
                    local_414[2] = 1;
                    local_414[4] = (uint)piVar3[uVar7 + 1] + (uint)*piVar3;
                    uVar7 = FUN_004b6780((int *)local_414, iVar6, local_41c, local_420, 1);
                    local_418 = local_418 | uVar7;
                    iVar6 = (int)((uint)iVar6
                                  + (uint)*(unsigned char *)(pbBase + (unsigned char)param_2[iVar8] + 0x504)
                                  + (uint)*(unsigned char *)(pbBase + 0x609));
                }
                else {
                    __builtin_memcpy(&local_420, &DAT_00aa7f9b[(uint)bVar2 * 4u], 4);
                }
                iVar8 = iVar8 + 1;
                cVar1 = param_2[iVar8];
            }
            return local_418;
        }
        return 0;
    }
    return 0;
}
