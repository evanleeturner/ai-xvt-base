/*
 * xvt.h - the types and imports every rebuilt X-Wing vs. TIE Fighter
 * function may use.
 *
 * The 1997 game was built against the Win32 SDK; a rebuilt function is
 * checked without it (tools/compile-check), so the base Win32 types it
 * needs are declared here at their i386 widths, under Microsoft's names.
 * Ghidra's decompiler type names are declared too, so a function still
 * carrying them compiles; a rebuilt name replaces each one as the
 * evidence allows.
 *
 * Then every function the two executables (Z_XVT__.EXE, the base game,
 * and z_bop__.exe, Balance of Power) import from a DLL, grouped by DLL,
 * each declared as the SDK header its group names declares it, with the
 * calling convention spelled as a keyword. A structure an import takes by
 * pointer is declared but not laid out: its fields are written when a
 * rebuilt function first reads one.
 *
 * Declares no game structures and no game functions: a rebuilt function
 * declares the game functions it calls itself.
 */
#ifndef XVT_H
#define XVT_H

#define NULL ((void *)0)
#define TRUE 1
#define FALSE 0

/* Win32 base types, i386 widths */
typedef unsigned char BYTE;
typedef unsigned short WORD;
typedef unsigned long DWORD;
typedef int BOOL;
typedef long LONG;
typedef unsigned int UINT;
typedef char CHAR;
typedef unsigned short WCHAR;
typedef void *LPVOID;
typedef const void *LPCVOID;
typedef char *LPSTR;
typedef const char *LPCSTR;
typedef WCHAR *LPWSTR;
typedef const WCHAR *LPCWSTR;
typedef BYTE *LPBYTE;
typedef BYTE *PBYTE;
typedef WORD *LPWORD;
typedef DWORD *LPDWORD;
typedef LONG *LPLONG;
typedef BOOL *LPBOOL;
typedef WORD ATOM;
typedef DWORD COLORREF;
typedef LONG HRESULT;
typedef UINT WPARAM;
typedef LONG LPARAM;
typedef LONG LRESULT;
typedef int HFILE;
typedef DWORD LCID;
typedef DWORD LCTYPE;
typedef void *HANDLE;
typedef HANDLE HWND;
typedef HANDLE HINSTANCE;
typedef HINSTANCE HMODULE;
typedef HANDLE HGLOBAL;
typedef HANDLE HLOCAL;
typedef HANDLE HRSRC;
typedef HANDLE HKEY;
typedef HANDLE HDC;
typedef HANDLE HGDIOBJ;
typedef HANDLE HFONT;
typedef HANDLE HICON;
typedef HICON HCURSOR;
typedef HANDLE HMENU;
/* the SDK's STRICT form, (void); without STRICT it is declared () */
typedef int(__stdcall *FARPROC)(void);

/* Ghidra's decompiler types, by width */
typedef unsigned char undefined;
typedef unsigned char undefined1;
typedef unsigned short undefined2;
typedef unsigned int undefined4;
typedef unsigned long long undefined8;
typedef unsigned char byte;
typedef unsigned char uchar;
typedef unsigned short ushort;
typedef unsigned short word;
typedef unsigned int uint;
typedef unsigned int dword;
typedef unsigned long ulong;
typedef unsigned char bool;
/* x87 80-bit in the binary; MSVC's long double is only 64-bit */
typedef long double float10;
/* Ghidra's code: a function of unknown parameters, called via (code *) */
typedef void code();

/*
 * KERNEL32.dll, as the SDK's winbase.h declares it; SetConsoleCtrlHandler
 * is wincon.h's and RtlUnwind is winnt.h's. Every one is WINAPI, which is
 * __stdcall. InterlockedIncrement and InterlockedDecrement are kept as the
 * binary imports them, KERNEL32's exports by name: mingw-w64's winnt.h
 * maps both to the compiler's __cdecl intrinsics (l.2380, l.2383).
 */
typedef struct _SECURITY_ATTRIBUTES SECURITY_ATTRIBUTES,
    *LPSECURITY_ATTRIBUTES;
