/* SPDX-License-Identifier: MIT */
/* z_bop 00450550 */
extern int DAT_005235d0;
extern int DAT_009993fc;
extern char DAT_0051c4bc;
extern char DAT_005240e0;
extern char *s_msglog_ld_txt_005240e4;
extern char *s__s__ld__ld__ld_005240d0;

int __cdecl _sprintf(char *buffer, const char *format, ...);
void * __cdecl FID_conflict___wfopen(const char *filename, const void *mode);
int __cdecl _fclose(void *stream);
int __cdecl _fprintf(void *stream, const char *format, ...);

void __stdcall FUN_00450550(void)
{
  void *_File;
  char *pcVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  char local_10[16];
  
  iVar3 = 0;
  do {
    _sprintf(local_10, s_msglog_ld_txt_005240e4, iVar3);
    _File = FID_conflict___wfopen(local_10, &DAT_0051c4bc);
    if (_File == (void *)0x0) {
      _File = FID_conflict___wfopen(local_10, &DAT_005240e0);
      break;
    }
    if (iVar3 == 99) {
      if (_File != (void *)0x0) {
        _fclose(_File);
      }
      _File = FID_conflict___wfopen(local_10, &DAT_005240e0);
      break;
    }
    iVar3 = iVar3 + 1;
  } while (iVar3 < 100);
  
  iVar3 = 0;
  if (_File != (void *)0x0) {
    if (DAT_005235d0 != 0) {
      iVar4 = 0;
      do {
        iVar2 = DAT_009993fc + iVar4;
        pcVar1 = (char *)(iVar2 + 0xe);
        if ((((*(char *)(iVar2 + 0xe) < '\t') &&
             (pcVar1 = (char *)(iVar2 + 0xf), *(char *)(iVar2 + 0xe) == '\x01')) && ('/' < *pcVar1))
           && (*pcVar1 < '4')) {
          pcVar1 = (char *)(iVar2 + 0x10);
        }
        iVar4 = iVar4 + 0x54;
        iVar3 = iVar3 + 1;
        _fprintf(_File, s__s__ld__ld__ld_005240d0, pcVar1,
                 (unsigned int)*(unsigned char *)(iVar2 + 8),
                 (unsigned int)*(unsigned char *)(iVar2 + 7),
                 (unsigned int)*(unsigned char *)(iVar2 + 6));
      } while (iVar3 < (int)(unsigned int)DAT_005235d0);
    }
    _fclose(_File);
  }
  return;
}
