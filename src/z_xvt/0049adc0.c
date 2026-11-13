/* SPDX-License-Identifier: MIT */
/* z_xvt 0049adc0 */
extern int DAT_00525568;
extern int DAT_006db730;
extern char DAT_0071b740;
extern int DAT_0071ecf4;
extern char DAT_0071f71d[];
extern char DAT_0071f71e[];
extern char DAT_0071f716[];
extern char DAT_0071e528;

extern int * __cdecl FUN_0049b0d0(int, int, void *);

void __cdecl FUN_0049adc0(int param_1) {
    int iVar4 = 0;
    if (DAT_00525568 == 0) {
        int local_8 = 0;
        int local_4 = 0;
        int iVar6 = 0;
        int *piVar5 = &DAT_006db730;
        char *pcVar7 = &DAT_0071b740;
        do {
            if (*pcVar7 != '\0' && iVar6 != DAT_0071ecf4 && *piVar5 != 0) {
                int iVar2 = *piVar5 + iVar4;
                int iVar1 = iVar2 * 0x12;
                ((char *)&local_8)[0] = DAT_0071f71d[iVar1];
                ((char *)&local_4)[1] = DAT_0071f71e[iVar1];
                int val_f716;
                __builtin_memcpy(&val_f716, DAT_0071f716 + iVar1, 4);
                int *piVar3 = FUN_0049b0d0(iVar6, val_f716 + param_1, &local_8);
                if (piVar3 != NULL) {
                    *piVar3 = 0;
                    piVar3[1] = 2;
                }
            }
            iVar4 += 0x1c2;
            piVar5 += 1;
            pcVar7 += 0x5bd;
            iVar6 += 1;
        } while (pcVar7 < &DAT_0071e528);
    }
}
