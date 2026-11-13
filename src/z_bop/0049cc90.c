/* SPDX-License-Identifier: MIT */
/* z_bop 0049cc90 */
extern unsigned int in_EAX;
extern unsigned short DAT_00999408;
extern unsigned int DAT_009a8da0;
extern unsigned short DAT_00999402;
extern unsigned int DAT_00a07bd0;
extern unsigned short DAT_00999400;
extern unsigned int DAT_009a8e18;
extern unsigned short DAT_00999404;
extern unsigned int DAT_009d6820;
extern unsigned short DAT_0099940a;
extern unsigned int DAT_009a1fe8;
extern unsigned short DAT_00612b94;
extern unsigned int DAT_009ec5f8;
extern unsigned short DAT_00622ba0;
extern unsigned int DAT_009a7800;
extern unsigned short DAT_00622ba8;
extern unsigned int DAT_00a07cc0;
extern unsigned short DAT_00622bac;
extern unsigned int DAT_009e965c;
extern unsigned char DAT_009d80c8;
extern unsigned int DAT_009d7674;
extern unsigned short DAT_00612b98;
extern unsigned int DAT_009a8c34;
extern unsigned short DAT_00622ba4;
extern unsigned int DAT_009a806c;
extern unsigned short DAT_00999406;
extern unsigned int DAT_009a8074;
extern unsigned int _DAT_009d8c1c;

extern unsigned int __cdecl FUN_004ac720(unsigned int);
extern void __cdecl FUN_0045a0c0(void);
extern unsigned int __cdecl FUN_004248b0(unsigned int);
extern void __cdecl FUN_00426c50(unsigned int);

void __stdcall FUN_0049cc90(void)
{
  unsigned int uVar1;
  unsigned int uVar2;
  unsigned int uVar3;
  unsigned int ret_004248b0;

  uVar1 = (in_EAX & 0xFFFF0000) | DAT_00999408;
  if (DAT_00999408 != 0) {
    uVar1 = FUN_004ac720(uVar1);
    DAT_009a8da0 = uVar1;
  }
  uVar1 = (uVar1 & 0xFFFF0000) | DAT_00999402;
  if (DAT_00999402 != 0) {
    uVar1 = FUN_004ac720(uVar1);
    DAT_00a07bd0 = uVar1;
  }
  uVar1 = (uVar1 & 0xFFFF0000) | DAT_00999400;
  if (DAT_00999400 != 0) {
    uVar1 = FUN_004ac720(uVar1);
    DAT_009a8e18 = uVar1;
  }
  uVar1 = (uVar1 & 0xFFFF0000) | DAT_00999404;
  if (DAT_00999404 != 0) {
    uVar1 = FUN_004ac720(uVar1);
    DAT_009d6820 = uVar1;
  }
  if (DAT_0099940a != 0) {
    DAT_009a1fe8 = FUN_004ac720((uVar1 & 0xFFFF0000) | DAT_0099940a);
    FUN_0045a0c0();
  }
  ret_004248b0 = FUN_004248b0(0);
  DAT_009ec5f8 = FUN_004ac720((ret_004248b0 & 0xFFFF0000) | DAT_00612b94);
  DAT_009a7800 = FUN_004ac720((DAT_009ec5f8 & 0xFFFF0000) | DAT_00622ba0);
  DAT_00a07cc0 = FUN_004ac720((DAT_009a7800 & 0xFFFF0000) | DAT_00622ba8);
  DAT_009e965c = FUN_004ac720((DAT_00a07cc0 & 0xFFFF0000) | DAT_00622bac);
  uVar2 = (DAT_009e965c & 0xFFFFFF00) | DAT_009d80c8;
  uVar3 = DAT_009a7800;
  if (((DAT_009d80c8 == 1) || (uVar3 = DAT_00a07cc0, DAT_009d80c8 == 2)) ||
      (uVar3 = DAT_009e965c, DAT_009d80c8 == 0)) {
    uVar2 = uVar3;
    DAT_009d7674 = uVar3;
  }
  DAT_009a8c34 = FUN_004ac720((uVar2 & 0xFFFF0000) | DAT_00612b98);
  DAT_009a806c = FUN_004ac720((DAT_009a8c34 & 0xFFFF0000) | DAT_00622ba4);
  FUN_00426c50(DAT_009a8c34);
  DAT_009a8074 = FUN_004ac720((DAT_009a806c & 0xFFFF0000) | DAT_00999406);
  _DAT_009d8c1c = DAT_009a8074;
  return;
}
