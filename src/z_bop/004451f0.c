/* SPDX-License-Identifier: MIT */
/* z_bop 004451f0 */
extern void * DAT_0055693d;
extern int DAT_00556930;
extern void * DAT_00556928;
extern void * DAT_0055696d;
extern void * DAT_00556959;

void __cdecl FUN_00444f00(uint param_1);
void * __cdecl _malloc(int size);
void __cdecl FUN_004445e0(void *param_1);
void __cdecl _free(void *ptr);
void __stdcall FUN_004438b0(void);
void __cdecl _exit(int code);

void __cdecl FUN_004451f0(uint param_1, undefined4 param_2, byte *param_3)
{
  byte bVar1;
  byte bVar2;
  void *pvVar3;
  byte *pbVar4;
  int iVar5;
  
  FUN_00444f00(param_1);
  DAT_0055693d = _malloc(DAT_00556930 * 9);
  if (DAT_0055693d != (void *)0x0) {
    iVar5 = 0;
    DAT_00556930 = 0;
    FUN_004445e0(DAT_00556928);
    if (0 < (int)param_1) {
      do {
        pbVar4 = (byte *)((int)DAT_0055693d + iVar5);
        iVar5 = iVar5 + 9;
        bVar1 = pbVar4[1];
        bVar2 = pbVar4[2];
        *param_3 = *pbVar4 >> 2;
        param_1 = param_1 - 1;
        param_3[1] = (byte)(bVar1 >> 2);
        param_3[2] = (byte)(bVar2 >> 2);
        param_3 = param_3 + 3;
      } while (param_1 != 0);
    }
    _free(DAT_0055693d);
    do {
      pvVar3 = *(void **)((int)DAT_0055696d + 0x29000);
      _free(DAT_0055696d);
      DAT_0055696d = pvVar3;
    } while (pvVar3 != (void *)0x0);
    DAT_00556959 = (void *)((int)DAT_00556959 + -0x3fc);
    _free(DAT_00556959);
    return;
  }
  FUN_004438b0();
  _exit(1);
}