typedef struct _STARTUPINFOA STARTUPINFOA, *LPSTARTUPINFOA;
typedef struct _PROCESS_INFORMATION PROCESS_INFORMATION,
    *LPPROCESS_INFORMATION;
typedef struct _WIN32_FIND_DATAA WIN32_FIND_DATAA, *LPWIN32_FIND_DATAA;
typedef struct _SYSTEMTIME SYSTEMTIME, *LPSYSTEMTIME;
typedef struct _TIME_ZONE_INFORMATION TIME_ZONE_INFORMATION,
    *LPTIME_ZONE_INFORMATION;
typedef struct _cpinfo CPINFO, *LPCPINFO;
typedef struct _OVERLAPPED OVERLAPPED, *LPOVERLAPPED;
typedef struct _RTL_CRITICAL_SECTION CRITICAL_SECTION, *LPCRITICAL_SECTION;
typedef struct _EXCEPTION_RECORD EXCEPTION_RECORD, *PEXCEPTION_RECORD;
typedef struct _EXCEPTION_POINTERS EXCEPTION_POINTERS;
typedef DWORD(__stdcall *LPTHREAD_START_ROUTINE)(LPVOID lpThreadParameter);
typedef BOOL(__stdcall *PHANDLER_ROUTINE)(DWORD CtrlType);

BOOL __stdcall CloseHandle(HANDLE hObject);
int __stdcall CompareStringA(LCID Locale, DWORD dwCmpFlags, LPCSTR lpString1,
    int cchCount1, LPCSTR lpString2, int cchCount2);
int __stdcall CompareStringW(LCID Locale, DWORD dwCmpFlags,
    LPCWSTR lpString1, int cchCount1, LPCWSTR lpString2, int cchCount2);
HANDLE __stdcall CreateFileA(LPCSTR lpFileName, DWORD dwDesiredAccess,
    DWORD dwShareMode, LPSECURITY_ATTRIBUTES lpSecurityAttributes,
    DWORD dwCreationDisposition, DWORD dwFlagsAndAttributes,
    HANDLE hTemplateFile);
BOOL __stdcall CreateProcessA(LPCSTR lpApplicationName, LPSTR lpCommandLine,
    LPSECURITY_ATTRIBUTES lpProcessAttributes,
    LPSECURITY_ATTRIBUTES lpThreadAttributes, BOOL bInheritHandles,
    DWORD dwCreationFlags, LPVOID lpEnvironment, LPCSTR lpCurrentDirectory,
    LPSTARTUPINFOA lpStartupInfo,
    LPPROCESS_INFORMATION lpProcessInformation);
HANDLE __stdcall CreateThread(LPSECURITY_ATTRIBUTES lpThreadAttributes,
    DWORD dwStackSize, LPTHREAD_START_ROUTINE lpStartAddress,
    LPVOID lpParameter, DWORD dwCreationFlags, LPDWORD lpThreadId);
void __stdcall DebugBreak(void);
void __stdcall DeleteCriticalSection(LPCRITICAL_SECTION lpCriticalSection);
BOOL __stdcall DeleteFileA(LPCSTR lpFileName);
void __stdcall EnterCriticalSection(LPCRITICAL_SECTION lpCriticalSection);
void __stdcall ExitProcess(UINT uExitCode);
void __stdcall ExitThread(DWORD dwExitCode);
void __stdcall FatalAppExitA(UINT uAction, LPCSTR lpMessageText);
BOOL __stdcall FindClose(HANDLE hFindFile);
HANDLE __stdcall FindFirstFileA(LPCSTR lpFileName,
    LPWIN32_FIND_DATAA lpFindFileData);
BOOL __stdcall FindNextFileA(HANDLE hFindFile,
    LPWIN32_FIND_DATAA lpFindFileData);
