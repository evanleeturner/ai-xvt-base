/* SPDX-License-Identifier: MIT */
/* z_bop 0042f810 */
extern int DAT_009ecc34;
extern unsigned char DAT_009e9670[];
extern int DAT_00523410;
extern char DAT_009d80c9;
extern char DAT_00bb7373;
extern char DAT_00bb737c;
extern unsigned char DAT_00bb737a;
extern int DAT_009a1fe8;
extern int DAT_009a1fe0;
extern char DAT_00a0adf0[];
extern unsigned char DAT_00521f1c[];

void __stdcall FUN_0042f110(void);
void __stdcall FUN_0042f270(void);
void __stdcall FUN_0042f5d0(void);
unsigned int __cdecl FUN_004a9300(int param_1, int param_2);
void __cdecl FUN_004a9380(int param_1);
void __cdecl FUN_004a9180(int param_1, int param_2, unsigned int param_3);
void __cdecl FUN_00450650(int param_1, int param_2);
void __cdecl FUN_0042d120(char *param_1, int param_2, int param_3, unsigned int param_4, int param_5, int param_6);
int __cdecl FUN_0041f410(int param_1, int param_2, int param_3);
unsigned int __cdecl FUN_0042e4d0(unsigned int param_1, unsigned int param_2, int param_3);

