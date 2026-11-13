/* SPDX-License-Identifier: MIT */
/* z_xvt 004cf740 */
extern int DAT_0064f438;
extern int DAT_0064f038;
extern const char s_Software_Microsoft_Windows_Curre_00532528[];
extern const char s_EnableAutodial_00532518[];

LONG __stdcall RegOpenKeyExA(HKEY hKey, LPCSTR lpSubKey, DWORD ulOptions, REGSAM samDesired, PHKEY phkResult);
LONG __stdcall RegQueryValueExA(HKEY hKey, LPCSTR lpValueName, LPDWORD lpReserved, LPDWORD lpType, LPBYTE lpData, LPDWORD lpcbData);
LONG __stdcall RegSetValueExA(HKEY hKey, LPCSTR lpValueName, DWORD Reserved, DWORD dwType, const BYTE *lpData, DWORD cbData);
LONG __stdcall RegCloseKey(HKEY hKey);

int __stdcall FUN_004cf740(void)
{
    LONG lStatus;
    HKEY local_c;
    DWORD local_8;
    BYTE local_4[4];

    DAT_0064f438 = 0;
    lStatus = RegOpenKeyExA((HKEY)0x80000001, s_Software_Microsoft_Windows_Curre_00532528, 0, 0xf003f, &local_c);
    if (lStatus != 0) {
        return 0;
    }
    local_8 = 5;
    lStatus = RegQueryValueExA(local_c, s_EnableAutodial_00532518, (LPDWORD)0x0, (LPDWORD)0x0, (LPBYTE)&DAT_0064f038, &local_8);
    if (lStatus != 0) {
        RegCloseKey(local_c);
        return 0;
    }
    if (DAT_0064f038 == 0) {
        RegCloseKey(local_c);
        return 0;
    }
    local_4[0] = 0;
    local_4[1] = 0;
    local_4[2] = 0;
    local_4[3] = 0;
    lStatus = RegSetValueExA(local_c, s_EnableAutodial_00532518, 0, 3, local_4, 4);
    if (lStatus != 0) {
        RegCloseKey(local_c);
        return 0;
    }
    DAT_0064f438 = 1;
    RegCloseKey(local_c);
    return 1;
}
