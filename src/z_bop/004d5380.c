/* SPDX-License-Identifier: MIT */
/* z_bop 004d5380 */
extern int __cdecl _sprintf(char *dst, const char *fmt, ...);
extern void * __cdecl FID_conflict___wfopen(const char *filename, const char *mode);
extern int __cdecl _fclose(void *stream);
extern uint __cdecl FUN_004d48e0(void);
extern uint __cdecl FUN_004b7b10(const char *param_1, uint param_2, uint param_3, uint param_4, uint param_5, uint param_6, uint param_7, uint param_8);
extern void __cdecl FUN_004d4970(void);

extern uint DAT_00aa6cfc;
extern uint DAT_00aa7b17;
extern uint DAT_00aa7b1f;
extern char DAT_00aa7b23;
extern const char DAT_0051a854[];

uint __stdcall FUN_004d5380(void)
{
    void *_File;
    uint uVar1;
    int iVar2;
    char local_40[64];

    iVar2 = 0;
    while (1) {
        _sprintf(local_40, "frontscreen%d.bmp", iVar2);
        _File = FID_conflict___wfopen(local_40, DAT_0051a854);
        if (_File == 0) {
            break;
        }
        iVar2 = iVar2 + 1;
        _fclose(_File);
    }
    DAT_00aa6cfc = FUN_004d48e0();
    uVar1 = FUN_004b7b10(local_40, DAT_00aa6cfc, 0x280, 0x1e0, DAT_00aa7b17, DAT_00aa7b1f, (uint)DAT_00aa7b23, 0xaa7b9e);
    FUN_004d4970();
    return uVar1;
}
