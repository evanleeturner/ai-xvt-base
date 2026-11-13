/* SPDX-License-Identifier: MIT */
/* z_bop 004d3600 */
extern char *DAT_00b69cac;
extern uint DAT_00b69cbc;
extern uint DAT_00b69cb0;
extern uint DAT_00b69cb8;
extern uint DAT_00b69ca8;
extern uint DAT_00b69cb4;
extern void *DAT_00b6a2ac;
extern void *DAT_00b69cd0;
extern void *DAT_00b6a248;
extern uint DAT_00b69e34;
extern void *DAT_00b69cd8;
extern uint DAT_00b69cc8;
extern uint DAT_0052ba5c;

extern char *s_nofrontflip_0052ba80;
extern char *s_nopageflip_00527fcc;
extern char *s_nofullscreen_00527fe4;
extern char *s_skipintro_0052ba74;
extern char *s_ishost_0052ba6c;
extern char *s_isclient_0052ba60;

extern char *_strstr(const char *, const char *);
extern int FUN_004d5b90(void);
extern void _exit(int);
extern void FUN_004d4770(HINSTANCE);
extern void _free(void *);
extern void FUN_004bf650(int);

bool __cdecl FUN_004d3600(HINSTANCE param_1, undefined4 param_2, char *param_3)
{
  char *pcVar1;
  int iVar2;
  
  DAT_00b69cac = param_3;
  pcVar1 = _strstr(param_3, s_nofrontflip_0052ba80);
  DAT_00b69cbc = (uint)(pcVar1 != (char *)0x0);
  pcVar1 = _strstr(param_3, s_nopageflip_00527fcc);
  if ((pcVar1 == (char *)0x0) &&
     (pcVar1 = _strstr(param_3, s_nofullscreen_00527fe4), pcVar1 == (char *)0x0)) {
    DAT_00b69cb0 = 0;
  }
  else {
    DAT_00b69cb0 = 1;
  }
  pcVar1 = _strstr(param_3, s_skipintro_0052ba74);
  DAT_00b69cb8 = (uint)(pcVar1 != (char *)0x0);
  pcVar1 = _strstr(param_3, s_ishost_0052ba6c);
  DAT_00b69ca8 = (uint)(pcVar1 != (char *)0x0);
  pcVar1 = _strstr(param_3, s_isclient_0052ba60);
  DAT_00b69cb4 = (uint)(pcVar1 != (char *)0x0);
  iVar2 = FUN_004d5b90();
  if (iVar2 == 0) {
    FUN_004d4770(param_1);
    if (DAT_00b6a2ac != (void *)0x0) {
      _free(DAT_00b6a2ac);
      DAT_00b6a2ac = (void *)0x0;
    }
    if (DAT_00b69cd0 != (void *)0x0) {
      _free(DAT_00b69cd0);
      DAT_00b69cd0 = (void *)0x0;
    }
    if (DAT_00b6a248 != (void *)0x0) {
      _free(DAT_00b6a248);
      DAT_00b6a248 = (void *)0x0;
      DAT_00b69e34 = 0;
    }
    if (DAT_00b69cd8 != (void *)0x0) {
      _free(DAT_00b69cd8);
      DAT_00b69cd8 = (void *)0x0;
      DAT_00b69cc8 = 0;
    }
    FUN_004bf650(0);
    return DAT_0052ba5c == 0;
  }
  _exit(0);
  return 0;
}
