/* SPDX-License-Identifier: MIT */
/* z_bop 0042e9a0 */
extern int DAT_009a1fe8;
extern int DAT_009ecc34;
extern int DAT_009fe7f4;
extern int DAT_00a00480;
extern int DAT_00a00488;
extern int DAT_00a00490;
extern int DAT_00a004a4;
extern int DAT_00a004a8;
extern int DAT_00a004b0;
extern int DAT_00a004b8;
extern int DAT_00a004cc;
extern int DAT_00a07cd4;
extern int DAT_00a07cd8;
extern int DAT_00a07cdc;
extern unsigned char DAT_009e9a5b;
extern unsigned char DAT_009e9a5f;
extern unsigned char DAT_009e9a63;

void __cdecl FUN_00458e20(unsigned int param_1, int param_2);
unsigned int __cdecl FUN_0046abf0(int param_1, int param_2);

int __cdecl FUN_0042e9a0(unsigned int param_1, int *param_2)
{
    int iVar1;
    int iVar2;
    unsigned int uVar3;
    short sVar4;
    short sVar5;
    int iVar6;
    short sVar7;
    int iVar8;
    short sStack_50;
    short sStack_48;
    short sStack_44;
    int iTmpA;
    int iTmpB;
    int iTmpC;

    if (param_1 == 0xffffffff) {
        return 0x40;
    }
    __builtin_memcpy(&iVar1,
                     (unsigned char *)(param_1 * 0x23u + 0x1fu + (unsigned int)DAT_009a1fe8),
                     4);
    if (iVar1 == 0) {
        FUN_00458e20(param_1, 0);
        __builtin_memcpy(&iTmpA, &DAT_009e9a63 + (unsigned int)DAT_009ecc34 * 0x5bdu, 4);
        sVar7 = (short)((int)(short)DAT_00a07cd4 - (int)(short)iTmpA);
        __builtin_memcpy(&iTmpB, &DAT_009e9a5b + (unsigned int)DAT_009ecc34 * 0x5bdu, 4);
        sVar5 = (short)((int)(short)DAT_00a07cdc - (int)(short)iTmpB);
        __builtin_memcpy(&iTmpC, &DAT_009e9a5f + (unsigned int)DAT_009ecc34 * 0x5bdu, 4);
        sVar4 = (short)((int)(short)DAT_00a07cd8 - (int)(short)iTmpC);
    }
    else {
        __builtin_memcpy(&iTmpA, (unsigned char *)iVar1 + 6, 4);
        __builtin_memcpy(&iTmpB, &DAT_009e9a5b + (unsigned int)DAT_009ecc34 * 0x5bdu, 4);
        sVar5 = (short)((int)(short)iTmpA - (int)(short)iTmpB);
        __builtin_memcpy(&iTmpA, (unsigned char *)iVar1 + 10, 4);
        __builtin_memcpy(&iTmpB, &DAT_009e9a5f + (unsigned int)DAT_009ecc34 * 0x5bdu, 4);
        sVar4 = (short)((int)(short)iTmpA - (int)(short)iTmpB);
        __builtin_memcpy(&iTmpA, (unsigned char *)iVar1 + 0xe, 4);
        __builtin_memcpy(&iTmpB, &DAT_009e9a63 + (unsigned int)DAT_009ecc34 * 0x5bdu, 4);
        sVar7 = (short)((int)(short)iTmpA - (int)(short)iTmpB);
    }
    iVar1 = (int)sVar7;
    iVar6 = (int)sVar4;
    iVar8 = (int)sVar5;
    iVar2 = (int)((unsigned int)iVar8 * (unsigned int)DAT_00a004a8 +
                  (unsigned int)iVar6 * (unsigned int)DAT_00a004cc +
                  (unsigned int)iVar1 * (unsigned int)DAT_00a00488);
    if (0x3fffffff < iVar2) {
        iVar2 = 0x3fffffff;
    }
    if (iVar2 < -0x3fffffff) {
        iVar2 = -0x3fff0000;
    }
    sStack_44 = (short)(iVar2 >> 0xf);
    iVar2 = (int)((unsigned int)iVar8 * (unsigned int)DAT_00a004b0 +
                  (unsigned int)iVar6 * (unsigned int)DAT_009fe7f4 +
                  (unsigned int)iVar1 * (unsigned int)DAT_00a00490);
    if (0x3fffffff < iVar2) {
        iVar2 = 0x3fffffff;
    }
    if (iVar2 < -0x3fffffff) {
        iVar2 = -0x3fff0000;
    }
    sStack_48 = (short)(iVar2 >> 0xf);
    uVar3 = FUN_0046abf0((int)sStack_44, (int)sStack_48);
    sVar4 = (short)uVar3;
    if ((0x3fff < sVar4) || (sVar4 < -0x3fff)) {
        iVar1 = (int)((unsigned int)iVar8 * (unsigned int)DAT_00a004a4 +
                      (unsigned int)iVar6 * (unsigned int)DAT_00a004b8 +
                      (unsigned int)iVar1 * (unsigned int)DAT_00a00480);
        if (0x3fffffff < iVar1) {
            iVar1 = 0x3fffffff;
        }
        if (iVar1 < -0x3fffffff) {
            iVar1 = -0x3fff0000;
        }
        sStack_50 = (short)(iVar1 >> 0xf);
        uVar3 = FUN_0046abf0((int)sStack_50, (int)sStack_48);
        sVar5 = (short)(-0x8000 - (int)(short)uVar3);
        sVar4 = (short)(-0x8000 - (int)sVar4);
        if (sVar5 < 0) {
            sVar5 = (short)(-(int)sVar5);
        }
        sVar7 = sVar4;
        if (sVar4 < 0) {
            sVar7 = (short)(-(int)sVar4);
        }
        iVar1 = (int)(short)(((short)(0x4000 - sVar5) >> 8) * ((short)(0x4000 - sVar7) >> 8));
        iVar1 = (int)(short)((short)((iVar1 + ((iVar1 >> 0x1f) & 0x3f)) >> 6) * (short)*param_2);
        *param_2 = (int)((unsigned int)*param_2 -
                         (unsigned int)(int)(short)((iVar1 + ((iVar1 >> 0x1f) & 0x7f)) >> 7));
    }
    sVar4 = sVar4 >> 7;
    if (sVar4 < -0x40) {
        sVar4 = -0x40;
    }
    if (0x3f < sVar4) {
        sVar4 = 0x3f;
    }
    return (int)(short)(sVar4 + 0x40);
}