HRSRC __stdcall FindResourceA(HMODULE hModule, LPCSTR lpName, LPCSTR lpType);
BOOL __stdcall FlushFileBuffers(HANDLE hFile);
BOOL __stdcall FreeEnvironmentStringsA(LPSTR lpszEnvironmentBlock);
BOOL __stdcall FreeEnvironmentStringsW(LPWSTR lpszEnvironmentBlock);
UINT __stdcall GetACP(void);
BOOL __stdcall GetCPInfo(UINT CodePage, LPCPINFO lpCPInfo);
LPSTR __stdcall GetCommandLineA(void);
DWORD __stdcall GetCurrentDirectoryA(DWORD nBufferLength, LPSTR lpBuffer);
HANDLE __stdcall GetCurrentProcess(void);
HANDLE __stdcall GetCurrentThread(void);
DWORD __stdcall GetCurrentThreadId(void);
UINT __stdcall GetDriveTypeA(LPCSTR lpRootPathName);
LPSTR __stdcall GetEnvironmentStrings(void);
LPWSTR __stdcall GetEnvironmentStringsW(void);
DWORD __stdcall GetFileAttributesA(LPCSTR lpFileName);
DWORD __stdcall GetFileType(HANDLE hFile);
DWORD __stdcall GetFullPathNameA(LPCSTR lpFileName, DWORD nBufferLength,
    LPSTR lpBuffer, LPSTR *lpFilePart);
DWORD __stdcall GetLastError(void);
void __stdcall GetLocalTime(LPSYSTEMTIME lpSystemTime);
int __stdcall GetLocaleInfoA(LCID Locale, LCTYPE LCType, LPSTR lpLCData,
    int cchData);
int __stdcall GetLocaleInfoW(LCID Locale, LCTYPE LCType, LPWSTR lpLCData,
    int cchData);
DWORD __stdcall GetLogicalDriveStringsA(DWORD nBufferLength, LPSTR lpBuffer);
DWORD __stdcall GetModuleFileNameA(HMODULE hModule, LPSTR lpFilename,
    DWORD nSize);
HMODULE __stdcall GetModuleHandleA(LPCSTR lpModuleName);
UINT __stdcall GetOEMCP(void);
FARPROC __stdcall GetProcAddress(HMODULE hModule, LPCSTR lpProcName);
void __stdcall GetStartupInfoA(LPSTARTUPINFOA lpStartupInfo);
HANDLE __stdcall GetStdHandle(DWORD nStdHandle);
BOOL __stdcall GetStringTypeA(LCID Locale, DWORD dwInfoType, LPCSTR lpSrcStr,
    int cchSrc, LPWORD lpCharType);
BOOL __stdcall GetStringTypeW(DWORD dwInfoType, LPCWSTR lpSrcStr, int cchSrc,
    LPWORD lpCharType);
void __stdcall GetSystemTime(LPSYSTEMTIME lpSystemTime);
int __stdcall GetThreadPriority(HANDLE hThread);
DWORD __stdcall GetTickCount(void);
DWORD __stdcall GetTimeZoneInformation(
    LPTIME_ZONE_INFORMATION lpTimeZoneInformation);
LCID __stdcall GetUserDefaultLCID(void);
DWORD __stdcall GetVersion(void);
BOOL __stdcall GetVolumeInformationA(LPCSTR lpRootPathName,
    LPSTR lpVolumeNameBuffer, DWORD nVolumeNameSize,
    LPDWORD lpVolumeSerialNumber, LPDWORD lpMaximumComponentLength,
    LPDWORD lpFileSystemFlags, LPSTR lpFileSystemNameBuffer,
    DWORD nFileSystemNameSize);
HGLOBAL __stdcall GlobalAlloc(UINT uFlags, DWORD dwBytes);
HGLOBAL __stdcall GlobalFree(HGLOBAL hMem);
HGLOBAL __stdcall GlobalHandle(LPCVOID pMem);
LPVOID __stdcall GlobalLock(HGLOBAL hMem);
BOOL __stdcall GlobalUnlock(HGLOBAL hMem);
LPVOID __stdcall HeapAlloc(HANDLE hHeap, DWORD dwFlags, DWORD dwBytes);
HANDLE __stdcall HeapCreate(DWORD flOptions, DWORD dwInitialSize,
    DWORD dwMaximumSize);
BOOL __stdcall HeapDestroy(HANDLE hHeap);
BOOL __stdcall HeapFree(HANDLE hHeap, DWORD dwFlags, LPVOID lpMem);
LPVOID __stdcall HeapReAlloc(HANDLE hHeap, DWORD dwFlags, LPVOID lpMem,
    DWORD dwBytes);
