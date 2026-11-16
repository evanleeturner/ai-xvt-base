/* SPDX-License-Identifier: MIT */
/* z_bop 00452960 */
extern unsigned char  DAT_005233c8;
extern int            DAT_005233d8;
extern unsigned short DAT_005235dc;
extern short          DAT_005569f8;
extern short          DAT_00556a50;
extern unsigned short DAT_009a1fe4;
extern unsigned short DAT_009a7390;
extern int            DAT_009a7b44;
extern unsigned short DAT_009a7bac;
extern int            DAT_009a7ec0;
extern unsigned int   DAT_009a806c;
extern int            DAT_009a8d94;
extern int            DAT_009a8d98;
extern unsigned short DAT_009d1148;
extern int            DAT_009d12f4;
extern unsigned short DAT_009d12f8;
extern int            DAT_009d6934;
extern unsigned short DAT_009ec470;
extern int            DAT_009ecc34;
extern unsigned short DAT_009ecc38;
extern unsigned char  DAT_009ed232;
extern unsigned char  DAT_009e9a77[];
extern int            DAT_00a080f8;
extern unsigned short DAT_00a08374;
/* 12-byte records addressed as ushort arrays: byte offset 12*idx == ushort index 6*idx */
extern unsigned short DAT_00a0921c[];
extern unsigned short DAT_00a0921e[];
extern unsigned short DAT_00a09228[];
extern unsigned short DAT_00a0922a[];
extern unsigned short DAT_00a09234[];
extern unsigned short DAT_00a09236[];

/* calling convention of FUN_004a9640 / FUN_00427150 not confirmed by evidence; cdecl assumed */
void __cdecl FUN_004a9640(char param_1);
void __cdecl FUN_00427150(unsigned int, unsigned int, unsigned int, unsigned int,
                          unsigned int, unsigned int, unsigned int, unsigned int,
                          unsigned int);

void __fastcall FUN_00452960(unsigned int param_1, unsigned int param_2)
{
    unsigned short uVar2;
    unsigned short uVar3;
    unsigned int x, y, u, v, w, h, color;
    unsigned int idx;
    unsigned int b;

    uVar2 = (unsigned short)DAT_005233c8;
    idx = (unsigned int)DAT_00a08374 * 6;

    if (DAT_00a080f8 < 1) {
        y = (unsigned short)DAT_009d1148;
        x = (unsigned short)DAT_005235dc;
        color = (unsigned short)((short)DAT_005233d8 * (short)DAT_009a7bac);
        uVar3 = (unsigned short)((short)DAT_009a7b44 - (short)DAT_009d1148);
        v = DAT_00a0921e[idx];
        u = DAT_00a0921c[idx];
        w = (unsigned short)((short)DAT_009a7ec0 - (short)DAT_005235dc);
        h = uVar3;
    } else {
        FUN_004a9640('\0');
        b = DAT_009ed232;
        y = (unsigned short)DAT_009d1148;
        uVar3 = (unsigned short)((short)DAT_009a7b44 - (short)DAT_009d1148);
        w = (unsigned short)((short)(b * 4) - (short)DAT_005235dc + (short)DAT_009a7ec0 + 1);
        u = DAT_00a0921c[idx];
        x = (unsigned short)((short)DAT_005235dc + (short)b * -2);
        v = DAT_00a0921e[idx];
        color = (unsigned short)((short)DAT_005233d8 * (short)DAT_009a7bac);
        h = uVar3;
    }
    FUN_00427150(DAT_009a806c, uVar2, x, y, u, v, w, h, color);

    idx = (unsigned int)DAT_00a08374 * 6;
    color = (unsigned short)((short)DAT_005233d8 * (short)DAT_009a7bac);

    if (DAT_009e9a77[(int)((unsigned int)DAT_009ecc34 * 0x5bdu)] == 0x12) {
        if (DAT_005569f8 != -1) {
            FUN_00427150(DAT_009a806c, uVar2, DAT_009a7390, DAT_009d12f8,
                         DAT_00a09228[idx],
                         (unsigned short)(DAT_009a1fe4 + -0xb),
                         (unsigned short)((short)DAT_009a8d98 - (short)DAT_009a7390),
                         (unsigned short)((short)DAT_009a8d94 - (short)DAT_009d12f8),
                         color);
        }
        if ((short)DAT_00556a50 != -1) {
            FUN_00427150(DAT_009a806c, uVar2, DAT_009ec470, DAT_009ecc38,
                         DAT_00a09234[idx],
                         (unsigned short)(DAT_009a1fe4 + -0x16),
                         (unsigned short)((short)DAT_009d12f4 - (short)DAT_009ec470),
                         (unsigned short)((short)DAT_009d6934 - (short)DAT_009ecc38),
                         color);
        }
    } else {
        if (DAT_005569f8 != -1) {
            FUN_00427150(DAT_009a806c, uVar2, DAT_009a7390, DAT_009d12f8,
                         DAT_00a09228[idx],
                         DAT_00a0922a[idx],
                         (unsigned short)((short)DAT_009a8d98 - (short)DAT_009a7390),
                         (unsigned short)((short)DAT_009a8d94 - (short)DAT_009d12f8),
                         color);
        }
        if ((short)DAT_00556a50 != -1) {
            FUN_00427150(DAT_009a806c, uVar2, DAT_009ec470, DAT_009ecc38,
                         DAT_00a09234[idx],
                         DAT_00a09236[idx],
                         (unsigned short)((short)DAT_009d12f4 - (short)DAT_009ec470),
                         (unsigned short)((short)DAT_009d6934 - (short)DAT_009ecc38),
                         color);
            return;
        }
    }
    return;
}
