/* SPDX-License-Identifier: MIT */
/* z_bop 004d3430 */
extern DWORD DAT_00aa7fd7;
extern DWORD DAT_00aa7fef;
extern DWORD DAT_00aa7ff7;
extern DWORD DAT_00aa7fe3;
extern DWORD DAT_00aa7feb;

extern void __cdecl FUN_004d3020(DWORD, DWORD, unsigned char);
extern DWORD __stdcall GetTickCount(void);

DWORD __stdcall FUN_004d3430(void)
{
    DWORD DVar1;
    DWORD v_aa7fd7;
    DWORD v_aa7fef;
    DWORD v_aa7feb;
    DWORD v_aa7ff7;
    DWORD v_aa7fe3;

    __builtin_memcpy(&v_aa7fd7, &DAT_00aa7fd7, 4);
    __builtin_memcpy(&v_aa7fef, &DAT_00aa7fef, 4);
    __builtin_memcpy(&v_aa7feb, &DAT_00aa7feb, 4);

    FUN_004d3020(v_aa7fd7, v_aa7fef / 60000, (unsigned char)((v_aa7fef % 60000) / 1000));
    DVar1 = GetTickCount();
    v_aa7ff7 = 0;
    __builtin_memcpy(&DAT_00aa7ff7, &v_aa7ff7, 4);
    v_aa7fe3 = v_aa7feb + DVar1 + 2000;
    __builtin_memcpy(&DAT_00aa7fe3, &v_aa7fe3, 4);
    return 1;
}
