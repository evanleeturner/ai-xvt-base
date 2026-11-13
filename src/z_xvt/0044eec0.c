/* SPDX-License-Identifier: MIT */
/* z_xvt 0044eec0 */
extern ushort DAT_00732a3c;
extern int DAT_006d4980;
extern int DAT_006d4988;
extern char DAT_006d4820[];
extern char DAT_006d9d78;
extern uint unaff_ESI;

char __cdecl FUN_0049f010(uint, ushort, ushort);
char __cdecl FUN_00429a20(uint);

void __cdecl FUN_0044eec0(ushort param_1, ushort param_2, short param_3, ushort param_4, ushort param_5, uint param_6)
{
    uint uVar5;
    ushort uVar6;
    int iVar1;
    char bVar2;
    ushort uVar3;
    ushort uVar4;

    uVar5 = DAT_00732a3c | ((uint)(short)(unaff_ESI >> 16) << 16);
    if ((int)(uint)DAT_00732a3c < DAT_006d4980) {
        do {
            iVar1 = DAT_006d4988;
            uVar6 = (ushort)(uVar5 & 0xffff);
            DAT_006d4820[uVar6 * 2 + (param_6 & 0xffff)] = 0;
            if (*(char *)(iVar1 + 3 + uVar6 * 0x23) != '\0') {
                bVar2 = FUN_0049f010(uVar5, param_1, param_2);
                uVar4 = (ushort)bVar2;
                bVar2 = FUN_0049f010(uVar5, param_4, param_5);
                uVar3 = (ushort)bVar2;
                if (param_3 == 1) {
                    uVar4 = uVar4 | uVar3;
                } else {
                    uVar4 = uVar4 & uVar3;
                }
                if ((uVar4 != 0) &&
                   (DAT_006d9d78 == '\0' ||
                    (*(short *)(*(int *)(*(int *)(DAT_006d4988 + 0x1f + uVar6 * 0x23) + 0xa7) + 0x1df) != 0)
                   )) {
                    bVar2 = FUN_00429a20(uVar6);
                    if (bVar2 != 0) {
                        DAT_006d4820[uVar6 * 2 + (param_6 & 0xffff)] = 1;
                    }
                }
            }
            uVar4 = (short)uVar5 + 1;
            uVar5 = ((uint)(short)(uVar5 >> 16) << 16) | uVar4;
        } while ((int)(uint)uVar4 < DAT_006d4980);
    }
    return;
}