BOOL __stdcall HeapValidate(HANDLE hHeap, DWORD dwFlags, LPCVOID lpMem);
void __stdcall InitializeCriticalSection(
    LPCRITICAL_SECTION lpCriticalSection);
LONG __stdcall InterlockedDecrement(LPLONG lpAddend);
LONG __stdcall InterlockedIncrement(LPLONG lpAddend);
BOOL __stdcall IsBadReadPtr(LPCVOID lp, UINT ucb);
BOOL __stdcall IsBadWritePtr(LPVOID lp, UINT ucb);
BOOL __stdcall IsValidCodePage(UINT CodePage);
BOOL __stdcall IsValidLocale(LCID Locale, DWORD dwFlags);
int __stdcall LCMapStringA(LCID Locale, DWORD dwMapFlags, LPCSTR lpSrcStr,
    int cchSrc, LPSTR lpDestStr, int cchDest);
int __stdcall LCMapStringW(LCID Locale, DWORD dwMapFlags, LPCWSTR lpSrcStr,
    int cchSrc, LPWSTR lpDestStr, int cchDest);
void __stdcall LeaveCriticalSection(LPCRITICAL_SECTION lpCriticalSection);
HMODULE __stdcall LoadLibraryA(LPCSTR lpLibFileName);
HGLOBAL __stdcall LoadResource(HMODULE hModule, HRSRC hResInfo);
HLOCAL __stdcall LocalAlloc(UINT uFlags, UINT uBytes);
HLOCAL __stdcall LocalFree(HLOCAL hMem);
LPVOID __stdcall LockResource(HGLOBAL hResData);
int __stdcall MultiByteToWideChar(UINT CodePage, DWORD dwFlags,
    LPCSTR lpMultiByteStr, int cchMultiByte, LPWSTR lpWideCharStr,
    int cchWideChar);
void __stdcall OutputDebugStringA(LPCSTR lpOutputString);
BOOL __stdcall ReadFile(HANDLE hFile, LPVOID lpBuffer,
    DWORD nNumberOfBytesToRead, LPDWORD lpNumberOfBytesRead,
    LPOVERLAPPED lpOverlapped);
DWORD __stdcall ResumeThread(HANDLE hThread);
void __stdcall RtlUnwind(LPVOID TargetFrame, LPVOID TargetIp,
    PEXCEPTION_RECORD ExceptionRecord, LPVOID ReturnValue);
BOOL __stdcall SetConsoleCtrlHandler(PHANDLER_ROUTINE HandlerRoutine,
    BOOL Add);
BOOL __stdcall SetCurrentDirectoryA(LPCSTR lpPathName);
BOOL __stdcall SetEndOfFile(HANDLE hFile);
BOOL __stdcall SetEnvironmentVariableA(LPCSTR lpName, LPCSTR lpValue);
DWORD __stdcall SetFilePointer(HANDLE hFile, LONG lDistanceToMove,
    LPLONG lpDistanceToMoveHigh, DWORD dwMoveMethod);
UINT __stdcall SetHandleCount(UINT uNumber);
void __stdcall SetLastError(DWORD dwErrCode);
BOOL __stdcall SetStdHandle(DWORD nStdHandle, HANDLE hHandle);
BOOL __stdcall SetThreadPriority(HANDLE hThread, int nPriority);
void __stdcall Sleep(DWORD dwMilliseconds);
DWORD __stdcall SuspendThread(HANDLE hThread);
BOOL __stdcall TerminateProcess(HANDLE hProcess, UINT uExitCode);
BOOL __stdcall TerminateThread(HANDLE hThread, DWORD dwExitCode);
DWORD __stdcall TlsAlloc(void);
BOOL __stdcall TlsFree(DWORD dwTlsIndex);
LPVOID __stdcall TlsGetValue(DWORD dwTlsIndex);
BOOL __stdcall TlsSetValue(DWORD dwTlsIndex, LPVOID lpTlsValue);
LONG __stdcall UnhandledExceptionFilter(EXCEPTION_POINTERS *ExceptionInfo);
LPVOID __stdcall VirtualAlloc(LPVOID lpAddress, DWORD dwSize,
    DWORD flAllocationType, DWORD flProtect);
