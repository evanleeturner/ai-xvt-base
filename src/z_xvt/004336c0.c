/* SPDX-License-Identifier: MIT */
/* z_xvt 004336c0 */
extern uint DAT_005251b0;
extern int DAT_005251b4;
extern uint DAT_005251ac;
extern int DAT_007330a8;
extern uint DAT_00739290;
extern int DAT_00733798[];
extern int DAT_00539a08[];

extern void __stdcall FID_conflict__memcpy(void *dst, const void *src, uint len);
extern DWORD __stdcall timeGetTime(void);

uint __stdcall FUN_004336c0(void)
{
    uint uVar1;
    DWORD DVar2;
    int iVar3;
    uint uVar4;
    uint uVar5;
    int *piVar6;

    iVar3 = 0;
    uVar1 = DAT_005251b0;
    uVar5 = DAT_005251b0;
    for (; 0 < DAT_005251b4; DAT_005251b4 = DAT_005251b4 + -1) {
        if (DAT_00539a08[uVar5 * 0x84] == 0 || (uVar4 = uVar1, DAT_007330a8 == DAT_00539a08[uVar5 * 0x84])) {
            uVar4 = uVar1 + 1;
            FID_conflict__memcpy(&DAT_00539a08[uVar1 * 0x84], &DAT_00539a08[uVar5 * 0x84], 0x210);
            if (0x3ff < uVar4) {
                uVar4 = 0;
            }
            iVar3 = iVar3 + 1;
        }
        uVar5 = uVar5 + 1;
        if (0x3ff < uVar5) {
            uVar5 = 0;
        }
        uVar1 = uVar4;
    }
    uVar5 = 0;
    DAT_005251ac = uVar1;
    DAT_005251b4 = iVar3;
    if (DAT_00739290 != 0) {
        piVar6 = DAT_00733798;
        do {
            if (*piVar6 != DAT_007330a8) {
                piVar6[1] = piVar6[2];
                piVar6[3] = piVar6[4];
                piVar6[5] = piVar6[6];
                DVar2 = timeGetTime();
                piVar6[0x89] = DVar2;
            }
            piVar6 = piVar6 + 0x8e;
            uVar5 = uVar5 + 1;
        } while (uVar5 < DAT_00739290);
    }
    return 1;
}
