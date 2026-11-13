/* SPDX-License-Identifier: MIT */
/* z_bop 004cccc0 */
// Forward declarations for called functions
unsigned int __cdecl FUN_004ccbc0(void);
int __cdecl _sprintf(char *dst, const char *fmt, ...);
LONG __stdcall RegCreateKeyA(HKEY hKey, LPCSTR lpSubKey, PHKEY phkResult);
LONG __stdcall RegCloseKey(HKEY hKey);
LONG __stdcall RegOpenKeyExA(HKEY hKey, LPCSTR lpSubKey, DWORD ulOptions, REGSAM samDesired, PHKEY phkResult);
int __cdecl __getdrive(void);
char * __cdecl __getcwd(char *buf, int size);
int __cdecl __chdir(const char *path);
int __cdecl _tolower(unsigned int c);
LONG __stdcall RegQueryValueExA(HKEY hKey, LPCSTR lpValueName, LPDWORD lpReserved, LPDWORD lpType, LPBYTE lpData, LPDWORD lpcbData);

// Global placeholders (addresses from evidence)
extern char DAT_00b69aa5;
extern char DAT_00b69aa6;
extern char DAT_00b69aa7;
extern char DAT_00b69ba7;
extern char DAT_0052b79c[];
extern const char s_SOFTWARE_LucasArts_Entertainment_0052b7e4[];
extern const char s_SOFTWARE_LucasArts_Entertainment_0052b7a0[];
extern const char s_Install_Path_0052b78c[];
extern const char s_SOFTWARE_LucasArts_Entertainment_0052b748[];

void __cdecl FUN_004cccc0(int param_1)
{
    char cVar1;
    unsigned char bVar2;
    unsigned int uVar3;
    LONG LVar4;
    int iVar5;
    unsigned int uVar6;
    char *pcVar7;
    HKEY local_108;
    DWORD local_104;
    char local_100[256];

    if (param_1 != 0) {
        uVar3 = FUN_004ccbc0();
        DAT_00b69aa6 = (char)uVar3;
    }
    _sprintf(local_100, s_SOFTWARE_LucasArts_Entertainment_0052b7e4, 2, 0);
    RegCreateKeyA((HKEY)0x80000002, local_100, &local_108);
    RegCloseKey(local_108);
    LVar4 = RegOpenKeyExA((HKEY)0x80000002, s_SOFTWARE_LucasArts_Entertainment_0052b7a0, 0, 0x20019, &local_108);
    if (LVar4 != 0) {
        iVar5 = __getdrive();
        DAT_00b69aa5 = (char)iVar5 + '`';
        __getcwd((char *)&DAT_00b69aa7, 0x100);
        __chdir(DAT_0052b79c);
        __getcwd(&DAT_00b69ba7, 0x100);
        __chdir((char *)&DAT_00b69aa7);
        return;
    }
    local_104 = 0x100;
    LVar4 = RegQueryValueExA(local_108, s_Install_Path_0052b78c, (LPDWORD)0x0, (LPDWORD)0x0, (LPBYTE)&DAT_00b69aa7, &local_104);
    bVar2 = (unsigned char)DAT_00b69aa7;
    if (LVar4 != 0) {
        iVar5 = __getdrive();
        DAT_00b69aa5 = (char)iVar5 + '`';
        __getcwd((char *)&DAT_00b69aa7, 0x100);
        __chdir(DAT_0052b79c);
        __getcwd(&DAT_00b69ba7, 0x100);
        __chdir((char *)&DAT_00b69aa7);
        RegCloseKey(local_108);
        return;
    }
    uVar6 = 0xFFFFFFFF;
    pcVar7 = (char *)&DAT_00b69aa7;
    do {
        if (uVar6 == 0) break;
        uVar6 = uVar6 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
    } while (cVar1 != '\0');
    if ((&DAT_00b69aa5)[~uVar6] == '\\') {
        uVar6 = 0xFFFFFFFF;
        pcVar7 = (char *)&DAT_00b69aa7;
        do {
            if (uVar6 == 0) break;
            uVar6 = uVar6 - 1;
            cVar1 = *pcVar7;
            pcVar7 = pcVar7 + 1;
        } while (cVar1 != '\0');
        (&DAT_00b69aa5)[~uVar6] = 0;
    }
    iVar5 = _tolower((unsigned int)bVar2);
    DAT_00b69aa5 = (char)iVar5;
    RegCloseKey(local_108);
    __chdir((char *)&DAT_00b69aa7);
    LVar4 = RegOpenKeyExA((HKEY)0x80000002, s_SOFTWARE_LucasArts_Entertainment_0052b748, 0, 0x20019, &local_108);
    if (LVar4 != 0) {
        __chdir(DAT_0052b79c);
        __getcwd(&DAT_00b69ba7, 0x100);
        __chdir((char *)&DAT_00b69aa7);
        return;
    }
    local_104 = 0x100;
    LVar4 = RegQueryValueExA(local_108, s_Install_Path_0052b78c, (LPDWORD)0x0, (LPDWORD)0x0, (LPBYTE)&DAT_00b69ba7, &local_104);
    if (LVar4 != 0) {
        __chdir(DAT_0052b79c);
        __getcwd(&DAT_00b69ba7, 0x100);
        __chdir((char *)&DAT_00b69aa7);
        RegCloseKey(local_108);
        return;
    }
    RegCloseKey(local_108);
    return;
}
