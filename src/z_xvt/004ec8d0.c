/* SPDX-License-Identifier: MIT */
/* z_xvt 004ec8d0 */
extern int DAT_00757c84;
extern int DAT_00533540;
extern int DAT_00743daf;
extern int DAT_00757cb0;
extern int DAT_007438d5;
extern int DAT_007579d0;
extern int DAT_00757870;
extern int DAT_00757cee;
extern int DAT_00757680;
extern int DAT_00757cfe;

int FUN_004ec8d0(int param_1)
{
    int *piVar1;
    int iVar2;
    int iVar3;
    int *piVar4;
    int *local_8;
    int local_4;
    
    if (param_1 == -1) {
        if (DAT_00757c84 != 0) {
            return *(int *)((char *)&DAT_00533540 + (unsigned int)(unsigned char)(&DAT_00743daf)[DAT_00757c84 + DAT_00757cb0 * 0x562] * 4);
        }
        if ((&DAT_007438d5)[DAT_00757cb0 * 0x562] != 0) {
            return *(int *)((char *)&DAT_00533540 + (unsigned int)(unsigned char)(&DAT_007438d5)[DAT_00757cb0 * 0x562] * 4);
        }
        return 0;
    }
    iVar2 = 0;
    piVar4 = &DAT_007579d0;
    local_8 = &DAT_00757870;
    do {
        iVar3 = 0;
        if (0 < *piVar4) {
            piVar1 = local_8;
            do {
                if (*piVar1 == *(int *)((int)&DAT_00757cee + param_1 * 0x2a)) {
                    local_4 = *(int *)((char *)&DAT_00757680 + (iVar3 + iVar2) * 4);
                    break;
                }
                piVar1 = piVar1 + 1;
                iVar3 = iVar3 + 1;
            } while (iVar3 < *piVar4);
        }
        iVar2 = iVar2 + 8;
        local_8 = local_8 + 8;
        piVar4 = piVar4 + 1;
        if ((int *)0x7579f7 < piVar4) {
            iVar2 = *(int *)((int)&DAT_00757cfe + param_1 * 0x2a);
            if (iVar2 != 0) {
                return *(int *)((char *)&DAT_00533540 + (unsigned int)(unsigned char)(&DAT_00743daf)[iVar2 + local_4 * 0x562] * 4);
            }
            if ((&DAT_007438d5)[local_4 * 0x562] != 0) {
                return *(int *)((char *)&DAT_00533540 + (unsigned int)(unsigned char)(&DAT_007438d5)[local_4 * 0x562] * 4);
            }
            return 0;
        }
    } while (1);
}
