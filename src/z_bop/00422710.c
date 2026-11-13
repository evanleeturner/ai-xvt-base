/* SPDX-License-Identifier: MIT */
/* z_bop 00422710 */
extern unsigned int DAT_009fe7d4;

void __stdcall FUN_004229f0(void);
void __stdcall FUN_00422d90(void);
void __stdcall FUN_004230f0(void);
void __stdcall FUN_00423480(void);
void __stdcall FUN_00423810(void);
void __stdcall FUN_00423b90(void);
void __stdcall FUN_00423f30(void);
void __stdcall FUN_004242e0(void);

void __stdcall FUN_00422710(void)
{
    unsigned short uVar1;
    
    uVar1 = *(unsigned short *)(DAT_009fe7d4 + 0x16);
    if (uVar1 < 8) {
        switch (uVar1) {
            case 0:
                FUN_004229f0();
                break;
            case 1:
                FUN_00422d90();
                break;
            case 2:
                FUN_004230f0();
                break;
            case 3:
                FUN_00423480();
                break;
            case 4:
                FUN_00423810();
                break;
            case 5:
                FUN_00423b90();
                break;
            case 6:
                FUN_00423f30();
                break;
            case 7:
                FUN_004242e0();
                break;
        }
    }
    return;
}
