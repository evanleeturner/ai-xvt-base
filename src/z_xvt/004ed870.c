/* SPDX-License-Identifier: MIT */
/* z_xvt 004ed870 */
HFILE __cdecl _fopen(const char *filename, const char *mode);
size_t __cdecl _fread(void *ptr, size_t size, size_t count, HFILE stream);
int __cdecl _fclose(HFILE stream);

extern const char s_cdrom_cfg_00533778[];
extern const char DAT_0052a330[];
extern unsigned short DAT_00757c68;

char __stdcall FUN_004ed870(void)
{
    HFILE _File;
    size_t sVar1;
    
    _File = _fopen(s_cdrom_cfg_00533778, DAT_0052a330);
    if (_File != (HFILE)0x0) {
        sVar1 = _fread(&DAT_00757c68, 1, 8, _File);
        _fclose(_File);
        if (sVar1 == 8) {
            return (((unsigned char)((unsigned short)DAT_00757c68 >> 8) & 0x80) == 0x80) + 1;
        }
    }
    return 0;
}
