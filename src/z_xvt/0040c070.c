/* SPDX-License-Identifier: MIT */
/* z_xvt 0040c070 */
extern unsigned char DAT_0085e795;
extern unsigned char DAT_0071b730[];   /* indexed by DAT_0071ecf4 * 0x5bd; int at +0 is unaligned */
extern int DAT_0071ecf4;
extern int DAT_006d4980;
extern unsigned char *DAT_006d4988;
extern unsigned char DAT_00523ab0[];
extern unsigned char DAT_00709b08[];
extern unsigned char DAT_0071f2db;     /* DAT_0071f2d8._3_1_ */
extern unsigned char DAT_0071f2dc;
extern unsigned char DAT_0071f2dd;
extern int DAT_0073b430[];
extern unsigned char DAT_0070ad48[];

extern unsigned short __cdecl FUN_004784d0(void);
extern int __cdecl FUN_004a18c0(unsigned char a, unsigned char b, unsigned char c);
extern void __cdecl FUN_0040c4c0(unsigned int a, char b, char c, char d, short e);

unsigned int __cdecl FUN_0040c070(int param_1, int param_2, int param_3, unsigned short param_4)
{
    unsigned short uVar1;
    int iVar2;
    unsigned char *psVar3;
    short sVar4;
    char cVar5;
    unsigned int uStack_8;
    int iTmp;
    unsigned short usTmp;

    if (DAT_0085e795 == '\0') {
        return 0;
    }
    __builtin_memcpy(&iTmp, &DAT_0071b730[(unsigned int)DAT_0071ecf4 * 0x5bd], sizeof(iTmp));
    if (iTmp == -1) {
        return 0;
    }
    if ((param_4 != 0xffff) && (uVar1 = FUN_004784d0(), param_4 <= uVar1)) {
        return 0;
    }
    sVar4 = -1;
    if (param_1 == 1) {
        if ((param_3 == -1) || (DAT_006d4980 <= param_3)) {
            return 0;
        }
        psVar3 = DAT_006d4988 + (unsigned int)param_3 * 0x23;
        __builtin_memcpy(&usTmp, &DAT_0071b730[(unsigned int)DAT_0071ecf4 * 0x5bd + 0xc], sizeof(usTmp));
        uStack_8 = (unsigned int)(unsigned char)DAT_00523ab0[
                        (unsigned char)DAT_00709b08[
                            (unsigned int)psVar3[0x16] + (unsigned int)usTmp * 0x30]];
        if (uStack_8 == 0xff) {
            return 0;
        }
        __builtin_memcpy(&sVar4, psVar3, sizeof(sVar4));
        iVar2 = FUN_004a18c0(DAT_0071f2db, DAT_0071f2dc, DAT_0071f2dd);
        if ((param_2 != 0x1b) && (param_2 != 0x1c)) {
            if ((DAT_0073b430[param_3] != 0) &&
                ((unsigned int)((unsigned int)iVar2 - (unsigned int)DAT_0073b430[param_3]) < 0xb)) {
                return 0;
            }
            DAT_0073b430[param_3] = iVar2;
        }
        __builtin_memcpy(&usTmp, &DAT_0071b730[(unsigned int)DAT_0071ecf4 * 0x5bd + 0xc], sizeof(usTmp));
        if (((uStack_8 == 4) || (uStack_8 == 5)) &&
            ((unsigned short)(unsigned char)DAT_0070ad48[
                 (unsigned int)DAT_006d4988[0x16 + (unsigned int)param_3 * 0x23] * 0x566] != usTmp)) {
            uStack_8 = 0xbU - (unsigned int)(uStack_8 == 4);
        }
    }
    if (param_1 == 1) {
        FUN_0040c4c0(uStack_8 + 0x2b5, '\x02', '\x01', '\0', sVar4);
        cVar5 = '\x01';
    } else {
        if (param_1 < 2) {
            return 1;
        }
        if (6 < param_1) {
            return 1;
        }
        cVar5 = '\0';
    }
    FUN_0040c4c0((unsigned int)param_2 + 0x2b5, '\x02', (char)param_1, cVar5, sVar4);
    return 1;
}
