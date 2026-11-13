/* SPDX-License-Identifier: MIT */
/* z_bop 00470060 */
extern uint DAT_00527028;
extern uint DAT_0052702c;
extern uint DAT_00527024;
extern uint DAT_005599a8[];
extern uint DAT_009994f8;
extern uint DAT_0099f6e0;
extern int DAT_00999be8[];

void __stdcall FID_conflict__memcpy(void *dst, const void *src, uint len);
DWORD __stdcall timeGetTime(void);

uint __stdcall FUN_00470060(void)
{
    uint uVar1;
    DWORD DVar2;
    int iVar3;
    uint uVar4;
    uint uVar5;
    int *piVar6;

    iVar3 = 0;
    uVar1 = DAT_00527028;
    uVar5 = DAT_00527028;
    for (; DAT_0052702c > 0; DAT_0052702c--) {
        if (DAT_005599a8[uVar5 * 0x84] == 0 || (uVar4 = uVar1, DAT_009994f8 == DAT_005599a8[uVar5 * 0x84])) {
            uVar4 = uVar1 + 1;
            FID_conflict__memcpy(&DAT_005599a8[uVar1 * 0x84], &DAT_005599a8[uVar5 * 0x84], 0x210);
            if (uVar4 > 0x3ff) {
                uVar4 = 0;
            }
            iVar3++;
        }
        uVar5++;
        if (uVar5 > 0x3ff) {
            uVar5 = 0;
        }
        uVar1 = uVar4;
    }
    uVar5 = 0;
    DAT_00527024 = uVar1;
    DAT_0052702c = iVar3;
    if (DAT_0099f6e0 != 0) {
        piVar6 = &DAT_00999be8[0];
        do {
            if (*piVar6 != (int)DAT_009994f8) {
                piVar6[1] = piVar6[2];
                piVar6[3] = piVar6[4];
                piVar6[5] = piVar6[6];
                DVar2 = timeGetTime();
                piVar6[0x89] = DVar2;
            }
            piVar6 += 0x8e;
            uVar5++;
        } while (uVar5 < DAT_0099f6e0);
    }
    return 1;
}
