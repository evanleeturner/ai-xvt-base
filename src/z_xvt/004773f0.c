/* SPDX-License-Identifier: MIT */
/* z_xvt 004773f0 */
extern int DAT_005f261c;
extern int DAT_005f1d44;
extern int DAT_0052ae50;
extern int DAT_005f1d7c;
extern int DAT_005f1d80;
extern int DAT_005f1d38;
extern int DAT_005f2630;
extern int DAT_005f1d58;
extern int DAT_005f2614;
extern int DAT_005f1d5c;
extern int DAT_005f2624;
extern int DAT_005f2620;
extern int DAT_005f1d74;
extern int DAT_005f2638;
extern int DAT_005f2634;

void __stdcall FUN_004773f0(void)
{
  int iVar1;
  unsigned int uVar2;
  int iVar3;
  unsigned short *puVar4;
  int iVar5;
  
  iVar5 = DAT_005f261c + 0x1000;
  iVar3 = DAT_005f1d44 + DAT_0052ae50;
  iVar1 = DAT_005f1d7c + DAT_005f1d80;
  for (puVar4 = (unsigned short *)(iVar3 + DAT_005f1d80 * 2); 
       puVar4 < (unsigned short *)(iVar3 + iVar1 * 2); 
       puVar4 = puVar4 + 1) {
    uVar2 = DAT_005f1d38 + DAT_005f2630;
    DAT_005f1d38 = uVar2 & 0xff;
    *puVar4 = *(unsigned short *)(iVar5 + ((unsigned int)*(unsigned char *)((((DAT_005f1d58 >> 8) << (DAT_005f2614 & 0x1f)) + (DAT_005f1d5c >> 8) & DAT_005f2624) + DAT_005f2620) + (uVar2 >> 8 & 0xf) * 0x100) * 2);
    DAT_005f2630 = DAT_005f2630 + DAT_005f1d74;
    DAT_005f1d5c = DAT_005f1d5c + DAT_005f2638;
    DAT_005f1d58 = DAT_005f1d58 + DAT_005f2634;
  }
  return;
}
