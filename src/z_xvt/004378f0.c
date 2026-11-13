/* SPDX-License-Identifier: MIT */
/* z_xvt 004378f0 */
extern int DAT_0052550c;
extern int _DAT_00525510;
extern int DAT_0052552c;
extern unsigned int DAT_0052f0f4;
extern int DAT_006da538;
extern int DAT_006db594;
extern int _DAT_006da4ec;
extern int DAT_0070acc8;
extern int DAT_00732f3c;
extern int _DAT_006d4994;
extern unsigned char DAT_0070acc4;
extern unsigned short DAT_0071ed00;

int FUN_004a6a40(void);

void __stdcall FUN_004378f0(void)
{
    int iVar1;

    iVar1 = FUN_004a6a40();
    DAT_0052550c = (int)((unsigned int)iVar1 * 0x1e0u);
    _DAT_00525510 = 1;
    if (DAT_0052552c == 0x13) {
        DAT_0052550c = 0x10000;
        _DAT_00525510 = 1;
        DAT_006da538 = 0x140;
        DAT_006db594 = 0xf0;
        _DAT_006da4ec = 0xf0;
        DAT_0070acc8 = FUN_004a6a40();
        DAT_00732f3c = 0x100;
        _DAT_006d4994 = 0x80;
        DAT_0070acc4 = 8;
        ((unsigned char *)&DAT_0052f0f4)[2] = 0x33;
        ((unsigned char *)&DAT_0052f0f4)[3] = 0x32;
        DAT_0071ed00 = 0;
        return;
    }
    if (DAT_0052552c != 0x101) {
        if (DAT_0052552c != 0x112) {
            DAT_0052550c = 0x10000;
            DAT_006da538 = 0x140;
            DAT_006db594 = 0xf0;
            _DAT_006da4ec = 0xf0;
            DAT_0070acc8 = FUN_004a6a40();
            DAT_00732f3c = 0x100;
            _DAT_006d4994 = 0x80;
            DAT_0070acc4 = 8;
            ((unsigned char *)&DAT_0052f0f4)[2] = 0x33;
            ((unsigned char *)&DAT_0052f0f4)[3] = 0x32;
            DAT_0071ed00 = 0;
            return;
        }
        DAT_006db594 = 0x168;
        DAT_006da538 = 0x1e0;
        _DAT_006da4ec = 0x1e0;
        DAT_0070acc8 = FUN_004a6a40();
        DAT_00732f3c = 0x200;
        _DAT_006d4994 = 0x100;
        DAT_0070acc4 = 9;
        ((unsigned char *)&DAT_0052f0f4)[2] = 0x34;
        ((unsigned char *)&DAT_0052f0f4)[3] = 0x38;
        DAT_0071ed00 = 0;
        return;
    }
    DAT_006da538 = 0x280;
    DAT_006db594 = 0x1e0;
    _DAT_006da4ec = 0x1e0;
    DAT_0070acc8 = FUN_004a6a40();
    DAT_00732f3c = 0x200;
    _DAT_006d4994 = 0x100;
    DAT_0070acc4 = 9;
    ((unsigned char *)&DAT_0052f0f4)[2] = 0x36;
    ((unsigned char *)&DAT_0052f0f4)[3] = 0x34;
    DAT_0071ed00 = 0;
    return;
}