void __stdcall FUN_0042f810(void)
{
    char cVar1;
    unsigned char bVar2;
    unsigned int uVar3;
    unsigned int uVar4;
    unsigned int uVar5;
    int iVar6;
    int iVar7;
    int iVar8;
    unsigned short uVar9;
    int iVar10;
    unsigned char *pbPlayer;
    unsigned char *pbEntry;
    unsigned char *pbOther;
    unsigned int uA;
    unsigned int uB;
    unsigned int uC;
    unsigned int uD;
    unsigned int uE;
    unsigned int uF;
    unsigned int uThresh;
    unsigned short uW;

    pbPlayer = DAT_009e9670 + (unsigned int)DAT_009ecc34 * 0x5bd;
    __builtin_memcpy(&uVar3, pbPlayer, 4);
    if (uVar3 != 0xffffffff) {
        FUN_0042f110();
        FUN_0042f270();
        FUN_0042f5d0();
        if ((((DAT_00523410 == 0) && (DAT_009d80c9 != '\0')) && (DAT_00bb7373 != '\0')) &&
            (DAT_00bb737c != '\0')) {
            __builtin_memcpy(&uVar9, pbPlayer + 0x2d, 2);
            if (uVar9 == 0xffff) {
                uVar4 = FUN_004a9300(0x4e, 0x100);
                if (uVar4 != 0) {
                    FUN_004a9380(0x4e);
                }
                uVar4 = FUN_004a9300(0x4f, 0x100);
                if (uVar4 != 0) {
                    FUN_004a9380(0x4f);
                }
            }
            else {
                cVar1 = *(char *)((unsigned int)uVar9 * 0x23 + 3 + DAT_009a1fe8);
                if (((cVar1 == '3') || (cVar1 == '4')) || ((cVar1 == '5' || (iVar10 = 0x4f, cVar1 == '6')))) {
                    iVar10 = 0x4e;
                }
                __builtin_memcpy(&uW, pbPlayer + 0x2f, 2);
                uVar4 = (((unsigned int)uW * (unsigned int)DAT_00bb737a) * 4) / 10;
                if (0x7f < uVar4) {
                    uVar4 = 0x7f;
                }
                uVar5 = FUN_004a9300(iVar10, 0x100);
                if (uVar5 == 0) {
                    FUN_00450650(0xdd, DAT_009ecc34);
                    FUN_0042d120(DAT_00a0adf0 + iVar10 * 6, 1, 1, uVar4, 0x41, 0x40);
                }
                else {
                    FUN_004a9180(iVar10, 0x600, uVar4);
                }
            }
            uVar4 = 0;
            if (0 < DAT_009a1fe0) {
                iVar10 = 0;
                do {
                    if (uVar3 != uVar4) {
                        pbEntry = (unsigned char *)(DAT_009a1fe8 + iVar10);
                        bVar2 = pbEntry[3];
                        if (bVar2 != 0) {
                            __builtin_memcpy(&uA, pbEntry + 0x1f, 4);
                            __builtin_memcpy(&uB, (unsigned char *)(uA + 0xa9), 4);
                            __builtin_memcpy(&uW, (unsigned char *)(uB + 0x1e5), 2);
                            __builtin_memcpy(&uVar9, (unsigned char *)(uA + 0x79), 2);
                            if (((*(char *)(uB + 7) == '\0') && (uW != 0)) && (uVar9 != 0)) {
                                uVar9 = 0xffff;
                                switch ((unsigned int)bVar2) {
                                case 1:
                                case 4:
                                case 0xe:
                                case 0xf:
                                    uVar9 = 0x4a;
                                    break;
                                case 2:
                                    uVar9 = 0x4b;
                                    break;
                                case 3:
                                case 0xd:
                                    uVar9 = 0x4c;
                                    break;
                                case 5:
                                case 6:
                                case 7:
                                case 8:
                                case 9:
                                    uVar9 = 0x48;
                                    break;
                                case 0xc:
                                case 0x10:
                                    uVar9 = 0x49;
                                    break;
                                case 0x11:
                                case 0x12:
                                case 0x13:
                                case 0x14:
                                case 0x15:
                                case 0x16:
                                case 0x17:
                                case 0x18:
                                case 0x19:
                                case 0x1e:
                                    uVar9 = 0x49;
                                    break;
                                case 0x26:
                                case 0x27:
                                    uVar9 = 0x4d;
                                }
                                if (uVar9 != 0xffff) {
                                    pbOther = (unsigned char *)(DAT_009a1fe8 + uVar3 * 0x23);
                                    __builtin_memcpy(&uA, pbEntry + 4, 4);
                                    __builtin_memcpy(&uB, pbOther + 4, 4);
                                    __builtin_memcpy(&uC, pbEntry + 8, 4);
                                    __builtin_memcpy(&uD, pbOther + 8, 4);
                                    __builtin_memcpy(&uE, pbEntry + 0xc, 4);
                                    __builtin_memcpy(&uF, pbOther + 0xc, 4);
                                    iVar6 = FUN_0041f410((int)(uA - uB), (int)(uC - uD), (int)(uE - uF));

                                    __builtin_memcpy(&uA, (unsigned char *)(DAT_009a1fe8 + 0x1f + iVar10), 4);
                                    __builtin_memcpy(&uB, (unsigned char *)(DAT_009a1fe8 + 0x1f + uVar3 * 0x23), 4);
                                    iVar7 = (int)uA;
                                    iVar8 = (int)uB;
                                    __builtin_memcpy(&uA, (unsigned char *)(iVar7 + 6), 4);
                                    __builtin_memcpy(&uB, (unsigned char *)(iVar8 + 6), 4);
                                    __builtin_memcpy(&uC, (unsigned char *)(iVar7 + 10), 4);
                                    __builtin_memcpy(&uD, (unsigned char *)(iVar8 + 10), 4);
                                    __builtin_memcpy(&uE, (unsigned char *)(iVar7 + 0xe), 4);
                                    __builtin_memcpy(&uF, (unsigned char *)(iVar8 + 0xe), 4);
                                    iVar7 = FUN_0041f410((int)(uA - uB), (int)(uC - uD), (int)(uE - uF));

                                    __builtin_memcpy(&uThresh, DAT_00521f1c + (unsigned int)bVar2 * 0x1a, 4);
                                    uThresh = uThresh + 0x400;
                                    if ((iVar6 < (int)uThresh) && ((int)uThresh <= iVar7)) {
                                        FUN_0042e4d0((unsigned int)uVar9, uVar4, DAT_009ecc34);
                                    }
                                }
                            }
                        }
                    }
                    iVar10 = (int)((unsigned int)iVar10 + 0x23);
                    uVar4 = uVar4 + 1;
                } while ((int)uVar4 < DAT_009a1fe0);
            }
        }
    }
    return;
}
