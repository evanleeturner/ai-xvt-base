/* SPDX-License-Identifier: MIT */
/* z_xvt 00455d80 */
extern unsigned int DAT_006d4980;
extern unsigned int DAT_006d4988;
extern unsigned int DAT_006d9d56;
extern unsigned int DAT_00732e54;
extern unsigned char DAT_00525bf7[];
extern unsigned char DAT_0052b0b4[];
extern unsigned char DAT_0052b0b6[];

unsigned int __stdcall FUN_00455d80(void)
{
    unsigned short idx;
    char cVar4;
    unsigned int uVar3;
    unsigned int uVar6;
    unsigned char bVar1;
    unsigned short uVar5;
    int iVar2;

    idx = *(unsigned short *)(DAT_006d9d56 + 0x18);
    if ((int)idx < (int)DAT_006d4980) {
        cVar4 = *(char *)((unsigned int)idx * 0x23 + 2 + DAT_006d4988);
        if (cVar4 == '\x03' || cVar4 == '\x05' || cVar4 == '\x04') {
            cVar4 = '\x02';
        }
        else {
            cVar4 = '\x01';
        }
    }
    else {
        cVar4 = '\x01';
    }

    uVar6 = 0;
    bVar1 = *(unsigned char *)(DAT_00732e54 + 0x228);
    uVar3 = (DAT_00732e54 & 0xFFFFFF00) | bVar1;

    if (bVar1 != 0) {
        do {
            if (DAT_00525bf7[*(unsigned char *)(DAT_00732e54 + 0x229 + uVar6)] == cVar4) {
                iVar2 = *(unsigned char *)(DAT_00732e54 + 4) * 0xef;
                uVar3 = DAT_0052b0b4[iVar2 + uVar6];
                uVar5 = uVar3;
                while (uVar5 <= DAT_0052b0b6[uVar6 + iVar2]) {
                    if (*(char *)(DAT_00732e54 + 0x346 + uVar3 * 4) != '\0') {
                        return 1;
                    }
                    uVar5 = uVar3 + 1;
                    uVar3 = uVar5;
                }
            }
            uVar3 = uVar6 + 1;
            uVar6 = uVar3;
        } while (uVar3 < bVar1);
    }
    return uVar3 & 0xFFFF0000;
}
