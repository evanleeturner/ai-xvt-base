/* SPDX-License-Identifier: MIT */
/* z_bop 004b2020 */
extern char *DAT_00664c80;
extern void **DAT_00528c98;
extern int DAT_00528cc0;
extern void **DAT_00664548;
extern void *DAT_00664974;

extern char * __cdecl FUN_004b1600(int param_1, int *param_2, int param_3);
extern void __stdcall FUN_004079e0(void);

int __stdcall FUN_004b2020(void)
{
    int iVar1;
    int local_34[12];
    int i;

    *DAT_00664c80 = 0xb;
    DAT_00664c80[1] = 0;
    *(short *)(DAT_00664c80 + 2) = 0;
    DAT_00664c80 = DAT_00664c80 + 4;

    iVar1 = (*(int(**)(void**))((char *)*DAT_00528c98 + 0x14))(DAT_00528c98);
    if (iVar1 != 0) {
        FUN_004b1600(iVar1, &DAT_00528cc0, 0x79);
        FUN_004079e0();
        return 0;
    }

    for (i = 0; i < 12; i++) {
        local_34[i] = 0;
    }

    (*(void(**)(void**, int*))((char *)*DAT_00528c98 + 0x18))(DAT_00528c98, local_34);

    iVar1 = (*(int(**)(void**, void**, void**, int))((char *)*DAT_00664548 + 0x20))(DAT_00664548, DAT_00528c98, DAT_00664974, 2);
    if (iVar1 != 0) {
        FUN_004b1600(iVar1, &DAT_00528cc0, 0x79);
        FUN_004079e0();
        return 0;
    }

    return 1;
}