BOOL __stdcall VirtualFree(LPVOID lpAddress, DWORD dwSize, DWORD dwFreeType);
BOOL __stdcall VirtualProtect(LPVOID lpAddress, DWORD dwSize,
    DWORD flNewProtect, LPDWORD lpflOldProtect);
int __stdcall WideCharToMultiByte(UINT CodePage, DWORD dwFlags,
    LPCWSTR lpWideCharStr, int cchWideChar, LPSTR lpMultiByteStr,
    int cchMultiByte, LPCSTR lpDefaultChar, LPBOOL lpUsedDefaultChar);
BOOL __stdcall WriteFile(HANDLE hFile, LPCVOID lpBuffer,
    DWORD nNumberOfBytesToWrite, LPDWORD lpNumberOfBytesWritten,
    LPOVERLAPPED lpOverlapped);
HFILE __stdcall _lclose(HFILE hFile);
HFILE __stdcall _lopen(LPCSTR lpPathName, int iReadWrite);
UINT __stdcall _lread(HFILE hFile, LPVOID lpBuffer, UINT uBytes);

/*
 * USER32.dll, as the SDK's winuser.h declares it. Every one is WINAPI,
 * __stdcall, except wsprintfA: it takes a variable argument list, so it
 * is WINAPIV, __cdecl, and its caller clears the stack.
 */
typedef struct tagRECT RECT, *LPRECT;
typedef struct tagPOINT POINT, *LPPOINT;
typedef struct tagMSG MSG, *LPMSG;
typedef struct tagPAINTSTRUCT PAINTSTRUCT, *LPPAINTSTRUCT;
typedef struct tagWNDCLASSA WNDCLASSA;

HDC __stdcall BeginPaint(HWND hWnd, LPPAINTSTRUCT lpPaint);
BOOL __stdcall ClientToScreen(HWND hWnd, LPPOINT lpPoint);
HWND __stdcall CreateWindowExA(DWORD dwExStyle, LPCSTR lpClassName,
    LPCSTR lpWindowName, DWORD dwStyle, int X, int Y, int nWidth,
    int nHeight, HWND hWndParent, HMENU hMenu, HINSTANCE hInstance,
    LPVOID lpParam);
LRESULT __stdcall DefWindowProcA(HWND hWnd, UINT Msg, WPARAM wParam,
    LPARAM lParam);
BOOL __stdcall DestroyWindow(HWND hWnd);
LRESULT __stdcall DispatchMessageA(const MSG *lpMsg);
int __stdcall DrawTextA(HDC hDC, LPCSTR lpString, int nCount, LPRECT lpRect,
    UINT uFormat);
BOOL __stdcall EndPaint(HWND hWnd, const PAINTSTRUCT *lpPaint);
HWND __stdcall FindWindowA(LPCSTR lpClassName, LPCSTR lpWindowName);
BOOL __stdcall GetCursorPos(LPPOINT lpPoint);
HDC __stdcall GetDC(HWND hWnd);
HWND __stdcall GetFocus(void);
HWND __stdcall GetForegroundWindow(void);
BOOL __stdcall GetKeyboardState(PBYTE lpKeyState);
BOOL __stdcall GetMessageA(LPMSG lpMsg, HWND hWnd, UINT wMsgFilterMin,
    UINT wMsgFilterMax);
int __stdcall GetSystemMetrics(int nIndex);
BOOL __stdcall GetUpdateRect(HWND hWnd, LPRECT lpRect, BOOL bErase);
HCURSOR __stdcall LoadCursorA(HINSTANCE hInstance, LPCSTR lpCursorName);
HICON __stdcall LoadIconA(HINSTANCE hInstance, LPCSTR lpIconName);
HANDLE __stdcall LoadImageA(HINSTANCE hInst, LPCSTR lpszName, UINT uType,
    int cxDesired, int cyDesired, UINT fuLoad);
BOOL __stdcall MessageBeep(UINT uType);
int __stdcall MessageBoxA(HWND hWnd, LPCSTR lpText, LPCSTR lpCaption,
    UINT uType);
