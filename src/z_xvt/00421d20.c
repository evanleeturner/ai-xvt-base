/* SPDX-License-Identifier: MIT */
/* z_xvt 00421d20 */
extern unsigned short DAT_005256b0;
extern unsigned short DAT_005256b4;
extern unsigned short DAT_005256b8;
extern unsigned short DAT_005256bc;
extern unsigned short DAT_005256c0;
extern unsigned short DAT_005256c4;
extern unsigned char DAT_00525670[];
extern char DAT_00525630[];

/* Return type is not resolved by the evidence: callers use it as a byte in the
   first two loops and as a ushort in the third. Declared as unsigned int
   (EAX) and truncated at each use. */
unsigned int __stdcall FUN_00478470(void);

void __stdcall FUN_00421d20(void)
{
    unsigned char bVar1;
    unsigned char bVar2;
    char cVar3;
    unsigned int uVar4;
    unsigned short uVar5;

    DAT_005256b0 = 4;
    DAT_005256b4 = 4;
    DAT_005256c0 = 4;
    DAT_005256c4 = 4;
    DAT_005256b8 = 3;
    DAT_005256bc = 3;
    for (uVar5 = 0; uVar5 < 0x16; uVar5 = (unsigned short)(uVar5 + 1)) {
        do {
            bVar1 = (unsigned char)FUN_00478470();
            bVar1 = (unsigned char)((bVar1 & 0xe) + 4);
        } while (0xc < bVar1);
        do {
            bVar2 = (unsigned char)FUN_00478470();
            bVar2 = (unsigned char)((bVar2 & 0xe) + 4);
        } while (0xc < bVar2);
        DAT_00525670[uVar5] = (unsigned char)(bVar2 * 0x10 + bVar1);
    }
    uVar4 = 0;
    do {
        uVar5 = (unsigned short)FUN_00478470();
        uVar5 = (unsigned short)(uVar5 & 0x1f);
        if (uVar5 < 3) {
            DAT_00525630[uVar4] = 0x75;
        }
        else {
            if (uVar5 < 0xc) {
                cVar3 = (char)((char)(uVar5 / 3) + 'u');
            }
            else {
                cVar3 = (char)((((unsigned char)uVar5) & 1) + 0x7d);
            }
            DAT_00525630[uVar4] = cVar3;
        }
        uVar5 = (unsigned short)((short)uVar4 + 1);
        uVar4 = (unsigned int)uVar5;
    } while (uVar5 < 0x16);
    return;
}
