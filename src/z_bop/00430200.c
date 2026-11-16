/* SPDX-License-Identifier: MIT */
/* z_bop 00430200 */
extern char DAT_00bb7375;
extern unsigned char DAT_009e9670[];   /* indexed by DAT_009ecc34 * 0x5bd; fields read unaligned */
extern unsigned char DAT_009e967c[];   /* same stride; ushort field read unaligned */
extern int DAT_009ecc34;
extern int DAT_009a1fe0;
extern int DAT_009a1fe8;
extern unsigned char DAT_00521080[];
extern unsigned char DAT_009d7188[];
extern unsigned char DAT_009d8c88[];
extern int DAT_00a0a7f0[];
extern unsigned char DAT_009ed228[];   /* byte [3] is 0x009ed22b */
extern unsigned char DAT_009ed22c;
extern unsigned char DAT_009ed22d;

unsigned short __cdecl FUN_00425d10(void);
int __cdecl FUN_00435a10(unsigned char a, unsigned char b, unsigned char c);
void __cdecl FUN_00430610(unsigned int a, char b, char c, char d, short e);

unsigned int __cdecl FUN_00430200(int param_1, int param_2, int param_3, unsigned short param_4)
{
    unsigned short uVar1;
    int iVar2;
    short sVar4;
    char cVar5;
    unsigned int uStack_8;
    int tmpInt;
    unsigned short tmpUShort;
    unsigned char *entry;

    uStack_8 = 0;

    if (DAT_00bb7375 == '\0') {
        return 0;
    }
    __builtin_memcpy(&tmpInt, &DAT_009e9670[(unsigned int)DAT_009ecc34 * 0x5bdu], sizeof(tmpInt));
    if (tmpInt == -1) {
        return 0;
    }
    if ((param_4 != 0xffff) && (uVar1 = FUN_00425d10(), param_4 <= uVar1)) {
        return 0;
    }
    sVar4 = -1;
    if (param_1 == 1) {
        if ((param_3 == -1) || (DAT_009a1fe0 <= param_3)) {
            return 0;
        }
        entry = (unsigned char *)(unsigned int)((unsigned int)DAT_009a1fe8 + (unsigned int)param_3 * 0x23u);
        __builtin_memcpy(&tmpUShort, &DAT_009e967c[(unsigned int)DAT_009ecc34 * 0x5bdu], sizeof(tmpUShort));
        uStack_8 = (unsigned int)(unsigned char)DAT_00521080[
                        (unsigned char)DAT_009d7188[
                            (unsigned int)entry[0x16] + (unsigned int)tmpUShort * 0x30u]];
        if (uStack_8 == 0xff) {
            return 0;
        }
        __builtin_memcpy(&sVar4, entry, sizeof(sVar4));
        iVar2 = FUN_00435a10(DAT_009ed228[3], DAT_009ed22c, DAT_009ed22d);
        if ((param_2 != 0x1b) && (param_2 != 0x1c)) {
            if ((DAT_00a0a7f0[param_3] != 0) &&
                ((unsigned int)((unsigned int)iVar2 - (unsigned int)DAT_00a0a7f0[param_3]) < 0xb)) {
                return 0;
            }
            DAT_00a0a7f0[param_3] = iVar2;
        }
        if ((uStack_8 == 4) || (uStack_8 == 5)) {
            __builtin_memcpy(&tmpUShort, &DAT_009e967c[(unsigned int)DAT_009ecc34 * 0x5bdu], sizeof(tmpUShort));
            if ((unsigned short)(unsigned char)DAT_009d8c88[
                    (unsigned int)((unsigned char *)(unsigned int)((unsigned int)DAT_009a1fe8 + 0x16u + (unsigned int)param_3 * 0x23u))[0] * 0x566u]
                != tmpUShort) {
                uStack_8 = 0xbu - (unsigned int)(uStack_8 == 4);
            }
        }
    }
    if (param_1 == 1) {
        FUN_00430610(uStack_8 + 0x2b8u, '\x02', '\x01', '\0', sVar4);
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
    FUN_00430610((unsigned int)param_2 + 0x2b8u, '\x02', (char)param_1, cVar5, sVar4);
    return 1;
}
