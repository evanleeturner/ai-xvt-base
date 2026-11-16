/* SPDX-License-Identifier: MIT */
/* z_bop 004b2130 */
extern unsigned int DAT_00528c54;
extern unsigned int *DAT_00664c80;
extern unsigned int DAT_00a90b20;
extern unsigned int DAT_00664750;
extern unsigned int DAT_00664978;
extern unsigned int _DAT_006644f4;
extern unsigned int DAT_00664544;
extern unsigned int DAT_006644f0;

unsigned char __cdecl FUN_004b48d0(unsigned int param_1);

void __cdecl FUN_004b2130(unsigned int param_1)
{
    unsigned char bVar1;

    if (DAT_00528c54 != param_1) {
        if (((DAT_00528c54 ^ param_1) & 0x8000) != 0) {
            *(unsigned char *)DAT_00664c80 = 8;
            *((unsigned char *)DAT_00664c80 + 1) = 8;
            *(unsigned short *)((unsigned char *)DAT_00664c80 + 2) = 1;
            DAT_00664c80 = DAT_00664c80 + 1;
            *DAT_00664c80 = 0xb;
            if ((param_1 & 0x8000) == 0) {
                DAT_00664c80[1] = 1;
            } else {
                DAT_00664c80[1] = 0;
            }
            DAT_00664c80 = DAT_00664c80 + 2;
        }
        if (((DAT_00528c54 ^ param_1) & 0x600) != 0) {
            *(unsigned char *)DAT_00664c80 = 8;
            *((unsigned char *)DAT_00664c80 + 1) = 8;
            *(unsigned short *)((unsigned char *)DAT_00664c80 + 2) = 4;
            if ((param_1 & 0x600) == 0) {
                DAT_00664c80 = DAT_00664c80 + 1;
                *DAT_00664c80 = 0x13;
                DAT_00664c80[1] = 2;
                DAT_00664c80 = DAT_00664c80 + 2;
                *DAT_00664c80 = 0x14;
                DAT_00664c80[1] = 1;
                DAT_00664c80 = DAT_00664c80 + 2;
                *DAT_00664c80 = 0x15;
                if ((param_1 & 0x400) == 0) {
                    DAT_00664c80[1] = 2;
                } else {
                    DAT_00664c80[1] = 4;
                }
                DAT_00664c80 = DAT_00664c80 + 2;
                *DAT_00664c80 = 0x1b;
                DAT_00664c80[1] = 0;
            } else {
                DAT_00664c80 = DAT_00664c80 + 1;
                *DAT_00664c80 = 0x13;
                DAT_00664c80[1] = 5;
                DAT_00664c80 = DAT_00664c80 + 2;
                *DAT_00664c80 = 0x14;
                DAT_00664c80[1] = 6;
                DAT_00664c80 = DAT_00664c80 + 2;
                *DAT_00664c80 = 0x15;
                if ((param_1 & 0x400) == 0) {
                    DAT_00664c80[1] = 2;
                } else {
                    DAT_00664c80[1] = 4;
                }
                DAT_00664c80 = DAT_00664c80 + 2;
                *DAT_00664c80 = 0x1b;
                DAT_00664c80[1] = 1;
            }
            DAT_00664c80 = DAT_00664c80 + 2;
        }
        if (((DAT_00528c54 ^ param_1) & 0x1800) != 0) {
            *(unsigned char *)DAT_00664c80 = 8;
            *((unsigned char *)DAT_00664c80 + 1) = 8;
            *(unsigned short *)((unsigned char *)DAT_00664c80 + 2) = 2;
            DAT_00664c80 = DAT_00664c80 + 1;
            *DAT_00664c80 = 0x17;
            if ((param_1 & 0x800) == 0) {
                DAT_00664c80[1] = 8;
            } else {
                bVar1 = FUN_004b48d0(DAT_00a90b20);
                DAT_00664c80[1] = (unsigned int)bVar1;
            }
            DAT_00664c80 = DAT_00664c80 + 2;
            *DAT_00664c80 = 0xe;
            if ((param_1 & 0x1000) == 0) {
                DAT_00664c80[1] = 0;
            } else {
                DAT_00664c80[1] = 1;
            }
            DAT_00664c80 = DAT_00664c80 + 2;
        }
        if (((DAT_00528c54 ^ param_1) & 0x180) != 0) {
            *(unsigned char *)DAT_00664c80 = 8;
            *((unsigned char *)DAT_00664c80 + 1) = 8;
            *(unsigned short *)((unsigned char *)DAT_00664c80 + 2) = 2;
            DAT_00664c80 = DAT_00664c80 + 1;
            *DAT_00664c80 = 0x11;
            DAT_00664c80[1] = 2 - (unsigned int)((param_1 & 0x80) == 0);
            DAT_00664c80 = DAT_00664c80 + 2;
            *DAT_00664c80 = 0x12;
            DAT_00664c80[1] = 2 - (unsigned int)((param_1 & 0x100) == 0);
            DAT_00664c80 = DAT_00664c80 + 2;
        }
        if (((DAT_00528c54 ^ param_1) & 0x40) != 0) {
            *(unsigned char *)DAT_00664c80 = 8;
            *((unsigned char *)DAT_00664c80 + 1) = 8;
            if ((param_1 & 0x40) != 0) {
                *(unsigned short *)((unsigned char *)DAT_00664c80 + 2) = 5;
                DAT_00664c80 = DAT_00664c80 + 1;
                *DAT_00664c80 = 0x1c;
                DAT_00664c80[1] = 1;
                DAT_00664c80 = DAT_00664c80 + 2;
                *DAT_00664c80 = 0x22;
                DAT_00664c80[1] = DAT_00664750 << 0x10 | DAT_00664978 << 8 | _DAT_006644f4;
                DAT_00664c80 = DAT_00664c80 + 2;
                *DAT_00664c80 = 0x23;
                DAT_00664c80[1] = 3;
                DAT_00664c80 = DAT_00664c80 + 2;
                *DAT_00664c80 = 0x24;
                DAT_00664c80[1] = DAT_00664544;
                DAT_00664c80 = DAT_00664c80 + 2;
                *DAT_00664c80 = 0x25;
                DAT_00664c80[1] = DAT_006644f0;
                DAT_00664c80 = DAT_00664c80 + 2;
                DAT_00528c54 = param_1;
                return;
            }
            *(unsigned short *)((unsigned char *)DAT_00664c80 + 2) = 1;
            DAT_00664c80 = DAT_00664c80 + 1;
            *DAT_00664c80 = 0x1c;
            DAT_00664c80[1] = 0;
            DAT_00664c80 = DAT_00664c80 + 2;
        }
        DAT_00528c54 = param_1;
    }
    return;
}