BOOL __stdcall PeekMessageA(LPMSG lpMsg, HWND hWnd, UINT wMsgFilterMin,
    UINT wMsgFilterMax, UINT wRemoveMsg);
BOOL __stdcall PostMessageA(HWND hWnd, UINT Msg, WPARAM wParam,
    LPARAM lParam);
void __stdcall PostQuitMessage(int nExitCode);
ATOM __stdcall RegisterClassA(const WNDCLASSA *lpWndClass);
BOOL __stdcall ReleaseCapture(void);
int __stdcall ReleaseDC(HWND hWnd, HDC hDC);
HWND __stdcall SetCapture(HWND hWnd);
HCURSOR __stdcall SetCursor(HCURSOR hCursor);
BOOL __stdcall SetCursorPos(int X, int Y);
HWND __stdcall SetFocus(HWND hWnd);
BOOL __stdcall SetForegroundWindow(HWND hWnd);
BOOL __stdcall SetRect(LPRECT lprc, int xLeft, int yTop, int xRight,
    int yBottom);
BOOL __stdcall SetWindowTextA(HWND hWnd, LPCSTR lpString);
int __stdcall ShowCursor(BOOL bShow);
BOOL __stdcall ShowWindowAsync(HWND hWnd, int nCmdShow);
BOOL __stdcall TranslateMessage(const MSG *lpMsg);
BOOL __stdcall UpdateWindow(HWND hWnd);
int __cdecl wsprintfA(LPSTR lpOut, LPCSTR lpFmt, ...);

/*
 * GDI32.dll, as the SDK's wingdi.h declares it. Every one is WINAPI,
 * __stdcall.
 */
typedef struct tagSIZE SIZE, *LPSIZE;
typedef struct tagPALETTEENTRY PALETTEENTRY, *LPPALETTEENTRY;
typedef struct _devicemodeA DEVMODEA;

HDC __stdcall CreateCompatibleDC(HDC hdc);
HDC __stdcall CreateDCA(LPCSTR lpszDriver, LPCSTR lpszDevice,
    LPCSTR lpszOutput, const DEVMODEA *lpInitData);
HFONT __stdcall CreateFontA(int nHeight, int nWidth, int nEscapement,
    int nOrientation, int fnWeight, DWORD fdwItalic, DWORD fdwUnderline,
    DWORD fdwStrikeOut, DWORD fdwCharSet, DWORD fdwOutputPrecision,
    DWORD fdwClipPrecision, DWORD fdwQuality, DWORD fdwPitchAndFamily,
    LPCSTR lpszFace);
BOOL __stdcall DeleteDC(HDC hdc);
BOOL __stdcall DeleteObject(HGDIOBJ hObject);
BOOL __stdcall ExtTextOutA(HDC hdc, int X, int Y, UINT fuOptions,
    const RECT *lprc, LPCSTR lpString, UINT cbCount, const int *lpDx);
int __stdcall GetObjectA(HGDIOBJ hgdiobj, int cbBuffer, LPVOID lpvObject);
COLORREF __stdcall GetPixel(HDC hdc, int nXPos, int nYPos);
HGDIOBJ __stdcall GetStockObject(int fnObject);
UINT __stdcall GetSystemPaletteEntries(HDC hdc, UINT iStartIndex,
    UINT nEntries, LPPALETTEENTRY lppe);
BOOL __stdcall GetTextExtentPoint32A(HDC hdc, LPCSTR lpString, int cbString,
    LPSIZE lpSize);
BOOL __stdcall Rectangle(HDC hdc, int nLeftRect, int nTopRect,
    int nRightRect, int nBottomRect);
