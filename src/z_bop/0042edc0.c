/* SPDX-License-Identifier: MIT */
/* z_bop 0042edc0 */
extern int DAT_00523410;
extern int DAT_009ecc34;
extern char DAT_009d80c9;
extern char DAT_00bb7372;
extern char DAT_00bb737b;
extern unsigned char DAT_009e9670[];
extern int DAT_009a1fe8;
extern int DAT_009d8110;
extern char DAT_00a0b3a8[];
extern char DAT_00a0b378[];
extern unsigned int DAT_00a0adf0[];

unsigned int __cdecl FUN_004a9300(int a, int b);
void __cdecl FUN_004a9380(int a);
unsigned int __cdecl FUN_0042e810(int a, unsigned int b);
void __cdecl FUN_0042d120(char *a, int b, int c, int d, unsigned int e, int f);

void __cdecl FUN_0042edc0(int param_1, int param_2)
{
    char cVar1;
    unsigned int uVar2;
    unsigned int uVar3;
    int iVar4;
    int iTmp0;
    int iTmp1;

    if ((((DAT_00523410 == 0) && (DAT_009ecc34 == param_2)) && (DAT_009d80c9 != '\0')) &&
        ((DAT_00bb7372 != '\0' && (DAT_00bb737b != '\0')))) {
        __builtin_memcpy(&iTmp0,
                         (unsigned char *)DAT_009e9670 + (int)((unsigned int)DAT_009ecc34 * 0x5bdU),
                         sizeof(int));
        __builtin_memcpy(&iTmp1,
                         (void *)(unsigned int)((unsigned int)iTmp0 * 0x23U + 0x1fU + (unsigned int)DAT_009a1fe8),
                         sizeof(int));
        __builtin_memcpy(&iVar4, (void *)(unsigned int)((unsigned int)iTmp1 + 0xa9U), sizeof(int));

        if ((param_1 == 0) || ((*(unsigned char *)(iVar4 + 0x1e6) & 1) == 0)) {
            iVar4 = 0x34;
            do {
                uVar2 = FUN_004a9300(iVar4, 0x100);
                if (uVar2 != 0) {
                    FUN_004a9380(iVar4);
                }
                iVar4 = iVar4 + 1;
            } while (iVar4 < 0x3e);
        }
        else {
            cVar1 = *(char *)(iVar4 + 0x23d);
            if (cVar1 == '\x01') {
                uVar2 = FUN_004a9300(0x34, 0x100);
                if (uVar2 != 0) {
                    return;
                }
                uVar2 = 0x35;
            }
            else {
                if (cVar1 != '\x02') {
                    if (cVar1 != '\x03') {
                        uVar2 = FUN_004a9300(0x3c, 0x100);
                        if (uVar2 != 0) {
                            return;
                        }
                        uVar2 = FUN_004a9300(0x3d, 0x100);
                        if (uVar2 != 0) {
                            return;
                        }
                        uVar2 = FUN_0042e810(-1, 0x3d);
                        FUN_0042d120(DAT_00a0b3a8, 1, 1, 0x7d, uVar2, 0x40);
                        return;
                    }
                    uVar2 = FUN_004a9300(0x3a, 0x100);
                    if (uVar2 != 0) {
                        return;
                    }
                    uVar2 = FUN_004a9300(0x3b, 0x100);
                    if (uVar2 != 0) {
                        return;
                    }
                    uVar2 = FUN_0042e810(-1, 0x3b);
                    FUN_0042d120(DAT_00a0b378, 1, 1, 0x7d, uVar2, 0x40);
                    return;
                }
                uVar2 = FUN_004a9300(0x37, 0x100);
                if (uVar2 != 0) {
                    return;
                }
                uVar2 = 0x38;
            }
            if (DAT_009d8110 == -1) {
                uVar3 = FUN_004a9300(uVar2 + 1, 0x100);
                if (uVar3 != 0) {
                    FUN_004a9380(uVar2 + 1);
                }
                uVar3 = FUN_004a9300(uVar2, 0x100);
                if (uVar3 == 0) {
                    uVar3 = FUN_0042e810(-1, uVar2);
                    FUN_0042d120((char *)(DAT_00a0adf0 + uVar2 * 6), 1, 1, 0x7d, uVar3, 0x40);
                    return;
                }
            }
            else {
                uVar3 = FUN_004a9300(uVar2, 0x100);
                if (uVar3 != 0) {
                    FUN_004a9380(uVar2);
                }
                uVar3 = FUN_004a9300(uVar2 + 1, 0x100);
                if (uVar3 == 0) {
                    uVar3 = FUN_0042e810(-1, uVar2 + 1);
                    FUN_0042d120((char *)(uVar2 * 0x18 + 0xa0ae08), 1, 1, 0x7d, uVar3, 0x40);
                    return;
                }
            }
        }
    }
    return;
}
