/* SPDX-License-Identifier: MIT */
/* z_xvt 0045b990 */
extern int DAT_0071bb1b;
extern int DAT_0071bb1f;
extern int DAT_0071bb23;
extern int DAT_0071ecf4;
extern int DAT_0072f448;
extern int DAT_0072f44c;
extern int DAT_0072f450;
extern int DAT_006db7ac;
extern int DAT_006db7b0;
extern int DAT_006db7b4;

extern int FUN_0040ca60(int, int, int);
extern int FUN_0040ca00(int, int, int);
extern int FUN_0040ca30(int, int, int);

uint __cdecl FUN_0045b990(int param_1, int param_2, int param_3, uint param_4)
{
    int iVar1;
    int iVar2;
    uint uVar3;
    int v1, v2, v3;

    __builtin_memcpy(&v1, (char *)&DAT_0071bb1b + DAT_0071ecf4 * 0x5bd, 4);
    __builtin_memcpy(&v2, (char *)&DAT_0071bb1f + DAT_0071ecf4 * 0x5bd, 4);
    __builtin_memcpy(&v3, (char *)&DAT_0071bb23 + DAT_0071ecf4 * 0x5bd, 4);
    DAT_0072f44c = param_1 - v1;
    DAT_0072f450 = param_2 - v2;
    DAT_0072f448 = param_3 - v3;
    DAT_006db7b4 = FUN_0040ca60(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
    uVar3 = param_4 & 0xffff;
    iVar1 = uVar3 + DAT_006db7b4;
    if (iVar1 < 0) {
        return DAT_006db7b4 & 0xffff0000;
    }
    if ((int)uVar3 < iVar1 >> 8) {
        return iVar1 >> 8 & 0xffff0000;
    }
    DAT_006db7ac = FUN_0040ca00(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
    iVar2 = DAT_006db7ac;
    if (DAT_006db7ac < 0) {
        iVar2 = -DAT_006db7ac;
    }
    if (iVar1 < (int)(iVar2 - uVar3)) {
        return iVar2 - uVar3 & 0xffff0000;
    }
    iVar2 = FUN_0040ca30(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
    DAT_006db7b0 = iVar2;
    if (iVar2 < 0) {
        iVar2 = -iVar2;
    }
    return (uint)((int)(iVar2 - uVar3) <= iVar1);
}
