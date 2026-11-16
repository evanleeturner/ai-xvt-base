/* SPDX-License-Identifier: MIT */
/* z_bop 004d2cb0 */
extern unsigned int DAT_00665418;
extern unsigned int DAT_00665018;
extern const char s_EnableAutodial_0052ba10[];
extern const char s_Software_Microsoft_Windows_Curre_0052ba20[];

LONG __stdcall RegOpenKeyExA(HKEY hKey, LPCSTR lpSubKey, DWORD ulOptions, REGSAM samDesired, PHKEY phkResult);
LONG __stdcall RegSetValueExA(HKEY hKey, LPCSTR lpValueName, DWORD Reserved, DWORD dwType, const BYTE *lpData, DWORD cbData);
LONG __stdcall RegCloseKey(HKEY hKey);

unsigned int __stdcall FUN_004d2cb0(void)
{
    LONG lStatus;
    HKEY hKey;

    if (DAT_00665418 == 0) {
        return 0;
    }
    lStatus = RegOpenKeyExA((HKEY)0x80000001, s_Software_Microsoft_Windows_Curre_0052ba20, 0, 0xf003f, &hKey);
    if (lStatus != 0) {
        return 0;
    }
    lStatus = RegSetValueExA(hKey, s_EnableAutodial_0052ba10, 0, 3, (const BYTE *)&DAT_00665018, 4);
    if (lStatus != 0) {
        RegCloseKey(hKey);
        return 0;
    }
    DAT_00665418 = 0;
    RegCloseKey(hKey);
    return 1;
}
