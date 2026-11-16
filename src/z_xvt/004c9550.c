/* SPDX-License-Identifier: MIT */
/* z_xvt 004c9550 */
extern char s_xvterr_txt_00532310[];
extern char DAT_0052a330[];

HFILE _fopen(const char *filename, const char *mode);
char *_fgets(char *buffer, int count, HFILE stream);
int _fclose(HFILE stream);

int __cdecl FUN_004c9550(int param_1, char *param_2)
{
  HFILE _File;
  int iVar1;
  char local_100[0x200];
  char *fgets_ret = 0;

  _File = _fopen(s_xvterr_txt_00532310, DAT_0052a330);
  if (_File == 0) {
    return 0;
  }
  if (-1 < param_1) {
    iVar1 = param_1 + 1;
    do {
      fgets_ret = _fgets(local_100, 0x200, _File);
      iVar1 = iVar1 + -1;
    } while (iVar1 != 0);
  }
  _fclose(_File);
  if (fgets_ret != 0) {
    iVar1 = 0;
    do {
      if ((local_100[iVar1] == '\\') && (local_100[iVar1 + 1] == 'n')) {
        *param_2 = '\n';
        iVar1 = iVar1 + 1;
      }
      else {
        *param_2 = local_100[iVar1];
      }
      param_2 = param_2 + 1;
      iVar1 = iVar1 + 1;
    } while (iVar1 < 0xff);
    return 1;
  }
  return 0;
}
