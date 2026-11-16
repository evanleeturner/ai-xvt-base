/* SPDX-License-Identifier: MIT */
/* z_xvt 0040a980 */
extern int DAT_00525550;
extern int DAT_0071ecf4;
extern char DAT_0070a189;
extern char DAT_0085e792;
extern char DAT_0085e79b;
extern char DAT_0071b730[];
extern int DAT_006d4988;
extern int DAT_0070a1d0;
extern char DAT_0073bfe8[];
extern char DAT_0073bfb8[];
extern char DAT_0073ba30[];

unsigned int __cdecl FUN_00439830(int a, int b);
void __cdecl FUN_004398b0(int a);
unsigned int __cdecl FUN_0040a4b0(unsigned int a, unsigned int b);
void __cdecl FUN_00490a50(char *a, int b, int c, int d, unsigned int e, int f);

void __cdecl FUN_0040a980(int param_1, int param_2)
{
    char cVar1;
    unsigned int uVar2;
    unsigned int uVar3;
    int iVar4;
    int t1;
    int t2;
    unsigned char *p;

    if ((((DAT_00525550 == 0) && (DAT_0071ecf4 == param_2)) && (DAT_0070a189 != '\0')) &&
        ((DAT_0085e792 != '\0' && (DAT_0085e79b != '\0')))) {
        __builtin_memcpy(&t1, DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bdu, 4);
        __builtin_memcpy(&t2,
                         (unsigned char *)(((unsigned int)t1 * 0x23u + 0x1fu) + (unsigned int)DAT_006d4988),
                         4);
        __builtin_memcpy(&iVar4, (unsigned char *)((unsigned int)t2 + 0xa7u), 4);
        p = (unsigned char *)iVar4;

        if ((param_1 == 0) || ((*(unsigned char *)(p + 0x1e0) & 1) == 0)) {
            iVar4 = 0x34;
            do {
                uVar2 = FUN_00439830(iVar4, 0x100);
                if (uVar2 != 0) {
                    FUN_004398b0(iVar4);
                }
                iVar4 = iVar4 + 1;
            } while (iVar4 < 0x3e);
        }
        else {
            cVar1 = *(char *)(p + 0x233);
            if (cVar1 == '\x01') {
                uVar2 = FUN_00439830(0x34, 0x100);
                if (uVar2 != 0) {
                    return;
                }
                uVar2 = 0x35;
            }
            else {
                if (cVar1 != '\x02') {
                    if (cVar1 != '\x03') {
                        uVar2 = FUN_00439830(0x3c, 0x100);
                        if (uVar2 != 0) {
                            return;
                        }
                        uVar2 = FUN_00439830(0x3d, 0x100);
                        if (uVar2 != 0) {
                            return;
                        }
                        uVar2 = FUN_0040a4b0(0xffffffffu, 0x3d);
                        FUN_00490a50(DAT_0073bfe8, 1, 1, 0x7d, uVar2, 0x40);
                        return;
                    }
                    uVar2 = FUN_00439830(0x3a, 0x100);
                    if (uVar2 != 0) {
                        return;
                    }
                    uVar2 = FUN_00439830(0x3b, 0x100);
                    if (uVar2 != 0) {
                        return;
                    }
                    uVar2 = FUN_0040a4b0(0xffffffffu, 0x3b);
                    FUN_00490a50(DAT_0073bfb8, 1, 1, 0x7d, uVar2, 0x40);
                    return;
                }
                uVar2 = FUN_00439830(0x37, 0x100);
                if (uVar2 != 0) {
                    return;
                }
                uVar2 = 0x38;
            }
            if (DAT_0070a1d0 == -1) {
                uVar3 = FUN_00439830(uVar2 + 1, 0x100);
                if (uVar3 != 0) {
                    FUN_004398b0(uVar2 + 1);
                }
                uVar3 = FUN_00439830(uVar2, 0x100);
                if (uVar3 == 0) {
                    uVar3 = FUN_0040a4b0(0xffffffffu, uVar2);
                    FUN_00490a50(DAT_0073ba30 + uVar2 * 0x18u, 1, 1, 0x7d, uVar3, 0x40);
                    return;
                }
            }
            else {
                uVar3 = FUN_00439830(uVar2, 0x100);
                if (uVar3 != 0) {
                    FUN_004398b0(uVar2);
                }
                uVar3 = FUN_00439830(uVar2 + 1, 0x100);
                if (uVar3 == 0) {
                    uVar3 = FUN_0040a4b0(0xffffffffu, uVar2 + 1);
                    FUN_00490a50((char *)(uVar2 * 0x18u + 0x73ba48u), 1, 1, 0x7d, uVar3, 0x40);
                    return;
                }
            }
        }
    }
    return;
}
