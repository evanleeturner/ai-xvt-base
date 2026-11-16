/* SPDX-License-Identifier: MIT */
/* z_xvt 0040b3d0 */
extern int DAT_0071ecf4;
extern unsigned char DAT_0071b730[];
extern unsigned char DAT_0071b75d[];
extern unsigned char DAT_0071b75f[];
extern int DAT_00525550;
extern char DAT_0070a189;
extern char DAT_0085e793;
extern char DAT_0085e79c;
extern unsigned char DAT_0085e79a;
extern int DAT_006d4988;
extern int DAT_006d4980;
extern unsigned char DAT_0073ba30[];
extern unsigned char DAT_0052686c[];

void FUN_0040acd0(void);
void FUN_0040ae30(void);
void FUN_0040b190(void);
unsigned int FUN_00439830(int, int);
void FUN_004398b0(int);
void FUN_004396b0(int, int, unsigned int);
void FUN_004a1d70(int, int);
void FUN_00490a50(char *, int, int, unsigned int, int, int);
int FUN_00412810(int, int, int);
void FUN_0040a1a0(unsigned int, unsigned int, int);

void __stdcall FUN_0040b3d0(void)
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
    unsigned short tmp16;

    __builtin_memcpy(&uVar3, &DAT_0071b730[(unsigned int)DAT_0071ecf4 * 0x5bdU], 4);
    if (uVar3 != 0xffffffffU) {
        FUN_0040acd0();
        FUN_0040ae30();
        FUN_0040b190();
        if ((((DAT_00525550 == 0) && (DAT_0070a189 != '\0')) && (DAT_0085e793 != '\0')) &&
            (DAT_0085e79c != '\0')) {
            __builtin_memcpy(&uVar9, &DAT_0071b75d[(unsigned int)DAT_0071ecf4 * 0x5bdU], 2);
            if (uVar9 == 0xffff) {
                uVar4 = FUN_00439830(0x4e, 0x100);
                if (uVar4 != 0) {
                    FUN_004398b0(0x4e);
                }
                uVar4 = FUN_00439830(0x4f, 0x100);
                if (uVar4 != 0) {
                    FUN_004398b0(0x4f);
                }
            }
            else {
                cVar1 = *(char *)((unsigned char *)(unsigned int)((unsigned int)uVar9 * 0x23U + 3U + (unsigned int)DAT_006d4988));
                if (((cVar1 == '3') || (cVar1 == '4')) || ((cVar1 == '5' || (iVar10 = 0x4f, cVar1 == '6')))) {
                    iVar10 = 0x4e;
                }
                __builtin_memcpy(&tmp16, &DAT_0071b75f[(unsigned int)DAT_0071ecf4 * 0x5bdU], 2);
                uVar4 = (((unsigned int)tmp16 * (unsigned int)DAT_0085e79a) * 4U) / 10U;
                if (0x7f < uVar4) {
                    uVar4 = 0x7f;
                }
                uVar5 = FUN_00439830(iVar10, 0x100);
                if (uVar5 == 0) {
                    FUN_004a1d70(0xdc, DAT_0071ecf4);
                    FUN_00490a50((char *)(DAT_0073ba30 + (unsigned int)iVar10 * 6U), 1, 1, uVar4, 0x41, 0x40);
                }
                else {
                    FUN_004396b0(iVar10, 0x600, uVar4);
                }
            }
            uVar4 = 0;
            if (0 < DAT_006d4980) {
                iVar10 = 0;
                do {
                    if (uVar3 != uVar4) {
                        unsigned char *pEnt;
                        unsigned char *pObj;
                        unsigned char *pSub;
                        unsigned int uTmp;
                        short s1, s2;
                        char c7;

                        iVar7 = (int)((unsigned int)DAT_006d4988 + (unsigned int)iVar10);
                        pEnt = (unsigned char *)(unsigned int)iVar7;
                        bVar2 = pEnt[3];
                        if (bVar2 != 0) {
                            __builtin_memcpy(&uTmp, pEnt + 0x1f, 4);
                            pObj = (unsigned char *)uTmp;
                            __builtin_memcpy(&uTmp, pObj + 0xa7, 4);
                            pSub = (unsigned char *)uTmp;
                            c7 = (char)pSub[7];
                            __builtin_memcpy(&s1, pSub + 0x1df, 2);
                            __builtin_memcpy(&s2, pObj + 0x79, 2);
                            if (((c7 == '\0') && (s1 != 0)) && (s2 != 0)) {
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
                                    unsigned char *pOther;
                                    int a0, a1, a2, b0, b1, b2;
                                    unsigned int uO;
                                    unsigned char *pOtherObj;
                                    unsigned char *pObjCur;
                                    unsigned int uC;

                                    iVar8 = (int)((unsigned int)DAT_006d4988 + uVar3 * 0x23U);
                                    pOther = (unsigned char *)(unsigned int)iVar8;
                                    __builtin_memcpy(&a0, pEnt + 4, 4);
                                    __builtin_memcpy(&b0, pOther + 4, 4);
                                    __builtin_memcpy(&a1, pEnt + 8, 4);
                                    __builtin_memcpy(&b1, pOther + 8, 4);
                                    __builtin_memcpy(&a2, pEnt + 0xc, 4);
                                    __builtin_memcpy(&b2, pOther + 0xc, 4);
                                    iVar6 = FUN_00412810((int)((unsigned int)a0 - (unsigned int)b0),
                                                         (int)((unsigned int)a1 - (unsigned int)b1),
                                                         (int)((unsigned int)a2 - (unsigned int)b2));
                                    __builtin_memcpy(&uC, (unsigned char *)(unsigned int)((unsigned int)DAT_006d4988 + 0x1fU + (unsigned int)iVar10), 4);
                                    pObjCur = (unsigned char *)uC;
                                    __builtin_memcpy(&uO, (unsigned char *)(unsigned int)((unsigned int)DAT_006d4988 + 0x1fU + uVar3 * 0x23U), 4);
                                    pOtherObj = (unsigned char *)uO;
                                    __builtin_memcpy(&a0, pObjCur + 6, 4);
                                    __builtin_memcpy(&b0, pOtherObj + 6, 4);
                                    __builtin_memcpy(&a1, pObjCur + 10, 4);
                                    __builtin_memcpy(&b1, pOtherObj + 10, 4);
                                    __builtin_memcpy(&a2, pObjCur + 0xe, 4);
                                    __builtin_memcpy(&b2, pOtherObj + 0xe, 4);
                                    iVar7 = FUN_00412810((int)((unsigned int)a0 - (unsigned int)b0),
                                                         (int)((unsigned int)a1 - (unsigned int)b1),
                                                         (int)((unsigned int)a2 - (unsigned int)b2));
                                    if ((iVar6 < (int)(*(unsigned short *)(DAT_0052686c + (unsigned int)bVar2 * 0x16U) + 0x400)) &&
                                        ((int)(*(unsigned short *)(DAT_0052686c + (unsigned int)bVar2 * 0x16U) + 0x400) <= iVar7)) {
                                        FUN_0040a1a0((unsigned int)uVar9, uVar4, DAT_0071ecf4);
                                    }
                                }
                            }
                        }
                    }
                    iVar10 = iVar10 + 0x23;
                    uVar4 = uVar4 + 1;
                } while ((int)uVar4 < DAT_006d4980);
            }
        }
    }
    return;
}
