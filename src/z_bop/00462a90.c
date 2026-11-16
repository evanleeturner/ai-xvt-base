/* SPDX-License-Identifier: MIT */
/* z_bop 00462a90 */
/* Prototype for the one callee. Calling convention assumed cdecl (Ghidra shows no stack cleanup). */
unsigned int FUN_004630c0(unsigned int *param_1, int param_2);

/* Globals: names kept as placeholders. Widths follow the P-code where it showed them. */
extern unsigned int DAT_009a7ba4;
extern unsigned short DAT_00a08300;
extern unsigned char *DAT_009a1fe8;
extern unsigned char *DAT_009a8da0;
extern unsigned int DAT_00a07ce8;
extern unsigned int DAT_009d767c;
extern unsigned char *DAT_009d6820;
extern unsigned int DAT_009d1308;
extern unsigned int DAT_009eca28;
extern unsigned short DAT_00a082c0;
extern unsigned char *DAT_00a07bd0;
extern unsigned int DAT_009fd430;
extern unsigned short DAT_00a082d8;
extern unsigned char *DAT_009a8e18;
extern unsigned int DAT_009ed228;
extern unsigned int DAT_009d8b70;
extern short DAT_009a2000;
extern unsigned int DAT_009a20c0;
extern unsigned int DAT_009d6940;
extern unsigned short DAT_009a8d40;
extern unsigned int DAT_009ec5e0;
extern int DAT_00a08294; /* width unverified: the decompile only shows an (int) cast */
extern unsigned int DAT_009d8c30;
extern unsigned int DAT_009fe800;
extern unsigned int DAT_009a8080;
extern unsigned int DAT_009a8bfc;
extern unsigned int DAT_009fd394;
extern unsigned char DAT_009d8c20;
extern unsigned int DAT_00a0814c;
extern unsigned int DAT_009a7b50;
extern unsigned int _DAT_00a07cec;
extern unsigned int DAT_009a1fe0;
extern unsigned int DAT_009d1144;
extern unsigned int DAT_009fe7dc;
extern unsigned int DAT_009e9640;
extern unsigned int DAT_00a00854;
extern unsigned int DAT_009a7b58;
extern unsigned int DAT_009d1140;
extern unsigned int DAT_009d1310;
extern unsigned int DAT_009ec47c;
extern unsigned int DAT_009a8e24;
extern unsigned int DAT_009cd274;
extern unsigned int DAT_009d1320;
extern unsigned int DAT_009a8068;
extern unsigned int DAT_009a8e40;
extern unsigned int DAT_009a73a4;
extern unsigned short DAT_009d113c;
extern unsigned int DAT_00a08138;
extern char DAT_009e9680;
extern char DAT_009ec468;

