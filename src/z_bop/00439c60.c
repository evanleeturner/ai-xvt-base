/* SPDX-License-Identifier: MIT */
/* z_bop 00439c60 */
extern short DAT_00a0a2fe;
extern ushort DAT_009a20ae;
extern char DAT_00a08ca0;
extern char DAT_00a08ca2;
extern char DAT_00a08ca6;
extern char *DAT_00a0a0e4;
extern char *DAT_00a0a0e8;
extern char *DAT_00a0a0e0;
extern char *DAT_00a08c34;
extern char DAT_009a1ed0;

void __stdcall FUN_004419d0(void);
void __cdecl FUN_004a95c0(uint param_1);
void __cdecl FUN_004a9640(char param_1);
void __cdecl FUN_004a9500(ushort param_1, ushort param_2);
void __cdecl FUN_004a9590(uint param_1);
void __cdecl FUN_004a97f0(char *param_1);
void __cdecl FUN_004a9860(uint param_1);
undefined8 __cdecl FUN_004278c0(void *param_1);
void __cdecl FUN_004a9520(short param_1, short param_2, short param_3, short param_4);
void __cdecl FUN_004a9960(void *param_1);
void __stdcall FUN_0043f390(void);
void __stdcall FUN_00440140(void);

void __stdcall FUN_00439c60(void)
{
  int iVar1;
  short sVar2;
  short sVar3;
  short sVar4;
  char *pcVar5;
  ushort uVar6;
  uint uVar7;
  undefined8 uVar8;
  
  if (DAT_00a0a2fe == -2) {
    FUN_004419d0();
    uVar7 = 0x8b;
    FUN_004a95c0(0x2c);
    FUN_004a9640('\x02');
    do {
      iVar1 = uVar7 * 0xc;
      FUN_004a9500(*(ushort *)(&DAT_00a08ca0 + iVar1), *(ushort *)(&DAT_00a08ca2 + iVar1));
      FUN_004a9590((uint)*(ushort *)(&DAT_00a08ca6 + iVar1));
      switch(uVar7) {
      case 0x8b:
        pcVar5 = DAT_00a0a0e4;
        break;
      case 0x8c:
        pcVar5 = DAT_00a0a0e8;
        break;
      case 0x8d:
        pcVar5 = DAT_00a0a0e0;
        break;
      case 0x8e:
        pcVar5 = DAT_00a08c34;
        break;
      default:
        goto switchD_00439cda_default;
      }
      FUN_004a97f0(pcVar5);
switchD_00439cda_default:
      uVar6 = (short)uVar7 + 1;
      uVar7 = (uint)uVar6;
      FUN_004a9860(0x3a);
      sVar2 = *(short *)(&DAT_00a08ca2 + iVar1);
      sVar3 = *(short *)(&DAT_00a08ca0 + iVar1);
      sVar4 = (ushort)DAT_009a20ae + sVar2;
      uVar8 = FUN_004278c0(&DAT_009a1ed0);
      FUN_004a9520(sVar3, sVar2, (short)uVar8 + sVar3, sVar4);
      FUN_004a9960(&DAT_009a1ed0);
    } while (uVar6 < 0x8f);
    DAT_00a0a2fe = 0;
  }
  FUN_0043f390();
  FUN_00440140();
  return;
}
