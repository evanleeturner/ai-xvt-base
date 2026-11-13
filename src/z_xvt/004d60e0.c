/* SPDX-License-Identifier: MIT */
/* z_xvt 004d60e0 */
extern HFILE __cdecl _fopen(char *filename, char *mode);
extern bool __cdecl FUN_004be320(HFILE stream, void *buf, size_t count);
extern char s_video_cfg_00532a10[];
extern char DAT_00523570[];
extern char DAT_0064f640[];

uint __stdcall FUN_004d60e0(void)
{
    HFILE hFile;
    bool bRet;
    
    hFile = _fopen(s_video_cfg_00532a10, (char *)&DAT_00523570);
    if (hFile == 0) {
        return 0;
    }
    bRet = FUN_004be320(hFile, &DAT_0064f640, 0x10);
    return (((short)bRet == 0) - 1) & 0x64f640;
}
