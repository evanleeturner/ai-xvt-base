/* SPDX-License-Identifier: MIT */
/* z_xvt 004bdea0 */
unsigned char FUN_004dbe00(char param_1);
unsigned int FUN_004dbe40(void);

unsigned int __stdcall FUN_004bdea0(void)
{
    unsigned int uVar2;
    int iVar3;

    uVar2 = FUN_004dbe40();
    if ((char)uVar2 == '\0') {
        if (FUN_004dbe00(0x12) != 0) {
            iVar3 = 0;
            do {
                if (FUN_004dbe00((char)iVar3 + 0x41) != 0) {
                    return (unsigned int)iVar3 + 0x7cU;
                }
                iVar3 = iVar3 + 1;
            } while (iVar3 < 0x1a);
            iVar3 = 0;
            do {
                if (FUN_004dbe00((char)iVar3 + 0x30) != 0) {
                    return (unsigned int)iVar3 - 0x6aU;
                }
                iVar3 = iVar3 + 1;
            } while (iVar3 < 10);
            return 0;
        }
        if (FUN_004dbe00(0x10) != 0) {
            iVar3 = 0;
            do {
                if (FUN_004dbe00((char)iVar3 + 0x70) != 0) {
                    return (unsigned int)iVar3 - 0x35U;
                }
                iVar3 = iVar3 + 1;
            } while (iVar3 < 0xc);
            return 0;
        }
        iVar3 = 0;
        do {
            if (FUN_004dbe00((char)iVar3 + 0x70) != 0) {
                return (unsigned int)iVar3 - 0x41U;
            }
            iVar3 = iVar3 + 1;
        } while (iVar3 < 0xc);
        if (FUN_004dbe00(0x25) != 0) { return 0xa0; }
        if (FUN_004dbe00(0x27) != 0) { return 0xa1; }
        if (FUN_004dbe00(0x26) != 0) { return 0xa2; }
        if (FUN_004dbe00(0x28) != 0) { return 0xa3; }
        if (FUN_004dbe00(0x2d) != 0) { return 0xa4; }
        if (FUN_004dbe00(0x2e) != 0) { return 0xa5; }
        if (FUN_004dbe00(0x24) != 0) { return 0xa6; }
        if (FUN_004dbe00(0x23) != 0) { return 0xa7; }
        if (FUN_004dbe00(0x21) != 0) { return 0xa8; }
        if (FUN_004dbe00(0x22) != 0) { return 0xa9; }
        if (FUN_004dbe00(0x2c) != 0) { return 0xaa; }
        if (FUN_004dbe00(0x91) != 0) { return 0xab; }
        if (FUN_004dbe00(0x14) != 0) { return 0xad; }
        if (FUN_004dbe00(0x60) != 0) { return 0xae; }
        if (FUN_004dbe00(0x61) != 0) { return 0xaf; }
        if (FUN_004dbe00(0x62) != 0) { return 0xb0; }
        if (FUN_004dbe00(99) != 0) { return 0xb1; }
        if (FUN_004dbe00(100) != 0) { return 0xb2; }
        if (FUN_004dbe00(0x65) != 0) { return 0xb3; }
        if (FUN_004dbe00(0x66) != 0) { return 0xb4; }
        if (FUN_004dbe00(0x67) != 0) { return 0xb5; }
        if (FUN_004dbe00(0x68) != 0) { return 0xb6; }
        if (FUN_004dbe00(0x69) != 0) { return 0xb7; }
        if (FUN_004dbe00(0x90) != 0) { return 0xb8; }
        if (FUN_004dbe00(0x6a) != 0) { return 0xba; }
        if (FUN_004dbe00(0x6b) != 0) { return 0xbc; }
        if (FUN_004dbe00(0x6d) != 0) { return 0xbb; }
        if (FUN_004dbe00(0x6e) != 0) { return 0xbe; }
        uVar2 = 0;
        if (FUN_004dbe00(0x6f) != 0) {
            uVar2 = 0xb9;
        }
    }
    return uVar2;
}