HGDIOBJ __stdcall SelectObject(HDC hdc, HGDIOBJ hgdiobj);
COLORREF __stdcall SetBkColor(HDC hdc, COLORREF crColor);
int __stdcall SetBkMode(HDC hdc, int iBkMode);
int __stdcall SetMapMode(HDC hdc, int fnMapMode);
COLORREF __stdcall SetPixel(HDC hdc, int X, int Y, COLORREF crColor);
int __stdcall SetTextCharacterExtra(HDC hdc, int nCharExtra);
COLORREF __stdcall SetTextColor(HDC hdc, COLORREF crColor);
BOOL __stdcall StretchBlt(HDC hdcDest, int nXOriginDest, int nYOriginDest,
    int nWidthDest, int nHeightDest, HDC hdcSrc, int nXOriginSrc,
    int nYOriginSrc, int nWidthSrc, int nHeightSrc, DWORD dwRop);

/*
 * ADVAPI32.dll: the registry, as the SDK's winreg.h declares it. Every
 * one is APIENTRY, which is __stdcall.
 */
typedef HKEY *PHKEY;
typedef DWORD REGSAM;

LONG __stdcall RegCloseKey(HKEY hKey);
LONG __stdcall RegCreateKeyA(HKEY hKey, LPCSTR lpSubKey, PHKEY phkResult);
LONG __stdcall RegOpenKeyExA(HKEY hKey, LPCSTR lpSubKey, DWORD ulOptions,
    REGSAM samDesired, PHKEY phkResult);
LONG __stdcall RegQueryValueExA(HKEY hKey, LPCSTR lpValueName,
    LPDWORD lpReserved, LPDWORD lpType, LPBYTE lpData, LPDWORD lpcbData);
LONG __stdcall RegSetValueExA(HKEY hKey, LPCSTR lpValueName, DWORD Reserved,
    DWORD dwType, const BYTE *lpData, DWORD cbData);

/*
 * WIN32.dll: winmm's functions, as the SDK's mmsystem.h declares them.
 * The digital release renamed the import, which the 1997 game made from
 * WINMM.dll; the rebuilt code imports them from WINMM.dll, by the 1997
 * name. Every one is WINAPI, __stdcall.
 */
typedef UINT MMRESULT;
typedef DWORD MCIERROR;
typedef UINT MCIDEVICEID;
typedef HANDLE HWAVEIN;
typedef struct tagAUXCAPSA AUXCAPSA, *LPAUXCAPSA;
typedef struct tagJOYCAPSA JOYCAPSA, *LPJOYCAPSA;
typedef struct joyinfoex_tag JOYINFOEX, *LPJOYINFOEX;
typedef struct wavehdr_tag WAVEHDR, *LPWAVEHDR;

MMRESULT __stdcall auxGetDevCapsA(UINT uDeviceID, LPAUXCAPSA pac, UINT cbac);
UINT __stdcall auxGetNumDevs(void);
MMRESULT __stdcall auxGetVolume(UINT uDeviceID, LPDWORD pdwVolume);
MMRESULT __stdcall auxSetVolume(UINT uDeviceID, DWORD dwVolume);
MMRESULT __stdcall joyGetDevCapsA(UINT uJoyID, LPJOYCAPSA pjc, UINT cbjc);
UINT __stdcall joyGetNumDevs(void);
MMRESULT __stdcall joyGetPosEx(UINT uJoyID, LPJOYINFOEX pji);
MCIERROR __stdcall mciSendCommandA(MCIDEVICEID mciId, UINT uMsg,
    DWORD dwParam1, DWORD dwParam2);
DWORD __stdcall timeGetTime(void);
MMRESULT __stdcall waveInAddBuffer(HWAVEIN hwi, LPWAVEHDR pwh, UINT cbwh);
MMRESULT __stdcall waveInClose(HWAVEIN hwi);
MMRESULT __stdcall waveInReset(HWAVEIN hwi);
MMRESULT __stdcall waveInStart(HWAVEIN hwi);
MMRESULT __stdcall waveInStop(HWAVEIN hwi);

/*
 * MSACM32.dll: the audio compression manager's streams, as the SDK's
 * msacm.h declares them. Every one is ACMAPI, which is __stdcall.
 */
typedef HANDLE HACMSTREAM;
typedef struct tACMSTREAMHEADER ACMSTREAMHEADER, *LPACMSTREAMHEADER;

MMRESULT __stdcall acmStreamClose(HACMSTREAM has, DWORD fdwClose);
MMRESULT __stdcall acmStreamConvert(HACMSTREAM has, LPACMSTREAMHEADER pash,
    DWORD fdwConvert);
