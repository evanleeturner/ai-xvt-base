/* SPDX-License-Identifier: MIT */
/* z_bop 00481ea0 */
extern int DAT_009a1fe8;
extern uint DAT_009e9670;
extern int FUN_00405820(uint param_1);

void __cdecl FUN_00481ea0(uint param_1, uint param_2, int param_3)
{
    uint *puVar2 = (uint *)((int)&DAT_009e9670 + param_3 * 0x5bd);
    int iVar1 = (param_2 & 0xffff) * 4;
    int *pRef = (int *)(*puVar2 * 0x23 + 0x1f + DAT_009a1fe8);
    int *pObj = (int *)(*pRef + 0xa9);
    int *pArr = (int *)(*pObj + 0x211);

    if (0 < *(int *)((int)pArr + iVar1)) {
        int iVar7 = FUN_00405820(*puVar2);
        int iVar4 = *pObj;
        int idx1 = (param_1 & 0xffff) * 4;
        int *piVar3 = (int *)((int)pArr + idx1);
        int iVar5 = *piVar3;
        short sVar6 = (short)iVar7 - (short)iVar5;

        if (0 < sVar6) {
            iVar7 = (int)sVar6;
            iVar4 = *(int *)((int)pArr + iVar1);
            if (iVar7 < iVar4) {
                *piVar3 = iVar5 + iVar7;
                *(int *)((int)pArr + iVar1) = *(int *)((int)pArr + iVar1) - iVar7;
                return;
            }
            *piVar3 = iVar5 + iVar4;
            *(int *)((int)pArr + iVar1) = 0;
        }
    }
    return;
}
