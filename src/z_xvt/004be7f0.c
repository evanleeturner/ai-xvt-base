/* SPDX-License-Identifier: MIT */
/* z_xvt 004be7f0 */
extern unsigned int __cdecl FUN_004be6f0(void);
extern int __cdecl _sprintf(char *dst, const char *fmt, ...);
extern long __stdcall RegCreateKeyA(HKEY hKey, LPCSTR lpSubKey, PHKEY phkResult);
extern long __stdcall RegCloseKey(HKEY hKey);
extern long __stdcall RegOpenKeyExA(HKEY hKey, LPCSTR lpSubKey, DWORD ulOptions, REGSAM samDesired, PHKEY phkResult);
extern long __stdcall RegQueryValueExA(HKEY hKey, LPCSTR lpValueName, LPDWORD lpReserved, LPDWORD lpType, LPBYTE lpData, LPDWORD lpcbData);
extern int __cdecl __getdrive(void);
extern int __cdecl __getcwd(char *buf, int len);
extern int __cdecl _tolower(unsigned int c);
extern int __cdecl __chdir(const char *path);

extern char DAT_0081b5da;
extern char DAT_0081b5db;
extern char DAT_0081b5dc;
extern const char s_SOFTWARE_LucasArts_Entertainment_00531aa8[];
extern const char s_SOFTWARE_LucasArts_Entertainment_00531a64[];
extern const char s_Install_Path_00531a54[];

void __cdecl FUN_004be7f0(int param_1)
{
  char cVar1;
  unsigned char bVar2;
  unsigned int uVar3;
  long LVar4;
  int iVar5;
  unsigned int uVar6;
  char *pcVar7;
  HKEY local_108;
  DWORD local_104;
  char local_100[256];

  if (param_1 != 0) {
    uVar3 = FUN_004be6f0();
    DAT_0081b5db = (unsigned char)uVar3;
  }
  _sprintf(local_100, s_SOFTWARE_LucasArts_Entertainment_00531aa8, 1, 1);
  RegCreateKeyA((HKEY)0x80000002, local_100, &local_108);
  RegCloseKey(local_108);
  LVar4 = RegOpenKeyExA((HKEY)0x80000002, s_SOFTWARE_LucasArts_Entertainment_00531a64, 0, 0x20019, &local_108);
  if (LVar4 != 0) {
    iVar5 = __getdrive();
    DAT_0081b5da = (char)iVar5 + '`';
    __getcwd((char *)&DAT_0081b5dc, 0x100);
    return;
  }
  local_104 = 0x100;
  LVar4 = RegQueryValueExA(local_108, s_Install_Path_00531a54, (LPDWORD)0x0, (LPDWORD)0x0, (LPBYTE)&DAT_0081b5dc, &local_104);
  bVar2 = DAT_0081b5dc;
  if (LVar4 != 0) {
    iVar5 = __getdrive();
    DAT_0081b5da = (char)iVar5 + '`';
    __getcwd((char *)&DAT_0081b5dc, 0x100);
    RegCloseKey(local_108);
    return;
  }
  uVar6 = 0xffffffff;
  pcVar7 = (char *)&DAT_0081b5dc;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar1 = *pcVar7;
    pcVar7 = pcVar7 + 1;
  } while (cVar1 != '\0');
  if ((&DAT_0081b5da)[~uVar6] == '\\') {
    uVar6 = 0xffffffff;
    pcVar7 = (char *)&DAT_0081b5dc;
    do {
      if (uVar6 == 0) break;
      uVar6 = uVar6 - 1;
      cVar1 = *pcVar7;
      pcVar7 = pcVar7 + 1;
    } while (cVar1 != '\0');
    (&DAT_0081b5da)[~uVar6] = 0;
  }
  iVar5 = _tolower((unsigned int)bVar2);
  DAT_0081b5da = (char)iVar5;
  RegCloseKey(local_108);
  __chdir((char *)&DAT_0081b5dc);
  return;
}