unsigned int __stdcall FUN_00462a90(void)
{
    unsigned int uVar1;
    unsigned int uVar2;
    int iVar3;
    int iVar4;
    char *pcVar5;
    int iVar6;
    unsigned int *puVar7;
    unsigned int local_4;

    iVar4 = 0;
    local_4 = 0;
    if (0 < (int)DAT_009a7ba4) {
        iVar6 = 0;
        iVar3 = (int)((unsigned int)DAT_00a08300 * 0x23u);
        do {
            if (*(char *)(DAT_009a1fe8 + 3 + iVar3) != '\0') {
                uVar1 = FUN_004630c0((unsigned int *)(DAT_009a8da0 + iVar6), 0x4c);
                local_4 = ((local_4 ^ uVar1) << 1) | (unsigned int)((int)(local_4 ^ uVar1) < 0);
            }
            iVar6 = (int)((unsigned int)iVar6 + 0x4cu);
            iVar4 = iVar4 + 1;
            iVar3 = (int)((unsigned int)iVar3 + 0x23u);
        } while (iVar4 < (int)DAT_009a7ba4);
    }
    iVar4 = 0;
    if (0 < (int)(DAT_00a07ce8 - DAT_009d767c)) {
        iVar3 = 0;
        iVar6 = 0;
        do {
            if (*(char *)(DAT_009a1fe8 + 3 + iVar6) != '\0') {
                uVar1 = FUN_004630c0((unsigned int *)(DAT_009d6820 + iVar3), 0x8b);
                local_4 = ((local_4 ^ uVar1) << 1) | (unsigned int)((int)(local_4 ^ uVar1) < 0);
            }
            iVar3 = (int)((unsigned int)iVar3 + 0xb1u);
            iVar4 = iVar4 + 1;
            iVar6 = (int)((unsigned int)iVar6 + 0x23u);
        } while (iVar4 < (int)(DAT_00a07ce8 - DAT_009d767c));
    }
    iVar4 = 0;
    if (0 < (int)(DAT_00a07ce8 - DAT_009d767c)) {
        iVar3 = 0;
        do {
            if (*(char *)(DAT_009a1fe8 + iVar3 + 3) != '\0') {
                uVar1 = FUN_004630c0((unsigned int *)(DAT_009a1fe8 + iVar3), 0x1f);
                local_4 = ((local_4 ^ uVar1) << 1) | (unsigned int)((int)(local_4 ^ uVar1) < 0);
            }
            iVar3 = (int)((unsigned int)iVar3 + 0x23u);
            iVar4 = iVar4 + 1;
        } while (iVar4 < (int)(DAT_00a07ce8 - DAT_009d767c));
    }
    if ((int)DAT_00a07ce8 < (int)(DAT_009d1308 + DAT_00a07ce8)) {
        iVar4 = (int)(DAT_00a07ce8 * 0x23u);
        uVar1 = DAT_00a07ce8;
        do {
            if (*(char *)(DAT_009a1fe8 + iVar4 + 3) != '\0') {
                uVar2 = FUN_004630c0((unsigned int *)(DAT_009a1fe8 + iVar4), 0x1f);
                local_4 = ((local_4 ^ uVar2) << 1) | (unsigned int)((int)(local_4 ^ uVar2) < 0);
            }
            iVar4 = (int)((unsigned int)iVar4 + 0x23u);
            uVar1 = uVar1 + 1;
        } while ((int)uVar1 < (int)(DAT_009d1308 + DAT_00a07ce8));
    }
    iVar3 = 0;
    iVar4 = 0;
    if (0 < (int)DAT_009eca28) {
        iVar6 = (int)((unsigned int)DAT_00a082c0 * 0x23u);
        do {
            if (*(char *)(DAT_009a1fe8 + 3 + iVar6) != '\0') {
                uVar1 = FUN_004630c0((unsigned int *)(DAT_00a07bd0 + iVar3), 0x412);
                local_4 = ((local_4 ^ uVar1) << 1) | (unsigned int)((int)(local_4 ^ uVar1) < 0);
            }
            iVar3 = (int)((unsigned int)iVar3 + 0x462u);
            iVar4 = iVar4 + 1;
            iVar6 = (int)((unsigned int)iVar6 + 0x23u);
        } while (iVar4 < (int)DAT_009eca28);
    }
    iVar3 = 0;
    iVar4 = 0;
    if (0 < (int)DAT_009fd430) {
        iVar6 = (int)((unsigned int)DAT_00a082d8 * 0x23u);
        do {
            if (*(char *)(DAT_009a1fe8 + 3 + iVar6) != '\0') {
                uVar1 = FUN_004630c0((unsigned int *)(DAT_009a8e18 + iVar3), 10);
                local_4 = ((local_4 ^ uVar1) << 1) | (unsigned int)((int)(local_4 ^ uVar1) < 0);
            }
            iVar3 = (int)((unsigned int)iVar3 + 10u);
            iVar4 = iVar4 + 1;
            iVar6 = (int)((unsigned int)iVar6 + 0x23u);
        } while (iVar4 < (int)DAT_009fd430);
    }
    uVar1 = FUN_004630c0(&DAT_009ed228, 8);
    uVar2 = FUN_004630c0(&DAT_009d8b70, 8);
    uVar2 = (((local_4 ^ uVar1) << 1) | (unsigned int)((int)(local_4 ^ uVar1) < 0)) ^ uVar2;
    local_4 = (uVar2 << 1) | (unsigned int)((int)uVar2 < 0);
    iVar4 = 0;
    if (0 < DAT_009a2000) {
        puVar7 = &DAT_009a20c0;
        do {
            uVar1 = FUN_004630c0(puVar7, 0x126);
            local_4 = ((local_4 ^ uVar1) << 1) | (unsigned int)((int)(local_4 ^ uVar1) < 0);
            puVar7 = (unsigned int *)(int)((unsigned int)(int)puVar7 + 0x126u);
            iVar4 = iVar4 + 1;
        } while (iVar4 < DAT_009a2000);
    }
    uVar1 = FUN_004630c0(&DAT_009d6940, 0xd30);
    uVar2 = (((local_4 ^ uVar1) << 1) | (unsigned int)((int)(local_4 ^ uVar1) < 0)) ^ (unsigned int)DAT_009a8d40;
    uVar1 = FUN_004630c0(&DAT_009ec5e0, 0x16);
    uVar1 = ((uVar2 << 1) | (unsigned int)((int)uVar2 < 0)) ^ uVar1;
    uVar1 = ((uVar1 << 1) | (unsigned int)((int)uVar1 < 0)) ^ (unsigned int)DAT_00a08294;
    uVar2 = FUN_004630c0((unsigned int *)&DAT_009a2000, 0xa2);
    uVar2 = ((uVar1 << 1) | (unsigned int)((int)uVar1 < 0)) ^ uVar2;
    local_4 = (uVar2 << 1) | (unsigned int)((int)uVar2 < 0);
    iVar4 = 0;
    if (0 < DAT_009a2000) {
        puVar7 = &DAT_009d8c30;
        do {
            uVar1 = FUN_004630c0(puVar7, 0x562);
            local_4 = ((local_4 ^ uVar1) << 1) | (unsigned int)((int)(local_4 ^ uVar1) < 0);
            puVar7 = (unsigned int *)(int)((unsigned int)(int)puVar7 + 0x566u);
            iVar4 = iVar4 + 1;
        } while (iVar4 < DAT_009a2000);
    }
    uVar1 = FUN_004630c0((unsigned int *)&DAT_009fe800, 0x1c80);
    local_4 = ((local_4 ^ uVar1) << 1) | (unsigned int)((int)(local_4 ^ uVar1) < 0);
    puVar7 = (unsigned int *)&DAT_009a8080;
    do {
        uVar1 = FUN_004630c0(puVar7, 0x126);
        local_4 = ((local_4 ^ uVar1) << 1) | (unsigned int)((int)(local_4 ^ uVar1) < 0);
        puVar7 = (unsigned int *)(int)((unsigned int)(int)puVar7 + 0x126u);
    } while (puVar7 < (unsigned int *)&DAT_009a8bfc);
    uVar1 = (((local_4 ^ DAT_009fd394) << 1) | (unsigned int)((int)(local_4 ^ DAT_009fd394) < 0)) ^
            (unsigned int)DAT_009d8c20;
    uVar1 = ((uVar1 << 1) | (unsigned int)((int)uVar1 < 0)) ^ DAT_009eca28;
    uVar1 = ((uVar1 << 1) | (unsigned int)((int)uVar1 < 0)) ^ DAT_009a7ba4;
    uVar1 = ((uVar1 << 1) | (unsigned int)((int)uVar1 < 0)) ^ DAT_009fd430;
    uVar1 = ((uVar1 << 1) | (unsigned int)((int)uVar1 < 0)) ^ DAT_00a0814c;
    uVar1 = ((uVar1 << 1) | (unsigned int)((int)uVar1 < 0)) ^ DAT_009a7b50;
    uVar1 = ((uVar1 << 1) | (unsigned int)((int)uVar1 < 0)) ^ DAT_009d767c;
    uVar1 = ((uVar1 << 1) | (unsigned int)((int)uVar1 < 0)) ^ _DAT_00a07cec;
    uVar1 = ((uVar1 << 1) | (unsigned int)((int)uVar1 < 0)) ^ DAT_009a1fe0;
    uVar1 = ((uVar1 << 1) | (unsigned int)((int)uVar1 < 0)) ^ DAT_009d1144;
    uVar1 = ((uVar1 << 1) | (unsigned int)((int)uVar1 < 0)) ^ DAT_009fe7dc;
    uVar1 = ((uVar1 << 1) | (unsigned int)((int)uVar1 < 0)) ^ DAT_009e9640;
    uVar1 = ((uVar1 << 1) | (unsigned int)((int)uVar1 < 0)) ^ DAT_00a00854;
    uVar1 = ((uVar1 << 1) | (unsigned int)((int)uVar1 < 0)) ^ DAT_009a7b58;
    uVar1 = ((uVar1 << 1) | (unsigned int)((int)uVar1 < 0)) ^ DAT_009d1140;
    uVar1 = ((uVar1 << 1) | (unsigned int)((int)uVar1 < 0)) ^ DAT_009d1310;
    uVar1 = ((uVar1 << 1) | (unsigned int)((int)uVar1 < 0)) ^ DAT_009ec47c;
    uVar1 = ((uVar1 << 1) | (unsigned int)((int)uVar1 < 0)) ^ DAT_009a8e24;
    uVar1 = ((uVar1 << 1) | (unsigned int)((int)uVar1 < 0)) ^ DAT_009cd274;
    uVar1 = ((uVar1 << 1) | (unsigned int)((int)uVar1 < 0)) ^ DAT_00a07ce8;
    uVar2 = ((uVar1 << 1) | (unsigned int)((int)uVar1 < 0)) ^ DAT_009d1308;
    uVar1 = FUN_004630c0(&DAT_009d1320, 0x5500);
    uVar1 = ((uVar2 << 1) | (unsigned int)((int)uVar2 < 0)) ^ uVar1;
    uVar2 = ((uVar1 << 1) | (unsigned int)((int)uVar1 < 0)) ^ DAT_009a8068;
    uVar1 = FUN_004630c0((unsigned int *)&DAT_009a8e40, 0x1ffff);
    uVar1 = ((uVar2 << 1) | (unsigned int)((int)uVar2 < 0)) ^ uVar1;
    uVar1 = ((uVar1 << 1) | (unsigned int)((int)uVar1 < 0)) ^ DAT_009a73a4;
    uVar1 = ((uVar1 << 1) | (unsigned int)((int)uVar1 < 0)) ^ (unsigned int)DAT_009d113c;
    uVar1 = ((uVar1 << 1) | (unsigned int)((int)uVar1 < 0)) ^ DAT_00a08138;
    local_4 = (uVar1 << 1) | (unsigned int)((int)uVar1 < 0);
    pcVar5 = &DAT_009e9680;
    do {
        if (*pcVar5 != '\0') {
            uVar1 = FUN_004630c0((unsigned int *)(pcVar5 + -0x10), 0x5bd);
            local_4 = ((local_4 ^ uVar1) << 1) | (unsigned int)((int)(local_4 ^ uVar1) < 0);
        }
        pcVar5 = pcVar5 + 0x5bd;
    } while (pcVar5 < &DAT_009ec468);
    return local_4;
}
