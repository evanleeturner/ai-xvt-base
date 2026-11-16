/* SPDX-License-Identifier: MIT */
/* z_xvt 004a7d20 */
extern unsigned char DAT_0070ac28;
extern unsigned int DAT_0071ecf4;
extern unsigned int DAT_0052552c;
extern unsigned int DAT_00525550; /* width unresolved: only compared with 0 */
extern unsigned short DAT_00655a94;
extern unsigned short DAT_00655a9c;
extern unsigned short DAT_00655aa0;
extern unsigned short DAT_00655a9e;
extern unsigned short DAT_00655a9a;
extern unsigned short DAT_00655aa2;
extern unsigned short DAT_00655414;
extern unsigned short DAT_0065517e;
extern unsigned short DAT_0065517a;
extern unsigned short DAT_00655a90;
extern unsigned short DAT_0065517c;
extern unsigned short DAT_006551cc;
extern unsigned short DAT_00655186;
extern unsigned short DAT_00655188;
extern unsigned short DAT_006551c8;
extern unsigned short DAT_00655a92;
extern unsigned short DAT_00655416;
extern unsigned short DAT_00655418;
extern unsigned short DAT_00655810;
extern unsigned short DAT_00655176;
extern char DAT_0071b743[];
extern unsigned short DAT_00655184;
extern unsigned char DAT_00655fb0[];
extern unsigned char DAT_00655fb2[];
extern unsigned short DAT_00656c90[];
extern unsigned short DAT_00655f8e;
extern unsigned short DAT_00655a84;
extern unsigned short DAT_00655812;
extern unsigned short DAT_0065518a;
extern unsigned short DAT_00655a8e;
extern unsigned short DAT_00655174;

void __cdecl FUN_004af9c0(void);
void __cdecl FUN_004afb90(void);
void __cdecl FUN_004a80d0(unsigned int param_1);
void __cdecl FUN_004a85e0(unsigned int param_1);

void __cdecl FUN_004a7d20(unsigned int param_1)
{
    unsigned short uVar1;
    unsigned int uVar2;

    DAT_0070ac28 = 1;
    if (param_1 == DAT_0071ecf4) {
        if (DAT_0052552c == 0x13) {
            DAT_00655a94 = 6;
            DAT_00655a9c = 0x8d;
            DAT_00655aa0 = 0x67;
            DAT_00655a9e = 100;
            DAT_00655a9a = 0x50;
            DAT_00655aa2 = 0x35;
            DAT_00655414 = 0x2f;
            DAT_0065517e = 2;
            DAT_0065517a = 0x6d;
            DAT_00655a90 = 0x35;
            DAT_0065517c = 0x46;
            DAT_006551cc = 0x79;
            DAT_00655186 = 0xba;
            DAT_00655188 = 0x6f;
            DAT_006551c8 = 0x34;
            DAT_00655a92 = 0x99;
            DAT_00655416 = 0x99;
            DAT_00655418 = 0x35;
            DAT_00655810 = 0x70;
            DAT_00655176 = 0x70;
        }
        else if (DAT_0052552c == 0x101) {
            DAT_00655a94 = 0xc;
            DAT_00655a9c = 0x11a;
            DAT_00655aa0 = 0xce;
            DAT_00655a9e = 200;
            DAT_00655a9a = 0xa0;
            DAT_00655aa2 = 0x6b;
            DAT_00655810 = 0xe1;
            DAT_00655414 = 0x5e;
            DAT_0065517e = 4;
            DAT_0065517a = 0xdb;
            DAT_00655176 = 0xe0;
            DAT_0065517c = 0x8c;
            DAT_006551cc = 0xf2;
            DAT_00655186 = 0x175;
            DAT_00655188 = 0xdf;
            DAT_006551c8 = 0x68;
            DAT_00655a90 = 0x6b;
            DAT_00655a92 = 0x132;
            DAT_00655416 = 0x132;
            DAT_00655418 = 0x6b;
        }
        else if (DAT_0052552c == 0x112) {
            DAT_00655a94 = 9;
            DAT_00655a9c = 0xd3;
            DAT_00655aa0 = 0x9a;
            DAT_00655a9e = 0x96;
            DAT_00655a9a = 0x78;
            DAT_00655aa2 = 0x50;
            DAT_00655414 = 0x46;
            DAT_0065517e = 3;
            DAT_0065517a = 0xa4;
            DAT_00655a90 = 0x50;
            DAT_0065517c = 0x69;
            DAT_006551cc = 0xb5;
            DAT_00655186 = 0x117;
            DAT_00655188 = 0xa7;
            DAT_006551c8 = 0x4e;
            DAT_00655a92 = 0xe5;
            DAT_00655416 = 0xe5;
            DAT_00655418 = 0x50;
            DAT_00655810 = 0xa8;
            DAT_00655176 = 0xa8;
        }
        if (DAT_0071b743[DAT_0071ecf4 * 0x5bd] != '\0') {
            DAT_00655810 = *(unsigned short *)(DAT_00655fb0 + (unsigned int)DAT_00655184 * 0xc);
            DAT_00655414 = *(unsigned short *)(DAT_00655fb2 + (unsigned int)DAT_00655184 * 0xc);
        }
        uVar2 = 0;
        do {
            uVar1 = 0;
            do {
                if ((uVar1 < 0x6a) || (0x6e < uVar1)) {
                    DAT_00656c90[uVar2 * 0x78 + (unsigned int)uVar1] = 0xfffe;
                }
                else {
                    DAT_00656c90[uVar2 * 0x78 + (unsigned int)uVar1] = 0xfffd;
                }
                if (uVar1 == 0x5d) {
                    DAT_00656c90[uVar2 * 0x78 + 0x5d] = 0xfffd;
                }
                uVar1 = uVar1 + 1;
            } while (uVar1 < 0x78);
            uVar1 = (unsigned short)uVar2 + 1;
            uVar2 = (unsigned int)uVar1;
        } while (uVar1 < 3);
        DAT_00655f8e = 1;
        DAT_00655a84 = 0xffff;
        DAT_00655812 = 0;
        DAT_0065518a = 0;
        DAT_00655a8e = 0;
        DAT_00655174 = 0;
        FUN_004af9c0();
        if (DAT_00525550 == 0) {
            FUN_004afb90();
        }
        FUN_004a80d0(DAT_0071ecf4);
        FUN_004a85e0(DAT_0071ecf4);
    }
    DAT_0070ac28 = 0;
    return;
}
