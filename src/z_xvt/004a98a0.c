/* SPDX-License-Identifier: MIT */
/* z_xvt 004a98a0 */
extern unsigned char DAT_0071b730[];
extern int DAT_0071ecf4;
extern int DAT_006d4988;
extern int DAT_006d4980;
extern int _DAT_00732a3c;
extern int DAT_0071b704;
extern int DAT_00732904;
extern int DAT_00732a38;
extern int DAT_00703c88;
extern unsigned char DAT_0052687a[];

extern unsigned short DAT_006551ca;
extern unsigned short DAT_00655812;
extern unsigned short DAT_00655154;
extern unsigned short DAT_00655a8e;
extern unsigned short DAT_00655a98;
extern unsigned short DAT_00655a96;
extern unsigned short DAT_00656c82;
extern unsigned short DAT_00656c84;
extern unsigned char DAT_00655174;
extern unsigned char DAT_0065518a;

extern void *DAT_00655170;
extern void *DAT_00655814;
extern void *DAT_006559c0;
extern void *DAT_00655150;
extern unsigned char DAT_006551d0[];
extern unsigned char DAT_006552f0[];
extern unsigned char DAT_00656f60[];
extern unsigned char DAT_00657080[];

extern void (*DAT_006d4990)(void);
extern void (*DAT_00703c64)(void *, unsigned int);
extern void (*DAT_006d4a48)(void *, unsigned int);
extern void (*DAT_0071e52c)(void);

unsigned int FUN_004440e0(unsigned short param_1);
void FUN_004a9bd0(unsigned short param_1);

void __stdcall FUN_004a98a0(void)
{
    unsigned char cVar1;
    unsigned short uVar2;
    unsigned int uVar3;
    unsigned int uVar5;
    int iVar6;
    int iVar7;
    int iVar8;
    unsigned char bVar9;
    unsigned char *pSlot;
    int iTmp;
    int iTmp2;
    unsigned int uTmp;
    unsigned short sTmp;

    pSlot = DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bdu;

    __builtin_memcpy(&uTmp, pSlot, 4);
    iTmp = (int)(uTmp * 0x23u + 0x1fu + (unsigned int)DAT_006d4988);
    __builtin_memcpy(&iTmp2, (void *)iTmp, 4);
    __builtin_memcpy(&iTmp, (void *)(iTmp2 + 0xa7), 4);
    __builtin_memcpy(&uVar2, (void *)(iTmp + 0x1db), 2);

    if (((uVar2 & 0x80) != 0) && ((uVar2 & 0x100) != 0)) {
        DAT_006551ca = DAT_00655812;
        DAT_00655154 = DAT_00655a8e;
        DAT_00655a8e = 0;
        DAT_00655812 = 0;
        DAT_00655a98 = DAT_00656c82;
        DAT_00655a96 = DAT_00656c84;
        if (DAT_00655174 == 0) {
            DAT_00655170 = DAT_006551d0;
            DAT_00655814 = DAT_006552f0;
            DAT_006559c0 = DAT_00656f60;
            DAT_00655150 = DAT_00657080;
        } else {
            DAT_00655170 = DAT_006552f0;
            DAT_00655814 = DAT_006551d0;
            DAT_006559c0 = DAT_00657080;
            DAT_00655150 = DAT_00656f60;
        }
        if (_DAT_00732a3c < DAT_006d4980) {
            iVar8 = (int)((unsigned int)_DAT_00732a3c * 0x23u);
            uVar5 = (unsigned int)_DAT_00732a3c;
            do {
                pSlot = DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bdu;
                __builtin_memcpy(&uTmp, pSlot, 4);
                if ((uTmp != uVar5) &&
                    ((DAT_0052687a[(unsigned int)*(unsigned char *)(DAT_006d4988 + iVar8 + 3) * 0x16] & 1) != 0)) {
                    __builtin_memcpy(&iTmp, (void *)(DAT_006d4988 + iVar8 + 0x1f), 4);
                    __builtin_memcpy(&iVar6, (void *)(iTmp + 0xa7), 4);
                    __builtin_memcpy(&sTmp, pSlot + 0x1a, 2);
                    if ((sTmp == uVar5) ||
                        (((uVar3 = FUN_004440e0((unsigned short)uVar5), (char)uVar3 == '\0') &&
                          (cVar1 = *(char *)(iVar6 + 7), cVar1 != '\x03')) && (cVar1 != '\x04'))) {
                        FUN_004a9bd0((unsigned short)uVar5);
                    }
                }
                iVar8 = (int)((unsigned int)iVar8 + 0x23u);
                uVar5 = uVar5 + 1u;
            } while ((int)uVar5 < DAT_006d4980);
        }
        if (DAT_0071b704 < DAT_00732904) {
            iVar8 = (int)((unsigned int)DAT_0071b704 * 0x23u);
            iVar6 = DAT_0071b704;
            do {
                if ((DAT_0052687a[(unsigned int)*(unsigned char *)(DAT_006d4988 + 3 + iVar8) * 0x16] & 1) != 0) {
                    FUN_004a9bd0((unsigned short)iVar6);
                }
                iVar8 = (int)((unsigned int)iVar8 + 0x23u);
                iVar6 = iVar6 + 1;
            } while (iVar6 < DAT_00732904);
        }
        iVar8 = (int)((unsigned int)DAT_00703c88 + (unsigned int)DAT_00732a38);
        if (DAT_00732a38 < iVar8) {
            iVar6 = (int)((unsigned int)DAT_00732a38 * 0x23u);
            iVar7 = DAT_00732a38;
            do {
                if ((DAT_0052687a[(unsigned int)*(unsigned char *)(DAT_006d4988 + 3 + iVar6) * 0x16] & 1) != 0) {
                    FUN_004a9bd0((unsigned short)iVar7);
                }
                iVar6 = (int)((unsigned int)iVar6 + 0x23u);
                iVar7 = iVar7 + 1;
                iVar8 = (int)((unsigned int)DAT_00703c88 + (unsigned int)DAT_00732a38);
            } while (iVar7 < iVar8);
        }
        if (DAT_0065518a != 0) {
            (*DAT_006d4990)();
        }
        /* High 16 bits of these arguments were leftover register garbage
           (extraout_var*) in the decompile; zero-extended here. */
        if (DAT_006551ca != 0) {
            (*DAT_00703c64)(DAT_00655170, (unsigned int)DAT_006551ca);
        }
        if (DAT_00655812 != 0) {
            (*DAT_006d4a48)(DAT_00655814, (unsigned int)DAT_00655812);
        }
        if (DAT_00655154 != 0) {
            (*DAT_00703c64)(DAT_006559c0, (unsigned int)DAT_00655154);
        }
        if (DAT_00655a8e != 0) {
            (*DAT_006d4a48)(DAT_00655150, (unsigned int)DAT_00655a8e);
        }
        pSlot = DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bdu;
        __builtin_memcpy(&sTmp, pSlot + 0x1a, 2);
        bVar9 = ((short)sTmp != -1);
        if (bVar9) {
            (*DAT_0071e52c)();
        }
        DAT_00655174 = DAT_00655174 ^ 1;
        DAT_0065518a = bVar9;
    }
    return;
}
