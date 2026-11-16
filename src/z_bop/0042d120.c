/* SPDX-License-Identifier: MIT */
/* z_bop 0042d120 */
extern int DAT_00a10514;
extern int DAT_00a606d0;
extern int DAT_00a6057c;
extern int DAT_00a60580;
extern int DAT_00a6052c;
extern int DAT_00a60574;
extern int DAT_00a60578;
extern int DAT_00a6056c;
extern int DAT_00a60570;

extern int FUN_0042dda0(char *param_1);
extern void FID_conflict__memcpy(void *dest, const void *src, unsigned int n);
extern char *_strncpy(char *dest, const char *src, unsigned int n);

int __cdecl FUN_0042d120(char *param_1, int param_2, int param_3, int param_4, int param_5, int param_6)
{
  int iVar1;
  int *piVar2;

  if (DAT_00a10514 == 0) {
    return 0;
  }
  if (*param_1 == '\0') {
    return 0;
  }
  iVar1 = FUN_0042dda0(param_1);
  if (iVar1 == -1) {
    return 0;
  }
  iVar1 = 0;
  if (0 < DAT_00a606d0) {
    piVar2 = &DAT_00a6057c;
    do {
      if (*piVar2 < param_4) break;
      piVar2 = piVar2 + 0x15;
      iVar1 = iVar1 + 1;
    } while (iVar1 < DAT_00a606d0);
  }
  if ((DAT_00a606d0 == iVar1) && (iVar1 == 4)) {
    return 0;
  }
  FID_conflict__memcpy((char *)&DAT_00a60580 + iVar1 * 0x54, (char *)&DAT_00a6052c + iVar1 * 0x54, (unsigned int)(DAT_00a606d0 - iVar1) * 0x54);
  _strncpy((char *)&DAT_00a6052c + iVar1 * 0x54, param_1, 0x40);
  ((int *)&DAT_00a60574)[iVar1 * 0x15] = param_3;
  ((int *)&DAT_00a60578)[iVar1 * 0x15] = param_2;
  ((int *)&DAT_00a6057c)[iVar1 * 0x15] = param_4;
  ((int *)&DAT_00a6056c)[iVar1 * 0x15] = param_5;
  ((int *)&DAT_00a60570)[iVar1 * 0x15] = param_6;
  DAT_00a606d0 = DAT_00a606d0 + 1;
  if (4 < DAT_00a606d0) {
    DAT_00a606d0 = 4;
  }
  return 1;
}
