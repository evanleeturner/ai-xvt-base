/* SPDX-License-Identifier: MIT */
/* z_xvt 004ee3d0 */
extern int DAT_00757cee;
extern int DAT_00757e3d;
extern int DAT_00757ce0;

unsigned int __stdcall FUN_004ee3d0(void)
{
    int *piVar4 = &DAT_00757cee;
    int iVar5 = 0;
    unsigned int uVar2;

    while (1) {
        if (*piVar4 == 0) {
            piVar4 = (int *)((int)piVar4 + 0x2a);
            iVar5++;
            if (&DAT_00757e3d < piVar4) {
                return 1;
            }
            continue;
        }
        uVar2 = iVar5 + 1;
        if (uVar2 < 8) {
            piVar4 = (int *)((int)piVar4 + 0x2a);
            iVar5++;
            if (&DAT_00757e3d < piVar4) {
                return 1;
            }
            continue;
        }

        int *piVar1 = (int *)((int)&DAT_00757cee + uVar2 * 0x2a);
        while (*piVar1 == 0) {
            piVar1 = (int *)((int)piVar1 + 0x2a);
            uVar2++;
            if (&DAT_00757e3d < piVar1) {
                piVar4 = (int *)((int)piVar4 + 0x2a);
                iVar5++;
                if (&DAT_00757e3d < piVar4) {
                    return 1;
                }
                continue;
            }
        }

        unsigned int *puVar6 = (unsigned int *)((int)&DAT_00757ce0 + uVar2 * 0x2a);
        unsigned int *puVar7 = (unsigned int *)((int)piVar4 - 0xe);
        for (int iVar3 = 10; iVar3 != 0; iVar3--) {
            *puVar7++ = *puVar6++;
        }
        *(unsigned short *)puVar7 = *(unsigned short *)puVar6;
        *(int *)((int)&DAT_00757cee + uVar2 * 0x2a) = 0;

        piVar4 = (int *)((int)piVar4 + 0x2a);
        iVar5++;
        if (&DAT_00757e3d < piVar4) {
            return 1;
        }
    }
}
