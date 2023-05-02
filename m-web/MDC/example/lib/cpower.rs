2-Jul-96, 6:54:35
Routine Save for Complex Mathematics Library function $%CPOWER^MATH
 ;
 ; Unless otherwise noted, the code below
 ; was approved in document X11/95-14
 ;
 ; If corrections have been applied,
 ; first the original line appears,
 ; with three semicolons at the beginning of the line.
 ;
 ; Then the source of the correction is acknowledged,
 ; then the corrected line appears, followed by a
 ; line containing three semicolons.
 ;
CPOWER(Z,N,PREC) ;
 New AR,NIM,NRE,PHI,PI,R,RHO,TH,ZIM,ZRE
 ;
 ;;;
 ; Alan Frank (October 1995)
 Set PREC=$Get(PREC,11)
 ;;;
 ;
 Set ZRE=+Z,ZIM=+$Piece(Z,"%",2)
 Set NRE=+N,NIM=+$Piece(N,"%",2)
 If 'ZRE,'ZIM,'NRE,'NIM Set $Ecode=",M28,"
 ;
 ;;; If 'ZRE,'ZIM Quit "0%0%
 ; Eli Reidler (28 June 1996)
 If 'ZRE,'ZIM Quit "0%0"
 ;;;
 ;
 Set PI=$%PI^MATH()
 ;
 ;;; Set R=$%SQRT^MATH(ZRE*ZRE+(ZIM*ZIM,PREC))
 ; Winfried Gerum (8 June 1995)
 ; Eli Reidler (28 June 1996)
 Set R=$%SQRT^MATH(ZRE*ZRE+(ZIM*ZIM),PREC)
 ;;;
 ;
 ;
 ;;; If ZRE Set TH=$%ARCTAN^MATH(ZIM,ZRE,PREC)
 ; Alan Frank (October 1995)
 If ZRE Set TH=$%ARCTAN^MATH(ZIM/ZRE,PREC)
 ;;;
 ;
 ;;; Else  Set TH=$SELECT(ZRE>0:PI/2,1:-PI/2)
 ; Winfried Gerum (8 June 1995)
 Else  Set TH=$SELECT(ZIM>0:PI/2,1:-PI/2)
 ;;;
 ;
 Set RHO=$%LOG^MATH(R,PREC)
 Set AR=$%EXP^MATH(RHO*NRE-(TH*NIM),PREC)
 Set PHI=RHO*NIM+(NRE*TH)
 Quit AR*$%COS^MATH(PHI,PREC)_"%"_(AR*$%SIN^MATH(PHI,PREC))
 ;===
 ;
 ;
 ;


