/* SPDX-License-Identifier: MIT */
/* z_xvt 004b2880 */
extern short DAT_006559b0[];
extern unsigned short DAT_00655184;
extern unsigned char DAT_00656d6c[];
extern unsigned char DAT_00656d68[];
extern unsigned char DAT_00656d4a[];
extern unsigned char DAT_00656d6a[];
extern unsigned char DAT_00656d66[];
extern unsigned char DAT_00656d64[];

void __stdcall FUN_00457700(void);
void __stdcall FUN_0044ff80(void);
void __stdcall FUN_00436840(void);
unsigned int __stdcall FUN_004825d0(void);
/* Callee ends with RET 0x0: caller-cleanup, so __cdecl. Ghidra's extra ESI/EDI args are unaffected registers. */
void __cdecl FUN_004588c0(int param_1);
void __stdcall FUN_00459fa0(void);

void __stdcall FUN_004b2880(void)
{
    short sVar1;
    unsigned int uVar2;
    unsigned short uVar3;
    unsigned int uVar4;

    uVar4 = 0;
    do {
        sVar1 = DAT_006559b0[uVar4];
        switch (uVar4) {
        case 0:
            if (sVar1 == -2) {
                FUN_00457700();
                *(short *)(DAT_00656d6c + (unsigned int)DAT_00655184 * 2) = (short)0xfffe;
                goto default_label;
            }
            break;
        case 1:
            if (sVar1 == -2) {
                FUN_0044ff80();
                *(short *)(DAT_00656d68 + (unsigned int)DAT_00655184 * 2) = (short)0xfffe;
                goto default_label;
            }
            break;
        case 2:
            if (sVar1 == -2) {
                FUN_00436840();
                *(short *)(DAT_00656d4a + (unsigned int)DAT_00655184 * 2) = (short)0xfffe;
                goto default_label;
            }
            break;
        case 3:
            if (sVar1 == -2) {
                FUN_004825d0();
                *(short *)(DAT_00656d6a + (unsigned int)DAT_00655184 * 2) = (short)0xfffe;
                goto default_label;
            }
            break;
        case 4:
            if (sVar1 == -2) {
                FUN_004588c0(1);
            store_66_a:
                *(short *)(DAT_00656d66 + (unsigned int)DAT_00655184 * 2) = sVar1;
                goto default_label;
            }
            break;
        case 5:
            if (sVar1 == -2) {
                FUN_004588c0(0);
                *(short *)(DAT_00656d64 + (unsigned int)DAT_00655184 * 2) = (short)0xfffe;
                goto default_label;
            }
            break;
        case 6:
            if (sVar1 == -2) {
                FUN_00459fa0();
                goto store_66_a;
            }
            break;
        default:
        default_label:
            if (sVar1 == -2) {
                DAT_006559b0[uVar4] = 0;
            }
        }
        uVar3 = (unsigned short)((unsigned short)uVar4 + 1);
        uVar4 = (unsigned int)uVar3;
    } while (uVar3 < 8);

    uVar3 = 0;
    do {
        sVar1 = DAT_006559b0[uVar3];
        switch ((unsigned int)uVar3) {
        case 0:
            if (sVar1 != 0) {
                FUN_00457700();
                *(short *)(DAT_00656d6c + (unsigned int)DAT_00655184 * 2) = sVar1;
            }
            break;
        case 1:
            if (sVar1 != 0) {
                FUN_0044ff80();
                *(short *)(DAT_00656d68 + (unsigned int)DAT_00655184 * 2) = sVar1;
            }
            break;
        case 2:
            if (sVar1 != 0) {
                FUN_00436840();
                *(short *)(DAT_00656d4a + (unsigned int)DAT_00655184 * 2) = sVar1;
            }
            break;
        case 3:
            if (sVar1 != 0) {
                uVar2 = FUN_004825d0();
                if ((short)uVar2 != 0) {
                    *(short *)(DAT_00656d6a + (unsigned int)DAT_00655184 * 2) = sVar1;
                }
            }
            break;
        case 4:
            if (sVar1 != 0) {
                FUN_004588c0(1);
            store_66_b:
                *(short *)(DAT_00656d66 + (unsigned int)DAT_00655184 * 2) = sVar1;
            }
            break;
        case 5:
            if (sVar1 != 0) {
                FUN_004588c0(0);
                *(short *)(DAT_00656d64 + (unsigned int)DAT_00655184 * 2) = sVar1;
            }
            break;
        case 6:
            if (sVar1 != 0) {
                FUN_00459fa0();
                goto store_66_b;
            }
        }
        uVar3 = (unsigned short)(uVar3 + 1);
    } while (uVar3 <= 7);
}
