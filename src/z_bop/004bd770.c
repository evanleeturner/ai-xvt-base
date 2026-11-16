/* SPDX-License-Identifier: MIT */
/* z_bop 004bd770 */
unsigned int FUN_004dcad0(void);
unsigned char FUN_004dca90(unsigned char param_1);

unsigned int FUN_004bd770(void)
{
    unsigned char bVar1;
    unsigned int uVar2;
    int iVar3;

    uVar2 = FUN_004dcad0();
    if ((char)uVar2 == '\0') {
        bVar1 = FUN_004dca90(0x12);
        if (bVar1 != 0) {
            iVar3 = 0;
            do {
                bVar1 = FUN_004dca90((unsigned char)(iVar3 + 0x41));
                if (bVar1 != 0) {
                    return (unsigned int)iVar3 + 0x80u;
                }
                iVar3 = iVar3 + 1;
            } while (iVar3 < 0x1a);
            iVar3 = 0;
            do {
                bVar1 = FUN_004dca90((unsigned char)(iVar3 + 0x30));
                if (bVar1 != 0) {
                    return (unsigned int)iVar3 - 0x66u;
                }
                iVar3 = iVar3 + 1;
            } while (iVar3 < 10);
            return 0;
        }
        bVar1 = FUN_004dca90(0x10);
        if (bVar1 != 0) {
            iVar3 = 0;
            do {
                bVar1 = FUN_004dca90((unsigned char)(iVar3 + 0x70));
                if (bVar1 != 0) {
                    return (unsigned int)iVar3 - 0x31u;
                }
                iVar3 = iVar3 + 1;
            } while (iVar3 < 0xc);
            return 0;
        }
        iVar3 = 0;
        do {
            bVar1 = FUN_004dca90((unsigned char)(iVar3 + 0x70));
            if (bVar1 != 0) {
                return (unsigned int)iVar3 - 0x3du;
            }
            iVar3 = iVar3 + 1;
        } while (iVar3 < 0xc);
        bVar1 = FUN_004dca90(0x25);
        if (bVar1 != 0) {
            return 0xa4;
        }
        bVar1 = FUN_004dca90(0x27);
        if (bVar1 != 0) {
            return 0xa5;
        }
        bVar1 = FUN_004dca90(0x26);
        if (bVar1 != 0) {
            return 0xa6;
        }
        bVar1 = FUN_004dca90(0x28);
        if (bVar1 != 0) {
            return 0xa7;
        }
        bVar1 = FUN_004dca90(0x2d);
        if (bVar1 != 0) {
            return 0xa8;
        }
        bVar1 = FUN_004dca90(0x2e);
        if (bVar1 != 0) {
            return 0xa9;
        }
        bVar1 = FUN_004dca90(0x24);
        if (bVar1 != 0) {
            return 0xaa;
        }
        bVar1 = FUN_004dca90(0x23);
        if (bVar1 != 0) {
            return 0xab;
        }
        bVar1 = FUN_004dca90(0x21);
        if (bVar1 != 0) {
            return 0xac;
        }
        bVar1 = FUN_004dca90(0x22);
        if (bVar1 != 0) {
            return 0xad;
        }
        bVar1 = FUN_004dca90(0x2c);
        if (bVar1 != 0) {
            return 0xae;
        }
        bVar1 = FUN_004dca90(0x91);
        if (bVar1 != 0) {
            return 0xaf;
        }
        bVar1 = FUN_004dca90(0x14);
        if (bVar1 != 0) {
            return 0xb1;
        }
        bVar1 = FUN_004dca90(0x60);
        if (bVar1 != 0) {
            return 0xb2;
        }
        bVar1 = FUN_004dca90(0x61);
        if (bVar1 != 0) {
            return 0xb3;
        }
        bVar1 = FUN_004dca90(0x62);
        if (bVar1 != 0) {
            return 0xb4;
        }
        bVar1 = FUN_004dca90(99);
        if (bVar1 != 0) {
            return 0xb5;
        }
        bVar1 = FUN_004dca90(100);
        if (bVar1 != 0) {
            return 0xb6;
        }
        bVar1 = FUN_004dca90(0x65);
        if (bVar1 != 0) {
            return 0xb7;
        }
        bVar1 = FUN_004dca90(0x66);
        if (bVar1 != 0) {
            return 0xb8;
        }
        bVar1 = FUN_004dca90(0x67);
        if (bVar1 != 0) {
            return 0xb9;
        }
        bVar1 = FUN_004dca90(0x68);
        if (bVar1 != 0) {
            return 0xba;
        }
        bVar1 = FUN_004dca90(0x69);
        if (bVar1 != 0) {
            return 0xbb;
        }
        bVar1 = FUN_004dca90(0x90);
        if (bVar1 != 0) {
            return 0xbc;
        }
        bVar1 = FUN_004dca90(0x6a);
        if (bVar1 != 0) {
            return 0xbe;
        }
        bVar1 = FUN_004dca90(0x6b);
        if (bVar1 != 0) {
            return 0xc0;
        }
        bVar1 = FUN_004dca90(0x6d);
        if (bVar1 != 0) {
            return 0xbf;
        }
        bVar1 = FUN_004dca90(0x6e);
        if (bVar1 != 0) {
            return 0xc2;
        }
        bVar1 = FUN_004dca90(0x6f);
        uVar2 = 0;
        if (bVar1 != 0) {
            uVar2 = 0xbd;
        }
    }
    return uVar2;
}