MMRESULT __stdcall acmStreamPrepareHeader(HACMSTREAM has,
    LPACMSTREAMHEADER pash, DWORD fdwPrepare);
MMRESULT __stdcall acmStreamSize(HACMSTREAM has, DWORD cbInput,
    LPDWORD pdwOutputBytes, DWORD fdwSize);
MMRESULT __stdcall acmStreamUnprepareHeader(HACMSTREAM has,
    LPACMSTREAMHEADER pash, DWORD fdwUnprepare);

/*
 * DDRAW.dll: DirectDraw, as the SDK's ddraw.h declares it, WINAPI,
 * __stdcall. The object it creates is called through its table of
 * methods, which is not laid out yet.
 */
typedef struct _GUID GUID, *LPGUID;
typedef struct IUnknown IUnknown, *LPUNKNOWN;
typedef struct IDirectDraw *LPDIRECTDRAW;

HRESULT __stdcall DirectDrawCreate(GUID *lpGUID, LPDIRECTDRAW *lplpDD,
    IUnknown *pUnkOuter);

/*
 * DINPUT.dll: DirectInput, as the SDK's dinput.h declares it, WINAPI,
 * __stdcall. The object it creates is not laid out yet.
 */
typedef struct IDirectInputA *LPDIRECTINPUTA;

HRESULT __stdcall DirectInputCreateA(HINSTANCE hinst, DWORD dwVersion,
    LPDIRECTINPUTA *lplpDirectInput, LPUNKNOWN punkOuter);

/*
 * DSOUND.dll: DirectSound, as the SDK's dsound.h declares it, WINAPI,
 * __stdcall. The object it creates is not laid out yet.
 */
typedef struct IDirectSound *LPDIRECTSOUND;

HRESULT __stdcall DirectSoundCreate(LPGUID lpGuid, LPDIRECTSOUND *ppDS,
    LPUNKNOWN pUnkOuter);

/*
 * DPLAYX.dll: DirectPlay, imported by ordinal, not by name. Ordinal 1 is
 * DirectPlayCreate, as the SDK's dplay.h declares it, and ordinal 4 is
 * DirectPlayLobbyCreateA, as dplobby.h declares it: the names Wine's
 * dplayx.dll exports at those ordinals. Both are WINAPI, __stdcall. The
 * objects they create are not laid out yet.
 */
typedef struct IDirectPlay *LPDIRECTPLAY;
typedef struct IDirectPlayLobby *LPDIRECTPLAYLOBBYA;

HRESULT __stdcall DirectPlayCreate(LPGUID lpGUID, LPDIRECTPLAY *lplpDP,
    IUnknown *pUnk);
HRESULT __stdcall DirectPlayLobbyCreateA(LPGUID lpguidSP,
    LPDIRECTPLAYLOBBYA *lplpDPL, IUnknown *lpUnk, LPVOID lpData,
    DWORD dwDataSize);

/*
 * smackw32.dll: RAD Game Tools' Smacker video player, imported by Balance
 * of Power alone. Each import's decorated name states its convention and
 * its argument bytes (_SmackOpen@12: __stdcall, 12 bytes), and each
 * declaration keeps both. The parameter and return types follow RAD's
 * smack.h; they are not yet checked against a copy of it, and no SDK
 * declares them. Smack, the player's state, is not laid out yet.
 */
typedef struct Smack Smack;

void __stdcall SmackClose(Smack *smk);
DWORD __stdcall SmackDoFrame(Smack *smk);
void __stdcall SmackNextFrame(Smack *smk);
Smack *__stdcall SmackOpen(LPCSTR name, DWORD flags, DWORD extrabuf);
BYTE __stdcall SmackSoundUseDirectSound(LPVOID dd);
void __stdcall SmackToBuffer(Smack *smk, DWORD left, DWORD top, DWORD Pitch,
    DWORD destheight, LPCVOID buf, DWORD Flags);
DWORD __stdcall SmackToBufferRect(Smack *smk, DWORD SmackSurface);
DWORD __stdcall SmackWait(Smack *smk);

#endif /* XVT_H */
