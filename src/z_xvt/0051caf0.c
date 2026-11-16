/* SPDX-License-Identifier: MIT */
/* z_xvt 0051caf0 */
extern char DAT_00536704;
extern CRITICAL_SECTION DAT_007427a0;
extern ushort DAT_0074244c;
extern HFILE DAT_006550f4;

void __stdcall EnterCriticalSection(LPCRITICAL_SECTION lpCriticalSection);
void __stdcall LeaveCriticalSection(LPCRITICAL_SECTION lpCriticalSection);
ushort __stdcall FUN_0051ca60(void);
int __cdecl FUN_0048f4f0(uint);
void __stdcall FUN_0048f510(void);
void __cdecl FUN_0051c130(int);
size_t __cdecl _fread(void *dst, size_t size, size_t count, HFILE stream);
void __cdecl _fclose(HFILE stream);

uint __stdcall FUN_0051caf0(void)
{
  void *dst_buf;
  size_t read_count;
  int ptr;
  ushort tmp;
  int val;

  if (DAT_00536704 == 0) {
    return 0;
  }
  tmp = FUN_0051ca60();
  if (tmp == 0) {
    return 0;
  }
  EnterCriticalSection(&DAT_007427a0);
  dst_buf = (void *)FUN_0048f4f0((uint)tmp);
  read_count = _fread(dst_buf, 1, 0x2000, DAT_006550f4);
  ptr = FUN_0048f4f0((uint)(read_count & 0xFFFF0000) | DAT_0074244c);
  __builtin_memcpy(&val, (void *)(ptr + 0x42), 4);
  val += (int)read_count;
  __builtin_memcpy((void *)(ptr + 0x42), &val, 4);
  if (read_count == 0x2000) {
    FUN_0048f510();
  }
  else {
    FUN_0051c130(0);
    *(char *)(ptr + 0x46) = 1;
    tmp = *(ushort *)(ptr + 0x48);
    FUN_0048f510();
    DAT_0074244c = tmp;
    _fclose(DAT_006550f4);
    DAT_006550f4 = 0;
  }
  FUN_0048f510();
  LeaveCriticalSection(&DAT_007427a0);
  return 1;
}
