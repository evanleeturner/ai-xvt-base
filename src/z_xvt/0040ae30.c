/* SPDX-License-Identifier: MIT */
/* z_xvt 0040ae30 */
typedef unsigned char  undefined1;

extern int            DAT_00525550;
extern char           DAT_0070a189;
extern char           DAT_0085e793;
extern unsigned char  DAT_0085e79c;
extern int            DAT_0071ecf4;
extern unsigned char  DAT_0071b730[];   /* packed, indexed by DAT_0071ecf4 * 0x5bd */
extern char           DAT_0071b741[];
extern char           DAT_0071b743[];
extern unsigned char *DAT_006d4988;
extern unsigned char  DAT_00536928;
extern char           DAT_0073ba30[];

unsigned int FUN_00439830(int a, int b);
void         FUN_004398b0(int a);
void         FUN_004396b0(int a, int b, int c);
unsigned int FUN_004783e0(unsigned short a, unsigned int b);
unsigned int FUN_00478320(unsigned int a, unsigned short b);
void         FUN_00490a50(char *a, int b, int c, int d, unsigned int e, int f);

void __stdcall FUN_0040ae30(void)
{
    int iVar1;
    unsigned int uVar2;
    unsigned char uVar3;
    unsigned short uVar4;
    unsigned int uVar5;
    unsigned short uVar6;
    int iVar7;
    int iVar8;
    int iStack_4 = 0;
    int iPtr;
    unsigned short usVal;

    if ((((DAT_00525550 == 0) && (DAT_0070a189 != '\0')) && (DAT_0085e793 != '\0')) &&
        (DAT_0085e79c != 0)) {
        iVar7 = -1;
        __builtin_memcpy(&iVar8, &DAT_0071b730[(unsigned int)DAT_0071ecf4 * 0x5bd], 4);
        if (iVar8 == -1) {
            uVar3 = (undefined1)iStack_4;
        }
        else {
            uVar3 = DAT_006d4988[(unsigned int)iVar8 * 0x23 + 3];
            switch (uVar3) {
            case 1:
            case 4:
            case 0xe:
            case 0xf:
                iVar7 = 0x43;
                iStack_4 = 11000;
                break;
            case 2:
                iVar7 = 0x44;
                iStack_4 = 0x157c;
                break;
            case 3:
            case 0xd:
                iVar7 = 0x45;
                iStack_4 = 0x157c;
                break;
            case 5:
            case 6:
            case 7:
            case 8:
            case 9:
                iVar7 = 0x46;
                iStack_4 = 0x157c;
                break;
            default:
                break;
            case 0xc:
            case 0x10:
                iVar7 = 0x47;
                iStack_4 = 11000;
            }
        }
        if (iVar7 == -1) {
            if (DAT_0071b743[(unsigned int)DAT_0071ecf4 * 0x5bd] != '\0') {
                iVar8 = -1;
                switch (DAT_00536928) {
                case 1:
                case 4:
                case 0xe:
                case 0xf:
                    iVar8 = 0x43;
                    break;
                case 2:
                    iVar8 = 0x44;
                    break;
                case 3:
                case 0xd:
                    iVar8 = 0x45;
                    break;
                case 5:
                case 6:
                case 7:
                case 8:
                case 9:
                    iVar8 = 0x46;
                    break;
                case 0xc:
                case 0x10:
                    iVar8 = 0x47;
                }
                uVar5 = FUN_00439830(iVar8, 0x100);
                if (uVar5 != 0) {
                    FUN_004398b0(iVar8);
                }
                uVar5 = FUN_00439830(0x4e, 0x100);
                if (uVar5 != 0) {
                    FUN_004398b0(0x4e);
                }
                uVar5 = FUN_00439830(0x4f, 0x100);
                if (uVar5 != 0) {
                    FUN_004398b0(0x4f);
                }
            }
        }
        else {
            DAT_00536928 = uVar3;
            if (DAT_0071b741[(unsigned int)DAT_0071ecf4 * 0x5bd] != '\x01') {
                __builtin_memcpy(&iPtr, &DAT_006d4988[(unsigned int)iVar8 * 0x23 + 0x1f], 4);
                __builtin_memcpy(&iVar1, (unsigned char *)iPtr + 0xa7, 4);
                if ((*(unsigned char *)(iVar1 + 0x1df) & 0x40) != 0) {
                    uVar4 = (unsigned short)DAT_0085e79c;
                    uVar6 = (unsigned short)((unsigned int)iVar8 >> 16);
                    __builtin_memcpy(&usVal, (unsigned char *)iVar1 + 0xbd, 2);
                    if (uVar4 < 10) {
                        uVar5 = ((unsigned int)uVar6 << 16) | (unsigned short)(uVar4 * 0xd);
                    }
                    else {
                        uVar5 = ((unsigned int)uVar6 << 16) | 0x7f;
                    }
                    uVar2 = FUN_004783e0(usVal, 0xffff);
                    iVar8 = (int)((unsigned int)iStack_4 +
                                  ((uVar2 & 0xffff) / 0x28f) * 0x37);
                    __builtin_memcpy(&usVal, (unsigned char *)iVar1 + 0xbd, 2);
                    uVar5 = FUN_00478320(uVar5, usVal);
                    uVar2 = FUN_00439830(iVar7, 0x100);
                    if (uVar2 == 0) {
                        FUN_004396b0(iVar7, 0x777, iVar8);
                        FUN_00490a50(&DAT_0073ba30[iVar7 * 6], 1, 1, 0x7d,
                                     uVar5 >> 1 & 0x7fff, 0x40);
                        return;
                    }
                    FUN_004396b0(iVar7, 0x777, iVar8);
                    return;
                }
            }
            uVar5 = FUN_00439830(iVar7, 0x100);
            if (uVar5 != 0) {
                FUN_004398b0(iVar7);
            }
            uVar5 = FUN_00439830(0x4e, 0x100);
            if (uVar5 != 0) {
                FUN_004398b0(0x4e);
            }
            uVar5 = FUN_00439830(0x4f, 0x100);
            if (uVar5 != 0) {
                FUN_004398b0(0x4f);
                return;
            }
        }
    }
    return;
}
