/* SPDX-License-Identifier: MIT */
/* z_xvt 00478040 */
typedef unsigned int  uint;
typedef unsigned short ushort;
typedef unsigned char byte;

extern char  DAT_00732e8f;
extern int   DAT_005f1d44;
extern int   DAT_006db7bc;
extern int   DAT_00525518;
extern int   DAT_006da4fc;
extern int   DAT_0070acc8;
extern byte  DAT_00525548;
extern int   DAT_0052ae50;
extern byte  DAT_006da540[];
extern byte  DAT_006da541[];
extern byte  DAT_006da542[];
extern ushort DAT_006d4984;
extern ushort DAT_007091ae;
extern int   DAT_006a888e; /* unaligned at 0x006a888e, read via memcpy */

void __stdcall FUN_004a6a80(void);
void __stdcall FUN_004a6f90(void);

void __stdcall FUN_00478040(void)
{
    short *psVar1;
    int iVar2;
    byte *pbVar3;
    int iVar4;
    uint uVar5;
    uint uVar6;
    int iVar7;
    int *piVar8;
    short sVar9;
    int iVar10;
    int iVar11;
    short *psVar12;
    int iStack_4;
    int iBase;

    if (DAT_00732e8f == '\0') {
        FUN_004a6a80();
    }
    DAT_005f1d44 = (int)((uint)DAT_006db7bc * (uint)DAT_00525518 +
                         (uint)DAT_006da4fc * (uint)DAT_0070acc8);
    if (DAT_00525518 == 2) {
        uVar5 = (uint)DAT_00525548;
        sVar9 = (short)((ushort)((ushort)(DAT_006da540[uVar5 * 3] >> 1) * 0x20u +
                                 (ushort)(DAT_006da541[uVar5 * 3] >> 1)) * 0x40u +
                        (ushort)(DAT_006da542[uVar5 * 3] >> 1));
        iVar7 = 0;
        iStack_4 = 0;
        if (DAT_006d4984 != 0) {
            do {
                iVar10 = 0;
                __builtin_memcpy(&iBase, &DAT_006a888e, 4);
                piVar8 = *(int **)(iBase + iVar7);
LAB_004780e5:
                if (piVar8 != (int *)0) {
                    psVar12 = (short *)((uint)DAT_005f1d44 + (uint)DAT_0052ae50 + (uint)iVar10 * 2u);
                    psVar1 = (short *)((uint)DAT_005f1d44 + (uint)DAT_0052ae50 + (uint)piVar8[1] * 2u);
                    if ((uint)psVar12 < (uint)psVar1) {
                        uVar5 = (uint)psVar1 + (1u - (uint)psVar12);
                        for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
                            *(uint *)psVar12 = ((uint)(ushort)sVar9 << 16) | (uint)(ushort)sVar9;
                            psVar12 = psVar12 + 2;
                        }
                        for (uVar5 = (uint)((uVar5 >> 1 & 1) != 0); uVar5 != 0; uVar5 = uVar5 - 1) {
                            *psVar12 = sVar9;
                            psVar12 = psVar12 + 1;
                        }
                    }
                    iVar10 = piVar8[2];
                    piVar8 = (int *)*piVar8;
                    if (piVar8 == (int *)0) goto LAB_00478145;
                    do {
                        if (iVar10 < piVar8[1]) break;
                        if (iVar10 < piVar8[2]) {
                            iVar10 = piVar8[2];
                        }
                        piVar8 = (int *)*piVar8;
                    } while (piVar8 != (int *)0);
                    goto LAB_004780e5;
                }
LAB_00478145:
                psVar12 = (short *)((uint)DAT_005f1d44 + (uint)DAT_0052ae50 + (uint)iVar10 * 2u);
                psVar1 = (short *)((uint)DAT_005f1d44 + (uint)DAT_0052ae50 + (uint)DAT_007091ae * 2u);
                if ((uint)psVar12 < (uint)psVar1) {
                    uVar5 = (uint)psVar1 + (1u - (uint)psVar12);
                    for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
                        *(uint *)psVar12 = ((uint)(ushort)sVar9 << 16) | (uint)(ushort)sVar9;
                        psVar12 = psVar12 + 2;
                    }
                    for (uVar5 = (uint)((uVar5 >> 1 & 1) != 0); uVar5 != 0; uVar5 = uVar5 - 1) {
                        *psVar12 = sVar9;
                        psVar12 = psVar12 + 1;
                    }
                }
                DAT_005f1d44 = (int)((uint)DAT_005f1d44 + (uint)DAT_0070acc8);
                iVar7 = iVar7 + 4;
                iStack_4 = iStack_4 + 1;
            } while (iStack_4 < (int)(uint)DAT_006d4984);
        }
    } else {
        iVar7 = 0;
        iVar10 = 0;
        if (DAT_006d4984 != 0) {
            do {
                iVar11 = 0;
                __builtin_memcpy(&iBase, &DAT_006a888e, 4);
                piVar8 = *(int **)(iBase + iVar7);
LAB_004781d3:
                if (piVar8 != (int *)0) {
                    iVar2 = (int)((uint)DAT_005f1d44 + (uint)DAT_0052ae50);
                    iVar4 = piVar8[1];
                    for (pbVar3 = (byte *)(iVar2 + iVar11); pbVar3 < (byte *)(iVar2 + iVar4);
                         pbVar3 = pbVar3 + 1) {
                        *pbVar3 = DAT_00525548;
                    }
                    iVar11 = piVar8[2];
                    piVar8 = (int *)*piVar8;
                    if (piVar8 == (int *)0) goto LAB_0047821f;
                    do {
                        if (iVar11 < piVar8[1]) break;
                        if (iVar11 < piVar8[2]) {
                            iVar11 = piVar8[2];
                        }
                        piVar8 = (int *)*piVar8;
                    } while (piVar8 != (int *)0);
                    goto LAB_004781d3;
                }
LAB_0047821f:
                iVar4 = (int)((uint)DAT_005f1d44 + (uint)DAT_0052ae50);
                uVar5 = (uint)DAT_007091ae;
                for (pbVar3 = (byte *)(iVar4 + iVar11); pbVar3 < (byte *)((uint)iVar4 + uVar5);
                     pbVar3 = pbVar3 + 1) {
                    *pbVar3 = DAT_00525548;
                }
                DAT_005f1d44 = (int)((uint)DAT_005f1d44 + (uint)DAT_0070acc8);
                iVar7 = iVar7 + 4;
                iVar10 = iVar10 + 1;
            } while (iVar10 < (int)(uint)DAT_006d4984);
        }
    }
    if (DAT_00732e8f == '\0') {
        FUN_004a6f90();
    }
    return;
}
