/* SPDX-License-Identifier: MIT */
/* z_bop 00443740 */
extern int DAT_00521ad0;
extern int DAT_00521ad4;
extern int DAT_00521ad8;
extern void *DAT_00556904;

void __stdcall FUN_00443740(void)
{
  int iVar1;
  char local_100[0x100];
  
  iVar1 = ((int (__stdcall *)(void *, int, char *))(*(void **)DAT_00556904 + 0x24))(DAT_00556904, 0x100, local_100);
  if (iVar1 == 0) {
    if (local_100[0x1E] == '\0' && local_100[0x2A] == '\0') {
      DAT_00521ad4 = 0;
    }
    else {
      DAT_00521ad4 = 1;
    }
    if (local_100[0x11] == '\0' && local_100[0x91] == '\0') {
      DAT_00521ad0 = 0;
    }
    else {
      DAT_00521ad0 = 1;
    }
    if (local_100[0x2C] == '\0' && local_100[0xAC] == '\0') {
      DAT_00521ad8 = 0;
      return;
    }
    DAT_00521ad8 = 1;
  }
  return;
}
