/* SPDX-License-Identifier: MIT */
/* z_bop 00406290 */
/* Globals (types inferred from the decompile; unaligned fields are accessed via memcpy) */
extern unsigned char *DAT_00a08104;
extern unsigned char *DAT_009a1fe8;
extern unsigned char *DAT_009a8e18;
extern int DAT_009e9640;
extern unsigned char DAT_009e96fd[];
extern unsigned char DAT_009e96dd[];
extern unsigned char DAT_009e968a[];
extern unsigned char DAT_009e9699[];
extern unsigned short DAT_0051a3de[];
extern unsigned int DAT_009d6990[];
extern unsigned char DAT_009d8c88[];
extern unsigned char DAT_009d8c84[];
extern unsigned char DAT_009d9155[];
extern unsigned char DAT_0051c5aa[];

unsigned int FUN_004065d0(int param_1, int param_2, int param_3);
void FUN_0042e720(unsigned int param_1, int param_2);
void FUN_00407910(unsigned short param_1);

int __cdecl FUN_00406290(int param_1, int param_2, int param_3, unsigned int param_4)
{
    int iVar1;
    unsigned char *pcVar2;
    unsigned char *pbVar3;
    unsigned short uVar4;
    int iVar5;
    unsigned char *iVar6;
    unsigned int uVar7;
    int iVar8;
    int iVar9;
    short sTmp;
    unsigned short usTmp;
    int iTmp;
    unsigned int uTmp;

    iVar6 = DAT_00a08104;
    iVar1 = (int)((unsigned int)param_1 * 0x23u);
    __builtin_memcpy(&iVar5, DAT_009a1fe8 + 0x1b + iVar1, 4);
    iVar9 = (int)((unsigned int)param_2 * 4u);
    if (DAT_00a08104[iVar9 + 0x370] == 0) {
        return -1;
    }
    if (DAT_00a08104[iVar9 + 0x372] == 0) {
        return -1;
    }
    uVar7 = FUN_004065d0(param_1, param_2, param_3);
    if (uVar7 == 0xffffffffu) {
        return -1;
    }
    DAT_00a08104[0x254] = (unsigned char)(DAT_00a08104[0x254] + 1);
    if (iVar5 != -1) {
        iTmp = (int)((unsigned int)iVar5 * 0x5bdu);
        __builtin_memcpy(&sTmp, DAT_009e96fd + iTmp, 2);
        sTmp = (short)(sTmp + 1);
        __builtin_memcpy(DAT_009e96fd + iTmp, &sTmp, 2);
        __builtin_memcpy(&uTmp, DAT_009e96dd + iTmp, 4);
        uTmp = uTmp - (unsigned int)DAT_0051a3de[param_3];
        __builtin_memcpy(DAT_009e96dd + iTmp, &uTmp, 4);
    }
    DAT_009d6990[DAT_009d8c88[(unsigned int)DAT_009a1fe8[0x16 + iVar1] * 0x566u]] =
        DAT_009d6990[DAT_009d8c88[(unsigned int)DAT_009a1fe8[0x16 + iVar1] * 0x566u]]
        - (unsigned int)DAT_0051a3de[param_3];
    FUN_0042e720(uVar7, iVar5);
    iVar8 = (int)((unsigned int)DAT_009a1fe8[0x16 + iVar1] * 0x566u);
    if ((DAT_009d8c84[iVar8] != 0x15) && (DAT_009d9155[iVar8] != 0x15)) {
        pcVar2 = DAT_00a08104 + 0x372 + iVar9;
        *pcVar2 = (unsigned char)(*pcVar2 - 1);
    }
    iVar8 = (int)(uVar7 - (unsigned int)DAT_009e9640);
    if (param_4 < 2) {
        iVar1 = (int)((unsigned int)iVar8 * 10u);
        __builtin_memcpy(&sTmp, DAT_00a08104 + 0x23b, 2);
        DAT_009a8e18[1 + iVar1] = (unsigned char)(sTmp / 0xec);
        if (6 < DAT_009a8e18[1 + iVar1]) {
            DAT_009a8e18[1 + iVar1] = 6;
        }
    }
    iVar1 = (int)((unsigned int)iVar8 * 10u);
    if (iVar5 == -1) {
        __builtin_memcpy(&usTmp, iVar6 + 0x42, 2);
        __builtin_memcpy(DAT_009a8e18 + 4 + iVar1, &usTmp, 2);
        __builtin_memcpy(&usTmp, iVar6 + 0x46, 2);
        __builtin_memcpy(DAT_009a8e18 + 2 + iVar1, &usTmp, 2);
        __builtin_memcpy(&uVar4, iVar6 + 0x42, 2);
        if ((uVar4 != 0xffff) && (uVar4 < 0x8000)) {
            __builtin_memcpy(&usTmp, DAT_009a1fe8 + (unsigned int)uVar4 * 0x23u, 2);
            __builtin_memcpy(DAT_009a8e18 + 6 + iVar1, &usTmp, 2);
            goto LAB_00406513;
        }
    } else {
        iTmp = (int)((unsigned int)iVar5 * 0x5bdu);
        __builtin_memcpy(&usTmp, DAT_009e968a + iTmp, 2);
        __builtin_memcpy(DAT_009a8e18 + 4 + iVar1, &usTmp, 2);
        __builtin_memcpy(&usTmp, DAT_009e9699 + iTmp, 2);
        __builtin_memcpy(DAT_009a8e18 + 2 + iVar1, &usTmp, 2);
        __builtin_memcpy(&usTmp, DAT_009e968a + iTmp, 2);
        if (usTmp != 0xffff) {
            __builtin_memcpy(&usTmp, DAT_009e968a + iTmp, 2);
            __builtin_memcpy(&usTmp, DAT_009a1fe8 + (unsigned int)usTmp * 0x23u, 2);
            __builtin_memcpy(DAT_009a8e18 + 6 + iVar1, &usTmp, 2);
            goto LAB_00406513;
        }
    }
    usTmp = 0;
    __builtin_memcpy(DAT_009a8e18 + 6 + iVar1, &usTmp, 2);
LAB_00406513:
    DAT_009a8e18[iVar1] = (unsigned char)iVar5;
    __builtin_memcpy(&uVar4, DAT_009a8e18 + 4 + iVar1, 2);
    if (uVar4 != 0xffff) {
        __builtin_memcpy(&iTmp, DAT_009a1fe8 + (unsigned int)uVar4 * 0x23u + 0x1b, 4);
        if (iTmp != -1) {
            FUN_00407910((unsigned short)iVar8);
        }
    }
    if (param_4 < 2) {
        if (DAT_00a08104[0x376 + (unsigned int)DAT_0051c5aa[param_4 + (unsigned int)DAT_00a08104[4] * 0x101u] * 4u] <=
            DAT_00a08104[0x372 + (unsigned int)DAT_0051c5aa[param_4 + (unsigned int)DAT_00a08104[4] * 0x101u] * 4u]) {
            DAT_00a08104[0x235 + param_4] = (unsigned char)(DAT_00a08104[0x235 + param_4] & 0x7f);
            return iVar8;
        }
        pbVar3 = DAT_00a08104 + 0x235 + param_4;
        *pbVar3 = (unsigned char)(*pbVar3 | 0x80);
    }
    return iVar8;
}
