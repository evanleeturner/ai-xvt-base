/* SPDX-License-Identifier: MIT */
/* z_bop 0042c750 */
extern int DAT_005233ec;
extern int DAT_009ecc34;
extern int DAT_009a1fe8;
extern unsigned char DAT_009e9670[];
extern char DAT_009d8b68;
extern unsigned short DAT_009a20ae;      /* width unresolved: decompiler casts to ushort */
extern unsigned char DAT_009d6940[];     /* byte at +3 is used */
extern unsigned short DAT_009d694e;      /* type unresolved */
extern unsigned short DAT_009d694a;      /* type unresolved */
extern unsigned short DAT_009d6950;      /* type unresolved */
extern int DAT_009d6944;                 /* type unresolved */
extern int DAT_00a606f0;
extern int DAT_00a606f4;
extern int DAT_00a606f8;
extern int DAT_00a606fc;
extern int DAT_00a60700;
extern void (*DAT_009d1300)(void);
extern void (*DAT_009a6fe4)(int);

void FUN_004a9640(char a);
void FUN_004a9520(short a, short b, short c, short d);
void FUN_004a9630(int a);
void FUN_004a95c0(int a);
void FUN_004a9590(int a);
void FUN_004a9500(short a, short b);
void FUN_004a9960(int a);
void FUN_004277f0(unsigned short a, int b, int c);   /* arg widths unresolved */
void FUN_0042cbd0(int a, int b, int c);              /* arg widths unresolved */

void __cdecl FUN_0042c750(short param_1, short param_2)
{
    short sVar1;
    short sVar2;
    short sVar3;
    short sStack_18;
    short sStack_14;
    short sStack_10;
    short sStack_c;
    short sStack_8;
    short sStack_4;
    /* Added locals (not in the Ghidra output): they split the nested
       pointer-chain expression so each unaligned read can use memcpy. */
    int iVar_a;
    int iVar_b;
    int iVar_c;
    unsigned char bVar_d;

    if (DAT_005233ec == 0x13) {
        sVar1 = 8;
        sStack_14 = 0x1a;
        sStack_10 = 0x35;
        sStack_18 = 0x5a;
        sStack_c = 0x14;
        sStack_8 = 0x4b;
        sStack_4 = 0x2d;
        sVar3 = (short)(param_2 + -6);
    }
    else if (DAT_005233ec == 0x101) {
        sVar1 = 0x10;
        sStack_14 = 0x34;
        sStack_10 = 0x6a;
        sStack_18 = 0xb4;
        sStack_c = 0x28;
        sStack_8 = 0x96;
        sStack_4 = 0x5a;
        param_1 = (short)(param_1 + 10);
        sVar3 = (short)(param_2 + -10);
    }
    else {
        sVar1 = 8;
        sStack_14 = 0x1a;
        sStack_10 = 0x35;
        sStack_18 = 0x5a;
        sStack_c = 0x14;
        sStack_8 = 0x4b;
        sStack_4 = 0x2d;
        sVar3 = (short)(param_2 + -6);
    }

    __builtin_memcpy(&iVar_a, DAT_009e9670 + (unsigned int)DAT_009ecc34 * 0x5bdu, 4);
    __builtin_memcpy(&iVar_b,
                     (unsigned char *)(unsigned int)((unsigned int)iVar_a * 0x23u + 0x1fu + (unsigned int)DAT_009a1fe8),
                     4);
    __builtin_memcpy(&iVar_c, (unsigned char *)(unsigned int)((unsigned int)iVar_b + 0xa9u), 4);
    bVar_d = *(unsigned char *)(unsigned int)((unsigned int)iVar_c + 4u);

    switch (bVar_d) {
    case 7:
    case 8:
    case 0xb:
    case 0xf:
        sVar2 = sVar1;
        break;
    default:
        sVar2 = (short)-sVar1;
    }
    sVar2 = (short)(param_1 + sVar2);

    if (DAT_009d8b68 != '\0') {
        FUN_004a9640('\x01');
        FUN_004a9520((short)(sVar1 + sVar2), sVar3, (short)(sVar1 * 10 + sVar2),
                     (short)((unsigned short)DAT_009a20ae + sVar3));
        FUN_004a9630(1);
        FUN_004a95c0(0x30);
        (*DAT_009d1300)();
        FUN_004a9590(0x49);
        FUN_004a9500((short)(sStack_14 + sVar2), sVar3);
        FUN_004a9960(DAT_00a606f0);
        FUN_004a9590(0x4a);
        FUN_004a9500((short)(sStack_10 + sVar2), sVar3);
        FUN_004277f0((unsigned short)DAT_009d6940[3], 2, 2);
        FUN_004a9520(sVar2, (short)((unsigned short)DAT_009a20ae + sVar3 + 1),
                     (short)(sStack_18 + sVar2),
                     (short)((unsigned short)DAT_009a20ae * 5 + sVar3 + 1));
        FUN_004a9590(0x45);
        FUN_004a9500(sVar2, (short)((unsigned short)DAT_009a20ae + sVar3 + 1));
        FUN_004a9960(DAT_00a606f4);
        FUN_004a9500(sVar2, (short)(sVar3 + (unsigned short)DAT_009a20ae * 2 + 1));
        FUN_004a9960(DAT_00a606f8);
        FUN_004a9590(0x4d);
        FUN_004a9500(sVar2, (short)((unsigned short)DAT_009a20ae * 3 + sVar3 + 1));
        FUN_004a9960(DAT_00a606fc);
        FUN_004a9590(0x51);
        FUN_004a9500((short)(sVar2 + sStack_c),
                     (short)(sVar3 + (unsigned short)DAT_009a20ae * 4 + 1));
        FUN_004a9960(DAT_00a60700);
    }

    FUN_004a9640('\x01');
    FUN_004a9520(sVar2, (short)((unsigned short)DAT_009a20ae + sVar3 + 1),
                 (short)(sStack_18 + sVar2),
                 (short)((unsigned short)DAT_009a20ae * 5 + sVar3 + 1));
    FUN_004a9630(1);
    FUN_004a95c0(0x30);
    FUN_004a9590(0x46);
    sStack_8 = (short)(sStack_8 + sVar2);
    FUN_004a9500(sStack_8, (short)((unsigned short)DAT_009a20ae + sVar3 + 1));
    FUN_004277f0(DAT_009d694e, 3, 1);
    (*DAT_009a6fe4)(0x20);
    FUN_004a9500(sStack_8, (short)(sVar3 + (unsigned short)DAT_009a20ae * 2 + 1));
    FUN_004277f0(DAT_009d694a, 3, 1);
    (*DAT_009a6fe4)(0x20);
    FUN_004a9590(0x4e);
    FUN_004a9500(sStack_8, (short)((unsigned short)DAT_009a20ae * 3 + sVar3 + 1));
    FUN_004277f0(DAT_009d6950, 3, 1);
    (*DAT_009a6fe4)(0x20);
    FUN_004a9590(0x52);
    FUN_004a9500((short)(sVar2 + sStack_4),
                 (short)(sVar3 + (unsigned short)DAT_009a20ae * 4 + 1));
    FUN_0042cbd0(DAT_009d6944, 6, 1);
    (*DAT_009a6fe4)(0x20);
    FUN_004a9640('\x02');
    return;
}
